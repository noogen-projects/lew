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
            size: 16,
            mode: UnselectedApplyMode::Line,
        })
    }

    pub fn with_textarea_selector(mut self, selector: impl Into<String>) -> Self {
        self.0.textarea_selector = selector.into();
        self
    }

    pub fn with_class(mut self, class: impl Into<String>) -> Self {
        self.0.class = class.into();
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.0.title = title.into();
        self
    }

    pub fn with_size(mut self, size: u32) -> Self {
        self.0.size = size;
        self
    }

    pub fn svg_icon(size: u32) -> Html {
        html! {
            <svg xmlns = "http://www.w3.org/2000/svg" viewBox = "0 0 16 16" width = size height = size>
                <path fill-rule = "evenodd" d = "M3.75 2a.75.75 0 01.75.75V7h7V2.75a.75.75 0 011.5 0v10.5a.75.75 0 \
                        01-1.5 0V8.5h-7v4.75a.75.75 0 01-1.5 0V2.75A.75.75 0 013.75 2z">
                </path>
            </svg>
        }
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
                { Self::svg_icon(self.size) }
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
            size: 16,
            mode: UnselectedApplyMode::Word,
        })
    }

    pub fn with_textarea_selector(mut self, selector: impl Into<String>) -> Self {
        self.0.textarea_selector = selector.into();
        self
    }

    pub fn with_class(mut self, class: impl Into<String>) -> Self {
        self.0.class = class.into();
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.0.title = title.into();
        self
    }

    pub fn with_size(mut self, size: u32) -> Self {
        self.0.size = size;
        self
    }

    pub fn svg_icon(size: u32) -> Html {
        html! {
            <svg xmlns = "http://www.w3.org/2000/svg" viewBox = "0 0 16 16" width = size height = size>
                <path fill-rule = "evenodd" d = "M4 2a1 1 0 00-1 1v10a1 1 0 001 1h5.5a3.5 3.5 0 001.852-6.47A3.5 3.5 0 \
                        008.5 2H4zm4.5 5a1.5 1.5 0 100-3H5v3h3.5zM5 9v3h4.5a1.5 1.5 0 000-3H5z">
                </path>
            </svg>
        }
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
                { Self::svg_icon(self.size) }
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
            size: 16,
            mode: UnselectedApplyMode::Word,
        })
    }

    pub fn with_textarea_selector(mut self, selector: impl Into<String>) -> Self {
        self.0.textarea_selector = selector.into();
        self
    }

    pub fn with_class(mut self, class: impl Into<String>) -> Self {
        self.0.class = class.into();
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.0.title = title.into();
        self
    }

    pub fn with_size(mut self, size: u32) -> Self {
        self.0.size = size;
        self
    }

    pub fn svg_icon(size: u32) -> Html {
        html! {
            <svg xmlns = "http://www.w3.org/2000/svg" viewBox = "0 0 16 16" width = size height = size>
                <path fill-rule = "evenodd" d = "M6 2.75A.75.75 0 016.75 2h6.5a.75.75 0 010 1.5h-2.505l-3.858 \
                        9H9.25a.75.75 0 010 1.5h-6.5a.75.75 0 010-1.5h2.505l3.858-9H6.75A.75.75 0 016 2.75z">
                </path>
            </svg>
        }
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
                { Self::svg_icon(self.size) }
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
            size: 16,
            mode: UnselectedApplyMode::FromWordToEndLine,
        })
    }

    pub fn with_textarea_selector(mut self, selector: impl Into<String>) -> Self {
        self.0.textarea_selector = selector.into();
        self
    }

    pub fn with_class(mut self, class: impl Into<String>) -> Self {
        self.0.class = class.into();
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.0.title = title.into();
        self
    }

    pub fn with_size(mut self, size: u32) -> Self {
        self.0.size = size;
        self
    }

    pub fn svg_icon(size: u32) -> Html {
        html! {
            <svg xmlns = "http://www.w3.org/2000/svg" viewBox = "0 0 16 16" width = size height = size>
                <path fill-rule = "evenodd" d = "M1.75 2.5a.75.75 0 000 1.5h10.5a.75.75 0 000-1.5H1.75zm4 5a.75.75 \
                        0 000 1.5h8.5a.75.75 0 000-1.5h-8.5zm0 5a.75.75 0 000 1.5h8.5a.75.75 0 000-1.5h-8.5zM2.5 \
                        7.75a.75.75 0 00-1.5 0v6a.75.75 0 001.5 0v-6z">
                </path>
            </svg>
        }
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
                { Self::svg_icon(self.size) }
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
            size: 16,
            mode: UnselectedApplyMode::Word,
        })
    }

    pub fn with_textarea_selector(mut self, selector: impl Into<String>) -> Self {
        self.0.textarea_selector = selector.into();
        self
    }

    pub fn with_class(mut self, class: impl Into<String>) -> Self {
        self.0.class = class.into();
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.0.title = title.into();
        self
    }

    pub fn with_size(mut self, size: u32) -> Self {
        self.0.size = size;
        self
    }

    pub fn svg_icon(size: u32) -> Html {
        html! {
            <svg xmlns = "http://www.w3.org/2000/svg" viewBox = "0 0 16 16" width = size height = size>
                <path fill-rule = "evenodd" d = "M4.72 3.22a.75.75 0 011.06 1.06L2.06 8l3.72 3.72a.75.75 0 11-1.06 \
                        1.06L.47 8.53a.75.75 0 010-1.06l4.25-4.25zm6.56 0a.75.75 0 10-1.06 1.06L13.94 8l-3.72 \
                        3.72a.75.75 0 101.06 1.06l4.25-4.25a.75.75 0 000-1.06l-4.25-4.25z">
                </path>
            </svg>
        }
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
                { Self::svg_icon(self.size) }
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
            size: 16,
            mode: UnselectedApplyMode::Word,
        })
    }

    pub fn with_textarea_selector(mut self, selector: impl Into<String>) -> Self {
        self.0.textarea_selector = selector.into();
        self
    }

    pub fn with_class(mut self, class: impl Into<String>) -> Self {
        self.0.class = class.into();
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.0.title = title.into();
        self
    }

    pub fn with_size(mut self, size: u32) -> Self {
        self.0.size = size;
        self
    }

    pub fn svg_icon(size: u32) -> Html {
        html! {
            <svg xmlns = "http://www.w3.org/2000/svg" viewBox = "0 0 16 16" width = size height = size>
                <path fill-rule = "evenodd" d = "M7.775 3.275a.75.75 0 001.06 1.06l1.25-1.25a2 2 0 112.83 2.83l-2.5 \
                        2.5a2 2 0 01-2.83 0 .75.75 0 00-1.06 1.06 3.5 3.5 0 004.95 0l2.5-2.5a3.5 3.5 0 00-4.95-4.95l-1.25 \
                        1.25zm-4.69 9.64a2 2 0 010-2.83l2.5-2.5a2 2 0 012.83 0 .75.75 0 001.06-1.06 3.5 3.5 0 00-4.95 \
                        0l-2.5 2.5a3.5 3.5 0 004.95 4.95l1.25-1.25a.75.75 0 00-1.06-1.06l-1.25 1.25a2 2 0 01-2.83 0z">
                </path>
            </svg>
        }
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
                { Self::svg_icon(self.size) }
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
            size: 16,
            mode: UnselectedApplyMode::Word,
        })
    }

    pub fn with_textarea_selector(mut self, selector: impl Into<String>) -> Self {
        self.0.textarea_selector = selector.into();
        self
    }

    pub fn with_class(mut self, class: impl Into<String>) -> Self {
        self.0.class = class.into();
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.0.title = title.into();
        self
    }

    pub fn with_size(mut self, size: u32) -> Self {
        self.0.size = size;
        self
    }

    pub fn svg_icon(size: u32) -> Html {
        html! {
            <svg xmlns = "http://www.w3.org/2000/svg" viewBox = "0 0 16 16" width = size height = size>
                <path fill-rule = "evenodd" d = "M1.75 2.5a.25.25 0 00-.25.25v10.5c0 .138.112.25.25.25h.94a.76.76 0 \
                        01.03-.03l6.077-6.078a1.75 1.75 0 012.412-.06L14.5 10.31V2.75a.25.25 0 00-.25-.25H1.75zm12.5 \
                        11H4.81l5.048-5.047a.25.25 0 01.344-.009l4.298 3.889v.917a.25.25 0 01-.25.25zm1.75-.25V2.75A1.75 \
                        1.75 0 0014.25 1H1.75A1.75 1.75 0 000 2.75v10.5C0 14.216.784 15 1.75 15h12.5A1.75 1.75 0 0016 \
                        13.25zM5.5 6a.5.5 0 11-1 0 .5.5 0 011 0zM7 6a2 2 0 11-4 0 2 2 0 014 0z">
                </path>
            </svg>
        }
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
                { Self::svg_icon(self.size) }
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
            size: 16,
            mode: UnselectedApplyMode::FromWordToEndLine,
        })
    }

    pub fn with_textarea_selector(mut self, selector: impl Into<String>) -> Self {
        self.0.textarea_selector = selector.into();
        self
    }

    pub fn with_class(mut self, class: impl Into<String>) -> Self {
        self.0.class = class.into();
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.0.title = title.into();
        self
    }

    pub fn with_size(mut self, size: u32) -> Self {
        self.0.size = size;
        self
    }

    pub fn svg_icon(size: u32) -> Html {
        html! {
            <svg xmlns = "http://www.w3.org/2000/svg" viewBox = "0 0 16 16" width = size height = size>
                <path fill-rule = "evenodd" d = "M2 4a1 1 0 100-2 1 1 0 000 2zm3.75-1.5a.75.75 0 000 1.5h8.5a.75.75 \
                        0 000-1.5h-8.5zm0 5a.75.75 0 000 1.5h8.5a.75.75 0 000-1.5h-8.5zm0 5a.75.75 0 000 1.5h8.5a.75.75 \
                        0 000-1.5h-8.5zM3 8a1 1 0 11-2 0 1 1 0 012 0zm-1 6a1 1 0 100-2 1 1 0 000 2z">
                </path>
            </svg>
        }
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
                { Self::svg_icon(self.size) }
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
            size: 16,
            mode: UnselectedApplyMode::FromWordToEndLine,
        })
    }

    pub fn with_textarea_selector(mut self, selector: impl Into<String>) -> Self {
        self.0.textarea_selector = selector.into();
        self
    }

    pub fn with_class(mut self, class: impl Into<String>) -> Self {
        self.0.class = class.into();
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.0.title = title.into();
        self
    }

    pub fn with_size(mut self, size: u32) -> Self {
        self.0.size = size;
        self
    }

    pub fn svg_icon(size: u32) -> Html {
        html! {
            <svg xmlns = "http://www.w3.org/2000/svg" viewBox = "0 0 16 16" width = size height = size>
                <path fill-rule = "evenodd" d = "M2.003 2.5a.5.5 0 00-.723-.447l-1.003.5a.5.5 0 00.446.895l.28-.14V6H.5a.5.5 \
                        0 000 1h2.006a.5.5 0 100-1h-.503V2.5zM5 3.25a.75.75 0 01.75-.75h8.5a.75.75 0 010 1.5h-8.5A.75.75 0 \
                        015 3.25zm0 5a.75.75 0 01.75-.75h8.5a.75.75 0 010 1.5h-8.5A.75.75 0 015 8.25zm0 5a.75.75 0 \
                        01.75-.75h8.5a.75.75 0 010 1.5h-8.5a.75.75 0 01-.75-.75zM.924 10.32l.003-.004a.851.851 0 \
                        01.144-.153A.66.66 0 011.5 10c.195 0 .306.068.374.146a.57.57 0 01.128.376c0 .453-.269.682-.8 \
                        1.078l-.035.025C.692 11.98 0 12.495 0 13.5a.5.5 0 00.5.5h2.003a.5.5 0 \
                        000-1H1.146c.132-.197.351-.372.654-.597l.047-.035c.47-.35 1.156-.858 1.156-1.845 \
                        0-.365-.118-.744-.377-1.038-.268-.303-.658-.484-1.126-.484-.48 0-.84.202-1.068.392a1.858 1.858 0 \
                        00-.348.384l-.007.011-.002.004-.001.002-.001.001a.5.5 0 00.851.525zM.5 10.055l-.427-.26.427.26z">
                </path>
            </svg>
        }
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
                { Self::svg_icon(self.size) }
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

    pub fn with_textarea_selector(mut self, selector: impl Into<String>) -> Self {
        self.0.textarea_selector = selector.into();
        self
    }

    pub fn with_class(mut self, class: impl Into<String>) -> Self {
        self.0.class = class.into();
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.0.title = title.into();
        self
    }

    pub fn with_size(mut self, size: u32) -> Self {
        self.0.size = size;
        self
    }

    pub fn svg_icon(size: u32) -> Html {
        html! {
            <svg xmlns = "http://www.w3.org/2000/svg" viewBox = "0 0 16 16" width = size height = size>
                <path fill-rule = "evenodd" d = "M2.5 2.75a.25.25 0 01.25-.25h10.5a.25.25 0 01.25.25v10.5a.25.25 0 \
                        01-.25.25H2.75a.25.25 0 01-.25-.25V2.75zM2.75 1A1.75 1.75 0 001 2.75v10.5c0 .966.784 1.75 \
                        1.75 1.75h10.5A1.75 1.75 0 0015 13.25V2.75A1.75 1.75 0 0013.25 1H2.75zm9.03 5.28a.75.75 0 \
                        00-1.06-1.06L6.75 9.19 5.28 7.72a.75.75 0 00-1.06 1.06l2 2a.75.75 0 001.06 0l4.5-4.5z">
                </path>
            </svg>
        }
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
                { Self::svg_icon(16) }
            </button>
        }
    }
}
