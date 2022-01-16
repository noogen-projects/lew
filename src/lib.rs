pub use self::{rich::*, simple::*};

pub mod dom;
mod rich;
mod simple;

pub trait Widget {
    fn build(&self) -> yew::Html;
}
