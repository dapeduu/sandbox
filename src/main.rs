//! Módulo principal, Lógica de execução, renderização, inicialização e chamada às funções.
#![deny(clippy::all)]

use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
mod particle;
use particle::*;
mod implparticles;
use crate::implparticles::*;

fn main() -> Result<(), Error> {
    //! Execução Prinicipal
    //! 
    //! A main pode ser dividida em 4 partes, inicialização, input,update,renderização, sendo as 3 últimas rodadas em loop
    //! 
    env_logger::init();
    let mut clickflag: bool = true;
    let mut particlekey: ParticleNum = ParticleNum::Sand;
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        let scaled_size = LogicalSize::new(WIDTH as f64 * 2.0, HEIGHT as f64 * 2.0);
        WindowBuilder::new()
            .with_title("Sandbox")
            .with_inner_size(scaled_size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    let mut particlevec: Vec<ParticleType> = Vec::new();

    event_loop.run(move |event, _, control_flow| {
        println!("Number of particles: {}", particlevec.len());

        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            draw(pixels.get_frame_mut(), particlevec.clone());
            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if input.key_pressed(VirtualKeyCode::P) {
                clickflag = !clickflag;
            }
            if input.key_pressed(VirtualKeyCode::Key1) {
                particlekey = ParticleNum::Base;
            }
            if input.key_pressed(VirtualKeyCode::Key2) {
                particlekey = ParticleNum::Sand;
            }
            if input.key_pressed(VirtualKeyCode::Key3) {
                particlekey = ParticleNum::Iron;
            }
            if input.key_pressed(VirtualKeyCode::Key4) {
                particlekey = ParticleNum::Acid;
            }
            if input.key_pressed(VirtualKeyCode::Key5) {
                particlekey = ParticleNum::Water;
            }
            if input.key_pressed(VirtualKeyCode::Key6) {
                particlekey = ParticleNum::Agitated;
            }
            if input.key_pressed(VirtualKeyCode::Key7) {
                particlekey = ParticleNum::Electricity;
            }

            if clickflag {
                if input.mouse_held(0) {
                    let pixref = &mut pixels as *mut Pixels;
                    match instanceparticle(&input, pixref, particlekey) {
                        Some(instancia) => {
                            particlevec.push(instancia);
                        }
                        None => {}
                    }
                }
            } else {
                if input.mouse_pressed(0) {
                    let pixref = &mut pixels as *mut Pixels;
                    match instanceparticle(&input, pixref, particlekey) {
                        Some(instancia) => {
                            particlevec.push(instancia);
                        }
                        None => {}
                    }
                }
            }
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height).unwrap();
            }

            update(particlevec.as_mut_slice(), pixels.get_frame_mut());

            window.request_redraw();
        }
    });
}

/// Instancia partículas
/// 
/// Inicialmente pega as coordenadas do mouse e utiliza [position_to_index] para associar ao frame
/// ```
/// let mousepos = (*input).mouse().unwrap();
/// let pixelpos = (*pixels).window_pos_to_pixel(mousepos).unwrap_or_else(|pos| (*pixels).clamp_pixel_pos(pos));
/// let index: usize = position_to_index(pixelpos.0 as u32, pixelpos.1 as u32);
/// ```
/// 
/// Em seguida, verifica se a partícula que vai ser instanciada não irá sobrepor outra do mesmo tipo devido a velocidade do processamento,
/// após isso cria e retorna a nova partícula com as novas coordenadas e seus valores de cores
/// ```
/// ParticleNum::Sand => {
///     let is_on_sand = frame[index] == 0x96 // R
///     && frame[index + 1] == 0x4b // G
///     && frame[index + 2] == 0x00 // B
///     && frame[index + 3] == 0xff; // A
///     if is_on_sand {
///     return None;
///     }
///     let novaparticula = SandParticle {
///     x: pixelpos.0 as u32,
///     y: pixelpos.1 as u32,
///     rgba: [0x96, 0x4b, 0x00, 0xff],
///     };
///     return Some(ParticleType::SandParticle(novaparticula));
/// }
/// ```

