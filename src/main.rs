extern crate sdl2;

mod phi;
mod views;

fn main() {
   ::phi::spawn("RustInvaders", |_| {
        Box::new(::views::ViewA)
    });
}
