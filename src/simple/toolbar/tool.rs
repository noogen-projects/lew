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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconSize {
    Px16,
    Px24,
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

    pub fn svg_icon(svg_size: u32, icon_size: IconSize) -> Html {
        match icon_size {
            IconSize::Px16 => html! {
                <svg xmlns = "http://www.w3.org/2000/svg" viewBox = "0 0 16 16" width = svg_size height = svg_size>
                    <path fill-rule = "evenodd" d = "M3.75 2a.75.75 0 01.75.75V7h7V2.75a.75.75 0 011.5 0v10.5a.75.75 0 \
                            01-1.5 0V8.5h-7v4.75a.75.75 0 01-1.5 0V2.75A.75.75 0 013.75 2z">
                    </path>
                </svg>
            },
            IconSize::Px24 => html! {
                <svg xmlns = "http://www.w3.org/2000/svg" viewBox = "0 0 24 24" width = svg_size height = svg_size>
                    <path fill-rule = "evenodd" d = "M6.25 4a.75.75 0 01.75.75V11h10V4.75a.75.75 0 011.5 0v14.5a.75.75 0 \
                            01-1.5 0V12.5H7v6.75a.75.75 0 01-1.5 0V4.75A.75.75 0 016.25 4z">
                    </path>
                </svg>
            },
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
                { Self::svg_icon(self.size, IconSize::Px16) }
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

    pub fn svg_icon(svg_size: u32, icon_size: IconSize) -> Html {
        match icon_size {
            IconSize::Px16 => html! {
                <svg xmlns = "http://www.w3.org/2000/svg" viewBox = "0 0 16 16" width = svg_size height = svg_size>
                    <path fill-rule = "evenodd" d = "M4 2a1 1 0 00-1 1v10a1 1 0 001 1h5.5a3.5 3.5 0 001.852-6.47A3.5 3.5 0 \
                            008.5 2H4zm4.5 5a1.5 1.5 0 100-3H5v3h3.5zM5 9v3h4.5a1.5 1.5 0 000-3H5z">
                    </path>
                </svg>
            },
            IconSize::Px24 => html! {
                <svg xmlns = "http://www.w3.org/2000/svg" viewBox = "0 0 24 24" width = svg_size height = svg_size>
                    <path fill-rule = "evenodd" d = "M6 4.75c0-.69.56-1.25 1.25-1.25h5a4.75 4.75 0 013.888 7.479A5 5 0 \
                            0114 20.5H7.25c-.69 0-1.25-.56-1.25-1.25V4.75zM8.5 13v5H14a2.5 2.5 0 000-5H8.5zm0-2.5h3.751A2.25 \
                            2.25 0 0012.25 6H8.5v4.5z">
                    </path>
                </svg>
            },
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
                { Self::svg_icon(self.size, IconSize::Px16) }
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

    pub fn svg_icon(svg_size: u32, icon_size: IconSize) -> Html {
        match icon_size {
            IconSize::Px16 => html! {
                <svg xmlns = "http://www.w3.org/2000/svg" viewBox = "0 0 16 16" width = svg_size height = svg_size>
                    <path fill-rule = "evenodd" d = "M6 2.75A.75.75 0 016.75 2h6.5a.75.75 0 010 1.5h-2.505l-3.858 \
                            9H9.25a.75.75 0 010 1.5h-6.5a.75.75 0 010-1.5h2.505l3.858-9H6.75A.75.75 0 016 2.75z">
                    </path>
                </svg>
            },
            IconSize::Px24 => html! {
                <svg xmlns = "http://www.w3.org/2000/svg" viewBox = "0 0 24 24" width = svg_size height = svg_size>
                    <path fill-rule = "evenodd" d = "M10 4.75a.75.75 0 01.75-.75h8.5a.75.75 0 010 1.5h-3.514l-5.828 \
                            13h3.342a.75.75 0 010 1.5h-8.5a.75.75 0 010-1.5h3.514l5.828-13H10.75a.75.75 0 01-.75-.75z">
                    </path>
                </svg>
            },
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
                { Self::svg_icon(self.size, IconSize::Px16) }
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

    pub fn svg_icon(svg_size: u32, icon_size: IconSize) -> Html {
        match icon_size {
            IconSize::Px16 => html! {
                <svg xmlns = "http://www.w3.org/2000/svg" viewBox = "0 0 16 16" width = svg_size height = svg_size>
                    <path fill-rule = "evenodd" d = "M1.75 2.5a.75.75 0 000 1.5h10.5a.75.75 0 000-1.5H1.75zm4 5a.75.75 \
                            0 000 1.5h8.5a.75.75 0 000-1.5h-8.5zm0 5a.75.75 0 000 1.5h8.5a.75.75 0 000-1.5h-8.5zM2.5 \
                            7.75a.75.75 0 00-1.5 0v6a.75.75 0 001.5 0v-6z">
                    </path>
                </svg>
            },
            IconSize::Px24 => html! {
                <svg xmlns = "http://www.w3.org/2000/svg" viewBox = "0 0 24 24" width = svg_size height = svg_size>
                    <path fill-rule = "evenodd" d = "M3 6.25a.75.75 0 01.75-.75h13.5a.75.75 0 010 1.5H3.75A.75.75 0 \
                            013 6.25zM3.75 11a.75.75 0 01.75.75v7a.75.75 0 01-1.5 0v-7a.75.75 0 01.75-.75zM8 12.313a.75.75 \
                            0 01.75-.75h11.5a.75.75 0 010 1.5H8.75a.75.75 0 01-.75-.75zm0 5.937a.75.75 0 01.75-.75h11.5a.75.75 \
                            0 010 1.5H8.75a.75.75 0 01-.75-.75z">
                    </path>
                </svg>
            },
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
                { Self::svg_icon(self.size, IconSize::Px16) }
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

    pub fn svg_icon(svg_size: u32, icon_size: IconSize) -> Html {
        match icon_size {
            IconSize::Px16 => html! {
                <svg xmlns = "http://www.w3.org/2000/svg" viewBox = "0 0 16 16" width = svg_size height = svg_size>
                    <path fill-rule = "evenodd" d = "M4.72 3.22a.75.75 0 011.06 1.06L2.06 8l3.72 3.72a.75.75 0 11-1.06 \
                            1.06L.47 8.53a.75.75 0 010-1.06l4.25-4.25zm6.56 0a.75.75 0 10-1.06 1.06L13.94 8l-3.72 \
                            3.72a.75.75 0 101.06 1.06l4.25-4.25a.75.75 0 000-1.06l-4.25-4.25z">
                    </path>
                </svg>
            },
            IconSize::Px24 => html! {
                <svg xmlns = "http://www.w3.org/2000/svg" viewBox = "0 0 24 24" width = svg_size height = svg_size>
                    <path fill-rule = "evenodd" d = "M8.78 4.97a.75.75 0 010 1.06L2.81 12l5.97 5.97a.75.75 0 11-1.06 \
                            1.06l-6.5-6.5a.75.75 0 010-1.06l6.5-6.5a.75.75 0 011.06 0zm6.44 0a.75.75 0 000 1.06L21.19 \
                            12l-5.97 5.97a.75.75 0 101.06 1.06l6.5-6.5a.75.75 0 000-1.06l-6.5-6.5a.75.75 0 00-1.06 0z">
                    </path>
                </svg>
            },
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
                { Self::svg_icon(self.size, IconSize::Px16) }
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

    pub fn svg_icon(svg_size: u32, icon_size: IconSize) -> Html {
        match icon_size {
            IconSize::Px16 => html! {
                <svg xmlns = "http://www.w3.org/2000/svg" viewBox = "0 0 16 16" width = svg_size height = svg_size>
                    <path fill-rule = "evenodd" d = "M7.775 3.275a.75.75 0 001.06 1.06l1.25-1.25a2 2 0 112.83 2.83l-2.5 \
                            2.5a2 2 0 01-2.83 0 .75.75 0 00-1.06 1.06 3.5 3.5 0 004.95 0l2.5-2.5a3.5 3.5 0 00-4.95-4.95l-1.25 \
                            1.25zm-4.69 9.64a2 2 0 010-2.83l2.5-2.5a2 2 0 012.83 0 .75.75 0 001.06-1.06 3.5 3.5 0 00-4.95 \
                            0l-2.5 2.5a3.5 3.5 0 004.95 4.95l1.25-1.25a.75.75 0 00-1.06-1.06l-1.25 1.25a2 2 0 01-2.83 0z">
                    </path>
                </svg>
            },
            IconSize::Px24 => html! {
                <svg xmlns = "http://www.w3.org/2000/svg" viewBox = "0 0 24 24" width = svg_size height = svg_size>
                    <path d = "M14.78 3.653a3.936 3.936 0 115.567 5.567l-3.627 3.627a3.936 3.936 0 01-5.88-.353.75.75 \
                            0 00-1.18.928 5.436 5.436 0 008.12.486l3.628-3.628a5.436 5.436 0 10-7.688-7.688l-3 3a.75.75 \
                            0 001.06 1.061l3-3z">
                    </path>
                    <path d = "M7.28 11.153a3.936 3.936 0 015.88.353.75.75 0 001.18-.928 5.436 5.436 0 \
                            00-8.12-.486L2.592 13.72a5.436 5.436 0 107.688 7.688l3-3a.75.75 0 10-1.06-1.06l-3 \
                            3a3.936 3.936 0 01-5.567-5.568l3.627-3.627z">
                    </path>
                </svg>
            },
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
                { Self::svg_icon(self.size, IconSize::Px16) }
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

    pub fn svg_icon(svg_size: u32, icon_size: IconSize) -> Html {
        match icon_size {
            IconSize::Px16 => html! {
                <svg xmlns = "http://www.w3.org/2000/svg" viewBox = "0 0 16 16" width = svg_size height = svg_size>
                    <path fill-rule = "evenodd" d = "M1.75 2.5a.25.25 0 00-.25.25v10.5c0 .138.112.25.25.25h.94a.76.76 0 \
                            01.03-.03l6.077-6.078a1.75 1.75 0 012.412-.06L14.5 10.31V2.75a.25.25 0 00-.25-.25H1.75zm12.5 \
                            11H4.81l5.048-5.047a.25.25 0 01.344-.009l4.298 3.889v.917a.25.25 0 01-.25.25zm1.75-.25V2.75A1.75 \
                            1.75 0 0014.25 1H1.75A1.75 1.75 0 000 2.75v10.5C0 14.216.784 15 1.75 15h12.5A1.75 1.75 0 0016 \
                            13.25zM5.5 6a.5.5 0 11-1 0 .5.5 0 011 0zM7 6a2 2 0 11-4 0 2 2 0 014 0z">
                    </path>
                </svg>
            },
            IconSize::Px24 => html! {
                <svg xmlns = "http://www.w3.org/2000/svg" viewBox = "0 0 24 24" width = svg_size height = svg_size>
                    <path fill-rule = "evenodd" d = "M2.25 4a.25.25 0 00-.25.25v15.5c0 .138.112.25.25.25h3.178L14 \
                            10.977a1.75 1.75 0 012.506-.032L22 16.44V4.25a.25.25 0 00-.25-.25H2.25zm3.496 17.5H21.75a1.75 \
                            1.75 0 001.75-1.75V4.25a1.75 1.75 0 00-1.75-1.75H2.25A1.75 1.75 0 00.5 4.25v15.5c0 .966.784 \
                            1.75 1.75 1.75h3.496zM22 19.75v-1.19l-6.555-6.554a.25.25 0 00-.358.004L7.497 20H21.75a.25.25 \
                            0 00.25-.25zM9 9.25a1.75 1.75 0 11-3.5 0 1.75 1.75 0 013.5 0zm1.5 0a3.25 3.25 0 11-6.5 0 3.25 \
                            3.25 0 016.5 0z">
                    </path>
                </svg>
            },
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
                { Self::svg_icon(self.size, IconSize::Px16) }
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

    pub fn svg_icon(svg_size: u32, icon_size: IconSize) -> Html {
        match icon_size {
            IconSize::Px16 => html! {
                <svg xmlns = "http://www.w3.org/2000/svg" viewBox = "0 0 16 16" width = svg_size height = svg_size>
                    <path fill-rule = "evenodd" d = "M2 4a1 1 0 100-2 1 1 0 000 2zm3.75-1.5a.75.75 0 000 1.5h8.5a.75.75 \
                            0 000-1.5h-8.5zm0 5a.75.75 0 000 1.5h8.5a.75.75 0 000-1.5h-8.5zm0 5a.75.75 0 000 1.5h8.5a.75.75 \
                            0 000-1.5h-8.5zM3 8a1 1 0 11-2 0 1 1 0 012 0zm-1 6a1 1 0 100-2 1 1 0 000 2z">
                    </path>
                </svg>
            },
            IconSize::Px24 => html! {
                <svg xmlns = "http://www.w3.org/2000/svg" viewBox = "0 0 24 24" width = svg_size height = svg_size>
                    <path fill-rule = "evenodd" d = "M4 7a1 1 0 100-2 1 1 0 000 2zm4.75-1.5a.75.75 0 000 1.5h11.5a.75.75 \
                            0 000-1.5H8.75zm0 6a.75.75 0 000 1.5h11.5a.75.75 0 000-1.5H8.75zm0 6a.75.75 0 000 \
                            1.5h11.5a.75.75 0 000-1.5H8.75zM5 12a1 1 0 11-2 0 1 1 0 012 0zm-1 7a1 1 0 100-2 1 1 0 000 2z">
                    </path>
                </svg>
            },
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
                { Self::svg_icon(self.size, IconSize::Px16) }
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

    pub fn svg_icon(svg_size: u32, icon_size: IconSize) -> Html {
        match icon_size {
            IconSize::Px16 => html! {
                <svg xmlns = "http://www.w3.org/2000/svg" viewBox = "0 0 16 16" width = svg_size height = svg_size>
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
            },
            IconSize::Px24 => html! {
                <svg xmlns = "http://www.w3.org/2000/svg" viewBox = "0 0 24 24" width = svg_size height = svg_size>
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
            },
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
                { Self::svg_icon(self.size, IconSize::Px16) }
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

    pub fn svg_icon(svg_size: u32, icon_size: IconSize) -> Html {
        match icon_size {
            IconSize::Px16 => html! {
                <svg xmlns = "http://www.w3.org/2000/svg" viewBox = "0 0 16 16" width = svg_size height = svg_size>
                    <path fill-rule = "evenodd" d = "M2.5 2.75a.25.25 0 01.25-.25h10.5a.25.25 0 01.25.25v10.5a.25.25 0 \
                            01-.25.25H2.75a.25.25 0 01-.25-.25V2.75zM2.75 1A1.75 1.75 0 001 2.75v10.5c0 .966.784 1.75 \
                            1.75 1.75h10.5A1.75 1.75 0 0015 13.25V2.75A1.75 1.75 0 0013.25 1H2.75zm9.03 5.28a.75.75 0 \
                            00-1.06-1.06L6.75 9.19 5.28 7.72a.75.75 0 00-1.06 1.06l2 2a.75.75 0 001.06 0l4.5-4.5z">
                    </path>
                </svg>
            },
            IconSize::Px24 => html! {
                <svg xmlns = "http://www.w3.org/2000/svg" viewBox = "0 0 24 24" width = svg_size height = svg_size>
                    <path d = "M17.28 9.28a.75.75 0 00-1.06-1.06l-5.97 5.97-2.47-2.47a.75.75 0 00-1.06 1.06l3 3a.75.75 \
                            0 001.06 0l6.5-6.5z">
                    </path>
                    <path fill-rule = "evenodd" d = "M3.75 2A1.75 1.75 0 002 3.75v16.5c0 .966.784 1.75 1.75 \
                            1.75h16.5A1.75 1.75 0 0022 20.25V3.75A1.75 1.75 0 0020.25 2H3.75zM3.5 3.75a.25.25 0 \
                            01.25-.25h16.5a.25.25 0 01.25.25v16.5a.25.25 0 01-.25.25H3.75a.25.25 0 01-.25-.25V3.75z">
                    </path>
                </svg>
            },
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
                { Self::svg_icon(16, IconSize::Px16) }
            </button>
        }
    }
}
