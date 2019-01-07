use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::error::Error;

static MINE: char = 'X';
static EMPTY: char = 'O';
// This is terminate symbol using to extend matrix from N x N to N+1 x N+1
static TERMINATE: char = '@';

fn print_matrix(matrix: &Vec<Vec<char>>) {
    for line in matrix {
        for value in line {
            print!{"{} ", value}
        }
        println!("");
    }
    println!("");
}

fn construct_matrix_with_mines_and_numbers(extended_matrix: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut output_matrix: Vec<Vec<char>> = vec![vec!['O'; extended_matrix.len() - 2]; extended_matrix.len() - 2];

    for i in 1..extended_matrix.len() - 1 {
        for j in 1..extended_matrix[i].len() - 1 {
            let value = extended_matrix[i][j];

            if value == MINE {
                output_matrix[i - 1][j - 1] = value;
            } else if value == EMPTY {
                output_matrix[i - 1][j - 1] = number_of_neighbors(&extended_matrix, i, j);
            } else {
                assert!(false, "Wrong symbols in input");
            }
        }
    }

    return output_matrix;
}

fn number_of_neighbors(matrix: &Vec<Vec<char>>, x: usize, y: usize) -> char {
    let mut n: u8 = 0;

    for i in x-1..x+2 {
        for j in y-1..y+2 {
            // We do not count element itself (matrix[x][y])
            // This does not matter now because we do not pass mines into this function, but can be important
            // if someone will use this function to calculate number of mines around a mine
            if matrix[i][j] == MINE && !(i == x && j == y) {
                n = n + 1;
            }
        }
    }

    // 48 is char code for 0
    return (n + 48) as char;
}

// Create matrix and add terminate symbols around it, so
//
//              T T T T
// X O  becomes T X O T , where T is terminate symbol
// O X          T O X T
//              T T T T
fn extended_matrix_from_input(content: String) -> Vec<Vec<char>> {
    // Files can end with empty string
    // Remove it and all empty strings that can appear by accident to calculate correct dimension of matrix
    let lines = content.lines().filter(|x| !x.is_empty());

    let lines: Vec<&str> = lines.collect();
    let dimension: usize = lines.len();


    let mut matrix: Vec<Vec<char>> = Vec::with_capacity(dimension + 1);
    let mut line_vector: Vec<char> = Vec::with_capacity(dimension + 1);

    matrix.push(vec![TERMINATE; dimension + 2]);
    for line in lines.iter() {
        line_vector.push(TERMINATE);

        let mut n_symbols: u32 = 0; // This counter uses in assert to ensure that matrix is square indeed

        for symbol in line.split_whitespace() {
            line_vector.push(symbol.parse::<char>().unwrap());
            n_symbols = n_symbols + 1;
        }
        assert_eq!(n_symbols, dimension as u32, "Wrong input");

        line_vector.push(TERMINATE);

        matrix.push(line_vector.to_owned());
        line_vector.clear();
    }

    matrix.push(vec![TERMINATE; dimension + 2]);

    return matrix;
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {:?} input_file_path", args[0]);
        return;
    }

    let path = Path::new(&args[1]);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Err(why) => panic!("couldn't read {}: {}", display, why.description()),
        Ok(_) => {},
    };

    let extended_matrix: Vec<Vec<char>> = extended_matrix_from_input(content);
    print_matrix(&extended_matrix);

    let final_matrix = construct_matrix_with_mines_and_numbers(extended_matrix);
    print_matrix(&final_matrix);
}