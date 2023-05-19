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
    col: i32,
    row: i32,
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
        let block_array = vec![vec![Block::default(); width]; height];

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
}
