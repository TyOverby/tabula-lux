extern crate tabula;
extern crate lux;

use tabula::{Id, Point, Rect, EventSource};
use lux::prelude::*;
use lux::interactive::Event as LuxEvent;
use tabula::Event as TabulaEvent;

pub struct DebugLuxBackend<'a>(pub &'a mut Frame);

pub fn translate_event(e: LuxEvent) -> Option<TabulaEvent> {
    match e {
        LuxEvent::MouseMoved((x, y)) => Some(TabulaEvent::PointerMove(EventSource(0), x as f32, y as f32)),
        LuxEvent::MouseDown(_) => Some(TabulaEvent::PointerDown(EventSource(0))),
        LuxEvent::MouseUp(_) => Some(TabulaEvent::PointerUp(EventSource(0))),
        _ => None
    }
}

impl <'a> tabula::components::ButtonRender for DebugLuxBackend<'a> {
    type Error = ();
    fn draw_button(&mut self, _id: Id, rect: Rect<f32>, _label: &str, hovered: bool, pressed: bool) -> Result<(), Self::Error> {
        let Point(x, y) = rect.0;
        let (w, h) = (rect.width(), rect.height());

        let color = match (hovered, pressed) {
            (false, false) => lux::color::RED,
            (true,  false) => lux::color::BLUE,
            (false, true)  => lux::color::GREEN,
            (true,  true)  => lux::color::YELLOW,
        };

        self.0.rect(x, y, w, h).color(color).fill();
        Ok(())
    }
}

impl <'a> tabula::components::SliderRender for DebugLuxBackend<'a> {
    type Error = ();

    fn draw_slider(
        &mut self,
        id: Id,
        rect: Rect<f32>,
        covering: f32) -> Result<(), Self::Error> {
        let Point(x, y) = rect.0;
        let (w, h) = (rect.width(), rect.height());

        self.0.rect(x, y, w, h).color(lux::color::BLUE).fill();
        println!("w {} cov {} {}", w, covering, w * covering);
        self.0.rect(x, y, w * covering, h).color(lux::color::RED).fill();
        Ok(())
    }
}
