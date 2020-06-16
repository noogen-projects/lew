use derive_more::{Deref, DerefMut};

use yew::{html, Callback, Html, MouseEvent};

use super::{replace_selected_in_textarea, textarea_selection, ReplaceFmt, UnselectedApplyMode};
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
        let onclick = Callback::from(move |_: MouseEvent| {
            replace_selected_in_textarea(textarea_selection(&selector), ("### ", ""), mode)
        });
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
        let onclick = Callback::from(move |_: MouseEvent| {
            replace_selected_in_textarea(textarea_selection(&selector), ("**", "**"), mode)
        });
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
        let onclick = Callback::from(move |_: MouseEvent| {
            replace_selected_in_textarea(textarea_selection(&selector), ("*", "*"), mode)
        });
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
            replace_selected_in_textarea(
                textarea_selection(&selector),
                ReplaceFmt::StartLine("> ".to_string()),
                mode,
            )
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

#[derive(Debug, Clone, Default, Deref, DerefMut)]
pub struct Code(pub Tool);

impl Code {
    pub fn new() -> Self {
        Self(Tool {
            textarea_selector: ".lew-simple__textarea".to_string(),
            class: "lew-simple__tool_button".to_string(),
            title: "Code".to_string(),
            size: 24,
            mode: UnselectedApplyMode::Word,
        })
    }
}

impl Widget for Code {
    fn build(&self) -> Html {
        let selector = self.textarea_selector.clone();
        let mode = self.mode;
        let onclick = Callback::from(move |_: MouseEvent| {
            let selection = textarea_selection(&selector);
            let (prefix, suffix) = selection
                .as_ref()
                .and_then(|(_, text, selection)| {
                    text.chars()
                        .skip(selection.start)
                        .take(selection.end)
                        .find(|&ch| ch == '\n')
                })
                .map(|_| ("\n```\n", "\n```\n"))
                .unwrap_or(("`", "`"));
            replace_selected_in_textarea(
                selection,
                ReplaceFmt::Around(prefix.to_string(), suffix.to_string()),
                mode,
            )
        });
        html! {
            <button class = &self.class style = format!("width: {0}; height: {0}", self.size)
                    type = "button" title = &self.title onclick = onclick>
                <svg xmlns = "http://www.w3.org/2000/svg" viewBox = "0 0 24 24" width = self.size height = self.size>
                    <path fill-rule = "evenodd" d = "M8.78 4.97a.75.75 0 010 1.06L2.81 12l5.97 5.97a.75.75 0 11-1.06 \
                            1.06l-6.5-6.5a.75.75 0 010-1.06l6.5-6.5a.75.75 0 011.06 0zm6.44 0a.75.75 0 000 1.06L21.19 \
                            12l-5.97 5.97a.75.75 0 101.06 1.06l6.5-6.5a.75.75 0 000-1.06l-6.5-6.5a.75.75 0 00-1.06 0z">
                    </path>
                </svg>
            </button>
        }
    }
}

#[derive(Debug, Clone, Default, Deref, DerefMut)]
pub struct Link(pub Tool);

impl Link {
    pub fn new() -> Self {
        Self(Tool {
            textarea_selector: ".lew-simple__textarea".to_string(),
            class: "lew-simple__tool_button".to_string(),
            title: "Link".to_string(),
            size: 24,
            mode: UnselectedApplyMode::Word,
        })
    }
}

impl Widget for Link {
    fn build(&self) -> Html {
        let selector = self.textarea_selector.clone();
        let mode = self.mode;
        let onclick = Callback::from(move |_: MouseEvent| {
            replace_selected_in_textarea(textarea_selection(&selector), ("[", "]()"), mode)
        });
        html! {
            <button class = &self.class style = format!("width: {0}; height: {0}", self.size)
                    type = "button" title = &self.title onclick = onclick>
                <svg xmlns = "http://www.w3.org/2000/svg" viewBox = "0 0 24 24" width = self.size height = self.size>
                    <path d = "M14.78 3.653a3.936 3.936 0 115.567 5.567l-3.627 3.627a3.936 3.936 0 01-5.88-.353.75.75 \
                            0 00-1.18.928 5.436 5.436 0 008.12.486l3.628-3.628a5.436 5.436 0 10-7.688-7.688l-3 3a.75.75 \
                            0 001.06 1.061l3-3z">
                    </path>
                    <path d = "M7.28 11.153a3.936 3.936 0 015.88.353.75.75 0 001.18-.928 5.436 5.436 0 \
                            00-8.12-.486L2.592 13.72a5.436 5.436 0 107.688 7.688l3-3a.75.75 0 10-1.06-1.06l-3 \
                            3a3.936 3.936 0 01-5.567-5.568l3.627-3.627z">
                    </path>
                </svg>
            </button>
        }
    }
}

