use crate::Board;
use crate::Field;

pub enum IterType {
    LineIter,
    ColIter,
    QuadIter,
    FieldIter,
}

pub struct BoardIter<'a> {
    itertype: IterType,
    b: &'a Board,
    line: usize,
    col: usize,
    idx: usize,
    quadrant_idx: usize,
    start_line: usize,
    start_col: usize,
}

impl<'a> Iterator for BoardIter<'a> {
    type Item = &'a Field;

    fn next(&mut self) -> Option<&'a Field> {
        if self.b.check_valid_coord(self.line, self.col) {
            self.idx = self.b.idx_from_line_col(self.line, self.col);
            self.advance_idx();
            Some(& self.b.content[self.idx])
        } else {
            None
        }
    }
}

impl<'a> BoardIter<'a> {
    pub fn new_line_iter(b: &'a Board, line: usize) -> Self {
        BoardIter { itertype: IterType::LineIter, b, line, col: 0, idx: 0, quadrant_idx: 0, start_line: 0, start_col: 0}
    }

    pub fn new_col_iter(b: &'a Board, col: usize) -> Self {
        BoardIter { itertype: IterType::ColIter, b, line: 0, col, idx: 0, quadrant_idx: 0, start_line: 0, start_col: 0}
    }

    pub fn new_quad_iter(b: &'a Board, line: usize, col: usize) -> Self {
        let (line, col) = b.quadrant_start_from_line_col(line, col);
        BoardIter { itertype: IterType::QuadIter, b, line, col, idx: 0, quadrant_idx: 0, start_line: line, start_col: col}
    }

    pub fn new_field_iter(b: &'a Board) -> Self {
        BoardIter { itertype: IterType::FieldIter, b, line: 0, col: 0, idx: 0, quadrant_idx: 0, start_line: 0, start_col: 0}
    }

    fn advance_idx(&mut self) {
        match self.itertype {
            IterType::LineIter => self.col += 1,
            IterType::ColIter  => self.line += 1,
            IterType::QuadIter => {
                self.quadrant_idx += 1;
                if self.quadrant_idx < self.b.line_size {
                    let line_adv = self.quadrant_idx / self.b.base;
                    let col_adv = self.quadrant_idx - line_adv * self.b.base;
                    self.line = self.start_line + line_adv;
                    self.col = self.start_col + col_adv;
                } else {
                    self.line = self.b.line_size;
                    self.col = self.b.line_size;
                }
            },
            IterType::FieldIter => {
                (self.line, self.col) = self.b.line_col_from_idx(self.idx + 1);
            },
        }
    }
}
