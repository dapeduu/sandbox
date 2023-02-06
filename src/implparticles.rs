use winit::event::ElementState;
use rand::Rng;
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

impl BaseParticle for WaterParticle {
    // Move para baixo se possível
    // Caso contrário, move aleatoriamente para esquerda ou direita
    // Objetivo: preencher todos os espaços do nível inferior

        fn move_particle(&mut self, frame: &mut [u8]){
            if self.colision(frame) {
                return;
            }
            let index_down = position_to_index(self.x, self.y + 1);
            if frame[index_down + 2] == 150 {
                self.y += 1;
            } else {
                let mut new_x = self.x;
                let mut new_y = self.y;
                let direction = rand::thread_rng().gen_range(0, 2);
                if self.x > 0 && direction == 0 {
                    let index_left = position_to_index(self.x - 1, self.y);
                    if frame[index_left + 2] == 150 {
                        new_x = self.x - 1;
                        new_y = self.y;
                    }
                }
                if self.x < WIDTH - 1 && direction == 1 {
                    let index_right = position_to_index(self.x + 1, self.y);
                    if frame[index_right + 2] == 150 {
                        new_x = self.x + 1;
                        new_y = self.y;
                    }
                }
                if new_x != self.x || new_y != self.y {
                    self.x = new_x;
                    self.y = new_y;
                }
            }



        }
    
        fn colision(&self, frame: &mut [u8]) -> bool {
            if self.y + 1 >= HEIGHT {
                return true;
            }
            return false;
    
        }
    
}

impl BaseParticle for AgitatedParticle {
    //Inspirada no comportamento da partícula de água.
    fn move_particle(&mut self, frame: &mut [u8]){
        let direction = rand::thread_rng().gen_range(0, 4);
        let mut new_x = self.x;
        let mut new_y = self.y;
        if direction == 0 && self.x > 0 {
            let index_left = position_to_index(self.x - 1, self.y);
            if frame[index_left + 2] == 150 {
                new_x = self.x - 1;
                new_y = self.y;
            }
        }
        if direction == 1 && self.x < WIDTH - 1 {
            let index_right = position_to_index(self.x + 1, self.y);
            if frame[index_right + 2] == 150 {
                new_x = self.x + 1;
                new_y = self.y;
            }
        }
        if direction == 2 && self.y > 0 {
            let index_up = position_to_index(self.x, self.y - 1);
            if frame[index_up + 2] == 150 {
                new_x = self.x;
                new_y = self.y - 1;
            }
        }
        if direction == 3 && self.y < HEIGHT - 1 {
            let index_down = position_to_index(self.x, self.y + 1);
            if frame[index_down + 2] == 150 {
                new_x = self.x;
                new_y = self.y + 1;
            }
        }
        if new_x != self.x || new_y != self.y {
            self.x = new_x;
            self.y = new_y;
        }
    }

    fn colision(&self, frame: &mut [u8]) -> bool {
        //Para que as partículas não grudem na borda, a colisão com a mesma é desconsiderada.
        return false;
    }
}