mod line;
use line::{Cell, Direction, Line};
use std::cell::RefCell;
use std::io::prelude::*;
use std::{fs::File, rc::Rc};
use yaml_rust::{Yaml, YamlLoader};

#[derive(Debug)]
pub struct Puzzle {
    _dimensions: (usize, usize),
    rows: Vec<Line>,
    columns: Vec<Line>,
}

impl Puzzle {
    pub fn import(path: &str) -> std::io::Result<Puzzle> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let puzzle = match Self::parse_yaml(contents) {
            Ok(p) => Ok(p),
            Err(e) => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, e)),
        };
        puzzle
    }

    fn parse_yaml(contents: String) -> Result<Puzzle, String> {
        let yaml = match YamlLoader::load_from_str(&contents) {
            Ok(y) => y,
            Err(_) => return Err("Failed to Scan File".to_string()),
        };
        let puzzle = &yaml[0];
        let dimensions = match Self::get_dimensions(&puzzle) {
            Some(dims) => dims,
            None => return Err("Failed to parse dimensions".to_string()),
        };

        let mut cells: Vec<Vec<Rc<RefCell<Cell>>>> = Vec::with_capacity(dimensions.0);
        for _ in 0..dimensions.0 {
            let mut row: Vec<Rc<RefCell<Cell>>> = Vec::with_capacity(dimensions.1);
            for _ in 0..dimensions.1 {
                row.push(Rc::new(RefCell::new(Cell::new())));
            }
            cells.push(row);
        }

        let column_hints = Self::parse_hints(&puzzle["puzzle"]["columns"])?;
        let mut columns: Vec<Line> = Vec::new();
        for (i, col) in column_hints.iter().enumerate() {
            let cell_refs: Vec<Rc<RefCell<Cell>>> =
                cells.iter().map(|r| Rc::clone(&r[i])).collect();
            columns.push(Line::new(cell_refs, col.to_vec(), Direction::Column));
        }

        let row_hints = Self::parse_hints(&puzzle["puzzle"]["rows"])?;
        let mut rows: Vec<Line> = Vec::new();
        for (i, row) in row_hints.iter().enumerate() {
            let cell_refs: Vec<Rc<RefCell<Cell>>> = cells[i].iter().map(|r| Rc::clone(r)).collect();
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
        let dimensions = match (
            &puzzle["puzzle"]["rows"].as_hash(),
            &puzzle["puzzle"]["columns"].as_hash(),
        ) {
            (Some(x), Some(y)) => Some((x.len(), y.len())),
            _ => None,
        };
        dimensions
    }

    fn parse_hints(hints: &yaml_rust::Yaml) -> Result<Vec<Vec<usize>>, String> {
        let hints = match hints.as_hash() {
            Some(h) => h.values(),
            None => return Err("Failed to parse hints".to_string()),
        };
        let lines = hints
            .into_iter()
            .map(|line| {
                line.as_vec()
                    .unwrap()
                    .iter()
                    .map(|x| x.as_i64().unwrap() as u64 as usize)
                    .collect()
            })
            .collect();
        Ok(lines)
    }

    pub fn display(&self) {
        let mut schema = String::new();
        let height = self
            .columns
            .iter()
            .map(|col| col.hint().len())
            .max()
            .unwrap();

        let width = self
            .rows
            .iter()
            .map(|row| (row.hint().len() * 2) - 1)
            .max()
            .unwrap();

        for i in 0..height {
            schema.push_str(&format!("{} ", " ".repeat(width)));
            for col in &self.columns {
                let hints: Vec<usize> = col.hint().iter().rev().cloned().collect();
                schema.push_str(&match hints.get(height - i - 1) {
                    Some(n) => format!("{n} "),
                    None => format!("  "),
                });
            }
            schema.push_str(&format!("\n"));
        }

        schema.push_str(&self.display_rows(width));

        println!("{}[2J {schema}", 27 as char);
    }

    pub fn display_rows(&self, width: usize) -> String {
        let mut schema = String::new();

        for row in &self.rows {
            schema.push_str(&if (row.hint().len() * 2) - 1 < width {
                let diff = width - (row.hint().len() * 2);
                format!("{} {}{row}\n", " ".repeat(diff), row.hint_obj())
            } else {
                format!("{}{row}\n", row.hint_obj())
            });
        }
        schema
    }

    pub fn act_on_cell(&self, play: Play, x: usize, y: usize) {
        let mut cell = self.rows[y - 1].cell(x - 1).borrow_mut();
        match play {
            Play::Mark => cell.mark(),
            Play::Block => cell.block(),
            Play::Wipe => cell.wipe(),
        }
    }

    pub fn check(&self) -> bool {
        self.rows.iter().fold(true, |acc, row| acc && row.check())
            && self
                .columns
                .iter()
                .fold(true, |acc, col| acc && col.check())
    }
}

pub enum Play {
    Mark,
    Block,
    Wipe,
}

impl Play {
    pub fn build(play: &str) -> Option<Play> {
        match play {
            "m" | "M" => Some(Play::Mark),
            "b" | "B" => Some(Play::Block),
            "w" | "W" => Some(Play::Wipe),
            _ => None,
        }
    }
}
