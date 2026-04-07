use std::{sync::Arc, thread, time::Duration};

use glam::uvec2;
use pollster::FutureExt;
use winit::{
    application::ApplicationHandler,
    event::{ElementState, MouseButton, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::KeyCode,
    platform::wayland::WindowAttributesExtWayland,
    window::{Cursor, CursorIcon, Fullscreen, Window, WindowAttributes, WindowId},
};

use crate::graphics::Graphics;

pub struct App {
    graphics: Graphics,
    window: Option<Arc<Window>>,
    cursor_icon_idx: usize,
}

impl App {
    pub fn new(event_loop: &EventLoop<()>) -> anyhow::Result<Self> {
        let graphics = Graphics::new(event_loop.owned_display_handle()).block_on()?;

        Ok(Self {
            graphics,
            window: None,
            cursor_icon_idx: 0,
        })
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attrs = WindowAttributes::default()
            .with_name("meowmeow_test", "")
            .with_fullscreen(Some(Fullscreen::Borderless(None)));

        let window = Arc::new(
            event_loop
                .create_window(attrs)
                .expect("Failed to create window"),
        );

        self.graphics
            .attach_window(window.clone())
            .expect("Failed to attach the window");

        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let window = match &self.window {
            Some(window) if window.id() == window_id => window,
            _ => return,
        };

        match event {
            WindowEvent::RedrawRequested => {
                let size = window.inner_size();

                // Throttle frame rendering a bit to force the condition
                // Perhaps i should've done this in a more controlled fashion?
                thread::sleep(Duration::from_millis(13));

                self.graphics
                    .draw(uvec2(size.width, size.height))
                    .expect("Failed to draw");

                window.request_redraw();
            }

            // Case 1: cursor visibility changes
            WindowEvent::MouseInput { state, button, .. } => {
                if button == MouseButton::Left {
                    window.set_cursor_visible(!state.is_pressed());
                }
            }

            // Case 2: cursor bitmap update
            WindowEvent::KeyboardInput { event, .. } => {
                if event.physical_key == KeyCode::Space
                    && event.state == ElementState::Pressed
                    && !event.repeat
                {
                    self.cursor_icon_idx = (self.cursor_icon_idx + 1) % 3;

                    let icon = match self.cursor_icon_idx {
                        1 => CursorIcon::Pointer,
                        2 => CursorIcon::Text,
                        _ => CursorIcon::Default,
                    };

                    window.set_cursor(Cursor::Icon(icon));
                }
            }

            WindowEvent::CloseRequested => event_loop.exit(),

            _ => {}
        }
    }
}
