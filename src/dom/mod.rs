pub mod html;
mod into_view;
pub mod renderer;

pub use into_view::*;

use blitz_dom::{namespace_url, ns, Atom, QualName};

pub(self) fn qual_name(local_name: &str, namespace: Option<&str>) -> QualName {
    QualName {
        prefix: None,
        ns: namespace.map(Atom::from).unwrap_or(ns!(html)),
        local: Atom::from(local_name),
    }
}
