mod leptos_application;
mod leptos_document;
pub mod web_document;

use crate::_leptos::into_view::IntoView;
use blitz_dom::net::Resource;
use blitz_net::Provider;
use blitz_shell::{create_default_event_loop, BlitzEvent, BlitzShellNetCallback, WindowConfig};
use blitz_traits::net::SharedCallback;
use leptos_application::LeptosNativeApplication;
use leptos_document::LeptosDocument;
use std::sync::Arc;

// blitz launch_cfg_with_props
pub fn launch<F, N>(f: F)
where
    F: FnOnce() -> N + 'static,
    N: IntoView + 'static,
{
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let _guard = rt.enter();

    let event_loop = create_default_event_loop::<BlitzEvent>();
    let proxy = event_loop.create_proxy();

    let net_callback = Arc::new(BlitzShellNetCallback::new(proxy));
    let net_provider = Arc::new(Provider::new(
        rt.handle().clone(),
        Arc::clone(&net_callback) as SharedCallback<Resource>,
    ));

    let doc = LeptosDocument::new(&rt, f, Some(net_provider));
    let window = WindowConfig::new(doc);

    // // Create application
    let mut application = LeptosNativeApplication::new(rt, event_loop.create_proxy());
    application.add_window(window);

    // // Run event loop
    event_loop.run_app(&mut application).unwrap();
}
