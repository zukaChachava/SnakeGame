use getrandom;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[wasm_bindgen]
#[derive(PartialEq, Copy, Clone)]
pub enum GameStatus {
    NotStarted,
    Started,
    Finished,
    Failed,
}

enum SnakeUpdateState {
    Grown,
    Same,
    Dead,
    Won,
}

#[derive(PartialEq)]
enum DirectionChange {
    Previous(Direction),
    Empty,
}

struct Snake {
    body: Vec<Cell>,
    direction: Direction,
    direction_change: DirectionChange,
}

struct Cell(usize);

#[wasm_bindgen]
pub struct World {
    width: usize,
    game_status: GameStatus,
    snake: Snake,
    reward_cell: usize,
}

fn get_random(modulo: usize) -> usize {
    let mut bytes: [u8; 1] = [0];
    let result = getrandom::getrandom(&mut bytes);

    match result {
        Result::Ok(_) => bytes[0] as usize / modulo,
        Result::Err(_) => panic!("Random Error"),
    }
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, start_index: usize) -> World {
        World {
            width: width,
            game_status: GameStatus::NotStarted,
            snake: Snake::new(start_index),
            reward_cell: get_random(width - 1),
        }
    }

    pub fn start_game(&mut self) {
        if self.game_status == GameStatus::NotStarted {
            self.game_status = GameStatus::Started;
            return;
        }

        panic!("Invalid Game Status");
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn snake_head(&self) -> usize {
        self.snake.get_head()
    }

    pub fn snake_len(&self) -> usize {
        self.snake.len()
    }

    pub fn snake_body_position(&self, body_index: usize) -> usize {
        self.snake.get_location(body_index)
    }

    pub fn change_direction(&mut self, direction: Direction) {
        self.snake.change_direction(direction);
    }

    pub fn get_reward_cell(&self) -> usize {
        self.reward_cell
    }

    pub fn get_game_status(&self) -> GameStatus {
        self.game_status
    }

    pub fn update(&mut self) {
        if self.game_status != GameStatus::Started {
            panic!("Can Not Update NonStarted Game");
        }

        let status = self.snake.update(self.width(), self.reward_cell);

        match status {
            SnakeUpdateState::Grown => self.generate_reward_cell(),
            SnakeUpdateState::Same => return,
            SnakeUpdateState::Dead => self.game_failed(),
            SnakeUpdateState::Won => self.game_finished(),
        }
    }

    fn game_finished(&mut self) {
        self.game_status = GameStatus::Finished
    }

    fn game_failed(&mut self) {
        self.game_status = GameStatus::Failed
    }

    fn generate_reward_cell(&mut self) {
        let mut new_index = get_random(self.width - 1);

        while self.snake.index_taken(new_index) {
            new_index = get_random(self.width - 1);
        }

        self.reward_cell = new_index;
    }
}

impl Snake {
    fn new(index: usize) -> Snake {
        Snake {
            body: vec![Cell(index)],
            direction: Direction::Right,
            direction_change: DirectionChange::Empty,
        }
    }

    fn get_head(&self) -> usize {
        self.body[0].0
    }

    fn len(&self) -> usize {
        self.body.len()
    }

    fn get_location(&self, body_index: usize) -> usize {
        if body_index > self.len() {
            return 0;
        }

        return self.body[body_index].0;
    }

    fn change_direction(&mut self, direction: Direction) {
        let valid_move = match &self.direction_change {
            DirectionChange::Empty => Snake::direction_valid(&self.direction, &direction),
            DirectionChange::Previous(from) => Snake::direction_valid(&from, &direction),
        };

        if !valid_move {
            return;
        }

        self.direction_change = DirectionChange::Previous(self.direction);
        self.direction = direction;
    }

    fn direction_valid(from: &Direction, direction: &Direction) -> bool {
        if *from == Direction::Left && *direction == Direction::Right
            || *from == Direction::Right && *direction == Direction::Left
            || *from == Direction::Up && *direction == Direction::Down
            || *from == Direction::Down && *direction == Direction::Up
        {
            return false;
        }

        return true;
    }

    fn grow(&mut self) {
        let last_cell = self.body[self.body.len() - 1].0;
        self.body.push(Cell(last_cell));
    }

    fn update(&mut self, width: usize, reward_cell: usize) -> SnakeUpdateState {
        let mut snake_update_state = SnakeUpdateState::Same;

        if reward_cell == self.get_head() {
            self.grow();
            snake_update_state = SnakeUpdateState::Grown;
        }

        match self.direction {
            Direction::Right => self.move_right(width),
            Direction::Down => self.move_down(width),
            Direction::Left => self.move_left(width),
            Direction::Up => self.move_up(width),
        }

        if self.is_dead() {
            snake_update_state = SnakeUpdateState::Dead;
        }

        if self.is_won(width) {
            snake_update_state = SnakeUpdateState::Won;
        }

        self.direction_change = DirectionChange::Empty;
        snake_update_state
    }

    fn move_right(&mut self, width: usize) {
        let mut index = self.get_head();
        if (index + 1) % width == 0 {
            index = index - (width - 1);
        } else {
            index += 1;
        }

        self.move_body();
        self.body[0].0 = index;
    }

    fn move_down(&mut self, width: usize) {
        let mut index = self.get_head() + width;

        if index > width * width - 1 {
            index = index % width;
        }

        self.move_body();
        self.body[0].0 = index;
    }

    fn move_left(&mut self, width: usize) {
        let mut index = self.get_head();

        if index % width == 0 {
            index += width - 1;
        } else {
            index -= 1;
        }

        self.move_body();
        self.body[0].0 = index;
    }

    fn move_up(&mut self, width: usize) {
        let mut index = self.get_head();

        if index < width {
            index = width * width - width + index
        } else {
            index -= width;
        }

        self.move_body();
        self.body[0].0 = index;
    }

    fn move_body(&mut self) {
        for i in (1..self.body.len()).rev() {
            self.body[i].0 = self.body[i - 1].0;
        }
    }

    fn index_taken(&self, index: usize) -> bool {
        for i in &self.body {
            if i.0 == index {
                return true;
            }
        }

        return false;
    }

    fn is_dead(&self) -> bool {
        for i in 1..self.body.len() {
            if self.body[0].0 == self.body[i].0 {
                return true;
            }
        }

        return false;
    }

    fn is_won(&self, width: usize) -> bool {
        self.body.len() == width * width - 1
    }
}
