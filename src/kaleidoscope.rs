use web_sys::js_sys::Math;
use web_sys::wasm_bindgen::JsCast;
use web_sys::wasm_bindgen::JsValue;
use web_sys::{
    console, CanvasGradient, CanvasPattern, CanvasRenderingContext2d,
    ImageData, OffscreenCanvas, OffscreenCanvasRenderingContext2d,
};

use crate::utils::{self, sqrt_of3_4};

const PI: f64 = std::f64::consts::PI;

pub fn flower(
    ctx: &OffscreenCanvasRenderingContext2d,
    pat_dim: f64,
    height: f64,
) -> Result<(), JsValue> {
    //first go to center of the flower
    //ctx.translate(pat_dim, pat_dim)?

    ctx.translate(pat_dim, height)?;
    let mut i = 0;

    while i < 6 {
        ctx.rotate(PI / 180.0 * 60.0)?;

        draw_triangle(ctx, pat_dim)?;
        i += 1;
    }

    // move to top left position
    ctx.translate(-pat_dim, -height)?;

    Ok(())
}

fn draw_triangle(ctx: &OffscreenCanvasRenderingContext2d, pat_dim: f64) -> Result<(), JsValue> {
    let offset = pat_dim / 2.0;
    let height = sqrt_of3_4() * pat_dim;

    let pt_a = (0.5, 0.0);
    let pt_b = (0.0, 1.0);
    let pt_c = (1.0, 1.0);

    ctx.begin_path();
    ctx.line_to(pt_c.0 * pat_dim - offset, pt_c.1 * height);
    ctx.line_to(pt_a.0 * pat_dim - offset, pt_a.1 * pat_dim);
    ctx.line_to(pt_b.0 * pat_dim - offset, pt_b.1 * height);
    ctx.close_path();

    //ctx.stroke();
    ctx.save();
    ctx.translate(pat_dim / 2.0, 0.0)?;
    ctx.fill();
    ctx.restore();
    Ok(())
}

pub fn create_pattern_offscreen(input: &OffscreenCanvas, ctx: &OffscreenCanvasRenderingContext2d) -> Result<CanvasPattern, JsValue> {
    let img = ctx
        .create_pattern_with_offscreen_canvas(&input, "repeat")
        //.create_pattern_with_html_image_element(&utils::to_image_element(image), "repeat")?
        .unwrap();
    Ok(img.unwrap())
}

pub fn create_pattern(
    ctx: &OffscreenCanvasRenderingContext2d,
    elem_id: &str,
) -> Result<CanvasPattern, JsValue> {
    let image = utils::document()
        .get_element_by_id(elem_id)
        .expect("Pattern Image Element not found");

    let img = ctx
        .create_pattern_with_html_image_element(&utils::to_image_element(image), "repeat")?
        .unwrap();
    Ok(img)
}

pub fn create_gradient(ctx: &OffscreenCanvasRenderingContext2d) -> Result<CanvasGradient, JsValue> {
    let grad = ctx.create_linear_gradient(0.0, 0.0, 1.0, 1.0);
    grad.add_color_stop(0.0, "black")?;
    grad.add_color_stop(1.0, "white")?;
    Ok(grad)
}

pub fn draw_debug_dot(ctx: &OffscreenCanvasRenderingContext2d, color: &str) {
    ctx.set_fill_style_str(color);
    ctx.fill_rect(-5.0, -5.0, 10.0, 10.0);
}

pub struct Kaleidoscope {
    ctx: OffscreenCanvasRenderingContext2d,
    pat_dim: f64,
}

impl Kaleidoscope {
    pub fn new(width: u32, height: u32) -> Result<Self, JsValue> {
        let canvas = OffscreenCanvas::new(width, height)?;

        let pat_dim = 150.0;

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<OffscreenCanvasRenderingContext2d>()
            .unwrap();

        let h = utils::sqrt_of3_4() * pat_dim;
        context.translate(-pat_dim * 0.5, -h).unwrap();

        Ok(Self {
            ctx: context,
            pat_dim,
        })
    }

    fn frame(&self, pat_dim: f64, height: f64) -> Result<(), JsValue> {
        self.clear();
        self.ctx.save();
        col(&self.ctx, pat_dim, height, 2)?;
        self.ctx.restore();

        tile(&self.ctx, pat_dim)?;

        Ok(())
    }

    pub fn get_update_frame(&self, in_canvas: &OffscreenCanvas) -> Result<ImageData, JsValue> {
        //ctx.set_fill_style_canvas_pattern(&create_pattern(ctx, "baseR")?);
        let pattern = create_pattern_offscreen(in_canvas, &self.ctx)?;
        self.ctx.set_fill_style_canvas_pattern(&pattern);
        self.get_frame()
    }

    pub fn get_frame(&self) -> Result<ImageData, JsValue> {
        let height = utils::sqrt_of3_4() * self.pat_dim;
        self.frame(self.pat_dim, height)?;
        let c = self.ctx.canvas();
        Ok(self
            .ctx
            .get_image_data(0.0, 0.0, c.width() as f64, c.height() as f64)?)
    }

    pub fn clear(&self) {
        let width = self.ctx.canvas().width() as f64;
        let height = self.ctx.canvas().height() as f64;

        self.ctx.clear_rect(0.0, 0.0, width, height);
    }
}

fn col(ctx: &OffscreenCanvasRenderingContext2d, pat_dim: f64, height: f64, count: i32) -> Result<(), JsValue> {
    // Starting Point of Global Coordinates
    //draw_debug_dot(&ctx, "grey");

    ctx.save();
    let mut y = 0;

    let c = count as f64;
    while y < count {
        let mut x = 0;
        while x < count {
            //draw_debug_dot(&ctx, "grey");
            flower(ctx, pat_dim, height)?;

            ctx.translate(pat_dim * 2.0 - (pat_dim * 0.5), height)?;
            flower(ctx, pat_dim, height)?;
            ctx.translate(pat_dim * 2.0 - (pat_dim * 0.5), -height)?;
            //draw_debug_dot(&ctx, "purple");

            x += 1;
        }

        ctx.translate(c * (-pat_dim * (c + 1.0)), height * 2.0)?;

        //draw_debug_dot(&ctx, "green");

        y += 1;
    }
    //draw_debug_dot(&ctx, "pink");
    //ctx.translate(c * (-pat_dim*(c + 1.0)), -height * 2.0)?;
    ctx.restore();

    //tile(ctx, pat_dim)?;
    //ctx.rotate(4.0)?;
    Ok(())
}

fn tile(ctx: &OffscreenCanvasRenderingContext2d, pat_dim: f64) -> Result<(), JsValue> {
    let pattern_height = Math::floor(utils::sqrt_of3_4() * pat_dim * 2.0);

    let row_data = ctx.get_image_data(0.0, 0.0, pat_dim * 3.0, pattern_height)?;

    let width = ctx.canvas().width();
    let height = ctx.canvas().height();

    let mut i = 0;
    while (pattern_height * i as f64) < (height as f64 + pat_dim) {
        let mut j = 0;
        while j as f64 * pat_dim < width as f64 + pat_dim {
            ctx.put_image_data(&row_data, j as f64 * pat_dim, i as f64 * pattern_height)?;
            j += 3;
        }
        i += 1;
    }

    Ok(())
}
