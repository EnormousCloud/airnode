use web3::types::H160;
use yew::web_sys::{Event, HtmlInputElement};
use yew::{html, Component, Context, Html, TargetCast};

#[derive(Debug, Clone)]
pub struct Entry {
    network: String,
    address: H160,
    min_block: u64,
    max_block: Option<u64>,
    batch_size: u64,
}

#[derive(Debug)]
pub enum Msg {
    UpdateNetwork(String),
    UpdateMinBlock(String),
    UpdateMaxBlock(String),
}

pub struct EntryForm {
    network: String,
    min_block: u64,
    max_block: Option<u64>,
}

impl Component for EntryForm {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            network: "http://localhost:8545/".to_owned(),
            min_block: 0,
            max_block: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateNetwork(s) => {
                self.network = s.clone();
                true // re-render
            }
            _ => {
                // TODO: !!!!
                false
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

        html! {
            <div style="text-align: center">
                <label>
                    { "Network URL:" }
                    <input
                        style="width: 70%"
                        placeholder="Network RPC URL"
                        value={self.network.clone()}
                        onchange={update_name(Msg::UpdateNetwork)}
                    />
                </label>
                <label>
                    { "Min Block:" }
                    <input
                        style="width: 30%"
                        placeholder="Min Block"
                        value={format!("{}", self.min_block)}
                        onchange={update_u64(Msg::UpdateMinBlock)}
                    />
                </label>
                <p>{ self.network.clone() }</p>
            </div>
        }
    }
}
