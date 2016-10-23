extern crate sdl2;

use sdl2::pixels::Color;
use std::thread;
use std::time::Duration;

fn main() {
    // Inicializamos sdl2
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    // Creamos la ventana
    let window = video.window("Rust Invaders", 800, 600)
        .position_centered().opengl()
        .build().unwrap();
    let mut renderer = window.renderer()
        .accelerated()
        .build().unwrap();

    // Renderizamos una ventana completamente negra
    renderer.set_draw_color(Color::RGB(0, 0, 0));
    renderer.clear();
    renderer.present();

    thread::sleep(Duration::from_millis(3000));
}
