mod game;
use game::minesweeper;

fn main() {
    let mut game = minesweeper::MinesweeperGame::new(3, 3);
    game.lay_mines(5);

    println!("This is mines\n{:?}", game);
}
