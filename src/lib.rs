mod board_iterator;

use rand::Rng;
pub use crate::board_iterator::BoardIter;

#[derive(Clone)]
pub enum Field {
    Number(usize),
    OptionList(Vec<usize>),
}

enum SolverStep {
    Unsolvable,
    BranchOnOptionList(usize),
    Solved,
}

pub struct Board {
    content: Vec<Field>,
    base: usize,
    line_size: usize,
}

// Order       Line        Board        String
//   2      2 * 2 =  4   4 *  4 =  16     16
//   3      3 * 3 =  9   9 *  9 =  81     81
//   4      4 * 4 = 16  16 * 16 = 256    512
//   5      5 * 5 = 25  25 * 25 = 625   1250
//   6      6 * 6 = 36
//   7      7 * 7 = 49
//   8      8 * 8 = 64
//   9      9 * 9 = 81

impl Board {
    pub fn new() -> Self {
        Board {
            content: Vec::new(),
            base: 0,
            line_size: 0,
        }
    }

    pub fn clone(&self) -> Self {
        Board {
            content: self.content.to_vec(),
            base: self.base,
            line_size: self.line_size,
        }
    }

    fn reset(&mut self, base: usize) {
        self.content.clear();
        self.base = base;
        self.line_size = self.base.pow(2);

        let board_size = self.line_size.pow(2);
        self.content.reserve_exact(board_size);
        for _ in 0..board_size {
            self.content.push( Field::OptionList( (0..self.line_size).collect() ) );
        }
    }

    pub fn read(&mut self, input: &str) -> bool {
        let len = input.len();
        
        // find base
        let mut board_size = 0;
        self.base = 0;
        self.line_size = 0;

        for i in 2usize..10usize {
            let p = i.pow(4);
            if (p == len) || (p == (len / 2)) {
                self.base = i;
                board_size = p;
                break;
            }
        }

        // bail out if string length not matching
        if self.base == 0 {
            return false;
        }

        // reset board content
        self.reset(self.base);

        // parse string and fill numbers
        let num_size = if self.base <= 3 {1usize} else {2usize};
        for i in 0..board_size {
            let i = i;
            let num = &input[(i*num_size)..((i+1)*num_size)];
            if let Ok(num) = num.parse::<usize>() {
                if !self.set_num_index(i, num - 1) {
                    return false;
                }
            }
        }

        return true;
    }

    pub fn print(&self, pretty_print: bool) -> String {
        let mut res = String::from("");
        let width = if self.line_size > 9 {2} else {1};
        for i in 0..self.content.len() {
            match self.content[i] {
                Field::Number(n) => res = format!("{0}{1:02$}", res, n + 1, width),
                _ => res = format!("{0}{1:.>2$}", res, "", width),
            }

            if pretty_print {
                match i {
                    n if ((n / self.line_size) % self.base == self.base - 1) && (n % self.line_size == self.line_size - 1) =>
                        res = format!("{0}\n{1:->2$}\n", res, "", width * self.line_size + self.base - 1),
                    n if n % self.line_size == self.line_size - 1 =>
                        res = format!("{}\n", res),
                    n if n % self.base == self.base - 1 =>
                        res = format!("{}|", res),
                    _ => (),
                }
            }
        }
        if !pretty_print {
            res = format!("{}\n", res);
        }

        res
    }

    fn get_num_index(&self, idx: usize) -> Option<usize> {
        if let Field::Number(n) = self.content[idx] {
            Some(n)
        } else {
            None
        }
    }

    fn clear_num_index(&mut self, idx: usize) {
        if let Field::Number(i) = self.content[idx] {
            self.content[idx] = Field::OptionList(vec!(i));
        }
    }

    fn set_num_index(&mut self, idx: usize, val: usize) -> bool {
        if self.base != 0 {
            // check inputs
            if !self.check_valid_number(val) {
                return false;
            }
            if !self.check_valid_index(idx) {
                return false;
            }

            // check if there is already a value
            if let Field::Number(_) = self.content[idx] {
                return false;
            }

            let (line, col) = self.line_col_from_idx(idx);

            // check line
            for i in 0..self.line_size {
                let board_index = self.idx_from_line_col(line , i);
                if board_index != idx {
                    if !self.update_field(board_index, val) {
                        return false;
                    }
                }
            }

            // check column
            for i in 0..self.line_size {
                let board_index = self.idx_from_line_col(i, col);
                if board_index != idx {
                    if !self.update_field(board_index, val) {
                        return false;
                    }
                }
            }

            // check quadrant
            let (quadrant_start_line, quadrant_start_col) = self.quadrant_start_from_line_col(line, col);
            for i in 0..self.base {
                for j in 0..self.base {
                    let board_index = self.idx_from_line_col(quadrant_start_line + i, quadrant_start_col + j);
                    if board_index != idx {
                        if !self.update_field(board_index, val) {
                            return false;
                        }
                    } 
                }
            }

            // insert value
            self.content[idx] = Field::Number(val);

            // maybe: update Option list if options for a number have the same line/column

            return true;
        }
        return false;
    }

