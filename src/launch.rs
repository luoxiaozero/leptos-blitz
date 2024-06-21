use crate::{
    documents::LeptosDocument,
    dom::IntoView,
    waker::{EventData, UserWindowEvent},
    window,
    window::View,
};
use blitz::RenderState;
use blitz_dom::DocumentLike;
use muda::{MenuEvent, MenuId};
use std::collections::HashMap;
use winit::event_loop::EventLoop;
use winit::window::WindowId;
use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
};

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

    let document = LeptosDocument::new(&rt, f);
    let window = View::new(document);
    launch_with_window(rt, window);
}

fn launch_with_window<Doc: DocumentLike + 'static>(
    rt: tokio::runtime::Runtime,
    window: View<'static, Doc>,
) {
    // Build an event loop for the application
    let event_loop = EventLoop::<UserWindowEvent>::with_user_event()
        .build()
        .unwrap();
    let proxy = event_loop.create_proxy();

    // Multiwindow ftw
    let mut windows: HashMap<WindowId, window::View<'_, Doc>> = HashMap::new();
    let mut pending_windows = Vec::new();

    pending_windows.push(window);
    let menu_channel = MenuEvent::receiver();

    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    let mut initial = true;

    // the move to winit wants us to use a struct with a run method instead of the callback approach
    // we want to just keep the callback approach for now
    #[allow(deprecated)]
    event_loop
        .run(move |event, event_loop| {
            event_loop.set_control_flow(ControlFlow::Wait);

            let mut on_resume = || {
                for (_, view) in windows.iter_mut() {
                    view.resume(event_loop, &proxy, &rt);
                }

                for view in pending_windows.iter_mut() {
                    view.resume(event_loop, &proxy, &rt);
                }

                for window in pending_windows.drain(..) {
                    let RenderState::Active(state) = &window.renderer.render_state else {
                        continue;
                    };
                    windows.insert(state.window.id(), window);
                }
            };

            #[cfg(not(any(target_os = "android", target_os = "ios")))]
            if initial {
                on_resume();
                initial = false;
            }

            match event {
                // Exit the app when close is request
                // Not always necessary
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => event_loop.exit(),

                Event::WindowEvent {
                    window_id,
                    event: winit::event::WindowEvent::RedrawRequested,
                } => {
                    if let Some(window) = windows.get_mut(&window_id) {
                        window.renderer.dom.as_mut().resolve();
                        window.renderer.render(&mut window.scene);
                    };
                }

                Event::UserEvent(UserWindowEvent(EventData::Poll, id)) => {
                    if let Some(view) = windows.get_mut(&id) {
                        if view.poll() {
                            view.request_redraw();
                        }
                    };
                }
                // Event::UserEvent(_redraw) => {
                //     for (_, view) in windows.iter() {
                //         view.request_redraw();
                //     }
                // }
                Event::NewEvents(_) => {
                    for id in windows.keys() {
                        _ = proxy.send_event(UserWindowEvent(EventData::Poll, *id));
                    }
                }

                Event::Suspended => {
                    for (_, view) in windows.iter_mut() {
                        view.suspend();
                    }
                }

                Event::Resumed => on_resume(),

                Event::WindowEvent {
                    window_id, event, ..
                } => {
                    if let Some(window) = windows.get_mut(&window_id) {
                        window.handle_window_event(event);
                    };
                }

                _ => (),
            }

            if let Ok(event) = menu_channel.try_recv() {
                if event.id == MenuId::new("dev.show_layout") {
                    for (_, view) in windows.iter_mut() {
                        view.renderer.devtools.show_layout = !view.renderer.devtools.show_layout;
                        view.request_redraw();
                    }
                }
            }
        })
        .unwrap();
}
