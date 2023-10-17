use winit::event::VirtualKeyCode;

const OFFSET_SPEED: i32 = 10;
const ZOOM_SPEED: f32 = 0.2;

#[derive(Debug)]
pub struct ImageFocus {
    pub x_offset: i32,
    pub y_offset: i32,
    pub zoom: f32,
}

impl ImageFocus {
    pub fn new() -> Self {
        Self {
            x_offset: 0,
            y_offset: 0,
            zoom: 1.0,
        }
    }

    pub fn update_on_key_input(&mut self, code: VirtualKeyCode) -> bool {
        match code {
            VirtualKeyCode::W => self.y_offset += OFFSET_SPEED,
            VirtualKeyCode::S => self.y_offset -= OFFSET_SPEED,
            VirtualKeyCode::A => self.x_offset += OFFSET_SPEED,
            VirtualKeyCode::D => self.x_offset -= OFFSET_SPEED,
            VirtualKeyCode::Plus => self.zoom += ZOOM_SPEED,
            VirtualKeyCode::Minus => self.zoom -= ZOOM_SPEED,
            VirtualKeyCode::R => {
                self.x_offset = 0;
                self.y_offset = 0;
                self.zoom = 1.0;
            },
            _ => return false,
        }
        true
    }
}
