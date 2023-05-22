mod game;
use game::minesweeper;

fn main() {
    let mut game = minesweeper::MinesweeperGame::new(16, 30);
    game.lay_mines(99);

    game.print_board();
}