#[derive(Debug, Clone, Default, Deref, DerefMut)]
pub struct ImageLink(pub Tool);

impl ImageLink {
    pub fn new() -> Self {
        Self(Tool {
            textarea_selector: ".lew-simple__textarea".to_string(),
            class: "lew-simple__tool_button".to_string(),
            title: "Image link".to_string(),
            size: 24,
            mode: UnselectedApplyMode::Word,
        })
    }
}

impl Widget for ImageLink {
    fn build(&self) -> Html {
        let selector = self.textarea_selector.clone();
        let mode = self.mode;
        let onclick = Callback::from(move |_: MouseEvent| {
            replace_selected_in_textarea(textarea_selection(&selector), ("![", "]()"), mode)
        });
        html! {
            <button class = &self.class style = format!("width: {0}; height: {0}", self.size)
                    type = "button" title = &self.title onclick = onclick>
                <svg xmlns = "http://www.w3.org/2000/svg" viewBox = "0 0 24 24" width = self.size height = self.size>
                    <path fill-rule = "evenodd" d = "M2.25 4a.25.25 0 00-.25.25v15.5c0 .138.112.25.25.25h3.178L14 \
                            10.977a1.75 1.75 0 012.506-.032L22 16.44V4.25a.25.25 0 00-.25-.25H2.25zm3.496 17.5H21.75a1.75 \
                            1.75 0 001.75-1.75V4.25a1.75 1.75 0 00-1.75-1.75H2.25A1.75 1.75 0 00.5 4.25v15.5c0 .966.784 \
                            1.75 1.75 1.75h3.496zM22 19.75v-1.19l-6.555-6.554a.25.25 0 00-.358.004L7.497 20H21.75a.25.25 \
                            0 00.25-.25zM9 9.25a1.75 1.75 0 11-3.5 0 1.75 1.75 0 013.5 0zm1.5 0a3.25 3.25 0 11-6.5 0 3.25 \
                            3.25 0 016.5 0z">
                    </path>
                </svg>
            </button>
        }
    }
}

#[derive(Debug, Clone, Default, Deref, DerefMut)]
pub struct BulletedList(pub Tool);

impl BulletedList {
    pub fn new() -> Self {
        Self(Tool {
            textarea_selector: ".lew-simple__textarea".to_string(),
            class: "lew-simple__tool_button".to_string(),
            title: "Bulleted list".to_string(),
            size: 24,
            mode: UnselectedApplyMode::FromWordToEndLine,
        })
    }
}

impl Widget for BulletedList {
    fn build(&self) -> Html {
        let selector = self.textarea_selector.clone();
        let mode = self.mode;
        let onclick = Callback::from(move |_: MouseEvent| {
            replace_selected_in_textarea(
                textarea_selection(&selector),
                ReplaceFmt::StartLine("- ".to_string()),
                mode,
            )
        });
        html! {
            <button class = &self.class style = format!("width: {0}; height: {0}", self.size)
                    type = "button" title = &self.title onclick = onclick>
                <svg xmlns = "http://www.w3.org/2000/svg" viewBox = "0 0 24 24" width = self.size height = self.size>
                    <path fill-rule = "evenodd" d = "M4 7a1 1 0 100-2 1 1 0 000 2zm4.75-1.5a.75.75 0 000 1.5h11.5a.75.75 \
                            0 000-1.5H8.75zm0 6a.75.75 0 000 1.5h11.5a.75.75 0 000-1.5H8.75zm0 6a.75.75 0 000 \
                            1.5h11.5a.75.75 0 000-1.5H8.75zM5 12a1 1 0 11-2 0 1 1 0 012 0zm-1 7a1 1 0 100-2 1 1 0 000 2z">
                    </path>
                </svg>
            </button>
        }
    }
}

#[derive(Debug, Clone, Default, Deref, DerefMut)]
pub struct OrderedList(pub Tool);

