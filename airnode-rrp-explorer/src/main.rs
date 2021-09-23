pub mod components;
pub mod entry;
pub mod footer;
pub mod header;
pub mod reader;
pub mod results;

use crate::entry::Entry;
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
    logs: Vec<Log>,
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
                    self.logs.push(l.clone());
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
                                    return Msg::Fail(format!("{}", e));
                                }
                            };
                            let contract = input.address;
                            let chain_id = s.chain_id;
                            let batches = s.batches.clone();
                            let rc: Rc<Scanner<web3::transports::Http>> = Rc::new(s);
                            batches.iter().enumerate().for_each(|(i, batch)| {
                                let s = rc.clone();
                                link.send_message(Msg::Started(chain_id, batch.clone()));
                                link.clone().send_future(async move {
                                    match s.query(&contract, i).await {
                                        Ok(x) => Msg::BatchDone(x.unwrap().clone()),
                                        Err(e) => Msg::Fail(format!("{}", e)),
                                    }
                                });
                            });
                            return Msg::Completed;
                        }
                        Err(e) => return Msg::Fail(format!("{}", e)),
                    };
                });
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <header::Header />
                {match &self.mode {
                    Mode::Input => html!{
                        <entry::EntryForm on_submit={ctx.link().callback(Msg::Submit)} />
                    },
                    Mode::Connecting => html!{
                        <div style="max-width: 480px; text-align: center;"> {"Connecting to RPC..."} </div>
                    },
                    Mode::InProgress => html!{
                        <>
                            <div style="max-width: 480px; text-align: center;">
                                {format!("PROCESSING {} (chain_id: {})", self.batch.clone().unwrap().status(), self.chain_id)}
                            </div>
                            <results::ResultsView input={self.logs.clone()} />
                        </>
                    },
                    Mode::Failure(e) => html!{
                        <>
                            <entry::EntryForm on_submit={ctx.link().callback(Msg::Submit)} />
                            <div style="max-width: 480px; text-align: center;"> {"ERROR: "} {e.clone()} </div>
                        </>
                    },
                    Mode::Done => html!{ <results::ResultsView input={self.logs.clone()} /> },
                }}
                <footer::Footer />
            </div>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
