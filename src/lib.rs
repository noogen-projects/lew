pub use self::{rich::*, simple::*};

mod rich;
mod simple;

pub trait Widget {
    fn build(&self) -> yew::Html;
}
