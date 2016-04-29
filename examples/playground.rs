extern crate lux;
#[macro_use]
extern crate tabula;
extern crate tabula_lux;

use lux::prelude::*;
use lux::color;
use tabula::components::*;
use tabula::{Rect, Point, Id};

fn main() {
    let mut window = Window::new_with_defaults().unwrap();
    let mut ui_ctx = tabula::UnloadedUiContext::new();

    let mut slider_pos = 0.5;

    while window.is_open() {
        ui_ctx.feed_events_for_frame(window.events().filter_map(::tabula_lux::translate_event));
        let mut frame = window.cleared_frame(rgb(0.9, 0.9, 0.9));
        let mut backend = tabula_lux::DebugLuxBackend(&mut frame);
        let mut ui_ctx = ui_ctx.load(backend);

        if button(&mut ui_ctx, id!(), Rect::xywh(0.0, 0.0, 100.0, 100.0), "hello world").unwrap() {
            println!("clicked");
        }

        slider_pos = slider(&mut ui_ctx, id!(), Rect::xywh(150.0, 0.0, 200.0, 50.0), slider_pos).unwrap();
    }
}
