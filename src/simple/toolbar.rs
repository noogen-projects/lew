use wasm_bindgen::JsCast;
use web_sys::HtmlTextAreaElement;
use yew::{html, utils, Html};

use crate::Widget;

pub struct SimpleToolbar {
    pub id: String,
    pub class: String,
    pub tools: Vec<Box<dyn Widget>>,
}

impl Default for SimpleToolbar {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            class: "lew-simple__toolbar".to_string(),
            tools: vec![
                Box::new(tool::Header::new()),
                Box::new(tool::Bold::new()),
                Box::new(tool::Italic::new()),
            ],
        }
    }
}

impl SimpleToolbar {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }

    pub fn with_class(mut self, class: impl Into<String>) -> Self {
        self.class = class.into();
        self
    }
}

impl Widget for SimpleToolbar {
    fn build(&self) -> Html {
        let item_class = format!("{}_item", self.class);
        html! {
            <ul id = &self.id class = &self.class>
                {
                    self.tools
                        .iter()
                        .map(|tool| html! { <li class = &item_class>{ tool.build() }</li> })
                        .collect::<Html>()
                }
            </ul>
        }
    }
}

pub enum ReplaceFmt {
    Around(String, String),
    StartLine(String),
}

impl ReplaceFmt {
    pub fn layout(&self, text: String, start_char: usize, end_char: usize) -> (String, usize) {
        let selected_chars = end_char - start_char;

        let before: String = text.chars().take(start_char).collect();
        let selected: String = text.chars().skip(start_char).take(selected_chars).collect();
        let after: String = text.chars().skip(end_char).collect();

        match self {
            ReplaceFmt::Around(prefix, suffix) => (
                before + prefix + &selected + suffix + &after,
                end_char + prefix.chars().count() + if selected_chars > 0 { suffix.chars().count() } else { 0 },
            ),
            ReplaceFmt::StartLine(prefix) => todo!(),
        }
    }
}

impl<Before: Into<String>, After: Into<String>> From<(Before, After)> for ReplaceFmt {
    fn from((before, after): (Before, After)) -> Self {
        Self::Around(before.into(), after.into())
    }
}

pub fn replace_selected_in_textarea(textarea_selector: impl AsRef<str>, fmt: impl Into<ReplaceFmt>) {
    if let Ok(Some(element)) = utils::document().query_selector(textarea_selector.as_ref()) {
        if let Ok(textarea) = element.dyn_into::<HtmlTextAreaElement>() {
            let text = textarea.value();
            let start_char = textarea
                .selection_start()
                .map(|start| start.unwrap_or(0) as usize)
                .unwrap_or(0);
            let mut end_char = textarea
                .selection_end()
                .map(|end| end.unwrap_or(0) as usize)
                .unwrap_or(0);
            if end_char < start_char {
                end_char = start_char;
            }

            let (text, cursor_pos) = fmt.into().layout(text, start_char, end_char);

            textarea.set_value(&text);
            textarea.focus().ok();
            textarea.set_selection_end(Some(cursor_pos as u32)).ok();
        }
    }
}

pub mod tool {
    use derive_more::{Deref, DerefMut};

    use yew::{html, Callback, Html, MouseEvent};

    use super::replace_selected_in_textarea;
    use crate::Widget;

    #[derive(Debug, Clone, Default)]
    pub struct Tool {
        pub textarea_selector: String,
        pub class: String,
        pub title: String,
        pub size: u32,
    }

    #[derive(Debug, Clone, Default, Deref, DerefMut)]
    pub struct Header(pub Tool);

    impl Header {
        pub fn new() -> Self {
            Self(Tool {
                textarea_selector: ".lew-simple__textarea".to_string(),
                class: "lew-simple__tool_button".to_string(),
                title: "Header".to_string(),
                size: 24,
            })
        }
    }

    impl Widget for Header {
        fn build(&self) -> Html {
            let selector = self.textarea_selector.clone();
            let onclick = Callback::from(move |_: MouseEvent| replace_selected_in_textarea(&selector, ("### ", "")));
            html! {
                <button class = &self.class style = format!("width: {0}; height: {0}", self.size)
                        type = "button" title = &self.title onclick = onclick>
                    <svg xmlns = "http://www.w3.org/2000/svg" viewBox = "0 0 24 24" width = self.size height = self.size>
                        <path fill-rule = "evenodd" d = "M6.25 4a.75.75 0 01.75.75V11h10V4.75a.75.75 0 011.5 0v14.5a.75.75 0 \
                                01-1.5 0V12.5H7v6.75a.75.75 0 01-1.5 0V4.75A.75.75 0 016.25 4z">
                        </path>
                    </svg>
                </button>
            }
        }
    }

    #[derive(Debug, Clone, Default, Deref, DerefMut)]
    pub struct Bold(pub Tool);

    impl Bold {
        pub fn new() -> Self {
            Self(Tool {
                textarea_selector: ".lew-simple__textarea".to_string(),
                class: "lew-simple__tool_button".to_string(),
                title: "Bold".to_string(),
                size: 24,
            })
        }
    }

    impl Widget for Bold {
        fn build(&self) -> Html {
            let selector = self.textarea_selector.clone();
            let onclick = Callback::from(move |_: MouseEvent| replace_selected_in_textarea(&selector, ("**", "**")));
            html! {
                <button class = &self.class style = format!("width: {0}; height: {0}", self.size)
                        type = "button" title = &self.title onclick = onclick>
                    <svg xmlns = "http://www.w3.org/2000/svg" viewBox = "0 0 24 24" width = self.size height = self.size>
                        <path fill-rule = "evenodd" d = "M6 4.75c0-.69.56-1.25 1.25-1.25h5a4.75 4.75 0 013.888 7.479A5 5 0 \
                                0114 20.5H7.25c-.69 0-1.25-.56-1.25-1.25V4.75zM8.5 13v5H14a2.5 2.5 0 000-5H8.5zm0-2.5h3.751A2.25 \
                                2.25 0 0012.25 6H8.5v4.5z">
                        </path>
                    </svg>
                </button>
            }
        }
    }

    #[derive(Debug, Clone, Default, Deref, DerefMut)]
    pub struct Italic(pub Tool);

    impl Italic {
        pub fn new() -> Self {
            Self(Tool {
                textarea_selector: ".lew-simple__textarea".to_string(),
                class: "lew-simple__tool_button".to_string(),
                title: "Italic".to_string(),
                size: 24,
            })
        }
    }

    impl Widget for Italic {
        fn build(&self) -> Html {
            let selector = self.textarea_selector.clone();
            let onclick = Callback::from(move |_: MouseEvent| replace_selected_in_textarea(&selector, ("*", "*")));
            html! {
                <button class = &self.class style = format!("width: {0}; height: {0}", self.size)
                        type = "button" title = &self.title onclick = onclick>
                    <svg xmlns = "http://www.w3.org/2000/svg" viewBox = "0 0 24 24" width = self.size height = self.size>
                        <path fill-rule = "evenodd" d = "M10 4.75a.75.75 0 01.75-.75h8.5a.75.75 0 010 1.5h-3.514l-5.828 \
                                13h3.342a.75.75 0 010 1.5h-8.5a.75.75 0 010-1.5h3.514l5.828-13H10.75a.75.75 0 01-.75-.75z">
                        </path>
                    </svg>
                </button>
            }
        }
    }
}
