use std::cell::RefCell;
use std::fmt::{Display, write};
use std::io::prelude::*;
use std::{fs::File, rc::Rc};
use yaml_rust::YamlLoader;

#[derive(Debug)]
pub struct Puzzle {
    dimensions: (usize, usize),
    rows: Vec<Row>,
    columns: Vec<Column>,
}

impl Puzzle {
    pub fn import(path: &str) -> std::io::Result<Puzzle> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let yaml = YamlLoader::load_from_str(&contents).unwrap();
        let puzzle = &yaml[0];

        let dimensions = &puzzle["puzzle"]["dimensions"];
        let dimensions = (
            dimensions[0].as_i64().unwrap() as u64 as usize,
            dimensions[1].as_i64().unwrap() as u64 as usize,
        );
        let mut cells: Vec<Vec<Rc<RefCell<Cell>>>> = Vec::with_capacity(dimensions.0);
        for _ in 0..dimensions.0 {
            let mut row: Vec<Rc<RefCell<Cell>>> = Vec::with_capacity(dimensions.1);
            for _ in 0..dimensions.1 {
                row.push(Rc::new(RefCell::new(Cell::new())));
            }
            cells.push(row);
        }

        let column_hints = Self::parse_hints(&puzzle["puzzle"]["columns"]);
        let mut columns: Vec<Column> = Vec::new();
        for (i, col) in column_hints.iter().enumerate() {
            let cell_refs: Vec<Rc<RefCell<Cell>>> = cells.iter().map(|r| Rc::clone(&r[i])).collect();
            columns.push(Column::new(cell_refs, col.to_vec()));
        }

        let row_hints = Self::parse_hints(&puzzle["puzzle"]["rows"]);
        let mut rows: Vec<Row> = Vec::new();
        for (i, row) in row_hints.iter().enumerate() {
            let cell_refs: Vec<Rc<RefCell<Cell>>> = cells[i].iter().map(|r| Rc::clone(r)).collect();
            rows.push(Row::new(cell_refs, row.to_vec()));
        }

        let puzzle = Puzzle {
            dimensions,
            columns,
            rows,
        };

        Ok(puzzle)
    }

    fn parse_hints(hints: &yaml_rust::Yaml) -> Vec<Vec<usize>> {
        let hints = hints.as_hash().unwrap().values();
        let mut lines: Vec<Vec<usize>> = Vec::new();
        for line in hints {
            lines.push(
                line.as_vec()
                    .unwrap()
                    .iter()
                    .map(|x| x.as_i64().unwrap() as u64 as usize)
                    .collect(),
            );
        }
        lines
    }

    pub fn display(&self) -> String{
		let mut schema = String::new();
        let height = self
            .columns
            .iter()
            .map(|col| col.hint.numbers.len())
            .max()
            .unwrap();
        
		let width = self
            .rows
            .iter()
            .map(|row| (row.hint.numbers.len() * 2) - 1)
            .max()
            .unwrap();

        for i in 0..height {
            schema.push_str(&format!("{} ", " ".repeat(width)));
            for col in &self.columns {
                schema.push_str(&match col.hint.numbers.get(i) {
                    Some(n) => format!("{n} "),
                    None => format!("  "),
                });
            }
            schema.push_str(&format!("\n"));
        }

		schema.push_str(&self.display_rows());

		println!("{schema}");
		schema
    }

	pub fn display_rows(&self) -> String {
		let width = self
            .rows
            .iter()
            .map(|row| (row.hint.numbers.len() * 2) - 1)
            .max()
            .unwrap();
		
		let mut schema = String::new();

		for row in &self.rows {

			schema.push_str(&if (row.hint.numbers.len() * 2) - 1 < width {
				let diff = width - (row.hint.numbers.len() * 2);
				format!("{} {}{row}\n", " ".repeat(diff), row.hint)
			} else {
				format!("{}{row}\n", row.hint)
			});
            
        }
		schema
	}

	pub fn display_grid(&self) -> String {
		let mut schema = String::new();

		for row in &self.rows {
			schema.push_str(&format!("{row}\n"));
        }
		schema
	}

	pub fn mark_cell(&self, x: usize, y: usize) {
		self.rows[y].cells[x].borrow_mut().mark();
	}

	pub fn block_cell(&self, x: usize, y: usize) {
		self.rows[y].cells[x].borrow_mut().block();
	}

	pub fn wipe_cell(&self, x: usize, y: usize) {
		self.rows[y].cells[x].borrow_mut().wipe();
	}

	pub fn check(&self) -> bool{
		self.rows.iter().fold(true, |acc, row| acc && row.check()) &&
		self.columns.iter().fold(true, |acc, col| acc && col.check())
	}
}

#[derive(Debug)]
struct Row {
    cells: Vec<Rc<RefCell<Cell>>>,
    hint: Hint,
}

impl Row {
    fn new(cells: Vec<Rc<RefCell<Cell>>>, hint: Vec<usize>) -> Row {
        let hint = Hint::new(hint);
        Row { cells, hint }
    }

	fn check(&self) -> bool {
		let mut checks = vec![];
		let mut full = 0;
		let mut other = 0;
		for cell in &self.cells {
			match cell.borrow().state {
				State::Full => full+=1,
				State::Blocked | State::Empty => {
					other+=1;
					if full!=0 {checks.push(full);}
					full = 0;
				},
			}
		}
		checks == self.hint.numbers
	}
}

impl Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut cells = String::new();
        for cell in &self.cells {
            cells.push_str(&format!("{} ", RefCell::borrow(cell)));
        }
        write!(f, "{cells}")
    }
}

#[derive(Debug)]
struct Column {
    cells: Vec<Rc<RefCell<Cell>>>,
    hint: Hint,
}

impl Column {
    fn new(cells: Vec<Rc<RefCell<Cell>>>, hint: Vec<usize>) -> Column {
        let hint = Hint::new(hint);
        Column { cells, hint }
    }

	fn check(&self) -> bool {
		let mut checks = vec![];
		let mut full = 0;
		let mut other = 0;
		for cell in &self.cells {
			match cell.borrow().state {
				State::Full => full+=1,
				State::Blocked | State::Empty => {
					other+=1;
					if full!=0 {checks.push(full);}
					full = 0;
				},
			}
		}
		checks == self.hint.numbers
	}
}
#[derive(Debug)]
struct Cell {
    state: State,
}
#[derive(Debug, PartialEq)]
enum State {
    Full,
    Blocked,
    Empty,
}

impl Cell {
    fn new() -> Cell {
        Cell {
            state: State::Empty,
        }
    }

    fn mark(&mut self) {
        self.state = State::Full;
    }

    fn block(&mut self) {
        self.state = State::Blocked;
    }

    fn wipe(&mut self) {
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
struct Hint {
    numbers: Vec<usize>,
}

impl Hint {
    fn new(numbers: Vec<usize>) -> Hint {
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
