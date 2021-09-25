use crate::reader::BlockBatch;
use airnode_events::AirnodeEvent;
use web3::types::Log;
use yew::{html, Callback, Component, Context, Html, Properties};

#[derive(Debug)]
pub enum Msg {
    GoBack,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub input: Vec<Log>,
    pub chain_id: u64,
    pub batch: Option<BlockBatch>,
    pub on_back: Callback<()>,
}

pub struct ResultsView;

impl ResultsView {
    pub fn tx_link(&self, chain_id: u64, log: &web3::types::Log) -> Html {
        let tx = format!("{:?}", log.transaction_hash.unwrap());
        let href = match chain_id {
            1 => Some(format!("https://etherscan.io/tx/{}#eventlog", tx)),
            3 => Some(format!("https://ropsten.etherscan.io/tx/{}#eventlog", tx)),
            4 => Some(format!("https://rinkeby.etherscan.io/tx/{}#eventlog", tx)),
            5 => Some(format!("https://goerli.etherscan.io/tx/{}#eventlog", tx)),
            41 => Some(format!("https://kovan.etherscan.io/tx/{}#eventlog", tx)),
            _ => None,
        };
        html! {
            <div>
                {log.block_number.unwrap()}
                {" "}
                {
                    match href {
                        Some(href) => html! {
                            <a
                                href={href}
                                target="_blank"
                                rel="noindex nofollow noopener noreferrer"
                            >
                                {tx}
                            </a>
                        },
                        None => html! { <span>{tx}</span> }
                    }
                }
            </div>
        }
    }
}

impl Component for ResultsView {
    type Message = Msg;
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GoBack => {
                ctx.props().on_back.emit(());
            }
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let chain_id = ctx.props().chain_id;
        html! {
            <div>
                <div class="top-static">
                    <div class="panel-flex">
                        <div class="button-wrapper primary" style="width: 200px;">
                        <input
                                type="submit"
                                class="button primary" style="width: 100%; text-align: center;"
                                onclick={ctx.link().callback(|_| Msg::GoBack)}
                                value={"Back"}
                            />
                        </div>
                        <div class="cell-title" style="flex:1; text-align: right">
                            <div>{"Airnode RRP Explorer"}</div>
                            <div>{format!("{} events", ctx.props().input.len())}</div>
                        </div>
                    </div>
                </div>
                <div class="panel-results">
                    <ol>
                    {
                        for ctx.props().input.iter().map(|log| {
                            match AirnodeEvent::from_log(&log) {
                                Ok(evt) => html!{
                                    <li>
                                        {self.tx_link(chain_id, &log)}
                                        {serde_json::to_string_pretty(&evt).unwrap() }
                                    </li>
                                },
                                Err(e) => html! {
                                    <li class="err" title={format!("{}", e)}>
                                        {self.tx_link(chain_id, &log)}
                                        { serde_json::to_string(log).unwrap() }
                                    </li>
                                },
                            }
                        })
                    }
                    </ol>
                </div>
            </div>
        }
    }
}
