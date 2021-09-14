use yew::{html, Component, Context, Html};

#[derive(Debug)]
pub enum Msg {}

pub struct Header;

impl Component for Header {
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
                <h1>{ "Airnode: Requests and Responses" }</h1>
            </div>
        }
    }
}
