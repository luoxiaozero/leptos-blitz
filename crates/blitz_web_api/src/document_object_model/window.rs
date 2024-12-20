use super::document::Document;

#[derive(Debug, Clone)]
pub struct Window {
    doc: Document,
}

impl Window {
    pub(super) fn new(doc: Document) -> Self {
        Self { doc }
    }
}

impl Window {
    pub fn document(&self) -> &Document {
        &self.doc
    }
}