impl OrderedList {
    pub fn new() -> Self {
        Self(Tool {
            textarea_selector: ".lew-simple__textarea".to_string(),
            class: "lew-simple__tool_button".to_string(),
            title: "Ordered list".to_string(),
            size: 24,
            mode: UnselectedApplyMode::FromWordToEndLine,
        })
    }
}

impl Widget for OrderedList {
    fn build(&self) -> Html {
        let selector = self.textarea_selector.clone();
        let mode = self.mode;
        let onclick = Callback::from(move |_: MouseEvent| {
            replace_selected_in_textarea(
                textarea_selection(&selector),
                ReplaceFmt::StartLine("1. ".to_string()),
                mode,
            )
        });
        html! {
            <button class = &self.class style = format!("width: {0}; height: {0}", self.size)
                    type = "button" title = &self.title onclick = onclick>
                <svg xmlns = "http://www.w3.org/2000/svg" viewBox = "0 0 24 24" width = self.size height = self.size>
                    <path d = "M3.604 3.089A.75.75 0 014 3.75V8.5h.75a.75.75 0 010 1.5h-3a.75.75 0 \
                            110-1.5h.75V5.151l-.334.223a.75.75 0 01-.832-1.248l1.5-1a.75.75 0 \
                            01.77-.037zM8.75 5.5a.75.75 0 000 1.5h11.5a.75.75 0 000-1.5H8.75zm0 6a.75.75 0 \
                            000 1.5h11.5a.75.75 0 000-1.5H8.75zm0 6a.75.75 0 000 1.5h11.5a.75.75 0 000-1.5H8.75zM5.5 \
                            15.75c0-.704-.271-1.286-.72-1.686a2.302 2.302 0 00-1.53-.564c-.535 \
                            0-1.094.178-1.53.565-.449.399-.72.982-.72 1.685a.75.75 0 001.5 \
                            0c0-.296.104-.464.217-.564A.805.805 0 013.25 15c.215 0 \
                            .406.072.533.185.113.101.217.268.217.565 0 \
                            .332-.069.48-.21.657-.092.113-.216.24-.403.419l-.147.14c-.152.144-.33.313-.52.504l-1.5 1.5a.75.75 \
                            0 00-.22.53v.25c0 .414.336.75.75.75H5A.75.75 0 005 \
                            19H3.31l.47-.47c.176-.176.333-.324.48-.465l.165-.156a5.98 5.98 0 \
                            00.536-.566c.358-.447.539-.925.539-1.593z">
                    </path>
                </svg>
            </button>
        }
    }
}

#[derive(Debug, Clone, Default, Deref, DerefMut)]
pub struct TaskList(pub Tool);

impl TaskList {
    pub fn new() -> Self {
        Self(Tool {
            textarea_selector: ".lew-simple__textarea".to_string(),
            class: "lew-simple__tool_button".to_string(),
            title: "Task list".to_string(),
            size: 24,
            mode: UnselectedApplyMode::FromWordToEndLine,
        })
    }
}

impl Widget for TaskList {
    fn build(&self) -> Html {
        let selector = self.textarea_selector.clone();
        let mode = self.mode;
        let onclick = Callback::from(move |_: MouseEvent| {
            replace_selected_in_textarea(
                textarea_selection(&selector),
                ReplaceFmt::StartLine("- [ ] ".to_string()),
                mode,
            )
        });
        html! {
            <button class = &self.class style = format!("width: {0}; height: {0}", self.size)
                    type = "button" title = &self.title onclick = onclick>
                <svg xmlns = "http://www.w3.org/2000/svg" viewBox = "0 0 24 24" width = self.size height = self.size>
                    <path d = "M17.28 9.28a.75.75 0 00-1.06-1.06l-5.97 5.97-2.47-2.47a.75.75 0 00-1.06 1.06l3 3a.75.75 \
                            0 001.06 0l6.5-6.5z">
                    </path>
                    <path fill-rule = "evenodd" d = "M3.75 2A1.75 1.75 0 002 3.75v16.5c0 .966.784 1.75 1.75 \
                            1.75h16.5A1.75 1.75 0 0022 20.25V3.75A1.75 1.75 0 0020.25 2H3.75zM3.5 3.75a.25.25 0 \
                            01.25-.25h16.5a.25.25 0 01.25.25v16.5a.25.25 0 01-.25.25H3.75a.25.25 0 01-.25-.25V3.75z">
                    </path>
                </svg>
            </button>
        }
    }
}
