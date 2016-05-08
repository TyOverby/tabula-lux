extern crate lux;
#[macro_use]
extern crate tabula;
extern crate tabula_lux;
extern crate parrot;

use lux::prelude::*;
use tabula::components::*;
use tabula::Rect;
use parrot::geom::Contains;

fn main() {
    let mut window = Window::new_with_defaults().unwrap();
    let mut ui_ctx = tabula::UiState::new();

    let mut slider_pos = 0.5;
    let mut region_rect = Rect::xywh(200.0, 200.0, 100.0, 100.0);
    let mut scrollbar_pos = 50.0;

    while window.is_open() {
        ui_ctx.feed_events_for_frame(window.events().filter_map(::tabula_lux::translate_event));
        let mut frame = window.cleared_frame(rgb(0.9, 0.9, 0.9));
        let mut backend = tabula_lux::DebugLuxBackend(&mut frame);
        let mut ui_ctx = ui_ctx.load(&mut backend);

        // Button
        if button(&mut ui_ctx, id!(), Rect::xywh(0.0, 0.0, 100.0, 100.0), "hello world").unwrap() {
            println!("clicked");
        }

        // Slider
        slider(&mut ui_ctx, id!(), Rect::xywh(150.0, 0.0, 200.0, 50.0), &mut slider_pos).unwrap();

        // Drag Region
        drag_region(&mut ui_ctx, id!(), &mut region_rect).unwrap();
        region_rect = region_rect.constrain_to(Rect::xywh(0.0, 0.0, window.width(), window.height()));

        // Container with button inside
        with_container(&mut ui_ctx, id!(), Rect::xywh(200.0, 200.0, 300.0, 300.0), true, |ui_ctx| {
            if try!(button(ui_ctx, id!(), Rect::xywh(-20.0, 0.0, 100.0, 50.0), "inside container")) {
                println!("clicked 2");
            }
            Ok(())
        }).unwrap();

        scrollbar(&mut ui_ctx, id!(), Rect::xywh(550.0, 100.0, 40.0, 400.0), 100.0, 200.0, &mut scrollbar_pos).unwrap();
    }
}
