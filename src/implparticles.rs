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
        if (frame[index + 2]) == 150 {
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

impl BaseParticle for IronParticle {
    fn move_particle(&mut self, frame: &mut [u8]) {
        if self.colision(frame) {
            return;
        }
    }
    fn colision(&self, _frame: &mut [u8]) -> bool {
        return false;
    }
}

impl BaseParticle for AcidParticle{
    //Casos para mover líquidos:
    //Mover para baixo se possível.
    //Mover para baixo e depois para esquerda ou direita aleatoriamente.
    //Não ser possível mais descer, logo, mover para a esquerda ou direita aleatoriamente.

    //Captar quais particulas o ácido pode dissolver.
    // fn acid_dissolves(&self) -> bool {
    // }

    //Atualmente tem o mesmo comportamento da areia, mas apenas pra não dar ruim na hora de rodar o cargo run. Comportamento ainda será implementado.
    fn move_particle(&mut self, frame: &mut [u8]){
        if self.colision(frame) {
            return;
        }
        let mut index: usize = position_to_index(self.x, self.y + 1);
        if (frame[index + 2]) != 150{
            if self.x != 0{
                index = position_to_index(self.x -1, self.y + 1);
                if frame[index + 2] == 150{
                    self.y += 1;
                    self.x -= 1;
                    return;
                }
            }
            if self.x != WIDTH-1{
                index = position_to_index(self.x +1, self.y + 1);
                if frame[index + 2] == 150{
                    self.y += 1;
                    self.x += 1;
                    return;
                }
            }
        }
        else{
            self.y += 1;
            return;
        }
    }
    fn colision(&self, frame: &mut [u8]) -> bool {
        if self.y + 1 >= HEIGHT {
            return true;
        }
        return false;
    }
}