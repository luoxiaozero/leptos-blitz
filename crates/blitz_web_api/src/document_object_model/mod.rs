mod comment;
mod document;
mod element;
mod event;
mod event_listener;
mod event_target;
mod node;
mod text;
mod window;

use document::Document;
use std::cell::RefCell;
use window::Window;

thread_local! {
    static BLITZ_DOCUMENT: RefCell<Option<blitz_dom::Document>> = RefCell::new(None);
    static WINDOW: RefCell<Option<Window>> = RefCell::new(None);
}

pub fn set_blitz_document(doc: blitz_dom::Document) {
    BLITZ_DOCUMENT.with(|document| {
        *document.borrow_mut() = Some(doc);
    });
}

pub fn blitz_document() -> &'static blitz_dom::Document {
    BLITZ_DOCUMENT.with(|doc| {
        let borrowed_doc = doc.borrow();
        if let Some(ref document) = *borrowed_doc {
            unsafe {
                std::mem::transmute::<&blitz_dom::Document, &'static blitz_dom::Document>(document)
            }
        } else {
            panic!("Document is None");
        }
    })
}

pub fn blitz_document_mut() -> &'static mut blitz_dom::Document {
    BLITZ_DOCUMENT.with(|doc| {
        let mut borrowed_doc = doc.borrow_mut();
        if let Some(ref mut document) = *borrowed_doc {
            unsafe {
                std::mem::transmute::<&mut blitz_dom::Document, &'static mut blitz_dom::Document>(
                    document,
                )
            }
        } else {
            panic!("Document is None");
        }
    })
}

pub fn blitz_document_take() -> blitz_dom::Document {
    BLITZ_DOCUMENT.with(|doc| {
        let mut borrowed_doc = doc.borrow_mut();
        if let Some(document) = borrowed_doc.take() {
            document
        } else {
            panic!("Document is None");
        }
    })
}

pub fn window() -> Window {
    WINDOW.with(|window| {
        let mut borrowed_window = window.borrow_mut();
        if borrowed_window.is_none() {
            let doc_id = blitz_document().root_node().id;
            *borrowed_window = Some(Window::new(Document::from(doc_id)));
        }
        if let Some(ref window) = *borrowed_window {
            window.clone()
        } else {
            unreachable!();
        }
    })
}
