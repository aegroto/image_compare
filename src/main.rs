use clap::{command, Parser};
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::PhysicalSize,
    event::{self, ElementState, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

use crate::{draw::draw, focus::ImageFocus, process::process_images};

mod draw;
mod focus;
mod process;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    paths: Vec<String>,
    #[arg(short, long)]
    crop_width: u32,
    #[arg(short, long)]
    crop_height: u32,
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
        .map(|image| {
            image.resize(
                args.crop_width,
                args.crop_height,
                image::imageops::FilterType::Nearest,
            )
        })
        .collect();

    let crops_count = images.len() as u32;
    let window_size = PhysicalSize::new(args.crop_width * crops_count, args.crop_height);

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

    let mut image_focus = ImageFocus::new();

    let mut need_redraw = true;

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();
        match event {
            event::Event::WindowEvent {
                event: window_event,
                ..
            } => match window_event {
                WindowEvent::CloseRequested => control_flow.set_exit(),
                WindowEvent::Resized(_) => need_redraw = true,
                WindowEvent::KeyboardInput { input, .. } => {
                    if input.state == ElementState::Released {
                        if let Some(code) = input.virtual_keycode {
                            need_redraw = image_focus.update_on_key_input(code);
                        }
                    }
                }
                _ => (),
            },
            event::Event::RedrawRequested(_) => pixels.render().unwrap(),
            event::Event::MainEventsCleared => {
                if need_redraw {
                    log::debug!("Redrawing...");
                    let window_size = window.outer_size();
                    pixels
                        .resize_buffer(window_size.width, window_size.height)
                        .unwrap();
                    pixels
                        .resize_surface(window_size.width, window_size.height)
                        .unwrap();
                    let processed_images = process_images(&images, &image_focus);
                    draw(
                        &mut pixels,
                        &processed_images,
                        (args.crop_width, args.crop_height),
                    );
                    window.request_redraw();
                    need_redraw = false;
                }
            }
            _ => (),
        }
    });
}
