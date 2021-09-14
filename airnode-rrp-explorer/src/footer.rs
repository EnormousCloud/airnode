use yew::{html, Component, Context, Html};

#[derive(Debug)]
pub enum Msg {}

pub struct Footer;

impl Component for Footer {
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
            <footer>
                <div class="inner">
                    { "© 2021 Enormous Cloud | " }
                    <a 
                        target="_blank"
                        rel="nofollow noopener noreferrer"
                        href="https://github.com/EnormousCloud/airnode/tree/main/airnode-rrp-explorer"
                    >
                        { "Github Source" }
                    </a>
                </div>
            </footer>
        }
    }
}
