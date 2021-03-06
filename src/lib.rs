extern crate tabula;
extern crate lux;

use tabula::{Id, Point, Rect, EventSource};
use lux::prelude::*;
use lux::interactive::Event as LuxEvent;
use tabula::Event as TabulaEvent;

pub struct DebugLuxBackend<'a>(pub &'a mut Frame);

pub fn translate_event(e: LuxEvent) -> Option<TabulaEvent> {
    match e {
        LuxEvent::MouseMoved((x, y)) => Some(TabulaEvent::PointerMove(EventSource(0, 0), x as f32, y as f32)),
        LuxEvent::MouseDown(_) => Some(TabulaEvent::PointerDown(EventSource(0, 0))),
        LuxEvent::MouseUp(_) => Some(TabulaEvent::PointerUp(EventSource(0, 0))),
        _ => None
    }
}

impl <'a> tabula::components::ButtonRender for DebugLuxBackend<'a> {
    type Error = ();
    fn draw_button(&mut self, _id: &Id, rect: Rect<f32>, _label: &str, hovered: bool, pressed: bool) -> Result<(), Self::Error> {
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
        _id: Id,
        rect: Rect<f32>,
        covering: f32) -> Result<(), Self::Error> {
        let Point(x, y) = rect.0;
        let (w, h) = (rect.width(), rect.height());

        self.0.rect(x, y, w, h).color(lux::color::BLUE).fill();
        self.0.rect(x, y, w * covering, h).color(lux::color::RED).fill();
        Ok(())
    }
}

impl <'a> tabula::components::DragRegionRender for DebugLuxBackend<'a> {
    type Error = ();

    fn draw_drag_region(&mut self, _id: &Id, rect: Rect<f32>, drag_action: tabula::components::DragAction) -> Result<(), Self::Error> {
        use tabula::components::DragAction;
        let Point(x, y) = rect.0;
        let (w, h) = (rect.width(), rect.height());
        let color = match drag_action {
            DragAction::None => lux::color::GREEN,
            _ => lux::color::PURPLE,
        };
        self.0.rect(x, y, w, h).color(color).fill();
        Ok(())
    }
}

impl <'a> tabula::components::ContainerRender for DebugLuxBackend<'a> {
    type Error = ();

    fn draw_container<R, F: FnOnce(&mut Self) -> Result<R, Self::Error>> (
        &mut self,
        _id: &Id,
        rect: Rect<f32>,
        Point(x, y): Point<f32>,
        f: F) -> Result<R, Self::Error> {

        let Point(rx, ry) = rect.0;
        let (w, h) = (rect.width(), rect.height());
        self.0.rect(rx, ry, w, h).color(lux::color::rgba(0.0, 0.0, 0.0, 0.2)).fill();

        self.0.translate(x, y);
        let r = f(self);
        self.0.translate(-x, -y);
        r
    }
}

impl <'a> tabula::components::ScrollbarRender for DebugLuxBackend<'a> {
    type Error = ();

    fn draw_scrollbar(&mut self, _id: &Id, rect: Rect<f32>, over_rect: Rect<f32>, _over: bool, _down: bool) -> Result<(), Self::Error> {
        {
            let Point(x, y) = rect.0;
            let (w, h) = (rect.width(), rect.height());
            self.0.rect(x, y, w, h).color(lux::color::BLUE).fill();
        } {
            let Point(x, y) = over_rect.0;
            let (w, h) = (over_rect.width(), over_rect.height());
            self.0.rect(x, y, w, h).color(lux::color::RED).fill();
        }

        Ok(())
    }
}
