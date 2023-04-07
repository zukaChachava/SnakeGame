use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn greet(name: &str){
    println!("Hello {}", name);
}

#[wasm_bindgen]
pub struct World{
    pub width: usize
}

#[wasm_bindgen]
impl World{
    pub fn new() -> World{
        World{
            width: 8
        }
    }
}