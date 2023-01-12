#![deny(clippy::all)]
#![forbid(unsafe_code)]

use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
//use pixels::wgpu::Color;
use std::process::exit;

const WIDTH: u32 = 200;
const HEIGHT: u32 = 150;

/// Representation of the application state. In this example, a box will bounce around the screen.
#[derive(Copy, Clone)]
struct World {

}

#[derive(Copy, Clone)]
struct Particle{
    x: u32,
    y: u32,
}

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        let scaled_size = LogicalSize::new(WIDTH as f64 * 2.0, HEIGHT as f64 * 2.0);
        WindowBuilder::new()
            .with_title("Hello Pixels")
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

    let mut particlevec: Vec<Particle>  = Vec::new();


    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            world.draw(pixels.get_frame_mut(),particlevec.clone());
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
            if input.mouse_pressed(0){
                let mousepos = input.mouse().unwrap();
                let pixelpos =  pixels.window_pos_to_pixel(mousepos).unwrap_or_else(|pos| pixels.clamp_pixel_pos(pos));
                let novaparticula = Particle {x : pixelpos.0 as u32, y : pixelpos.1  as u32 };  
                particlevec.push(novaparticula);

            }
             //Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            // Update internal state and request a redraw
            world.update();
            window.request_redraw();
        }
    });
}



pub fn clear(ary: &mut [u8]) {
    ary.iter_mut().for_each(|m| *m = 0)
}


impl World {
    /// Create a new `World` instance that can draw a moving box.
    fn new() -> Self {
        Self {
        }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    fn update(&mut self) {
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&self, frame: &mut [u8], vec: Vec<Particle> ) {

        clear(frame);
    
        for part in &vec{
                   let index :usize =((part.y*WIDTH + part.x)*4) as usize;      
                   frame[index] = 0x00;
                   frame[index+1] = 0x00;
                   frame[index+2] = 0xff;
                   frame[index+3] = 0xff;              
                }
        

    }
}
