use yew::prelude::*;
use yew::web_sys::{Event, HtmlInputElement};
use yew::{html, Component, ComponentLink, Context, Html};

// use yew::{classes, html, Callback, Component, Context, Html, Properties, TargetCast};

#[derive(Debug)]
enum Msg {
    UpdateNetwork(String),
}

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    network: String,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self) -> Self {
        Self {
            link,
            network: "http://localhost:8545/".to_owned(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateNetwork(s) => {
                self.network = s.clone();
                true // re-render 
            }
        }
    }

    fn change(&mut self, _props: Self::Properties, _ctx: &Context<Self>) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let link = self.link;
        // let Self { client, .. } = self;
        
        let update_name = |f: fn(String) -> Msg| {
            link.callback(move |e: Event| {
                let input = e.target().unwrap();
                f(input.value())
            })
        };

        html! {
            <div style="text-align: center">
                <input
                    style="width: 70%"
                    onchange={update_name(Msg::UpdateNetwork)}
                />
                <p>{ self.network }</p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
