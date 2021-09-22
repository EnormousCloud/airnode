use yew::{html, Component, Context, Html, Properties};

#[derive(Debug)]
pub enum Msg {}

pub struct Panel;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub title: String,
    pub content: Html,
}

impl Component for Panel {
    type Message = Msg;
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self {

        }
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="bordered-wrapper">
                <div class="bordered-panel">
                    <div class="bordered-box">
                        <div class="bordered-left"></div>
                        <div class="bordered-inner">
                            <div class="bordered-title big-title">{ ctx.props().title.clone() }</div>
                            <div class="bordered-content">{ ctx.props().content.clone() }</div>
                        </div>
                        <div class="bordered-right"></div>
                    </div>
                </div>
            </div>
        }
    }
}
