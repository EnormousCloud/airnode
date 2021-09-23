use web3::types::Log;
use yew::{html, Component, Context, Html, Properties};

#[derive(Debug)]
pub enum Msg {}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub input: Vec<Log>,
}

pub struct ResultsView;

impl Component for ResultsView {
    type Message = Msg;
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                { "results..." }
                <ol>
                {
                    for ctx.props().input.iter().map(|log| html! {
                        <li>{ serde_json::to_string(log).unwrap() }</li>
                    })
                }
                </ol>
            </div>
        }
    }
}
