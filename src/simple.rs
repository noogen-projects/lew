use yew::{html, Component, ComponentLink, Html, Properties};

pub struct SimpleEditor {
    id: String,
    class: String,
    cols: usize,
    rows: usize,
    name: String,
    placeholder: String,
    text: String,
}

#[derive(Clone, Properties, Default)]
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
}

impl Component for SimpleEditor {
    type Message = ();
    type Properties = SimpleEditorProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            id: props.id,
            class: props.class,
            cols: props.cols,
            rows: props.rows,
            name: props.name,
            placeholder: props.placeholder,
            text: props.text,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        false
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.id = props.id;
        self.class = props.class;
        self.cols = props.cols;
        self.rows = props.rows;
        self.name = props.name;
        self.placeholder = props.placeholder;
        self.text = props.text;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div id = &self.id class = &self.class>
                <textarea cols = self.cols rows = self.rows class = "lew-simple__textarea" name = &self.name placeholder = &self.placeholder>
                    { &self.text }
                </textarea>
            </div>
        }
    }
}
