mod game;
use game::minesweeper;

fn main() {
    let game = minesweeper::MinesweeperGame::new(27, 32);
    println!("Hello, world!");
}
