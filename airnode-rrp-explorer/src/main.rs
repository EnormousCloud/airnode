pub mod entry;
pub mod footer;
pub mod header;
pub mod reader;
pub mod results;
pub mod components;

use std::rc::Rc;
use yew::{html, Component, Context, Html};
use web3::types::Log;
use crate::entry::Entry;
use crate::reader::{BlockBatch, Scanner};
// use wasm_bindgen_futures::JsFuture;

#[derive(Debug, Clone)]
enum Msg {
    Reset,
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
    Done
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
            },
            Msg::Completed => {
                self.mode = Mode::Done;
            },
            Msg::Started(chain_id, batch) => {
                self.mode = Mode::InProgress;
                self.batch = Some(batch);
                self.chain_id = chain_id;
            },
            Msg::Reset => {
                self.mode = Mode::Input;
                self.logs = vec![];
                self.batch = None;
                self.chain_id = 0;
            },
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
                            // let w3 = web3::Web3::new(transport);
                            // let rs = Scanner::new(
                            //     &w3,
                            //     input.min_block, 
                            //     input.max_block, 
                            //     input.batch_size,
                            // ).await;
                            // let mut s = match rs {
                            //     Ok(s) => s,
                            //     Err(e) => { return Msg::Fail(format!("{}", e)); }
                            // };
                            // let contract = input.address;
                            // let chain_id = s.chain_id;
                            // let rc: Rc<Scanner<web3::transports::Http>> = Rc::new(s);
                            // loop {
                            //     let s = rc.clone();
                            //     if let Some(batch) = s.current() {
                            //         link.send_message(Msg::Started(chain_id, batch.clone()));
                            //         link.clone().send_future(async move {
                            //             let s = rc.clone();
                            //             match s.as_ref().pick(contract.clone()).await {
                            //                 Ok(x) => {
                            //                     Msg::BatchDone(x.unwrap())
                            //                 },
                            //                 Err(e) => {
                            //                     Msg::Fail(format!("{}", e))
                            //                 },
                            //             }
                            //         });
                            //     } else {
                            //         break;
                            //     }
                            // }
                            return Msg::Completed;
                        },
                        Err(e) => {
                            return Msg::Fail(format!("{}", e))
                        }
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
