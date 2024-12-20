mod blitz_document;
mod comment;
mod document;
mod element;
mod event;
mod event_listener;
mod event_target;
mod node;
mod text;
mod window;

pub use blitz_document::*;
pub use comment::*;
pub use document::*;
pub use element::*;
pub use node::*;
pub use text::*;
pub use window::*;

use std::cell::RefCell;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomError {
    #[error("The {0} is not of {1} type")]
    Type(&'static str, &'static str),
}

thread_local! {
    static WINDOW: RefCell<Option<Window>> = RefCell::new(None);
}

pub fn window() -> Window {
    WINDOW.with(|window| {
        let mut borrowed_window = window.borrow_mut();
        if borrowed_window.is_none() {
            let doc_id = blitz_document::BlitzDocument::document().root_node().id;
            *borrowed_window = Some(Window::new(Document::from(doc_id)));
        }
        if let Some(ref window) = *borrowed_window {
            window.clone()
        } else {
            unreachable!();
        }
    })
}
