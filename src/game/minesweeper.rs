use fastrand;

#[derive(Debug, Clone)]
enum BlockState {
    Hidden,
    Flagged,
    Questioned,
    Revealed,
    Exploded, // Only for bombs
}

#[derive(Debug, Clone)]
enum Block {
    Empty { state: BlockState, adjacent_bombs: usize },
    Bomb { state: BlockState },
}

impl Default for Block {
    fn default() -> Self {
        Block::Empty { state: BlockState::Hidden, adjacent_bombs: 0 }
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

        for _ in 0..num_mines {

            let (c, r) = loop {
                let (c, r) = (
                    fastrand::usize(..self.width),
                    fastrand::usize(..self.height)
                );

                match self.block_array[c][r] {
                    Block::Empty { state: _, adjacent_bombs: _ } => {
                        break (c, r)
                    }
                    _ => continue,
                }
            };

            self.block_array[c][r] = Block::Bomb { state: BlockState::Hidden };

            self.increment_adjacent_bombs(c, r);

        }
                /*
        while mines < num_mines {
            let col = fastrand::usize(..self.width);
            let row = fastrand::usize(..self.height);

            if self.block_array[col][row].is_bomb {
                continue;
            }

            self.block_array[col][row].is_bomb = true;

            mines += 1;
        }
        */
    }

    fn increment_adjacent_bombs(&mut self, col: usize, row: usize) {
        let neighbors = [
            (col.wrapping_sub(1), row.wrapping_sub(1)),
            (col.wrapping_sub(1), row),
            (col.wrapping_sub(1), row + 1),
            (col, row.wrapping_sub(1)),
            (col, row + 1),
            (col + 1, row.wrapping_sub(1)),
            (col + 1, row),
            (col + 1, row + 1),
        ];

        for &(ncol, nrow) in &neighbors {
            if ncol < self.width && nrow < self.height {
                match &mut self.block_array[ncol][nrow] {
                    Block::Empty { adjacent_bombs, .. } => *adjacent_bombs += 1,
                    _ => {},
                }
            }
        }
    }

    pub fn print_board(&self) {
        for row in &self.block_array {
            for block in row {
                let symbol = match block {
                    Block::Bomb  { .. }  => "💣",
                    Block::Empty { adjacent_bombs, .. } => {
                        match adjacent_bombs {
                            0 => "　",
                            //0 => "🟦",
                            1 => "１",
                            2 => "２",
                            3 => "３",
                            4 => "４",
                            5 => "５",
                            6 => "６",
                            7 => "７",
                            8 => "８",
                            _ => unreachable!(),
                        }
                    }
                };
                /*
                let symbol = match block.state {
                    BlockState::ReadEmpty => ".",
                    BlockState::ClickedQuestionMark => "?",
                    BlockState::BlackBomb => "*",
                    BlockState::BombWithX => "X",
                    BlockState::BombRedBackground => "R",
                    BlockState::QuestionMark => "?",
                    BlockState::Flag => "F",
                    BlockState::EmptyUnclicked => ".",
                    BlockState::BorderValue => "B",
                };
                */
                print!("{}", symbol);
            }
            println!();
        }
    }
}
