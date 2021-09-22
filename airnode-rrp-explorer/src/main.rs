pub mod entry;
pub mod footer;
pub mod header;
pub mod results;
pub mod components;

use yew::{html, Component, Context, Html};

#[derive(Debug)]
enum Msg {}

struct App {
    // TODO: embed the full state of the application
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <div>
                <header::Header />
                <entry::EntryForm />
                <results::ResultsView />
                <footer::Footer />
            </div>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
