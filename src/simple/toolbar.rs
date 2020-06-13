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
            tools: vec![Box::new(tool::Bold::new())],
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

pub struct ReplaceFmt {
    pub before: String,
    pub use_selected: bool,
    pub after: String,
}

impl From<String> for ReplaceFmt {
    fn from(value: String) -> Self {
        Self {
            before: value.clone(),
            use_selected: true,
            after: value,
        }
    }
}

impl From<&String> for ReplaceFmt {
    fn from(value: &String) -> Self {
        value.clone().into()
    }
}

impl From<&str> for ReplaceFmt {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

impl<Before: Into<String>, After: Into<String>> From<(Before, After)> for ReplaceFmt {
    fn from((before, after): (Before, After)) -> Self {
        Self {
            before: before.into(),
            use_selected: true,
            after: after.into(),
        }
    }
}

pub fn replace_selected_in_textarea(textarea_selector: impl AsRef<str>, replace_fmt: impl Into<ReplaceFmt>) {
    if let Ok(Some(element)) = utils::document().query_selector(textarea_selector.as_ref()) {
        if let Ok(textarea) = element.dyn_into::<HtmlTextAreaElement>() {
            let text = textarea.value();
            let start = textarea
                .selection_start()
                .map(|start| start.unwrap_or(0) as usize)
                .unwrap_or(0);
            let mut end = textarea
                .selection_end()
                .map(|end| end.unwrap_or(0) as usize)
                .unwrap_or(0);
            if end < start {
                end = start;
            }

            let before: String = text.chars().take(start).collect();
            let selected: String = text.chars().skip(start).take(end - start).collect();
            let after: String = text.chars().skip(end).collect();

            let fmt = replace_fmt.into();
            let text = before + &fmt.before + if fmt.use_selected { &selected } else { "" } + &fmt.after + &after;

            textarea.set_value(&text);
            textarea.focus().ok();
            textarea
                .set_selection_end(Some(
                    (end + fmt.before.chars().count() + if end - start > 0 { fmt.after.chars().count() } else { 0 })
                        as u32,
                ))
                .ok();
        }
    }
}

pub mod tool {
    use yew::{html, Callback, Html, MouseEvent};

    use super::replace_selected_in_textarea;
    use crate::Widget;

    pub struct Bold {
        pub textarea_selector: String,
        pub class: String,
        pub title: String,
        pub size: u32,
    }

    impl Default for Bold {
        fn default() -> Self {
            Self {
                textarea_selector: ".lew-simple__textarea".to_string(),
                class: "lew-simple__tool_bold".to_string(),
                title: "Bold".to_string(),
                size: 24,
            }
        }
    }

    impl Bold {
        pub fn new() -> Self {
            Self::default()
        }
    }

    impl Widget for Bold {
        fn build(&self) -> Html {
            let selector = self.textarea_selector.clone();
            let onclick = Callback::from(move |_: MouseEvent| replace_selected_in_textarea(&selector, "**"));
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
}
