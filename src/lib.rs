extern crate console_error_panic_hook;
extern crate once_cell;
extern crate web_sys;
extern crate hex_color;

use std::cell::RefCell;
use std::sync::Mutex;
use std::{panic, rc::Rc};

use kaleidoscope::{create_pattern, draw_debug_dot, Kaleidoscope};
use once_cell::sync::Lazy;
use web_sys::console;
use web_sys::wasm_bindgen::prelude::*;
use crate::image_input::InputCanvas;

mod kaleidoscope;
mod image_input;
mod utils;

// Called when the Wasm module is instantiated
#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let canvas = utils::get_canvas(
        utils::document()
            .get_element_by_id("c")
            .expect("No Canvas Element found"),
    );
    let pat_dim = 150.0;
    let height = utils::sqrt_of3_4() * pat_dim;

    let debug_canvas = utils::get_canvas(utils::document().get_element_by_id("debug").expect("Debug Canvas not found"));
    debug_canvas.set_width(pat_dim as u32);
    debug_canvas.set_height(height as u32);

    let debug_context = utils::get_context(debug_canvas);

    let context = utils::get_context(canvas);

    let ic = InputCanvas::new(pat_dim as u32 + 2, height as u32 * 2);
    ic.ctx.translate(pat_dim / 2.0, 0.0)?;

    let kal = Kaleidoscope::new(1024, 1024)?;

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let bg_pattern = create_pattern(&ic.ctx, "baseR")?;
    
    *g.borrow_mut() = Some(Closure::new(move || {
        //frame(&context, pat_dim, height).unwrap();
        ic.ctx.rotate(-0.01).unwrap();
        ic.draw_onto(&bg_pattern, pat_dim, height);

        
        let debug_image_data = ic.frame().unwrap();
        debug_context.put_image_data(&debug_image_data, 0.0, 0.0).unwrap();
        
        let data = kal.get_update_frame(&ic.ctx.canvas()).unwrap();
        //console::log_1(&data);
        context.put_image_data(&data, 0.0, 0.0).unwrap();


        // Schedule ourself for another requestAnimationFrame callback.
        utils::request_animation_frame(f.borrow().as_ref().unwrap());
    }));

    utils::request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}

