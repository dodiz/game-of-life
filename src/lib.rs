
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

    let mut grid:Vec<Vec<u32>> = vec![vec![0;40];40];
    grid[20][19] = 1;
    grid[20][20] = 1;
    grid[19][20] = 1;
    grid[21][21] = 1;
    grid[20][21] = 1;
    grid[36][36] = 1;
    grid[36][35] = 1;
    grid[35][36] = 1;
    grid[37][37] = 1;
    grid[36][37] = 1;
    render_game(ctx.clone(), grid.clone());
}

fn render_game(ctx: CanvasRenderingContext2d, grid: Vec<Vec<u32>>) {
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

    let mut next_grid = grid.clone();
    loop {
        let mut i:u32 = 0;
        let mut j:u32;

        while i < 40 {
            j = 0;
            while j < 40 {
                let top_left = if i == 0 || j == 0 { 0 } else { get_cell(&grid, i - 1, j - 1) };
                let top = if i == 0 { 0 } else { get_cell(&grid, i - 1, j) };
                let top_right = if i == 0 { 0 } else { get_cell(&grid, i - 1, j + 1) };
                let right = get_cell(&grid, i, j + 1);
                let bottom_right = get_cell(&grid, i + 1, j + 1);
                let bottom = get_cell(&grid, i + 1, j);
                let bottom_left = if  j == 0  { 0 } else { get_cell(&grid, i + 1, j - 1)} ;
                let left = if j == 0  { 0 } else { get_cell(&grid, i, j - 1) };

                let sum = top_left + top + top_right + right + bottom_right + bottom + bottom_left + left;

                let cell_status = if get_cell(&grid, i, j) == 1 {
                    match sum {
                    0 | 1 => 0,
                    2 | 3 => 1,
                    _ => 0
                    }
                } else {
                    if sum == 3 { 1 } else { 0 }
                };
                next_grid[i as usize][j as usize] = cell_status;
                j += 1;
            }
            i += 1;
        }
        for (i, row) in next_grid.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if *cell == 1 {
                    fill_cell(&ctx, (i as u32, j as u32));
                }
            }
        }
        break;
    }

    let frame = Closure::wrap(Box::new( move || {
        render_game(ctx.clone(), next_grid.clone());
    }) as Box<dyn FnMut()>);

    window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(frame.as_ref().unchecked_ref(), 50).expect("should register `requestAnimationFrame` OK");
    frame.forget();
}


fn get_cell(map: &Vec<Vec<u32>>, row: u32, column: u32) -> u32 {
    let neighbour = map.get(row as usize).unwrap_or(&Vec::new()).get(column as usize).unwrap_or(&0).clone();
    neighbour
}

fn draw_line(ctx: &CanvasRenderingContext2d, v1: (f64, f64), v2: (f64, f64)) {
    ctx.set_stroke_style(&JsValue::from_str("#333"));
    ctx.move_to(v1.0, v1.1);
    ctx.line_to(v2.0, v2.1);
    ctx.stroke();
}

fn fill_cell(ctx: &CanvasRenderingContext2d, v: (u32, u32)) {
    let x = v.0 as f64 * CELL_SIZE;
    let y = v.1 as f64 * CELL_SIZE;
    ctx.set_fill_style(&JsValue::from_str("#999"));
    ctx.fill_rect(x, y,  CELL_SIZE,
        CELL_SIZE);
}