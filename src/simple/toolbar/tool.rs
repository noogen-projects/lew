use derive_more::{Deref, DerefMut};

use yew::{html, Callback, Html, MouseEvent};

use super::{replace_selected_in_textarea, ReplaceFmt, UnselectedApplyMode};
use crate::Widget;

#[derive(Debug, Clone, Default)]
pub struct Tool {
    pub textarea_selector: String,
    pub class: String,
    pub title: String,
    pub size: u32,
    pub mode: UnselectedApplyMode,
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
            mode: UnselectedApplyMode::Line,
        })
    }
}

impl Widget for Header {
    fn build(&self) -> Html {
        let selector = self.textarea_selector.clone();
        let mode = self.mode;
        let onclick = Callback::from(move |_: MouseEvent| replace_selected_in_textarea(&selector, ("### ", ""), mode));
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
            mode: UnselectedApplyMode::Word,
        })
    }
}

impl Widget for Bold {
    fn build(&self) -> Html {
        let selector = self.textarea_selector.clone();
        let mode = self.mode;
        let onclick = Callback::from(move |_: MouseEvent| replace_selected_in_textarea(&selector, ("**", "**"), mode));
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
            mode: UnselectedApplyMode::Word,
        })
    }
}

impl Widget for Italic {
    fn build(&self) -> Html {
        let selector = self.textarea_selector.clone();
        let mode = self.mode;
        let onclick = Callback::from(move |_: MouseEvent| replace_selected_in_textarea(&selector, ("*", "*"), mode));
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

#[derive(Debug, Clone, Default, Deref, DerefMut)]
pub struct Quote(pub Tool);

impl Quote {
    pub fn new() -> Self {
        Self(Tool {
            textarea_selector: ".lew-simple__textarea".to_string(),
            class: "lew-simple__tool_button".to_string(),
            title: "Quote".to_string(),
            size: 24,
            mode: UnselectedApplyMode::FromWordToEndLine,
        })
    }
}

impl Widget for Quote {
    fn build(&self) -> Html {
        let selector = self.textarea_selector.clone();
        let mode = self.mode;
        let onclick = Callback::from(move |_: MouseEvent| {
            replace_selected_in_textarea(&selector, ReplaceFmt::StartLine("> ".to_string()), mode)
        });
        html! {
            <button class = &self.class style = format!("width: {0}; height: {0}", self.size)
                    type = "button" title = &self.title onclick = onclick>
                <svg xmlns = "http://www.w3.org/2000/svg" viewBox = "0 0 24 24" width = self.size height = self.size>
                    <path fill-rule = "evenodd" d = "M3 6.25a.75.75 0 01.75-.75h13.5a.75.75 0 010 1.5H3.75A.75.75 0 \
                            013 6.25zM3.75 11a.75.75 0 01.75.75v7a.75.75 0 01-1.5 0v-7a.75.75 0 01.75-.75zM8 12.313a.75.75 \
                            0 01.75-.75h11.5a.75.75 0 010 1.5H8.75a.75.75 0 01-.75-.75zm0 5.937a.75.75 0 01.75-.75h11.5a.75.75 \
                            0 010 1.5H8.75a.75.75 0 01-.75-.75z">
                    </path>
                </svg>
            </button>
        }
    }
}
