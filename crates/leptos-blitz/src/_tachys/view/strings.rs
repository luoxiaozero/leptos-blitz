use super::{Mountable, Position, Render, RenderHtml};
use crate::_tachys::renderer::{types, Rndr};
use crate::no_attrs;
use std::{borrow::Cow, rc::Rc, sync::Arc};

no_attrs!(&'a str);
no_attrs!(String);
no_attrs!(Arc<str>);
no_attrs!(Cow<'a, str>);

/// Retained view state for `&str`.
pub struct StrState<'a> {
    pub(crate) node: types::Text,
    str: &'a str,
}

impl<'a> Render for &'a str {
    type State = StrState<'a>;

    fn build(self) -> Self::State {
        let node = Rndr::create_text_node(self);
        StrState { node, str: self }
    }

    fn rebuild(self, state: &mut Self::State) {
        let StrState { node, str } = state;
        if &self != str {
            Rndr::set_text(node, self);
            *str = self;
        }
    }
}

impl RenderHtml for &str {
    type AsyncOutput = Self;

    const MIN_LENGTH: usize = 0;

    fn dry_resolve(&mut self) {}

    async fn resolve(self) -> Self::AsyncOutput {
        self
    }

    fn html_len(&self) -> usize {
        self.len()
    }

    fn to_html_with_buf(
        self,
        buf: &mut String,
        position: &mut Position,
        escape: bool,
        _mark_branches: bool,
    ) {
        // add a comment node to separate from previous sibling, if any
        if matches!(position, Position::NextChildAfterText) {
            buf.push_str("<!>")
        }
        if self.is_empty() && escape {
            buf.push(' ');
        } else if escape {
            let escaped = html_escape::encode_text(self);
            buf.push_str(&escaped);
        } else {
            buf.push_str(self);
        }
        *position = Position::NextChildAfterText;
    }
}

// impl ToTemplate for &str {
//     const TEMPLATE: &'static str = " <!>";

//     fn to_template(
//         buf: &mut String,
//         _class: &mut String,
//         _style: &mut String,
//         _inner_html: &mut String,
//         position: &mut Position,
//     ) {
//         if matches!(*position, Position::NextChildAfterText) {
//             buf.push_str("<!>")
//         }
//         buf.push(' ');
//         *position = Position::NextChildAfterText;
//     }
// }

impl Mountable for StrState<'_> {
    fn unmount(&mut self) {
        self.node.unmount()
    }

    fn mount(&mut self, parent: &types::Element, marker: Option<&types::Node>) {
        Rndr::insert_node(parent, &self.node, marker);
    }

    fn insert_before_this(&self, child: &mut dyn Mountable) -> bool {
        self.node.insert_before_this(child)
    }
}

/// Retained view state for `String`.
pub struct StringState {
    node: types::Text,
    str: String,
}

impl Render for String {
    type State = StringState;

    fn build(self) -> Self::State {
        let node = Rndr::create_text_node(&self);
        StringState { node, str: self }
    }

    fn rebuild(self, state: &mut Self::State) {
        let StringState { node, str } = state;
        if &self != str {
            Rndr::set_text(node, &self);
            *str = self;
        }
    }
}

impl RenderHtml for String {
    const MIN_LENGTH: usize = 0;
    type AsyncOutput = Self;

    fn dry_resolve(&mut self) {}

    async fn resolve(self) -> Self::AsyncOutput {
        self
    }

    fn html_len(&self) -> usize {
        self.len()
    }

    fn to_html_with_buf(
        self,
        buf: &mut String,
        position: &mut Position,
        escape: bool,
        mark_branches: bool,
    ) {
        <&str as RenderHtml>::to_html_with_buf(self.as_str(), buf, position, escape, mark_branches)
    }
}

// impl ToTemplate for String {
//     const TEMPLATE: &'static str = <&str as ToTemplate>::TEMPLATE;

//     fn to_template(
//         buf: &mut String,
//         class: &mut String,
//         style: &mut String,
//         inner_html: &mut String,
//         position: &mut Position,
//     ) {
//         <&str as ToTemplate>::to_template(
//             buf, class, style, inner_html, position,
//         )
//     }
// }

impl Mountable for StringState {
    fn unmount(&mut self) {
        self.node.unmount()
    }

    fn mount(&mut self, parent: &types::Element, marker: Option<&types::Node>) {
        Rndr::insert_node(parent, &self.node, marker);
    }

    fn insert_before_this(&self, child: &mut dyn Mountable) -> bool {
        self.node.insert_before_this(child)
    }
}

/// Retained view state for `Rc<str>`.
pub struct RcStrState {
    node: types::Text,
    str: Rc<str>,
}

impl Render for Rc<str> {
    type State = RcStrState;

    fn build(self) -> Self::State {
        let node = Rndr::create_text_node(&self);
        RcStrState { node, str: self }
    }

    fn rebuild(self, state: &mut Self::State) {
        let RcStrState { node, str } = state;
        if !Rc::ptr_eq(&self, str) {
            Rndr::set_text(node, &self);
            *str = self;
        }
    }
}

