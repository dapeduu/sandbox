use crate::*;
pub static WIDTH: u32 = 200;
pub static HEIGHT: u32 = 150;

pub fn position_to_index(x: u32, y: u32) -> usize {
    return ((y * WIDTH + x) * 4) as usize;
}

impl BaseParticle for Particle {
    fn move_particle(&mut self, frame: &mut [u8]) {
        if self.colision(frame) {
            return;
        }
        self.y += 1
    }

    fn colision(&self, frame: &mut [u8]) -> bool {
        if self.y + 1 >= HEIGHT {
            return true;
        }

        let index: usize = position_to_index(self.x, self.y + 1);
        if (frame[index + 2]) != self.rgba[2] {
            return false;
        } else {
            return true;
        }
    }
}

impl BaseParticle for SandParticle {
    fn move_particle(&mut self, frame: &mut [u8]) {
        if self.colision(frame) {
            return;
        }
        let mut index: usize = position_to_index(self.x, self.y + 1);
        if (frame[index + 2]) != 150 {
            if self.x != 0 {
                index = position_to_index(self.x - 1, self.y + 1);
                if frame[index + 2] == 150 {
                    self.y += 1;
                    self.x -= 1;
                    return;
                }
            }
            if self.x != WIDTH - 1 {
                index = position_to_index(self.x + 1, self.y + 1);
                if frame[index + 2] == 150 {
                    self.y += 1;
                    self.x += 1;
                    return;
                }
            }
        } else {
            self.y += 1;
            return;
        }
    }
    fn colision(&self, _frame: &mut [u8]) -> bool {
        if self.y + 1 >= HEIGHT {
            return true;
        }
        return false;
    }
}
