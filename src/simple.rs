use yew::{html, Component, ComponentLink, Html, InputData, Properties};

pub use self::toolbar::SimpleToolbar;
use crate::Widget;

pub mod toolbar;

pub struct SimpleEditor {
    id: String,
    class: String,
    cols: usize,
    rows: usize,
    name: String,
    placeholder: String,
    text: String,
    toolbar: Option<Html>,
    oninput: fn(InputData),
    link: ComponentLink<Self>,
}

#[derive(Clone, Properties)]
pub struct SimpleEditorProps {
    #[prop_or_default]
    pub id: String,

    #[prop_or("lew-simple".to_string())]
    pub class: String,

    #[prop_or(80)]
    pub cols: usize,

    #[prop_or(5)]
    pub rows: usize,

    #[prop_or_default]
    pub name: String,

    #[prop_or_default]
    pub placeholder: String,

    #[prop_or_default]
    pub text: String,

    #[prop_or(Some(SimpleToolbar::new().build()))]
    pub toolbar: Option<Html>,

    #[prop_or(|_| ())]
    pub oninput: fn(InputData),
}

impl Component for SimpleEditor {
    type Message = ();
    type Properties = SimpleEditorProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            id: props.id,
            class: props.class,
            cols: props.cols,
            rows: props.rows,
            name: props.name,
            placeholder: props.placeholder,
            text: props.text,
            toolbar: props.toolbar,
            oninput: props.oninput,
            link,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        false
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        let SimpleEditorProps {
            id,
            class,
            cols,
            rows,
            name,
            placeholder,
            text,
            toolbar,
            oninput,
        } = props;

        self.id = id;
        self.class = class;
        self.cols = cols;
        self.rows = rows;
        self.name = name;
        self.placeholder = placeholder;
        self.text = text;
        self.toolbar = toolbar;
        self.oninput = oninput;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div id = &self.id class = &self.class>
                { self.toolbar.as_ref().cloned().unwrap_or(html! {}) }
                <textarea cols = self.cols rows = self.rows class = "lew-simple__textarea"
                        name = &self.name placeholder = &self.placeholder oninput = self.link.callback(self.oninput)>
                    { &self.text }
                </textarea>
            </div>
        }
    }
}
