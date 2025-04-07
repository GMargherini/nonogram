use std::{cell::RefCell, fmt::Display, rc::Rc};

#[derive(Debug)]
pub struct Line {
    cells: Vec<Rc<RefCell<Cell>>>,
    hint: Hint,
    direction: Direction,
}

#[derive(Debug)]
pub enum Direction {
    Row,
    Column,
}

impl Line {
    pub fn new(cells: Vec<Rc<RefCell<Cell>>>, hint: Vec<usize>, direction: Direction) -> Line {
        let hint = Hint::new(hint);
        Line {
            cells,
            hint,
            direction,
        }
    }

    pub fn hint(&self) -> &Vec<usize> {
        &self.hint.numbers
    }

    pub fn hint_obj(&self) -> &Hint {
        &self.hint
    }

    pub fn cell(&self, n: usize) -> &Rc<RefCell<Cell>> {
        &self.cells[n]
    }

    pub fn check(&self) -> bool {
        let mut checks = vec![];
        let mut full = 0;
        let mut other = 0;
        for cell in &self.cells {
            match cell.borrow().state {
                State::Full => {
                    full += 1;
                }
                State::Blocked | State::Empty => {
                    other += 1;
                    if full != 0 {
                        checks.push(full);
                    }
                    full = 0;
                }
            }
        }
        if checks.is_empty() {
            if other == 0 {
                checks.push(self.cells.len());
            } else {
                checks.push(full);
            }
        }
        checks == self.hint.numbers
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut cells = String::new();
        for cell in &self.cells {
            cells.push_str(&format!("{} ", RefCell::borrow(cell)));
        }
        match self.direction {
            Direction::Row => write!(f, "{cells}"),
            Direction::Column => write!(f, ""),
        }
    }
}

#[derive(Debug)]
pub struct Cell {
    state: State,
}
#[derive(Debug, PartialEq)]
enum State {
    Full,
    Blocked,
    Empty,
}

impl Cell {
    pub fn new() -> Cell {
        Cell {
            state: State::Empty,
        }
    }

    pub fn mark(&mut self) {
        self.state = State::Full;
    }

    pub fn block(&mut self) {
        self.state = State::Blocked;
    }

    pub fn wipe(&mut self) {
        self.state = State::Empty;
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = match self.state {
            State::Full => "■",
            State::Blocked => "⊠",
            State::Empty => "□",
        };
        write!(f, "{repr}")
    }
}

#[derive(Debug)]
pub struct Hint {
    numbers: Vec<usize>,
}

impl Hint {
    pub fn new(numbers: Vec<usize>) -> Hint {
        Hint { numbers }
    }
}

impl Display for Hint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut nums = String::new();
        for n in &self.numbers {
            nums.push_str(&format!("{n} "));
        }
        write!(f, "{}", &nums)
    }
}
