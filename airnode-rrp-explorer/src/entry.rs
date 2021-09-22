use gloo::console as console;
use web3::types::H160;
use yew::web_sys::{Event, HtmlInputElement};
use yew::{html, Component, Context, Html, TargetCast};
use crate::components::panel::Panel;

#[derive(Debug, Clone)]
pub struct Entry {
    pub network: String,
    pub address: H160,
    pub min_block: u64,
    pub max_block: Option<u64>,
    pub batch_size: u64,
}

#[derive(Debug)]
pub enum Msg {
    UpdateNetwork(String),
    UpdateMinBlock(String),
    UpdateMaxBlock(String),
    UpdateBatchSize(String),
}

pub struct EntryForm {
    pub network: String,
    pub min_block: u64,
    pub max_block: Option<u64>,
    pub batch_size: u64,
}

impl Component for EntryForm {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            network: "http://localhost:8545/".to_owned(),
            min_block: 0,
            max_block: None,
            batch_size: 1000,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateNetwork(s) => {
                self.network = s.clone();
                true // re-render
            }
            Msg::UpdateMinBlock(s) => {
                console::log!("UpdateMinBlock {}", s);
                false // re-render
            }
            Msg::UpdateMaxBlock(s) => {
                console::log!("UpdateMaxBlock {}", s);
                false // re-render
            }
            Msg::UpdateBatchSize(s) => {
                console::log!("UpdateBatchSize {}", s);
                false // re-render
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let update_name = |f: fn(String) -> Msg| {
            link.callback(move |e: Event| {
                let input: HtmlInputElement = e.target_unchecked_into();
                // info!("input {}", input.value());
                f(input.value())
            })
        };
        let update_u64 = |f: fn(String) -> Msg| {
            link.callback(move |e: Event| {
                let input: HtmlInputElement = e.target_unchecked_into();
                // info!("input {}", input.value());
                f(input.value())
            })
        };
        let form = html! {
            <div style="text-align: center">
                <div class="dash-row" style="margin-bottom: 20px;">
                    <div class="dash-col-100">
                        <label>
                            <h3 class="cell-title">{ "Network RPC URL:" }</h3>
                            <input
                                style="width: 60%; max-width: 400px; text-align: center;"
                                placeholder="Network RPC URL"
                                value={self.network.clone()}
                                onchange={update_name(Msg::UpdateNetwork)}
                            />
                        </label>
                    </div>
                </div>
                
                <div class="dash-row" style="margin-bottom: 20px;">
                    <div class="cell dash-col-3">
                        <h3 class="cell-title">{ "Min Block:" }</h3>
                        <input
                            style="width: 120px; text-align: center"
                            placeholder=""
                            value={format!("{}", self.min_block)}
                            onchange={update_u64(Msg::UpdateMinBlock)}
                        />
                    </div>
                    <div class="cell dash-col-3">
                        <h3 class="cell-title">{ "Max Block:" }</h3>
                        <input
                            style="width: 120px; text-align: center"
                            placeholder="HEAD"
                            value={
                                match self.max_block {
                                    Some(x) => format!("{}", x),
                                    None => "".to_owned()
                                }
                            }
                            onchange={update_u64(Msg::UpdateMaxBlock)}
                        />
                    </div>
                    <div class="cell dash-col-3">
                        <h3 class="cell-title">{ "Batch Size:" }</h3>
                        <input
                            style="width: 120px; text-align: center"
                            placeholder=""
                            value={format!("{}", self.batch_size)}
                            onchange={update_u64(Msg::UpdateBatchSize)}
                        />
                    </div>
                </div>
                <div class="dash-row" style="margin-bottom: 20px;">
                    <div class="dash-col-3">
                        <div class="button-wrapper primary" style="width: 60%; margin: 0px auto; max-width: 240px; text-align: center;">
                            <button
                                class="button primary"
                                style="width: 100%; text-align: center;"
                                onchange={update_name(Msg::UpdateNetwork)}
                            >
                                {"Scan JSON+RPC"}
                            </button>
                            <div class="underline"></div>
                        </div>
                    </div>
                </div>
            </div>
        };

        html!{
            <div>
                <Panel title="Airnode RRP Explorer" content={form} />
                <p>{ self.network.clone() }</p>
            </div>
        }
    }
}
