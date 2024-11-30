use web_sys::wasm_bindgen::prelude::*;
use web_sys::js_sys::{Math, Object};
use web_sys::wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, Document, Element, HtmlCanvasElement, HtmlElement, HtmlImageElement, OffscreenCanvas, Window};

pub fn sqrt_of3_4() -> f64 {
    Math::sqrt(3.0) / 2.0
}

pub fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

pub fn get_canvas(elem: Element) -> HtmlCanvasElement {
    elem.dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap()
}

pub fn get_offscreen_context(canvas: Object) -> CanvasRenderingContext2d {
    canvas.dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap()
}

pub fn get_context(canvas: HtmlCanvasElement) -> CanvasRenderingContext2d {
    canvas.get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap()
}

pub fn to_image_element(elem: Element) -> HtmlImageElement {
    elem.dyn_into::<web_sys::HtmlImageElement>().unwrap()
}

pub fn window() -> Window {
    web_sys::window().expect("no global `window` exists")
}

pub fn document() -> Document {
    window().document().expect("should have a document on window")
}

pub fn body() -> HtmlElement {
    document().body().expect("document should have a body")
}