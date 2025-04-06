use std::fs::File;
use std::io::prelude::*;
use yaml_rust::YamlLoader;

#[derive(Debug)]
pub struct Puzzle<'a> {
	dimensions: (usize, usize),
	cells: Vec<Vec<Cell>>,
	rows: Vec<Row<'a>>,
	columns: Vec<Column<'a>>,
}

impl Puzzle<'_>	{
	pub fn import(path: &str) -> std::io::Result<Puzzle> {
		let mut file = File::open(path)?;
		let mut contents = String::new();
		file.read_to_string(&mut contents)?;
		
		let yaml = YamlLoader::load_from_str(&contents).unwrap();
		let puzzle = &yaml[0];
		
		let dimensions = &puzzle["puzzle"]["dimensions"];
		let dimensions = (
			dimensions[0].as_i64().unwrap() as u64 as usize,
			dimensions[1].as_i64().unwrap() as u64 as usize
		);
		let mut cells: Vec<Vec<Cell>> = Vec::with_capacity(dimensions.0);
		for _ in 0..dimensions.0 {
			let mut row: Vec<Cell> = Vec::new();
			for _ in 0..dimensions.1 {
				row.push(Cell::new());
			}
			cells.push(row);
		}
		
		let column_hints = Self::parse_hints(&puzzle["puzzle"]["columns"]);
		let mut columns: Vec<Column> = Vec::new();
		for (i, col) in column_hints.iter().enumerate() {
			let cell_refs: Vec<&Cell> = cells.iter().map(|r| &r[i]).collect();
			columns.push(
				Column::new(
					cell_refs,
					col.to_vec()
				)
			);
		}

		let row_hints = Self::parse_hints(&puzzle["puzzle"]["rows"]);
		let mut rows: Vec<Row> = Vec::new();
		for (i, row) in row_hints.iter().enumerate() {
			let cell_refs: Vec<&Cell> = cells[i].iter().collect();
			rows.push(
				Row::new(
					cell_refs,
					row.to_vec()
				)
			);
		}

		let cells = cells.iter().map(|r| r.iter().map(|c| c.copy()).collect()).collect();
		
		let puzzle = Puzzle {
			dimensions,
			cells,
			columns,
			rows,
		};
		println!("{:#?}", puzzle);

		Ok(puzzle)
	}

	fn parse_hints(hints: &yaml_rust::Yaml) -> Vec<Vec<usize>> {
		let hints = hints.as_hash().unwrap().values();
		let mut lines: Vec<Vec<usize>> = Vec::new(); 
		for line in hints {
			lines.push(
				line.as_vec().unwrap()
							.iter()
							.map(|x| x.as_i64().unwrap() as u64 as usize)
							.collect()
			);
		}
		lines
	}
}

#[derive(Debug)]
struct Row<'a> {
	cells: Vec<&'a Cell>,
	hint: Hint,
}

impl Row<'_> {
	fn new(cells: Vec<&Cell>, hint: Vec<usize>) -> Row {
		let hint = Hint::new(hint);
		Row {
			cells,
			hint,
		}
	}
}

#[derive(Debug)]
struct Column<'a> {
	cells: Vec<&'a Cell>,
	hint: Hint,
}

impl Column<'_> {
	fn new(cells: Vec<&Cell>, hint: Vec<usize>) -> Column {
		let hint = Hint::new(hint);
		Column {
			cells,
			hint,
		}
	}
}

#[derive(Debug, Copy)]
enum Cell {
	Full,
	Blocked,
	Empty,
}

impl Cell {
	fn new() -> Cell {
		Cell::Empty
	}
}

#[derive(Debug)]
struct Hint {
	numbers: Vec<usize>,
}

impl Hint {
	fn new(numbers: Vec<usize>) -> Hint {
		Hint {
			numbers
		}
	}
}
