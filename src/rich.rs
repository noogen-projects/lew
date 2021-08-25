#![allow(dead_code)]

use yew::{html, utils, Component, ComponentLink, Html, Properties};

pub struct RichEditor {
    id: String,
    class: String,
    caret_index: usize,
    text: String,
}

#[derive(Clone, Properties, Default)]
pub struct RichEditorProps {
    pub id: String,
    pub class: String,
    pub caret_index: usize,
    pub text: String,
}

impl Component for RichEditor {
    type Message = ();
    type Properties = RichEditorProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            id: props.id,
            class: props.class,
            caret_index: props.caret_index,
            text: props.text,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        false
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.id = props.id;
        self.class = props.class;
        self.caret_index = props.caret_index;
        self.text = props.text;
        true
    }

    fn view(&self) -> Html {
        self.update_content();
        html! {
            <div id = self.id.clone() class = self.class.clone()>
                <div class = "lew-rich">
                    { self.generate_content() }
                </div>
                <span class = "lew-rich-blinking-cursor">{ "|" }</span>
            </div>
        }
    }
}

impl RichEditor {
    fn text_before_caret(&self) -> &str {
        if self.caret_index == 0 {
            ""
        } else {
            &self.text[..self.caret_index]
        }
    }

    fn text_after_caret(&self) -> &str {
        if self.caret_index == self.text.len() {
            ""
        } else {
            &self.text[self.caret_index..]
        }
    }

    fn generate_content(&self) -> Html {
        html! {
            <>{ self.text_before_caret() }<span class = "lew-rich-cursor-placeholder">{ "|" }</span>{ self.text_after_caret() }</>
        }
    }

    fn update_content(&self) {
        let _document = utils::document();
        // let editor = if self.id.is_empty() {
        //     document.get_elements_by_class_name("lew-editor").item(0)
        // } else {
        //     document
        //         .get_element_by_id(&self.id)
        //         .and_then(|el| el.first_element_child())
        // }
        // .expect("Cannot find editor element");

        todo!()
    }

    fn type_item(&mut self, item: &str) {
        self.text.insert_str(self.caret_index, item);
        self.caret_index += item.len();
    }

    fn before_char_len(&self) -> usize {
        self.text_before_caret()
            .chars()
            .next_back()
            .map(|ch| ch.len_utf8())
            .unwrap_or(0)
    }

    fn after_char_len(&self) -> usize {
        self.text_after_caret()
            .chars()
            .next()
            .map(|ch| ch.len_utf8())
            .unwrap_or(0)
    }

    fn delete_char_before_caret(&mut self) -> bool {
        if self.caret_index > 0 {
            self.caret_index -= self.before_char_len();
            self.text.remove(self.caret_index);
            true
        } else {
            false
        }
    }

    fn move_before(&mut self) -> bool {
        if self.caret_index == 0 {
            false
        } else {
            self.caret_index -= self.before_char_len();
            true
        }
    }

    fn move_after(&mut self) -> bool {
        if self.caret_index == self.text.len() {
            false
        } else {
            self.caret_index += self.after_char_len();
            true
        }
    }
}
