extern crate sdl2;

mod phi;
mod views;

fn main() {
   ::phi::spawn("RustInvaders", |phi| {
        Box::new(::views::ShipView::new(phi))
    });
}
