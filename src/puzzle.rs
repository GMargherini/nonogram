mod line;
mod parser;
use line::Line;
use parser::{InputError, PuzzleParser};

#[derive(Debug)]
pub struct Puzzle {
    _dimensions: (usize, usize),
    rows: Vec<Line>,
    columns: Vec<Line>,
}

impl Puzzle {
    pub fn build(path: &str) -> Result<Self, InputError> {
        PuzzleParser::import(path)
    }

    pub fn display(&self) {
        let mut schema = String::new();
        let height = self
            .columns
            .iter()
            .map(|col| col.hint().len())
            .max()
            .unwrap_or(0);

        let width = self
            .rows
            .iter()
            .map(|row| (row.hint().len() * 2) - 1)
            .max()
            .unwrap_or(0);

        for i in 0..height {
            schema.push_str(&" ".repeat(width));
            for col in &self.columns {
                let hints: Vec<usize> = col.hint().iter().rev().cloned().collect();
                schema.push_str(&match hints.get(height - i - 1) {
                    Some(n) => format!("{n} "),
                    None => "  ".to_string(),
                });
            }
            schema.push('\n');
        }

        schema.push_str(&self.display_rows(width));

        println!("{esc}[2J{esc}[1;1H {schema}", esc = 27 as char);
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
        self.rows.iter().all(|row| row.check()) && self.columns.iter().all(|col| col.check())
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
