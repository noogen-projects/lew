use lew::{toolbar::textarea_selection, SimpleEditor, SimpleToolbar, Widget};
use pulldown_cmark::{html as cmark_html, Options, Parser};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{html, utils, Callback, Component, ComponentLink, Html, InputData, MouseEvent};

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
                    set_preview(&text);
                }
            } else {
                set_preview("");
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
        html! {
            <div>
                <div id = PREVIEW_ID>
                </div>
                <SimpleEditor id = EDITOR_ID toolbar = self.toolbar.build() placeholder = "Leave a comment" oninput = Callback::from(editor_input) />
            </div>
        }
    }
}

fn new_cmark_parser(text: &str) -> Parser {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    Parser::new_ext(text, options)
}

fn set_preview(text: &str) {
    let preview = utils::document()
        .get_element_by_id(PREVIEW_ID)
        .expect("Preview container expected");

    let parser = new_cmark_parser(text);

    let mut html_output = String::new();
    cmark_html::push_html(&mut html_output, parser);

    preview.set_inner_html(&html_output);
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
        set_preview(&data.value);
    }
}

fn main() {
    yew::start_app::<Root>();
}
