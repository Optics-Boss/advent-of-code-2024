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
            if character == &'X' {
                let character_m = peek_in_line(i_2 as i32, 1, character_vector.clone());
                let character_a = peek_in_line(i_2 as i32, 2, character_vector.clone());
                let character_s = peek_in_line(i_2 as i32, 3, character_vector.clone());

                let character_m_vert = peek_in_vertical(i as i32, i_2 as i32, 1, vector.clone());
                let character_a_vert = peek_in_vertical(i as i32, i_2 as i32, 2, vector.clone());
                let character_s_vert = peek_in_vertical(i as i32, i_2 as i32, 3, vector.clone());

                let character_m_left = peek_in_diagonal_left(i as i32, i_2 as i32, 1, vector.clone());
                let character_a_left = peek_in_diagonal_left(i as i32, i_2 as i32, 2, vector.clone());
                let character_s_left = peek_in_diagonal_left(i as i32, i_2 as i32, 3, vector.clone());

                let character_m_right = peek_in_diagonal_right(i as i32, i_2 as i32, 1, vector.clone());
                let character_a_right = peek_in_diagonal_right(i as i32, i_2 as i32, 2, vector.clone());
                let character_s_right = peek_in_diagonal_right(i as i32, i_2 as i32, 3, vector.clone());

                let mut found_xmas = false;
                let mut found_xmas_vert = false;
                let mut found_xmas_left = false;
                let mut found_xmas_right = false;

                if character_m == 'M' && character_a == 'A' && character_s == 'S'{
                    found_xmas = true;
                } else if character_m_vert == 'M' && character_a_vert == 'A' && character_s_vert == 'S'{
                    found_xmas_vert = true;
                } else if character_m_left == 'M' && character_a_left == 'A' && character_s_left == 'S' {
                    found_xmas_left = true;
                } else if character_m_right == 'M' && character_a_right == 'A' && character_s_right == 'S' { 
                    found_xmas_right = true;
                }


                if found_xmas {
                    print!("{}{}{}{}", character, character_m, character_a, character_s);
                    number_of_xmas += 1;
                } else if found_xmas_vert {
                    number_of_xmas += 1;
                } else if found_xmas_left {
                    number_of_xmas += 1;
                } else if found_xmas_right {
                    number_of_xmas += 1;
                }

            } else if character == &'S' {
                let character_a = peek_in_line(i_2 as i32, 1, character_vector.clone());
                let character_m = peek_in_line(i_2 as i32, 2, character_vector.clone());
                let character_x = peek_in_line(i_2 as i32, 3, character_vector.clone());

                let character_a_vert = peek_in_vertical(i as i32, i_2 as i32, 1, vector.clone());
                let character_m_vert = peek_in_vertical(i as i32, i_2 as i32, 2, vector.clone());
                let character_x_vert = peek_in_vertical(i as i32, i_2 as i32, 3, vector.clone());

                let character_a_left = peek_in_diagonal_left(i as i32, i_2 as i32, 1, vector.clone());
                let character_m_left = peek_in_diagonal_left(i as i32, i_2 as i32, 2, vector.clone());
                let character_x_left = peek_in_diagonal_left(i as i32, i_2 as i32, 3, vector.clone());

                let character_a_right = peek_in_diagonal_right(i as i32, i_2 as i32, 1, vector.clone());
                let character_m_right = peek_in_diagonal_right(i as i32, i_2 as i32, 2, vector.clone());
                let character_x_right = peek_in_diagonal_right(i as i32, i_2 as i32, 3, vector.clone());

                let mut found_xmas = false;
                let mut found_xmas_vert = false;
                let mut found_xmas_left = false;
                let mut found_xmas_right = false;

                if character_a == 'A' && character_m == 'M' && character_x == 'X'{
                    found_xmas = true;
                } else if character_a_vert == 'A' && character_m_vert == 'M' && character_x_vert == 'X'{
                    found_xmas_vert = true;
                } else if character_a_left == 'A' && character_m_left == 'M' && character_x_left == 'X'{
                    found_xmas_left = true;
                } else if character_a_right == 'A' && character_m_right == 'M' && character_x_right == 'X'{
                    found_xmas_right = true;
                }

                if found_xmas {
                    number_of_xmas += 1;
                } else if found_xmas_vert {
                    number_of_xmas += 1;
                } else if found_xmas_left {
                    number_of_xmas += 1;
                } else if found_xmas_right {
                    number_of_xmas += 1;
                }
            } else {
                print!(".");
            }
        }
        println!("");
    }

    number_of_xmas
}

fn peek_in_line (index: i32, position: i32, vector: Vec<char>) -> char {
    return *vector.get((index + position) as usize).unwrap_or(&'N');
}

fn peek_in_vertical (index_vertical: i32, index_horizontal: i32,
                     position: i32, vector: Vec<Vec<char>>) -> char {
    return *vector
        .get((index_vertical + position) as usize)
        .unwrap_or(&Vec::new())
        .get(index_horizontal as usize)
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


