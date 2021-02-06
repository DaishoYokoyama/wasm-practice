use std::ops;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

static CELL_SIZE: u32 = 5;
static COL_NUM: u32 = 120;
static ROW_NUM: u32 = 100;

static CANVAS_WIDTH: u32 = CELL_SIZE * COL_NUM;
static CANVAS_HEIGHT: u32 = CELL_SIZE * ROW_NUM;


#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    start();
    Ok(())
}

pub fn start() {
    let canvas = canvas("canvas");
    init_canvas(canvas);

    let cells = generate_cells(COL_NUM, ROW_NUM);
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global window exists")
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

fn canvas(id: &str) -> web_sys::HtmlCanvasElement {
    document()
        .get_element_by_id(id)
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap()
}

fn requestAnimationFrame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("shold register");
}

fn init_canvas(canvas: web_sys::HtmlCanvasElement) {
    canvas.set_width(CANVAS_WIDTH);
    canvas.set_height(CANVAS_HEIGHT);
}

fn generate_cells(col_num: u32, row_num: u32) {
}
