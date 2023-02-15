//! Implementação das traits [base](BaseParticle) para cada partícula e definição de [position_to_index]
//!
//! Todas as partículas se movem 1 píxel por loop lógico, não sendo aplicada aceleração ou outros conceitos físicos, apenas um deslocamento unitário
//!
use crate::*;
use rand::Rng;
///Largura Tela
pub static WIDTH: u32 = 200;
///Altura Tela
pub static HEIGHT: u32 = 150;

/// Mapeia uma posição na tela a um índice de píxel no frame da tela
///
/// Um frame da tela codifica os pixels como um array, unidimensional encadeando suas componentes rgba.
/// Dessa forma o píxel 0,0 possui suas componentes r,g,b,a respecitvamente nos índices 0,1,2,3 enquanto que o píxel 1,0, tem suas componentes nos índices 4,5,6,7
/// de acordo com a largura e altura da janela.
///
/// Exemplo com largura 400
/// ```text
/// (0   1    2    3) (4    5   6   7)  ...
/// (400 401 402 403)  ...
/// (800 801 802 803)  ...
/// ```
/// Sendo assim, a função calcula a "linha" a partir de y, e a "coluna" para o píxel a partir de x, retornando o índice da componente r, na qual os próximos 3
/// serão os outros componentes.
/// ```
/// return ((y * WIDTH + x) * 4) as usize;
/// ```
pub fn position_to_index(x: u32, y: u32) -> usize {
    return ((y * WIDTH + x) * 4) as usize;
}
//[][][][][] WIDTH*Heigh /30000  0   1    2    3      --- 400
//                               400 401 402 403          400
//                               800 801 803 803 -

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

impl BaseParticle for WaterParticle {
    //Move para baixo se possível
    //Caso contrário, move aleatoriamente para esquerda ou direita
    //Objetivo: preencher todos os espaços do nível inferior
    fn move_particle(&mut self, frame: &mut [u8]) {
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

    fn colision(&self, _: &mut [u8]) -> bool {
        if self.y + 1 >= HEIGHT {
            return true;
        }
        return false;
    }
}

impl BaseParticle for AgitatedParticle {
    fn move_particle(&mut self, frame: &mut [u8]) {
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

    fn colision(&self, _: &mut [u8]) -> bool {
        //Para que as partículas não grudem na borda, a colisão com a mesma é desconsiderada.
        return false;
    }
}

impl BaseParticle for ElectricityParticle {
    fn move_particle(&mut self, frame: &mut [u8]) {
        let direction = rand::thread_rng().gen_range(0, 4);
        let mut new_x = self.x;
        let mut new_y = self.y;

        fn is_on_conducting_element(frame: &mut [u8], index: usize) -> bool {
            let is_on_water = frame[index] == 0x00 // R
                && frame[index + 1] == 0x00 // G
                && frame[index + 2] == 0xff // B
                && frame[index + 3] == 0xff; // A

            let is_on_metal = frame[index] == 0x80 // R
                && frame[index + 1] == 0x80 // G
                && frame[index + 2] == 0x80 // B
                && frame[index + 3] == 0xff; // A

            return is_on_water || is_on_metal;
        }

        if direction == 0 && self.x > 0 {
            let index_left = position_to_index(self.x - 1, self.y);

            if is_on_conducting_element(frame, index_left) {
                new_x = self.x - 1;
                new_y = self.y;
            }
        }
        if direction == 1 && self.x < WIDTH - 1 {
            let index_right = position_to_index(self.x + 1, self.y);
            if is_on_conducting_element(frame, index_right) {
                new_x = self.x + 1;
                new_y = self.y;
            }
        }
        if direction == 2 && self.y > 0 {
            let index_up = position_to_index(self.x, self.y - 1);
            if is_on_conducting_element(frame, index_up) {
                new_x = self.x;
                new_y = self.y - 1;
            }
        }
        if direction == 3 && self.y < HEIGHT - 1 {
            let index_down = position_to_index(self.x, self.y + 1);
            if is_on_conducting_element(frame, index_down) {
                new_x = self.x;
                new_y = self.y + 1;
            }
        }
        if new_x != self.x || new_y != self.y {
            self.x = new_x;
            self.y = new_y;
        }
    }

    fn colision(&self, _: &mut [u8]) -> bool {
        return false;
    }
}
