pub mod components;
pub mod entry;
pub mod footer;
pub mod input;
pub mod logevent;
pub mod reader;
pub mod results;

use crate::entry::Entry;
use crate::logevent::LogEvent;
use crate::reader::{BlockBatch, Scanner};
use std::rc::Rc;
use web3::types::Log;
use yew::{html, Component, Context, Html};

#[derive(Debug, Clone)]
enum Msg {
    Submit(Entry),
    Started(u64, BlockBatch),
    BatchDone(Vec<Log>),
    Completed,
    Fail(String),
    Back,
    NOP, //no operation
}

#[derive(Debug, Clone)]
enum Mode {
    Input,
    Connecting,
    InProgress,
    Failure(String),
    Done,
}

struct App {
    mode: Mode,
    chain_id: u64,
    batch: Option<BlockBatch>,
    logs: Vec<LogEvent>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {
            mode: Mode::Input,
            chain_id: 0,
            batch: None,
            logs: vec![],
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::NOP => {} // no operation
            Msg::Back => {
                self.mode = Mode::Input;
                self.batch = None;
                self.chain_id = 0;
                self.logs = vec![];
            }
            Msg::Fail(s) => {
                self.mode = Mode::Failure(s);
            }
            Msg::Completed => {
                self.mode = Mode::Done;
            }
            Msg::Started(chain_id, batch) => {
                self.mode = Mode::InProgress;
                self.batch = Some(batch);
                self.chain_id = chain_id;
            }
            Msg::BatchDone(logs) => {
                for l in logs {
                    self.logs.push(LogEvent::new(l));
                }
            }
            Msg::Submit(input) => {
                self.mode = Mode::Connecting;
                let link = ctx.link().clone();
                ctx.link().send_future(async move {
                    match web3::transports::Http::new(&input.network) {
                        Ok(transport) => {
                            // There can be a failure on the block number height detection
                            let w3 = web3::Web3::new(transport);
                            let rs = Scanner::new(
                                &w3,
                                input.min_block,
                                input.max_block,
                                input.batch_size,
                            )
                            .await;
                            let s = match rs {
                                Ok(s) => s,
                                Err(e) => {
                                    return Msg::Fail(
                                        format!("Connection {}", e).replace("\n", " "),
                                    );
                                }
                            };
                            let contract = input.address;
                            let chain_id = s.chain_id;
                            let batches = s.batches.clone();
                            let num_batches = batches.len();
                            let rc: Rc<Scanner<web3::transports::Http>> = Rc::new(s);
                            batches.iter().enumerate().for_each(|(i, batch)| {
                                let b = batch.clone();
                                // todo: we still get this all at once
                                link.clone()
                                    .send_future(async move { Msg::Started(chain_id, b) });

                                let s = rc.clone();
                                let l = link.clone();
                                link.clone().send_future(async move {
                                    match s.query(&contract, i).await {
                                        Ok(x) => {
                                            if num_batches == i + 1 {
                                                l.send_future(async move { Msg::Completed });
                                            } else {
                                                l.send_future(async move {
                                                    Msg::BatchDone(x.unwrap().clone())
                                                });
                                            }
                                            Msg::NOP
                                        }
                                        Err(e) => Msg::Fail(format!("{:?}", e).replace("\n", " ")),
                                    }
                                });
                            });
                            return Msg::NOP;
                        }
                        Err(e) => return Msg::Fail(format!("{:?}", e).replace("\n", " ")),
                    };
                });
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                {match &self.mode {
                    Mode::Input => html!{
                        <entry::EntryForm
                            on_submit={ctx.link().callback(Msg::Submit)}
                        />
                    },
                    Mode::Connecting => html!{
                        <div class="results-status">
                            <div class="cell-title">{"Airnode RRP Explorer"}</div>
                            <div class="cell-title">{"Connecting to RPC..."}</div>
                        </div>
                    },
                    Mode::InProgress => html!{
                        <results::ResultsView
                            batch={self.batch.clone()}
                            chain_id={self.chain_id}
                            on_back={ctx.link().callback(|_| Msg::Back)}
                            input={self.logs.clone()} />
                    },
                    Mode::Failure(e) => html!{
                        <>
                            <entry::EntryForm on_submit={ctx.link().callback(Msg::Submit)} />
                            <div class="results-error"> {"ERROR: "} {e.clone()} </div>
                        </>
                    },
                    Mode::Done => html!{
                        <results::ResultsView
                            batch={self.batch.clone()}
                            chain_id={self.chain_id}
                            on_back={ctx.link().callback(|_| Msg::Back)}
                            input={self.logs.clone()}
                        />
                    },
                }}
                <div class="pre-footer"></div>
                <footer::Footer />
            </div>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
