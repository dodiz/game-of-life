use wasm_bindgen::prelude::*;
use web_sys::{window, CanvasRenderingContext2d};

const CELL_SIZE:f64 = 20.0;

#[wasm_bindgen]
pub fn run(id: &str) {
    let document = window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(id).unwrap();

    canvas.set_attribute("width", "800").expect("Failed to set width");
    canvas.set_attribute("height", "800").expect("Failed to set height");
    let game: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
    let ctx: CanvasRenderingContext2d = game.get_context("2d").unwrap().unwrap().dyn_into::<CanvasRenderingContext2d>().unwrap();
    let game_width = ctx.canvas().unwrap().width() as f64;
    let game_height = ctx.canvas().unwrap().height() as f64;
    ctx.set_fill_style(&JsValue::from_str("#181818"));
    ctx.fill_rect(0.0, 0.0, game_width, game_height);
    let cells = game_width / CELL_SIZE;
    let mut i = 0;
    while i <= cells as i32  {
        draw_line(&ctx, ((i as f64 * CELL_SIZE) as f64, 0.0), ((i as f64 * CELL_SIZE) as f64, game_height));
        draw_line(&ctx, (0.0, (i as f64 * CELL_SIZE) as f64), (game_width, (i as f64 * CELL_SIZE) as f64));
        i += 1;
    }
}

fn draw_line(ctx: &CanvasRenderingContext2d, v1: (f64, f64), v2: (f64, f64)) {
    ctx.set_stroke_style(&JsValue::from_str("#333"));
    ctx.move_to(v1.0, v1.1);
    ctx.line_to(v2.0, v2.1);
    ctx.stroke();
}