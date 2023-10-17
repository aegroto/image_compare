use winit::event::VirtualKeyCode;

const OFFSET_SPEED: i32 = 10;

#[derive(Debug)]
pub struct ImageFocus {
    pub x_offset: i32,
    pub y_offset: i32,
}

impl ImageFocus {
    pub fn new() -> Self {
        Self {
            x_offset: 0,
            y_offset: 0,
        }
    }

    pub fn update_on_key_input(&mut self, code: VirtualKeyCode) -> bool {
        match code {
            VirtualKeyCode::W => self.y_offset += OFFSET_SPEED,
            VirtualKeyCode::S => self.y_offset -= OFFSET_SPEED,
            VirtualKeyCode::A => self.x_offset += OFFSET_SPEED,
            VirtualKeyCode::D => self.x_offset -= OFFSET_SPEED,
            _ => return false,
        }
        true
    }
}
