use web_sys::{Document, Window};

#[track_caller]
pub fn window() -> Window {
    web_sys::window().expect("Can't get Window")
}

#[track_caller]
pub fn document() -> Document {
    window().document().expect("Can't get Document")
}