pub fn instanceparticle(
    input: *const WinitInputHelper,
    pixels: *mut Pixels,
    particlekey: ParticleNum,
) -> Option<ParticleType> {
    unsafe {
        let mousepos = (*input).mouse().unwrap();
        let pixelpos = (*pixels)
            .window_pos_to_pixel(mousepos)
            .unwrap_or_else(|pos| (*pixels).clamp_pixel_pos(pos));
        let index: usize = position_to_index(pixelpos.0 as u32, pixelpos.1 as u32);
        let frame: &mut [u8] = (*pixels).get_frame_mut();

        match particlekey {
            ParticleNum::Base => {
                let is_on_base = frame[index] == 0x00 // R
                    && frame[index + 1] == 0xef // G
                    && frame[index + 2] == 0x00 // B
                    && frame[index + 3] == 0xff; // A

                if is_on_base {
                    return None;
                }

                let novaparticula = Particle {
                    x: pixelpos.0 as u32,
                    y: pixelpos.1 as u32,
                    rgba: [0x00, 0xef, 0x00, 0xff],
                };
                return Some(ParticleType::Particle(novaparticula));
            }
            ParticleNum::Sand => {
                let is_on_sand = frame[index] == 0x96 // R
                    && frame[index + 1] == 0x4b // G
                    && frame[index + 2] == 0x00 // B
                    && frame[index + 3] == 0xff; // A

                if is_on_sand {
                    return None;
                }

                let novaparticula = SandParticle {
                    x: pixelpos.0 as u32,
                    y: pixelpos.1 as u32,
                    rgba: [0x96, 0x4b, 0x00, 0xff],
                };
                return Some(ParticleType::SandParticle(novaparticula));
            }
            ParticleNum::Iron => {
                let is_on_iron = frame[index] == 0x80 // R
                && frame[index + 1] == 0x80 // G
                && frame[index + 2] == 0x80 // B
                && frame[index + 3] == 0xff; // A

                if is_on_iron {
                    return None;
                }

                let novaparticula = IronParticle {
                    x: pixelpos.0 as u32,
                    y: pixelpos.1 as u32,
                    rgba: [0x80, 0x80, 0x80, 0xff],
                };
                return Some(ParticleType::IronParticle(novaparticula));
            }
            ParticleNum::Acid => {
                let is_on_acid = frame[index] == 0x0 // R
                    && frame[index + 1] == 0x80 // G
                    && frame[index + 2] == 0x0 // B
                    && frame[index + 3] == 0xff; // A

                if is_on_acid {
                    return None;
                }

                let novaparticula = AcidParticle {
                    x: pixelpos.0 as u32,
                    y: pixelpos.1 as u32,
                    rgba: [0x0, 0x80, 0x0, 0xff],
                };
                return Some(ParticleType::AcidParticle(novaparticula));
            }
            ParticleNum::Water => {
                let is_on_water = frame[index] == 0x0 // R
                    && frame[index + 1] == 0x0 // G
                    && frame[index + 2] == 0xff // B
                    && frame[index + 3] == 0xff; // A

                if is_on_water {
                    return None;
                }

                let novaparticula = WaterParticle {
                    x: pixelpos.0 as u32,
                    y: pixelpos.1 as u32,
                    rgba: [0x0, 0x0, 0xff, 0xff],
                };
                return Some(ParticleType::WaterParticle(novaparticula));
            }
            ParticleNum::Agitated => {
                let is_on_agitated = frame[index] == 0x16 // R
                    && frame[index + 1] == 0x16 // G
                    && frame[index + 2] == 0x00 // B
                    && frame[index + 3] == 0xff; // A

                if is_on_agitated {
                    return None;
                }

                let novaparticula = AgitatedParticle {
                    x: pixelpos.0 as u32,
                    y: pixelpos.1 as u32,
                    rgba: [0x16, 0x16, 0x00, 0xff],
                };
                return Some(ParticleType::AgitatedParticle(novaparticula));
            }
            ParticleNum::Electricity => {
                let is_on_electricity = frame[index] == 0xff // R
                    && frame[index + 1] == 0xff // G
                    && frame[index + 2] == 0x00 // B
                    && frame[index + 3] == 0xff; // A

                if is_on_electricity {
                    return None;
                }

                let novaparticula = ElectricityParticle {
                    x: pixelpos.0 as u32,
                    y: pixelpos.1 as u32,
                    life_time: 5,
                    rgba: [0xff, 0xff, 0x00, 0xff],
                };
                return Some(ParticleType::ElectricityParticle(novaparticula));
            }
        }
    }
}

