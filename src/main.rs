use clap::{command, Parser};
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::PhysicalSize,
    event::{self, ElementState, VirtualKeyCode, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

use crate::draw::draw;

mod draw;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    paths: Vec<String>,
}

struct ImageFocus {
    x_offset: i32,
    y_offset: i32,
}

fn main() {
    env_logger::init();
    println!("Hello, world!");
    let args = Args::parse();

    log::info!("{:?}", args);

    let images: Vec<_> = args
        .paths
        .iter()
        .map(|path| image::io::Reader::open(path).unwrap().decode().unwrap())
        .collect();

    let crop_width: u32 = 768;
    let crop_height: u32 = 512;
    let crops_count = images.len() as u32;
    let window_size = PhysicalSize::new(crop_width * crops_count, crop_height);

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(window_size)
        .build(&event_loop)
        .unwrap();

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        log::info!("{:?}", window_size);
        Pixels::new(window_size.width, window_size.height, surface_texture).unwrap()
    };

    let mut image_focus = ImageFocus {
        x_offset: 0,
        y_offset: 0,
    };

    let mut need_redraw = false;

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();
        match event {
            event::Event::WindowEvent {
                event: window_event,
                ..
            } => match window_event {
                WindowEvent::CloseRequested => control_flow.set_exit(),
                WindowEvent::KeyboardInput { input, .. } => {
                    if input.state == ElementState::Released {
                        if let Some(code) = input.virtual_keycode {
                            need_redraw = true;
                            match code {
                                VirtualKeyCode::W => image_focus.y_offset += 1,
                                VirtualKeyCode::S => image_focus.y_offset -= 1,
                                _ => need_redraw = false,
                            }
                        }
                    }
                }
                _ => (),
            },
            event::Event::RedrawRequested(_) => pixels.render().unwrap(),
            event::Event::MainEventsCleared => {
                if need_redraw {
                    log::info!("Redrawing...");
                    let window_size = window.outer_size();
                    log::info!("{:?}", window_size);
                    pixels
                        .resize_buffer(window_size.width, window_size.height)
                        .unwrap();
                    pixels
                        .resize_surface(window_size.width, window_size.height)
                        .unwrap();
                    draw(&mut pixels, &images, (crop_width, crop_height));
                    window.request_redraw();
                    need_redraw = false;
                }
            }
            _ => (),
        }
    });
}