// can't Send an Rc<str> between threads, so can't implement async HTML rendering that might need
// to send it
/*
impl RenderHtml for Rc<str>
where

{
    type AsyncOutput = Self;

    const MIN_LENGTH: usize = 0;

    async fn resolve(self) -> Self::AsyncOutput {
    self
    }

    fn html_len(&self) -> usize {
        self.len()
    }

    fn to_html_with_buf(self, buf: &mut String, position: &mut Position, escape: bool, mark_branches: bool) {
        <&str as RenderHtml>::to_html_with_buf(&self, buf, position)
    }

    fn hydrate<const FROM_SERVER: bool>(
        self,
        cursor: &Cursor,
        position: &PositionState,
    ) -> Self::State {
        let this: &str = self.as_ref();
        let StrState { node, .. } =
            this.hydrate::<FROM_SERVER>(cursor, position);
        RcStrState { node, str: self }
    }
}*/

// impl ToTemplate for Rc<str> {
//     const TEMPLATE: &'static str = <&str as ToTemplate>::TEMPLATE;

//     fn to_template(
//         buf: &mut String,
//         class: &mut String,
//         style: &mut String,
//         inner_html: &mut String,
//         position: &mut Position,
//     ) {
//         <&str as ToTemplate>::to_template(
//             buf, class, style, inner_html, position,
//         )
//     }
// }

impl Mountable for RcStrState {
    fn unmount(&mut self) {
        self.node.unmount()
    }

    fn mount(&mut self, parent: &types::Element, marker: Option<&types::Node>) {
        Rndr::insert_node(parent, &self.node, marker);
    }

    fn insert_before_this(&self, child: &mut dyn Mountable) -> bool {
        self.node.insert_before_this(child)
    }
}

/// Retained view state for `Arc<str>`.
pub struct ArcStrState {
    node: types::Text,
    str: Arc<str>,
}

impl Render for Arc<str> {
    type State = ArcStrState;

    fn build(self) -> Self::State {
        let node = Rndr::create_text_node(&self);
        ArcStrState { node, str: self }
    }

    fn rebuild(self, state: &mut Self::State) {
        let ArcStrState { node, str } = state;
        if !Arc::ptr_eq(&self, str) {
            Rndr::set_text(node, &self);
            *str = self;
        }
    }
}

impl RenderHtml for Arc<str> {
    type AsyncOutput = Self;

    const MIN_LENGTH: usize = 0;

    fn dry_resolve(&mut self) {}

    async fn resolve(self) -> Self::AsyncOutput {
        self
    }

    fn html_len(&self) -> usize {
        self.len()
    }

    fn to_html_with_buf(
        self,
        buf: &mut String,
        position: &mut Position,
        escape: bool,
        mark_branches: bool,
    ) {
        <&str as RenderHtml>::to_html_with_buf(&self, buf, position, escape, mark_branches)
    }
}

// impl ToTemplate for Arc<str> {
//     const TEMPLATE: &'static str = <&str as ToTemplate>::TEMPLATE;

//     fn to_template(
//         buf: &mut String,
//         class: &mut String,
//         style: &mut String,
//         inner_html: &mut String,
//         position: &mut Position,
//     ) {
//         <&str as ToTemplate>::to_template(
//             buf, class, style, inner_html, position,
//         )
//     }
// }

impl Mountable for ArcStrState {
    fn unmount(&mut self) {
        self.node.unmount()
    }

    fn mount(&mut self, parent: &types::Element, marker: Option<&types::Node>) {
        Rndr::insert_node(parent, &self.node, marker);
    }

    fn insert_before_this(&self, child: &mut dyn Mountable) -> bool {
        self.node.insert_before_this(child)
    }
}

/// Retained view state for `Cow<'_, str>`.
pub struct CowStrState<'a> {
    node: types::Text,
    str: Cow<'a, str>,
}

impl<'a> Render for Cow<'a, str> {
    type State = CowStrState<'a>;

    fn build(self) -> Self::State {
        let node = Rndr::create_text_node(&self);
        CowStrState { node, str: self }
    }

    fn rebuild(self, state: &mut Self::State) {
        let CowStrState { node, str } = state;
        if self != *str {
            Rndr::set_text(node, &self);
            *str = self;
        }
    }
}

impl RenderHtml for Cow<'_, str> {
    type AsyncOutput = Self;

    const MIN_LENGTH: usize = 0;

    fn dry_resolve(&mut self) {}

    async fn resolve(self) -> Self::AsyncOutput {
        self
    }

    fn html_len(&self) -> usize {
        self.len()
    }

    fn to_html_with_buf(
        self,
        buf: &mut String,
        position: &mut Position,
        escape: bool,
        mark_branches: bool,
    ) {
        <&str as RenderHtml>::to_html_with_buf(&self, buf, position, escape, mark_branches)
    }
}

// impl ToTemplate for Cow<'_, str> {
//     const TEMPLATE: &'static str = <&str as ToTemplate>::TEMPLATE;

//     fn to_template(
//         buf: &mut String,
//         class: &mut String,
//         style: &mut String,
//         inner_html: &mut String,
//         position: &mut Position,
//     ) {
//         <&str as ToTemplate>::to_template(
//             buf, class, style, inner_html, position,
//         )
//     }
// }

impl Mountable for CowStrState<'_> {
    fn unmount(&mut self) {
        self.node.unmount()
    }

    fn mount(&mut self, parent: &types::Element, marker: Option<&types::Node>) {
        Rndr::insert_node(parent, &self.node, marker);
    }

    fn insert_before_this(&self, child: &mut dyn Mountable) -> bool {
        self.node.insert_before_this(child)
    }
}
