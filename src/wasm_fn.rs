use yew::Renderer;
use wasm_bindgen::prelude::*;

use crate::recangle;
#[wasm_bindgen(start)]
pub fn run_app() {
    Renderer::<recangle::Model>::new().render();
}