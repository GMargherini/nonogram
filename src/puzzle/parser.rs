use super::Puzzle;
use super::line::{Cell, Direction, Line};
use std::cell::RefCell;
use std::fmt::Display;
use std::io::prelude::*;
use std::{fs::File, rc::Rc};
use yaml_rust::{ScanError, Yaml, YamlLoader};

pub type CellRef = Rc<RefCell<Cell>>;

pub struct PuzzleParser;

impl PuzzleParser {
    pub fn import(path: &str) -> Result<Puzzle, InputError> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        Self::parse_yaml(contents)
    }

    fn parse_yaml(contents: String) -> Result<Puzzle, InputError> {
        let yaml = YamlLoader::load_from_str(&contents)?;
        let puzzle = &yaml[0];
        let dimensions = match Self::get_dimensions(puzzle) {
            Some(dims) => dims,
            None => {
                return Err(InputError {
                    message: "Failed to parse dimensions".to_string(),
                });
            }
        };

        let mut cells: Vec<Vec<CellRef>> = Vec::with_capacity(dimensions.0);
        for _ in 0..dimensions.0 {
            let mut row: Vec<CellRef> = Vec::with_capacity(dimensions.1);
            for _ in 0..dimensions.1 {
                row.push(Rc::new(RefCell::new(Cell::new())));
            }
            cells.push(row);
        }

        let column_hints = Self::parse_hints(&puzzle["puzzle"]["columns"])?;
        let mut columns: Vec<Line> = Vec::new();
        for (i, col) in column_hints.iter().enumerate() {
            let cell_refs: Vec<CellRef> =
                cells.iter().map(|r| Rc::clone(&r[i])).collect();
            columns.push(Line::new(cell_refs, col.to_vec(), Direction::Column));
        }

        let row_hints = Self::parse_hints(&puzzle["puzzle"]["rows"])?;
        let mut rows: Vec<Line> = Vec::new();
        for (i, row) in row_hints.iter().enumerate() {
            let cell_refs: Vec<CellRef> = cells[i].iter().map(Rc::clone).collect();
            rows.push(Line::new(cell_refs, row.to_vec(), Direction::Row));
        }

        let puzzle = Puzzle {
            _dimensions: dimensions,
            columns,
            rows,
        };

        Ok(puzzle)
    }

    fn get_dimensions(puzzle: &Yaml) -> Option<(usize, usize)> {
        match (
            &puzzle["puzzle"]["rows"].as_hash(),
            &puzzle["puzzle"]["columns"].as_hash(),
        ) {
            (Some(x), Some(y)) => Some((x.len(), y.len())),
            _ => None,
        }
    }

    fn parse_hints(hints: &yaml_rust::Yaml) -> Result<Vec<Vec<usize>>, InputError> {
        let hints = match hints.as_hash() {
            Some(h) => h.values(),
            None => {
                return Err(InputError {
                    message: "Failed to parse hints".to_string(),
                });
            }
        };
        let lines = hints
            .into_iter()
            .map(|line| {
                line.as_vec()
                    .expect("Poorly formatted hints")
                    .iter()
                    .map(|x| x.as_i64().expect("Hint is not a number") as u64 as usize)
                    .collect()
            })
            .collect();
        Ok(lines)
    }
}

#[derive(Debug)]
pub struct InputError {
    message: String,
}
impl Display for InputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl From<std::io::Error> for InputError {
    fn from(value: std::io::Error) -> Self {
        InputError {
            message: value.to_string(),
        }
    }
}

impl From<ScanError> for InputError {
    fn from(value: ScanError) -> Self {
        InputError {
            message: format!(
                "Failed to parse yaml: line {}, col {}\nReason: {}",
                value.marker().line(),
                value.marker().col(),
                value,
            ),
        }
    }
}
