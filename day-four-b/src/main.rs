use std::{io, usize};
use std::fs::File;
use std::io::BufRead;
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("input.txt") {
        let mut vector : Vec<Vec<char>> = Vec::new();
        for line in lines.flatten() {
            vector.push(line.chars().collect());
        }
        let number_of_xmas = change_letters_to_dot(vector);
        println!("Result: {}", number_of_xmas)
    }
}

fn change_letters_to_dot (vector: Vec<Vec<char>>) -> i32 {
    let mut number_of_xmas = 0;
    for (i, character_vector) in vector.iter().enumerate() {
        println!("{:?}", character_vector);
        for (i_2, character) in character_vector.iter().enumerate() {
            if character == &'A' {
                let character_m_left_top = peek_in_diagonal_left_top(
                    i as i32, i_2 as i32, 1, vector.clone()
                );
                let character_s_left_top = peek_in_diagonal_left_top(
                    i as i32, i_2 as i32, 1, vector.clone()
                );

                let character_m_right_top = peek_in_diagonal_right_top(
                    i as i32, i_2 as i32, 1, vector.clone()
                );
                let character_s_right_top = peek_in_diagonal_right_top(
                    i as i32, i_2 as i32, 1, vector.clone());

                let character_m_left_down = peek_in_diagonal_left(
                    i as i32, i_2 as i32, 1, vector.clone());
                let character_s_left_down = peek_in_diagonal_left(
                    i as i32, i_2 as i32, 1, vector.clone());

                let character_m_right_down = peek_in_diagonal_right(
                    i as i32, i_2 as i32, 1, vector.clone());
                let character_s_right_down = peek_in_diagonal_right(
                    i as i32, i_2 as i32, 1, vector.clone());

                let mut found_xmas_top_left = false;
                let mut found_xmas_top_right = false;
                let mut found_xmas_left = false;
                let mut found_xmas_right = false;

                if character_m_left_top == 'M' && character_s_right_down == 'S'{
                    found_xmas_top_left = true;
                } 

                if character_m_right_top == 'M' && character_s_left_down == 'S'{
                    found_xmas_top_right = true;
                }

                if character_m_left_down == 'M' && character_s_right_top == 'S' {
                    found_xmas_left = true;
                }

                if character_m_right_down == 'M' && character_s_left_top == 'S' { 
                    found_xmas_right = true;
                }


                if (found_xmas_top_left && found_xmas_top_right) || 
                   (found_xmas_top_left && found_xmas_left) ||
                   (found_xmas_top_left && found_xmas_right) ||
                   (found_xmas_top_right && found_xmas_left) ||
                   (found_xmas_top_right && found_xmas_right) ||
                   (found_xmas_right && found_xmas_left) {
                    print!("X");
                    number_of_xmas += 1;
                } 
            }  else {
                print!(".");
            }
        }
        println!("");
    }

    number_of_xmas
}

fn peek_in_diagonal_left_top (index_vertical: i32, index_horizontal: i32,
                     position: i32, vector: Vec<Vec<char>>) -> char {
    return *vector
        .get((index_vertical - position) as usize)
        .unwrap_or(&Vec::new())
        .get((index_horizontal - position) as usize)
        .unwrap_or(&'N');
}

fn peek_in_diagonal_right_top (index_vertical: i32, index_horizontal: i32,
                     position: i32, vector: Vec<Vec<char>>) -> char {
    return *vector
        .get((index_vertical - position) as usize)
        .unwrap_or(&Vec::new())
        .get((index_horizontal + position) as usize)
        .unwrap_or(&'N');
}

fn peek_in_diagonal_left (index_vertical: i32, index_horizontal: i32,
                     position: i32, vector: Vec<Vec<char>>) -> char {
    return *vector
        .get((index_vertical + position) as usize)
        .unwrap_or(&Vec::new())
        .get((index_horizontal - position) as usize)
        .unwrap_or(&'N');
}

fn peek_in_diagonal_right (index_vertical: i32, index_horizontal: i32,
                     position: i32, vector: Vec<Vec<char>>) -> char {
    return *vector
        .get((index_vertical + position) as usize)
        .unwrap_or(&Vec::new())
        .get((index_horizontal + position) as usize)
        .unwrap_or(&'N');
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


