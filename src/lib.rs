use wasm_bindgen::prelude::*;
use web_sys::{window, CanvasRenderingContext2d};


#[wasm_bindgen]
pub fn run(id: &str) {
    let document = window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(id).unwrap();

    canvas.set_attribute("width", "800").expect("Failed to set width");
    canvas.set_attribute("height", "800").expect("Failed to set height");
    let game: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
    let ctx: CanvasRenderingContext2d = game.get_context("2d").unwrap().unwrap().dyn_into::<CanvasRenderingContext2d>().unwrap();

    ctx.set_fill_style(&JsValue::from_str("red"));
    ctx.fill_rect(0.0,0.0, 800.0, 800.0);

}
