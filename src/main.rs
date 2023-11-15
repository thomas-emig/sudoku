use sudoku::Board;

fn print_help() {
    println!("Sudoku generator and solver");
    println!("Usage:");
    println!("        sudoku [Options]");
    println!("");
    println!("    -s :                  Solve puzzles from stdin. Board size is determined from input string.");
    println!("    -b :    -b <Base>     Set base for puzzle generation (2-9).");
    println!("    -n :    -n <Number>   Generate n puzzles.");
    println!("    -p :                  Pretty print puzzles instead of one-line output.");
    println!("    -h :                  Print help.");
}

fn main() {
    let mut solve = false;
    let mut pretty_print = false;
    let mut base = 3;
    let mut num = 1;

    let args : Vec<String> = std::env::args().collect();
    let mut set_base = false;
    let mut set_number = false;
    for arg in args {
        if set_base {
            if let Ok(n) = arg.parse::<usize>() {
                base = n;
            }
        }

        if set_number {
            if let Ok(n) = arg.parse::<usize>() {
                num = n;
            }
        }

        set_base = false;
        set_number = false;

        match arg.as_str() {
            "-s" => solve = true,
            "-b" => set_base = true,
            "-n" => set_number = true,
            "-p" => pretty_print = true,
            "-h" => {
                print_help();
                return;
            }
            _ => (),
        }
    }

    if solve {
        // solve sudokus from stdin
        loop {
            let mut puzzle = String::new();
            let res = std::io::stdin().read_line(&mut puzzle);
            match res {
                Err(_) => break,
                Ok(0)  => break,   // EOF
                _      => (),
            }

            let mut b = Board::new();
            if !b.read(puzzle.trim()) {
                println!("Error: can not read board.");
                continue;
            }

            let solution = b.solve();

            if let Some(board) = solution {
                print!("{}", board.print(pretty_print));
            } else {
                println!("Could not find solution!");
            }
        }
    } else {
        // generate sudokus
        for _ in 0..num {
            let b = Board::generate(base);
            print!("{}", b.print(pretty_print));
        }
    }
}
