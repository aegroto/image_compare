use clap::{command, Parser};
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::PhysicalSize,
    event::{self, WindowEvent},
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
        Pixels::new(window_size.width, window_size.height, surface_texture).unwrap()
    };

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();
        match event {
            event::Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                control_flow.set_exit();
            }
            event::Event::RedrawRequested(_) => pixels.render().unwrap(),
            event::Event::MainEventsCleared => {
                log::info!("Redrawing...");
                pixels
                    .resize_surface(window.inner_size().width, window.inner_size().height)
                    .unwrap();
                draw(&mut pixels, &images, (crop_width, crop_height));
                window.request_redraw();
            }
            _ => (),
        }
    });
}
