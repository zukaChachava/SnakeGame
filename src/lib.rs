use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

struct Snake{
    body: Vec<Cell>
}

struct Cell(usize);

#[wasm_bindgen]
pub struct World{
    width: usize,
    snake: Snake
}

#[wasm_bindgen]
impl World{
    pub fn new() -> World{
        World{
            width: 16,
            snake: Snake::new(12)
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn snake_head(&self) -> usize{
        self.snake.body[0].0
    }
}

impl Snake{
    fn new(index: usize) -> Snake {
        Snake { body: vec![Cell(index)] }
    }
}