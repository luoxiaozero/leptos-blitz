mod leptos_application;
mod leptos_document;

use crate::_leptos::into_view::IntoView;
use blitz_net::Provider;
use blitz_shell::{create_default_event_loop, BlitzEvent, BlitzShellNetCallback, WindowConfig};
use leptos_application::LeptosNativeApplication;
use leptos_document::LeptosDocument;

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
    let net_provider = {
        let proxy = event_loop.create_proxy();
        let net_callback = BlitzShellNetCallback::shared(proxy);
        let net_provider = Provider::shared(net_callback);

        Some(net_provider)
    };

    let doc = LeptosDocument::new(&rt, f, net_provider);
    let window = WindowConfig::new(doc);

    // // Create application
    let mut application = LeptosNativeApplication::new(event_loop.create_proxy());
    application.add_window(window);

    // // Run event loop
    event_loop.run_app(&mut application).unwrap();
}
