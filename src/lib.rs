use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(PartialEq, Copy, Clone)]
pub enum Direction{
    Up,
    Right,
    Down,
    Left
}

#[derive(PartialEq)]
enum DirectionChange {
    Previous(Direction),
    Empty
}

struct Snake{
    body: Vec<Cell>,
    direction: Direction,
    direction_change: DirectionChange 
}

struct Cell(usize);

#[wasm_bindgen]
pub struct World{
    width: usize,
    snake: Snake
}

#[wasm_bindgen]
impl World{
    pub fn new(width: usize, start_index: usize) -> World{
        World{
            width: width,
            snake: Snake::new(start_index)
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn snake_head(&self) -> usize{
        self.snake.get_head()
    }

    pub fn change_direction(&mut self, direction: Direction){
        self.snake.change_direction(direction);
    }

    pub fn update(&mut self){
        self.snake.update(self.width());
    }
}

impl Snake{
    fn new(index: usize) -> Snake {
        Snake { body: vec![Cell(index)], direction: Direction::Right, direction_change: DirectionChange::Empty }
    }

    fn get_head(&self) -> usize {
        self.body[0].0
    }

    fn change_direction(&mut self, direction: Direction){

        let valid_move = match &self.direction_change {
            DirectionChange::Empty => Snake::direction_valid(&self.direction, &direction),
            DirectionChange::Previous(from) => Snake::direction_valid(&from, &direction)
        };

        if !valid_move {
            return;
        }

        self.direction_change = DirectionChange::Previous(self.direction);
        self.direction = direction;
    }

    fn direction_valid(from: &Direction, direction: &Direction) -> bool {
        if *from == Direction::Left && *direction == Direction::Right ||
           *from == Direction::Right && *direction == Direction::Left  ||
           *from == Direction::Up && *direction == Direction::Down ||
           *from == Direction::Down && *direction == Direction::Up  {
            return false;
        }

        return true;
    }

    fn update(&mut self, width: usize){
        match self.direction {
            Direction::Right => self.move_right(width),
            Direction::Down => self.move_down(width),
            Direction::Left => self.move_left(width),
            Direction::Up => self.move_up(width)
        }

        self.direction_change = DirectionChange::Empty;
    }

    fn move_right(&mut self, width: usize){
        let mut index = self.get_head();

        if (index + 1) % width == 0  {
            index = index - (width - 1);
        }
        else{
            index += 1;
        }

        self.body[0].0 = index;
    }

    fn move_down(&mut self, width: usize){
        let mut index = self.get_head() + width;

        if index > width * width - 1 {
            index = index % width;
        }

        self.body[0].0 = index;
    }

    fn move_left(&mut self, width: usize){
        let mut index = self.get_head();

        if index % width == 0{
            index += width - 1;
        }
        else{
            index -= 1;
        }

        self.body[0].0 = index;
    }

    fn move_up(&mut self, width: usize){
        let mut index = self.get_head();

        if index < width {
            index = width * width - width  + index
        }
        else{
            index -= width;
        }

        self.body[0].0 = index;
    }
}