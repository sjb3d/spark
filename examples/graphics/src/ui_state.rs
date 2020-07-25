use std::time::{Duration, Instant};
use winit::event::{ElementState, MouseButton, WindowEvent};

trait AsFSecs {
    fn as_f32_secs(&self) -> f32;
}

impl AsFSecs for Duration {
    fn as_f32_secs(&self) -> f32 {
        (self.as_secs() as f32) + (self.subsec_nanos() as f32) / 1_000_000_000.0
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct MouseState {
    pos: (i32, i32),
    pressed: (bool, bool, bool),
}

pub struct UiState {
    pub context: imgui::Context,
    mouse_state: MouseState,
    last_instant: Instant,
    last_delta_time: f32,
}

impl UiState {
    pub fn new() -> Self {
        let mut context = imgui::Context::create();
        context.fonts().add_font(&[imgui::FontSource::DefaultFontData {
            config: Some(imgui::FontConfig {
                size_pixels: 13.0,
                ..Default::default()
            }),
        }]);

        let mouse_state = MouseState::default();
        let last_instant = Instant::now();

        Self {
            context,
            mouse_state,
            last_instant,
            last_delta_time: 0f32,
        }
    }

    pub fn process_window_event(&mut self, event: &WindowEvent) {
        match *event {
            WindowEvent::CursorMoved { position: pos, .. } => {
                self.mouse_state.pos = pos.into();
            }
            WindowEvent::MouseInput { state, button, .. } => match button {
                MouseButton::Left => self.mouse_state.pressed.0 = state == ElementState::Pressed,
                MouseButton::Right => self.mouse_state.pressed.1 = state == ElementState::Pressed,
                MouseButton::Middle => self.mouse_state.pressed.2 = state == ElementState::Pressed,
                _ => {}
            },
            _ => {}
        }
    }

    pub fn start_ui(&mut self, width: u32, height: u32) -> imgui::Ui {
        let io = self.context.io_mut();
        io.display_size = [width as f32, height as f32];
        io.display_framebuffer_scale = [1.0f32, 1.0f32];
        io.mouse_pos = [self.mouse_state.pos.0 as f32, self.mouse_state.pos.1 as f32];
        io.mouse_down = [
            self.mouse_state.pressed.0,
            self.mouse_state.pressed.1,
            self.mouse_state.pressed.2,
            false,
            false,
        ];
        let new_instant = io.update_delta_time(self.last_instant);
        let delta_time = new_instant.duration_since(self.last_instant).as_f32_secs();
        self.last_instant = new_instant;
        self.last_delta_time = delta_time;
        self.context.frame()
    }
}
