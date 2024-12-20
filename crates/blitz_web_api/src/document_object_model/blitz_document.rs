use blitz_dom::Document;
use std::cell::RefCell;

thread_local! {
    static BLITZ_DOCUMENT: RefCell<Option<Document>> = RefCell::new(None);
}

pub struct BlitzDocument;

impl BlitzDocument {
    pub fn set_document(doc: Document) {
        BLITZ_DOCUMENT.with(|document| {
            *document.borrow_mut() = Some(doc);
        });
    }

    pub fn document() -> &'static Document {
        BLITZ_DOCUMENT.with(|doc| {
            let borrowed_doc = doc.borrow();
            if let Some(ref document) = *borrowed_doc {
                unsafe { std::mem::transmute::<&Document, &'static Document>(document) }
            } else {
                panic!("BLITZ_DOCUMENT is None");
            }
        })
    }

    pub fn document_mut() -> &'static mut Document {
        BLITZ_DOCUMENT.with(|doc| {
            let mut borrowed_doc = doc.borrow_mut();
            if let Some(ref mut document) = *borrowed_doc {
                unsafe { std::mem::transmute::<&mut Document, &'static mut Document>(document) }
            } else {
                panic!("BLITZ_DOCUMENT is None");
            }
        })
    }

    pub fn document_take() -> Document {
        BLITZ_DOCUMENT.with(|doc| {
            let mut borrowed_doc = doc.borrow_mut();
            if let Some(document) = borrowed_doc.take() {
                document
            } else {
                panic!("BLITZ_DOCUMENT is None");
            }
        })
    }
}
