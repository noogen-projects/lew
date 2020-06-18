use lew::{toolbar::textarea_selection, SimpleEditor, SimpleToolbar, Widget};
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlInputElement};
use yew::{html, utils, Component, ComponentLink, Html, InputData, MouseEvent};

const EDITOR_ID: &str = "editor";
const PREVIEW_ID: &str = "preview";
const PREVIEW_CHECKBOX_ID: &str = "preview_checkbox";

struct Preview {
    link: ComponentLink<Root>,
}

impl Widget for Preview {
    fn build(&self) -> Html {
        let click = |_: MouseEvent| {
            if is_preview_enabled() {
                if let Some((_, text, _)) = textarea_selection(format!("#{} > textarea", EDITOR_ID)) {
                    get_preview_container().set_inner_html(&text);
                }
            } else {
                get_preview_container().set_inner_html("");
            }
        };

        html! {
            <div id = "preview_tool">
                <input type = "checkbox" id = PREVIEW_CHECKBOX_ID name = PREVIEW_CHECKBOX_ID
                        onclick = self.link.callback(click)/>
                <label for = PREVIEW_CHECKBOX_ID id = "preview_label">{ "Preview" }</label>
            </div>
        }
    }
}

struct Root {
    toolbar: SimpleToolbar,
}

impl Component for Root {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut toolbar = SimpleToolbar::new();
        toolbar.tools.insert(0, Box::new(Preview { link }));

        Self { toolbar }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let input = editor_input as fn(InputData);
        html! {
            <div>
                <div id = PREVIEW_ID>
                </div>
                <SimpleEditor id = EDITOR_ID toolbar = self.toolbar.build() placeholder = "Leave a comment" oninput = input />
            </div>
        }
    }
}

fn get_preview_container() -> Element {
    utils::document()
        .get_element_by_id(PREVIEW_ID)
        .expect("Preview container expected")
}

fn is_preview_enabled() -> bool {
    utils::document()
        .get_element_by_id(PREVIEW_CHECKBOX_ID)
        .expect("Preview checkbox expected")
        .dyn_into::<HtmlInputElement>()
        .expect("Preview checkbox mus be input element")
        .checked()
}

fn editor_input(data: InputData) {
    if is_preview_enabled() {
        get_preview_container().set_inner_html(&data.value);
    }
}

fn main() {
    yew::start_app::<Root>();
}
