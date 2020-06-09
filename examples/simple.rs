use yew::{html, Component, ComponentLink, Html};

struct Root;

impl Component for Root {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <lew::SimpleEditor placeholder = "Leave a comment" />
        }
    }
}

fn main() {
    yew::start_app::<Root>();
}
