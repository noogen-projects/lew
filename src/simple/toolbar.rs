use wasm_bindgen::JsCast;
use web_sys::HtmlTextAreaElement;
use yew::{html, utils, Html};

use crate::Widget;

pub mod tool;

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
