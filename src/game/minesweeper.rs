use rand::Rng;

#[derive(Debug, Clone)]
enum BlockState {
    ReadEmpty,
    ClickedQuestionMark,
    BlackBomb,
    BombWithX,
    BombRedBackground,
    QuestionMark,
    Flag,
    EmptyUnclicked,
    BorderValue,
}

#[derive(Debug, Clone)]
struct Block {
    state: BlockState,
    is_bomb: bool,
    is_revealed: bool,
}

impl Default for Block {
    fn default() -> Self {
        Block {
            state: BlockState::EmptyUnclicked,
            is_bomb: false,
            is_revealed: false,
        }
    }
}


#[derive(Debug)]
enum Smile {
    Normal,
    Wow,
    Lost,
    Winner,
    Clicked,
}

#[derive(Debug)]
struct BoardPoint {
    col: usize,
    row: usize,
}

#[derive(Debug)]
pub struct MinesweeperGame {
    smile: Smile,

    width: usize,
    height: usize,

    block_array: Vec<Vec<Block>>,

    timer_sec: i32,

    num_empty_blocks: i32,
    num_revealed_blocks: i32,
    
}

impl MinesweeperGame {
    pub fn new(width: usize, height: usize) -> MinesweeperGame {
        let block_array = vec![vec![Block::default(); height]; width];

        MinesweeperGame {
            smile: Smile::Normal,
            width: width,
            height: height,
            block_array: block_array,
            timer_sec: 0,
            num_empty_blocks: 0,
            num_revealed_blocks: 0,
        }
    }

    pub fn lay_mines(&mut self, num_mines: usize) {
        let mut rng = rand::thread_rng();
        let mut mines: usize = 0;

        while mines < num_mines {
            let col = rng.gen_range(0..self.width);
            let row = rng.gen_range(0..self.height);

            if self.block_array[col][row].is_bomb {
                continue;
            }

            self.block_array[col][row].is_bomb = true;

            mines += 1;
        }
    }

}
