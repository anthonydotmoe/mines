extern crate sdl2

use sdl2::event::Event
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::path::Path;
use std::process;

#[cfg(target_os = "emscripten")]
pub mod emscripten;

fn get_tile_rect(i: i32) -> Result<Rect, &'static str> {
    match i {
        0..=15 => Ok(Rect::new(0, 16 * i, 16, 16)),
        _ => Err("Not a valid tile index!"),
    }
}

pub fn main() {
    let ctx = sdl2::init().unwrap();
    let video = ctx.video().unwrap();
    let window = match video
        .window("rust-sdl-canvas-wasm", 320, 240)
        .opengl()
        .build()
    {
        Ok(window) => window,
        Err(e) => panic!("Failed to create window: {}", e),
    };

    let mut canvas = window.into_canvas().build().unwrap();

    let tc = canvas.texture_creator();
    let texture = tc
        .load_texture(Path::new("static/tiles.png"))
        .expect("Cannot load image");

    //let mut iteration = 0;
    canvas.set_draw_color(Color::RGB(192, 192, 192));
    canvas.clear();

    let mut rect = Rect::new(0, 10, 16, 16);

    for i in 0..16 {
        canvas
            .copy(&texture, get_tile_rect(i).unwrap(), rect)
            .expect("Uh oh");
        rect.set_x(rect.x + 17);
    }

    let mut events = ctx.event_pump().unwrap();

    let mut main_loop = || {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    process::exit(1);
                }
                _ => {}
            }
        }

        /*
        // example: draw a moving rectangle

        // red background
        // moving blue rectangle
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.clear();

        iteration = (iteration + 1) % 255;
        let rect = Rect::new(iteration, 50, 50, 50);
        let _ = canvas.copy(&texture, None, rect).expect("Uh oh:");
        */

        canvas.present();
    };

    #[cfg(target_os = "emscripten")]
    use emscripten::emscripten;

    #[cfg(target_os = "emscripten")]
    emscripten::set_main_loop_callback(main_loop);

    #[cfg(not(target_os = "emscripten"))]
    loop {
        main_loop();
    }
}
