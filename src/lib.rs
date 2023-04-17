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

    pub fn snake_len(&self) -> usize {
        self.snake.len()
    }

    pub fn snake_body_position(&self, body_index: usize) -> usize{
        self.snake.get_location(body_index)
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
        Snake { body: vec![Cell(index), Cell(index - 1), Cell(index - 2)], direction: Direction::Right, direction_change: DirectionChange::Empty }
    }

    fn get_head(&self) -> usize {
        self.body[0].0
    }

    fn len(&self) -> usize {
        self.body.len()
    }

    fn get_location(&self, body_index: usize) -> usize{
        if body_index > self.len(){
            return 0;
        }

        return self.body[body_index].0;
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
        let prev_index = index;

        if (index + 1) % width == 0  {
            index = index - (width - 1);
        }
        else{
            index += 1;
        }

        self.body[0].0 = index;
        self.move_body(prev_index);
    }

    fn move_down(&mut self, width: usize){
        let mut index = self.get_head() + width;
        let prev_index = self.get_head();

        if index > width * width - 1 {
            index = index % width;
        }

        self.body[0].0 = index;
        self.move_body(prev_index);
    }

    fn move_left(&mut self, width: usize){
        let mut index = self.get_head();
        let prev_index = index;

        if index % width == 0{
            index += width - 1;
        }
        else{
            index -= 1;
        }

        self.body[0].0 = index;
        self.move_body(prev_index);
    }

    fn move_up(&mut self, width: usize){
        let mut index = self.get_head();
        let prev_index = index;

        if index < width {
            index = width * width - width  + index
        }
        else{
            index -= width;
        }

        self.body[0].0 = index;
        self.move_body(prev_index);
    }

    fn move_body(&mut self, previous_head: usize){
        if self.body.len() < 2 {
            return;
        }

        for i in (2..self.body.len()).rev() {
            self.body[i].0 = self.body[i-1].0;
        }

        self.body[1].0 = previous_head;
    }
}