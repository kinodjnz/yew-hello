use yew::{html, Component, ComponentLink, Html, ShouldRender};

struct Model {
    link: ComponentLink<Self>,
    message: String,
}

enum Msg {
    DoIt,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            link,
            message: "Click me!".into(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DoIt => {
                self.message = "hello, yew".into();
                true
            }
        }
    }

    fn view(&self) -> Html {
        html! {
            <button onclick=self.link.callback(|_| Msg::DoIt)>{ self.message.clone() }</button>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
