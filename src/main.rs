mod puzzle;
use puzzle::Puzzle;

fn main() {
    let puzzle = Puzzle::import("puzzle.yaml").unwrap();
    _ = puzzle.display();
	puzzle.mark_cell(0, 0);
	puzzle.display();
	puzzle.mark_cell(2, 0);
	puzzle.display();
	println!("{}",puzzle.check());
}
