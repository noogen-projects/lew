pub use self::rich::*;
pub use self::simple::*;

mod rich;
mod simple;

pub trait Widget {
    fn build(&self) -> yew::Html;
}
