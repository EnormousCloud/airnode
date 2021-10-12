use crate::components::panel::Panel;
use crate::input::Input;

use gloo::storage::{SessionStorage, Storage};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use web3::types::H160;
use yew::web_sys::{Event, HtmlInputElement, InputEvent};
use yew::{html, Callback, Component, Context, Html, Properties, TargetCast};

/// structure that will be passed to the parent when
#[derive(Debug, Clone, Serialize)]
pub struct Entry {
    pub network: String,
    pub address: H160,
    pub min_block: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_block: Option<u64>,
    pub batch_size: u64,

    // provider ID was only in pre-alpha version of the protocol
    pub by_provider_id: String,
    pub by_endpoint_id: String,
    pub by_request_id: String,
    pub by_requester_index: String,
    // by any address of RRP participants (airnode, sponsor, requester, designatedWallet, clientAddress)
    pub by_address: Option<H160>,
}

impl Default for Entry {
    fn default() -> Self {
        Self {
            network: "http://localhost:8545/".to_owned(),
            address: H160::from_str("0").unwrap(),
            min_block: 0,
            max_block: None,
            batch_size: 10000,
            by_provider_id: String::from(""),
            by_endpoint_id: String::from(""),
            by_request_id: String::from(""),
            by_requester_index: String::from(""),
            by_address: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub on_submit: Callback<Entry>,
}

#[derive(Debug)]
pub enum Msg {
    Submit,
    UpdateNetwork(String),
    UpdateAddress(String),
    UpdateMinBlock(String),
    UpdateMaxBlock(String),
    UpdateBatchSize(String),
}

// state is Entry + whether each field is valid
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryForm {
    pub network: Input<String>,
    pub address: Input<H160>,
    pub min_block: Input<u64>,
    pub max_block: Input<Option<u64>>,
    pub batch_size: Input<u64>,
    pub by_provider_id: Input<String>,
    pub by_endpoint_id: Input<String>,
    pub by_request_id: Input<String>,
    pub by_requester_index: Input<String>,
    pub by_address: Input<Option<H160>>,
}

impl EntryForm {
    const KEY: &'static str = "airnode.rrp.v20211012";

    pub fn load() -> Self {
        SessionStorage::get(Self::KEY).unwrap_or_default()
    }

    pub fn remove() {
        SessionStorage::delete(Self::KEY);
    }

    pub fn store(&self) {
        let _ = SessionStorage::set(Self::KEY, self);
    }
}

impl Default for EntryForm {
    fn default() -> Self {
        Self {
            network: Input::str("http://localhost:8545/"),
            address: Input::no_address(),
            min_block: Input::u64(7812260),
            max_block: Input::opt_u64(),
            batch_size: Input::u64(50000),
            by_provider_id: Input::str(""),
            by_endpoint_id: Input::str(""),
            by_request_id: Input::str(""),
            by_requester_index: Input::str(""),
            by_address: Input::none_address(),
        }
    }
}

impl EntryForm {
    pub fn entry(&self) -> Option<Entry> {
        let network = match self.network.msg {
            Some(_) => return None,
            None => self.network.value.clone(),
        };
        if self.address.value
            == H160::from([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
        {
            return None;
        }
        let address = match self.address.msg {
            Some(_) => return None,
            None => self.address.value,
        };
        let min_block = match self.min_block.msg {
            Some(_) => return None,
            None => self.min_block.value,
        };
        let max_block = match self.max_block.msg {
            Some(_) => return None,
            None => self.max_block.value,
        };
        let batch_size = match self.batch_size.msg {
            Some(_) => return None,
            None => self.batch_size.value,
        };
        let by_provider_id = match self.by_provider_id.msg {
            Some(_) => return None,
            None => self.by_provider_id.value.clone(),
        };
        let by_endpoint_id = match self.by_endpoint_id.msg {
            Some(_) => return None,
            None => self.by_endpoint_id.value.clone(),
        };
        let by_request_id = match self.by_request_id.msg {
            Some(_) => return None,
            None => self.by_request_id.value.clone(),
        };
        let by_requester_index = match self.by_requester_index.msg {
            Some(_) => return None,
            None => self.by_requester_index.value.clone(),
        };
        let by_address = match self.by_address.msg {
            Some(_) => return None,
            None => self.by_address.value,
        };
        Some(Entry {
            network,
            address,
            min_block,
            max_block,
            batch_size,
            by_endpoint_id,
            by_provider_id,
            by_request_id,
            by_requester_index,
            by_address,
        })
    }

    fn check_range(&mut self) {
        if let None = self.min_block.msg {
            if let None = self.max_block.msg {
                if let Some(max_value) = self.max_block.value {
                    if self.min_block.value > max_value {
                        self.min_block.msg = Some("Invalid Range".to_owned());
                        self.max_block.msg = Some("Invalid Range".to_owned());
                    }
                }
            }
        }
    }
}

impl Component for EntryForm {
    type Message = Msg;
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self::load()
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Submit => {
                if let Some(entry) = &self.entry() {
                    ctx.props().on_submit.emit(entry.clone());
                    self.store();
                }
                true
            }
            Msg::UpdateNetwork(s) => self.network.parse_url(&s),
            Msg::UpdateAddress(s) => self.address.parse_address(&s),
            Msg::UpdateMinBlock(s) => {
                self.min_block.parse_u64(&s);
                self.check_range();
                true
            }
            Msg::UpdateMaxBlock(s) => {
                self.max_block.parse_opt_u64(&s);
                self.check_range();
                true
            }
            Msg::UpdateBatchSize(s) => self.batch_size.parse_u64(&s),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let form = html! {
            <form spellcheck="false" autocomplete="on">
                <div style="text-align: center">
                    <div class="dash-row" style="margin-bottom: 20px;">
                        <div class="dash-col-100">
                            <label>
                                <h3 class="cell-title">{ "Network RPC URL:" }</h3>
                                <input
                                    name="jsonrpc"
                                    style="width: 480px; text-align: center;"
                                    placeholder="Network RPC URL"
                                    value={self.network.s.clone()}
                                    oninput={link.callback(move |e: InputEvent| {
                                        Msg::UpdateNetwork(e.target_unchecked_into::<HtmlInputElement>().value())
                                    })}
                                    onchange={link.callback(move |e: Event| {
                                        Msg::UpdateNetwork(e.target_unchecked_into::<HtmlInputElement>().value())
                                    })}
                                />
                                {for self.network.clone().msg.map(|m| html!{ <div class="input-warn">{m}</div> })}
                            </label>
                        </div>
                    </div>
                    <div class="dash-row" style="margin-bottom: 20px;">
                        <div class="dash-col-100">
                            <label>
                                <h3 class="cell-title">{ "Contract Address: " }</h3>
                                <input
                                    name="contract"
                                    style="width: 480px; text-align: center; font-family: monospace; font-size: 0.9rem;"
                                    placeholder=""
                                    value={self.address.s.clone()}
                                    oninput={link.callback(move |e: InputEvent| {
                                        Msg::UpdateAddress(e.target_unchecked_into::<HtmlInputElement>().value())
                                    })}
                                    onchange={link.callback(move |e: Event| {
                                        Msg::UpdateAddress(e.target_unchecked_into::<HtmlInputElement>().value())
                                    })}
                                />
                                {for self.address.clone().msg.map(|m| html!{ <div class="input-warn">{m}</div> })}
                            </label>
                        </div>
                    </div>
                    <div class="dash-row" style="margin-bottom: 20px;">
                        <div class="cell dash-col-3" style="width: 120px;">
                            <h3 class="cell-title">{ "Min Block:" }</h3>
                            <input
                                name="min_block"
                                style="width: 120px; text-align: center"
                                placeholder="0"
                                value={self.min_block.s.clone()}
                                oninput={link.callback(move |e: InputEvent| {
                                    Msg::UpdateMinBlock(e.target_unchecked_into::<HtmlInputElement>().value())
                                })}
                                onchange={link.callback(move |e: Event| {
                                    Msg::UpdateMinBlock(e.target_unchecked_into::<HtmlInputElement>().value())
                                })}
                            />
                            {for self.min_block.clone().msg.map(|m| html!{ <div class="input-warn">{m}</div> })}
                        </div>
                        <div class="cell dash-col-3" style="width: 120px;">
                            <h3 class="cell-title">{ "Max Block:" }</h3>
                            <input
                                name="max_block"
                                style="width: 120px; text-align: center"
                                placeholder="HEAD"
                                value={self.max_block.s.clone()}
                                oninput={link.callback(move |e: InputEvent| {
                                    Msg::UpdateMaxBlock(e.target_unchecked_into::<HtmlInputElement>().value())
                                })}
                                onchange={link.callback(move |e: Event| {
                                    Msg::UpdateMaxBlock(e.target_unchecked_into::<HtmlInputElement>().value())
                                })}
                            />
                            {for self.max_block.clone().msg.map(|m| html!{ <div class="input-warn">{m}</div> })}
                        </div>
                        <div class="cell dash-col-3" style="width: 120px;">
                            <h3 class="cell-title">{ "Batch Size:" }</h3>
                            <input
                                name="batch_size"
                                style="width: 120px; text-align: center"
                                placeholder=""
                                value={self.batch_size.s.clone()}
                                oninput={link.callback(move |e: InputEvent| {
                                    Msg::UpdateBatchSize(e.target_unchecked_into::<HtmlInputElement>().value())
                                })}
                                onchange={link.callback(move |e: Event| {
                                    Msg::UpdateBatchSize(e.target_unchecked_into::<HtmlInputElement>().value())
                                })}
                            />
                            {for self.batch_size.clone().msg.map(|m| html!{ <div class="input-warn">{m}</div> })}
                        </div>
                    </div>
                    <div class="dash-row" style="margin-bottom: 20px;">
                        <div class="dash-col-3">
                            <div
                                class="button-wrapper primary"
                                style="width: 60%; margin: 0px auto; max-width: 240px; text-align: center;"
                            >
                                {match self.entry() {
                                    Some(_) => {
                                        html! {
                                            <>
                                                <input
                                                    type="submit"
                                                    class="button primary" style="width: 100%; text-align: center;"
                                                    onclick={link.callback(move |_| Msg::Submit)}
                                                    value={"Scan Logs"}
                                                />
                                                <div class="underline"></div>
                                            </>
                                        }
                                    },
                                    None => {
                                        html! {
                                            <>
                                                <button
                                                    class="button disabled"
                                                    disabled={true}
                                                    style="width: 100%; text-align: center;"
                                                >
                                                    {"Scan Logs"}
                                                </button>
                                                <div class="underline disabled"></div>
                                            </>
                                        }
                                    }
                                }}

                            </div>
                        </div>
                    </div>
                </div>
            </form>
        };

        html! {
            <div>
                <Panel title="Airnode RRP Explorer" content={form} />
            </div>
        }
    }
}

// <pre>{ serde_json::to_string_pretty(self).unwrap() }</pre>
// <pre style="color: darkgreen">{ serde_json::to_string_pretty(&self.entry()).unwrap() }</pre>
