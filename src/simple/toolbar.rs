use yew::{html, Html};

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

pub mod tool {
    use yew::{html, Html};

    use crate::Widget;

    pub struct Bold {
        pub class: String,
        pub size: u32,
    }

    impl Default for Bold {
        fn default() -> Self {
            Self {
                class: "lew-simple__tool_bold".to_string(),
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
            html! {
                <button class = &self.class type = "button" style = format!("width: {0}; height: {0}", self.size)>
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
