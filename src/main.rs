#![deny(clippy::all)]
#![forbid(unsafe_code)]

use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

static WIDTH: u32 = 200;
static HEIGHT: u32 = 150;

trait BaseParticle {
    /// Handles the particle movement
    fn move_particle(&mut self, frame: &mut [u8]);

    /// Colision checks on the screen borders and particles
    /// Returns true if there is colision
    fn colision(&self, frame: &mut [u8]) -> bool;
}

#[derive(Clone)]
struct Particle {
    x: u32,
    y: u32,
    rgba: [u8; 4],
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

fn main() -> Result<(), Error> {
    env_logger::init();
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
    let mut world = World::new();
    //pixels.set_clear_color([0,0,]);

    let mut particlevec: Vec<Particle> = Vec::new();

    event_loop.run(move |event, _, control_flow| {
        println!("Number of particles: {}", particlevec.len());

        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            world.draw(pixels.get_frame_mut(), particlevec.clone());
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
            if input.mouse_held(0) {
                let mousepos = input.mouse().unwrap();
                let pixelpos = pixels
                    .window_pos_to_pixel(mousepos)
                    .unwrap_or_else(|pos| pixels.clamp_pixel_pos(pos));
                let index: usize = position_to_index(pixelpos.0 as u32, pixelpos.1 as u32);
                let frame:&mut[u8] = pixels.get_frame_mut();
                if (frame[index] == 150){
                    let novaparticula = Particle {
                        x: pixelpos.0 as u32,
                        y: pixelpos.1 as u32,
                        rgba: [0x00, 0x00, 0xef, 0xff],
                    };
                    particlevec.push(novaparticula);
                }
            }
            //Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height).unwrap();
            }

            // Update internal state and request a redraw
            world.update(particlevec.as_mut_slice(), pixels.get_frame_mut());
            window.request_redraw();
        }
    });
}

struct World {}
impl World {
    /// Create a new `World` instance that can draw a moving box.
    fn new() -> Self {
        Self {}
    }

    /// Update the `World` internal state; bounce the box around the screen.
    fn update(&mut self, vec: &mut [Particle], frame: &mut [u8]) {
        for particle in vec {
            particle.move_particle(frame);
        }
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&self, frame: &mut [u8], vec: Vec<Particle>) {
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
}

pub fn position_to_index(x: u32, y: u32) -> usize {
    return ((y * WIDTH + x) * 4) as usize;
}
