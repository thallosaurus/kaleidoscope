use std::collections::HashSet;

use hex_color::Display;
use hex_color::HexColor;
use web_sys::js_sys::Math::random;
use web_sys::wasm_bindgen::JsCast;
use web_sys::wasm_bindgen::JsValue;
use web_sys::CanvasPattern;
use web_sys::CanvasRenderingContext2d;
use web_sys::{ImageData, OffscreenCanvas, OffscreenCanvasRenderingContext2d};

pub struct InputCanvas {
    pub ctx: OffscreenCanvasRenderingContext2d,
}

impl InputCanvas {
    pub fn new(width: u32, height: u32) -> Self {
        let canvas = OffscreenCanvas::new(width, height).unwrap();

        let pat_dim = 150.0;

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<OffscreenCanvasRenderingContext2d>()
            .unwrap();

        Self { ctx: context }
    }

    pub fn draw_onto(&self, pattern: &CanvasPattern, pat_dim: f64, height: f64) {
        self.ctx.set_fill_style_canvas_pattern(pattern);

        self.ctx.fill_rect(-pat_dim, 0.0, pat_dim, pat_dim);
        self.ctx.fill_rect(0.0, -pat_dim, pat_dim, pat_dim);
        self.ctx.fill_rect(-pat_dim, -pat_dim, pat_dim, pat_dim);
        self.ctx.fill_rect(0.0, 0.0, pat_dim, pat_dim);
    }

    pub fn frame(&self) -> Result<ImageData, JsValue> {
        let w = self.ctx.canvas().width() as f64;
        let h = self.ctx.canvas().height() as f64;

        let data = self.ctx.get_image_data(0.0, 0.0, w, h)?;
        //let proc = Self::process_channels(&data);

        //let d = self.rgb_effect(proc, &data)?;

        self.ctx.put_image_data(&data, 0.0, 0.0)?;

        Ok(data)
    }
}

fn get_offscreen_context(canvas: OffscreenCanvas) -> OffscreenCanvasRenderingContext2d {
    canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<OffscreenCanvasRenderingContext2d>()
        .unwrap()
}

fn xy_to_index(x: u32, y: u32, w: u32) -> usize {
    ((y * w) + x) as usize
}

#[derive(Clone, Copy)]
enum RGB {
    R(u8),
    G(u8),
    B(u8),
    A(u8),
}

impl RGB {
    fn to_u32(&self) -> u32 {
        match self {
            RGB::R(r) => (*r as u32) << 24,
            RGB::G(g) => (*g as u32) << 16,
            RGB::B(b) => (*b as u32) << 8,
            RGB::A(a) => *a as u32,
        }
    }

    fn to_u8(&self) -> u8 {
        match self {
            RGB::R(v) => *v,
            RGB::G(v) => *v,
            RGB::B(v) => *v,
            RGB::A(v) => *v,
        }
    }
}
