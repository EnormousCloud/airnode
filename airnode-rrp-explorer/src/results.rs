use yew::{html, Component, Context, Html};

#[derive(Debug)]
pub enum Msg {}

pub struct ResultsView;

impl Component for ResultsView {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <div>
                { "results..." }
            </div>
        }
    }
}
