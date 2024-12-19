use super::LeptosDocument;
use blitz_shell::{BlitzApplication, BlitzEvent, WindowConfig};
use winit::{
    application::ApplicationHandler,
    event::StartCause,
    event_loop::{ActiveEventLoop, EventLoopProxy},
};

pub struct LeptosNativeApplication {
    inner: BlitzApplication<LeptosDocument>,
}

impl LeptosNativeApplication {
    pub fn new(rt: tokio::runtime::Runtime, proxy: EventLoopProxy<BlitzEvent>) -> Self {
        Self {
            inner: BlitzApplication::new(rt, proxy.clone()),
        }
    }

    pub fn add_window(&mut self, window_config: WindowConfig<LeptosDocument>) {
        self.inner.add_window(window_config);
    }
}

impl ApplicationHandler<BlitzEvent> for LeptosNativeApplication {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.inner.resumed(event_loop);
    }

    fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        self.inner.suspended(event_loop);
    }

    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: StartCause) {
        self.inner.new_events(event_loop, cause);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        self.inner.window_event(event_loop, window_id, event);
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: BlitzEvent) {
        match event {
            BlitzEvent::Embedder(any) => {
                // TODo
                // if let Some(event) = event.downcast_ref::<DioxusNativeEvent>() {
                //     self.handle_blitz_shell_event(event_loop, event);
                // }
            }
            event => self.inner.user_event(event_loop, event),
        }
    }
}
