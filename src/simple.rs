use yew::{html, Callback, Component, Context, Html, InputEvent, Properties};

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
    oninput: Callback<InputEvent>,
}

#[derive(Clone, Properties, PartialEq)]
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

    #[prop_or(Callback::noop())]
    pub oninput: Callback<InputEvent>,
}

impl Component for SimpleEditor {
    type Message = ();
    type Properties = SimpleEditorProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            id: ctx.props().id.clone(),
            class: ctx.props().class.clone(),
            cols: ctx.props().cols,
            rows: ctx.props().rows,
            name: ctx.props().name.clone(),
            placeholder: ctx.props().placeholder.clone(),
            text: ctx.props().text.clone(),
            toolbar: ctx.props().toolbar.clone(),
            oninput: ctx.props().oninput.clone(),
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
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
        } = ctx.props().clone();

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

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div id = { self.id.clone() } class = { self.class.clone() }>
                { self.toolbar.as_ref().cloned().unwrap_or(html! {}) }
                <textarea cols = { self.cols.to_string() } rows = { self.rows.to_string() } class = "lew-simple__textarea"
                        name = { self.name.clone() } placeholder = { self.placeholder.clone() } oninput = { self.oninput.clone() }>
                    { &self.text }
                </textarea>
            </div>
        }
    }
}