/// Atualiza as partículas;
/// 
/// Para cada partícula do vetor realiza o match de acordo com o tipo e chama sua função de movimentação
/// ```text
///     for partenum in vec {
///         match partenum {
///             ParticleType::SandParticle(part) => {
///                 part.move_particle(frame);
///                 }
///          ...
///          ...
/// ```
pub fn update(vec: &mut [ParticleType], frame: &mut [u8]) {
    for partenum in vec {
        match partenum {
            ParticleType::SandParticle(part) => {
                part.move_particle(frame);
            }
            ParticleType::Particle(part) => {
                part.move_particle(frame);
            }
            ParticleType::IronParticle(part) => {
                part.move_particle(frame);
            }
            ParticleType::AcidParticle(part) => {
                part.move_particle(frame);
            }
            ParticleType::WaterParticle(part) => {
                part.move_particle(frame);
            }
            ParticleType::AgitatedParticle(part) => {
                part.move_particle(frame);
            }
            ParticleType::ElectricityParticle(part) => {
                part.move_particle(frame);
            }
        }
    }
}

/// Renderização na tela.
/// 
/// Inicialmente limpa a tela, preenchendo todos os componentes dos pixels com o valor 150
/// ```
/// frame.fill(150);
/// ```
/// 
/// Em seguida, para cada partícula instanciada, dá match de acordo com seu tipo, utiliza [position_to_index] para pegar os valores de posição
/// da partícula e associar a índices no frame, em seguida preenche as componetes rgba do píxel de acordo com as cores da partícula
/// 
/// ```
/// ParticleType::SandParticle(part) => {
///     let index: usize = position_to_index(part.x, part.y);
///     frame[index] = part.rgba[0]; //r
///     frame[index + 1] = part.rgba[1]; //g
///     frame[index + 2] = part.rgba[2]; //b
///     frame[index + 3] = part.rgba[3]; //a
///     }
/// ```
/// 
pub fn draw(frame: &mut [u8], vec: Vec<ParticleType>) {
    //clear(frame);
    frame.fill(150);

    for partenum in vec {
        match partenum {
            ParticleType::SandParticle(part) => {
                let index: usize = position_to_index(part.x, part.y);
                frame[index] = part.rgba[0]; //r
                frame[index + 1] = part.rgba[1]; //g
                frame[index + 2] = part.rgba[2]; //b
                frame[index + 3] = part.rgba[3]; //a
            }
            ParticleType::Particle(part) => {
                let index: usize = position_to_index(part.x, part.y);
                frame[index] = part.rgba[0]; //r
                frame[index + 1] = part.rgba[1]; //g
                frame[index + 2] = part.rgba[2]; //b
                frame[index + 3] = part.rgba[3]; //a
            }
            ParticleType::IronParticle(part) => {
                let index: usize = position_to_index(part.x, part.y);
                frame[index] = part.rgba[0]; //r
                frame[index + 1] = part.rgba[1]; //g
                frame[index + 2] = part.rgba[2]; //b
                frame[index + 3] = part.rgba[3]; //a
            }
            ParticleType::AcidParticle(part) => {
                let index: usize = position_to_index(part.x, part.y);
                frame[index] = part.rgba[0]; //r
                frame[index + 1] = part.rgba[1]; //g
                frame[index + 2] = part.rgba[2]; //b
                frame[index + 3] = part.rgba[3]; //a
            }
            ParticleType::WaterParticle(part) => {
                let index: usize = position_to_index(part.x, part.y);
                frame[index] = part.rgba[0]; //r
                frame[index + 1] = part.rgba[1]; //g
                frame[index + 2] = part.rgba[2]; //b
                frame[index + 3] = part.rgba[3]; //a
            }
            ParticleType::AgitatedParticle(part) => {
                let index: usize = position_to_index(part.x, part.y);
                frame[index] = part.rgba[0]; //r
                frame[index + 1] = part.rgba[1]; //g
                frame[index + 2] = part.rgba[2]; //b
                frame[index + 3] = part.rgba[3]; //a
            }
            ParticleType::ElectricityParticle(part) => {
                let index: usize = position_to_index(part.x, part.y);
                frame[index] = part.rgba[0]; //r
                frame[index + 1] = part.rgba[1]; //g
                frame[index + 2] = part.rgba[2]; //b
                frame[index + 3] = part.rgba[3]; //a
            }
        }
    }

}