    fn get_first_option_list(&self, idx: usize) -> Option<usize> {
        if let Field::OptionList(list) = &self.content[idx] {
            if list.len() > 0 {
                Some(list[0])
            } else {
                None
            }
        } else {
            None
        }
    }

    fn get_option_list(&self, idx: usize) -> Option<&[usize]> {
        if let Field::OptionList(list) = &self.content[idx] {
            if list.len() > 0 {
                Some(&list)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn remove_from_option_list(&mut self, idx: usize, val: usize) {
        if self.check_valid_number(val) && self.check_valid_index(idx) {
            if let Field::OptionList(list) = &mut self.content[idx] {
                if let Some(i) = list.iter().position(|&x| x == val) {
                    list.swap_remove(i);
                }
            }
        }
    }

    fn update_field(&mut self, idx: usize, val: usize) -> bool {
        match &mut self.content[idx] {
            // if number already present, we cannot set the given number
            Field::Number(n) if *n == val => false,
            // remove number from option lists
            Field::OptionList(v) => {
                if let Some(i) = v.iter().position(|&x| x == val) {
                    v.swap_remove(i);
                }
                true
            },
            _ => true,
        }
    }

    fn idx_from_line_col(&self, line: usize, col: usize) -> usize {
        line * self.line_size + col
    }

    fn line_col_from_idx(&self, idx: usize) -> (usize, usize) {
        let line = idx / self.line_size;
        (line, idx - line * self.line_size)
    }

    fn quadrant_start_from_line_col(&self, line: usize, col: usize) -> (usize, usize) {
        ((line / self.base) * self.base, (col / self.base) * self.base)
    }

    fn check_valid_index(&self, idx: usize) -> bool {
        !(idx >= (self.content.len()))
    }

    fn check_valid_number(&self, num: usize) -> bool {
        !(num >= self.line_size)
    }

    fn check_valid_coord(&self, line: usize, col: usize) -> bool {
        self.check_valid_index(self.idx_from_line_col(line, col)) &&
        (line < self.line_size) &&
        (col < self.line_size)
    }

    pub fn line_iter(&self, line: usize) -> BoardIter {
        if self.check_valid_coord(line, 0) {
            BoardIter::new_line_iter(self, line)
        } else {
            BoardIter::new_line_iter(self, 0)
        }
    }

    pub fn col_iter(&self, col: usize) -> BoardIter {
        if self.check_valid_coord(0, col) {
            BoardIter::new_col_iter(self, col)
        } else {
            BoardIter::new_col_iter(self, 0)
        }
    }

    pub fn quad_iter(&self, quad_idx: usize) -> BoardIter {
        if quad_idx <= self.line_size {
            let col = (quad_idx * self.base) % self.line_size;
            BoardIter::new_quad_iter(self, quad_idx, col)
        } else {
            BoardIter::new_quad_iter(self, 0, 0)
        }
    }

    pub fn iter(&self) -> BoardIter {
        BoardIter::new_field_iter(self)
    }

    fn find_single_option<'a, F, G>(&'a self, get_iterator: F, get_idx: G) -> Option<(usize, usize)>  // find a number that has only one option within a quadrant/line/col
    where
        F: Fn(usize) -> BoardIter<'a>,
        G: Fn(usize, usize) -> usize
    {
        for q_idx in 0..self.line_size {
            for n in 0..self.line_size {
                let iter = get_iterator(q_idx);
    
                let contains_number = iter.enumerate().filter(|x|
                    if let Field::OptionList(l) = x.1 {
                        l.iter().any(|elem| *elem == n)
                    } else {
                        false
                    }
                );
    
                let mut idx = 0;
                let mut count = 0;
                for (current_idx, _) in contains_number {
                    count += 1;
                    idx = current_idx;
                }
    
                if count == 1 {
                    return Some((n, get_idx(idx, q_idx)));
                }
            }
        }
        None
    }

    pub fn solve(self) -> Option<Self> {  // stack solver implementation
        let mut stack: Vec<Board> = Vec::new();
        stack.push(self);
    
        loop {
            let current_board = stack.pop();
            if let None = current_board {
                break None;
            }
            let mut current_board = current_board.unwrap();
    
            let next_step = loop {
                let mut cont = false;
    
                // line search
                if let Some((num, index)) = current_board.find_single_option(
                    |idx| current_board.line_iter(idx),
                    |col, line| current_board.idx_from_line_col(line, col)
                ) {
                    cont = true;
                    let res = current_board.set_num_index(index, num);
                    if !res {
                        break SolverStep::Unsolvable;
                    }
                }
    
                // column search
                if let Some((num, index)) = current_board.find_single_option(
                    |idx| current_board.col_iter(idx),
                    |line, col| current_board.idx_from_line_col(line, col)
                ) {
                    cont = true;
                    let res = current_board.set_num_index(index, num);
                    if !res {
                        break SolverStep::Unsolvable;
                    }
                }
    
                // quadrant search
                if let Some((num, index)) = current_board.find_single_option(
                    |idx| current_board.quad_iter(idx),
                    |intra_quadrant_idx, quadrant_idx| {
                        let quad_line = intra_quadrant_idx / current_board.base;
                        let quad_col = intra_quadrant_idx % current_board.base;
    
                        let quad_start_line = (quadrant_idx / current_board.base) * current_board.base;
                        let quad_start_col = (quadrant_idx % current_board.base) * current_board.base;
    
                        current_board.idx_from_line_col(quad_start_line + quad_line, quad_start_col + quad_col)
                    }
                ) {
                    cont = true;
                    let res = current_board.set_num_index(index, num);
                    if !res {
                        break SolverStep::Unsolvable;
                    }
                }
    
                // find shortest option list
                let res = current_board.iter().enumerate().min_by(|x, y|
                    if let Field::OptionList(xl) = x.1 {
                        if let Field::OptionList(yl) = y.1 {
                            xl.len().cmp(&yl.len())
                        } else {
                            std::cmp::Ordering::Less
                        }
                    } else {
                        std::cmp::Ordering::Greater
                    }
                );
    
                let mut shortest_list_idx = 0;
                if let Some((idx, field)) = res {
                    if let Field::OptionList(list) = field {
                        match list.len() {
                            0 => {
                                break SolverStep::Unsolvable;
                            },
                            1 => {
                                cont = true;
                                let num = list[0];
                                let res = current_board.set_num_index(idx, num);
                                if !res {
                                    break SolverStep::Unsolvable;
                                }
                            },
                            _ => {
                                shortest_list_idx = idx;
                            },
                        }
                    }
                }
    
                // check if all numbers set
                if current_board.iter().all(|x| if let Field::Number(_) = x {true} else {false}) {
                    break SolverStep::Solved;
                }
    
                if !cont {
                    break SolverStep::BranchOnOptionList(shortest_list_idx);
                }
            };
    
            match next_step {
                SolverStep::Solved => {
                    break Some(current_board);
                },
                SolverStep::BranchOnOptionList(idx) => {
                    let number = current_board.get_num_index(idx);
                    if let Some(_) = number {
                        break None;
                    }
    
                    let number = current_board.get_first_option_list(idx);
                    if let None = number {
                        break None;
                    }
                    let number = number.unwrap();
    
                    let mut new_board = current_board.clone();
    
                    if !current_board.set_num_index(idx, number) {
                        break None;
                    }
                    new_board.remove_from_option_list(idx, number);
    
                    stack.push(new_board);
                    stack.push(current_board);
                },
                SolverStep::Unsolvable => (), // abandon this branch
            }
        }
    }

    pub fn generate(base: usize) -> Board { // a board generator based on 5 random numbers from the number range of the board
        let numbers_count_max = 5;
        let board_size = base.pow(4);
        let num_to_delete = board_size * 100 / 70;
    
        let mut b = Board::new();
        b.reset(base);
    
        let mut numbers_count = 0;
        let mut solution = loop {
            let mut current_board = b.clone();
            loop {
                let (idx, num) = loop {
                    let idx = rand::thread_rng().gen_range(0..board_size);
                    let list = current_board.get_option_list(idx);
                    if let Some(list) = list {
                        let list_idx = rand::thread_rng().gen_range(0..list.len());
                        break (idx, list[list_idx]);
                    }
                };
    
                if !current_board.set_num_index(idx, num) {
                    continue;
                }

                numbers_count += 1;
                if numbers_count >= numbers_count_max {
                    break;
                }
            }

            if let Some(solution) = current_board.solve() {
                break solution;
            }
        };
    
        // delete numbers randomly
        for _ in 0..num_to_delete {
            solution.clear_num_index(rand::thread_rng().gen_range(0..board_size));
        }

        solution
    }
}
