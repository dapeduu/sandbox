#![deny(clippy::all)]

use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use std::process;
mod particle;
use particle::*;
mod implparticles;

fn main() -> Result<(), Error> {
    env_logger::init();
    let mut clickflag: bool = true;
    let mut particlekey: i32 =0;
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
    //pixels.set_clear_color([0,0,]);

    let mut particlevec: Vec<SandParticle> = Vec::new();


    let mut particlevec2: Vec<ParticleType> = Vec::new();

    // Teste vec de enums
    /* 
    let parteste1 = SandParticle {
        x: 1 as u32,
        y: 2 as u32,
        rgba: [0xef, 0xef, 0x00, 0xff],
    };
    let parteste2 = Particle {
        x: 3 as u32,
        y: 4 as u32,
        rgba: [0xef, 0xef, 0x00, 0xff],
    };
    
    particlevec2.push(ParticleType::SandParticle(parteste1));
    particlevec2.push(ParticleType::Particle(parteste2));

    for partenum in particlevec2{
        match partenum{
            ParticleType::SandParticle(part)=> {
                println!("{}", part.x);
                        }
            ParticleType::Particle(part) => {
                println!("{}", part.x);
            }
        }
    }

    process::exit(1);

    */
    // Teste vec de enums


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
                
            if input.key_pressed(VirtualKeyCode::P){
                clickflag = !clickflag;
            }
            if clickflag {
                if input.mouse_held(0) {
        
                    let mousepos = input.mouse().unwrap();
                    let pixelpos = pixels
                    .window_pos_to_pixel(mousepos)
                    .unwrap_or_else(|pos| pixels.clamp_pixel_pos(pos));
                    let index: usize = position_to_index(pixelpos.0 as u32, pixelpos.1 as u32);
                    let frame:&mut[u8] = pixels.get_frame_mut();
                    if (frame[index] == 150){
                        let novaparticula = SandParticle {
                            x: pixelpos.0 as u32,
                            y: pixelpos.1 as u32,
                            rgba: [0xef, 0xef, 0x00, 0xff],
                        };
                        particlevec.push(novaparticula);
                    }
                }
            }
            else{
                if input.mouse_pressed(0) {
                    let mousepos = input.mouse().unwrap();
                    let pixelpos = pixels
                    .window_pos_to_pixel(mousepos)
                    .unwrap_or_else(|pos| pixels.clamp_pixel_pos(pos));
                    let index: usize = position_to_index(pixelpos.0 as u32, pixelpos.1 as u32);
                    let frame:&mut[u8] = pixels.get_frame_mut();
                    if (frame[index] == 150){
                        let novaparticula = SandParticle {
                            x: pixelpos.0 as u32,
                            y: pixelpos.1 as u32,
                            rgba: [0xef, 0xef, 0x00, 0xff],
                        };
                        particlevec.push(novaparticula);
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
pub fn update(vec: &mut [SandParticle], frame: &mut [u8]) {
        for particle in vec {
            particle.move_particle(frame);
        }
    }

pub fn draw(frame: &mut [u8], vec: Vec<SandParticle>) {
        //clear(frame);
        frame.fill(150);
        for particle in &vec {
            let index: usize = position_to_index(particle.x, particle.y);
            frame[index] = particle.rgba[0]; //r
            frame[index + 1] = particle.rgba[1]; //g
            frame[index + 2] = particle.rgba[2]; //b
            frame[index + 3] = particle.rgba[3]; //a
        }
        //[][][][][] WIDTH*Heigh /30000  0   1    2    3      --- 400
        //                                400 401 402 403          400
        //                                800 801 803 803 -
    }
