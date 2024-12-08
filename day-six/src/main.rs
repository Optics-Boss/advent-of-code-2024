use std::{io, usize};
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use std::collections::HashMap;

fn main() {
    let mut result = 0;
    if let Ok(lines) = read_lines("input.txt") {
        let mut vector : Vec<Vec<char>> = Vec::new();
        let mut coordinates : HashMap<i32, Vec<i32>> = HashMap::new();

        for line in lines.flatten() {
            vector.push(line.chars().collect());
        }

        for (i, character_vector) in vector.iter().enumerate() {
            for (i_2, character) in character_vector.iter().enumerate() {
                if character == &'^' {
                    let mut guard = character;
                    let mut index_vertical = i;
                    let mut index_horizontal = i_2;

                    loop {
                        println!("vertical {:?} - horizontal {:?} - {:?}", index_vertical, index_horizontal, guard);
                        if guard == &'^' {
                            if peek_above(index_vertical as i32, index_horizontal as i32, 1, vector.clone()) == '#' {
                                guard = &'>';
                            } else if peek_above(index_vertical as i32, index_horizontal as i32, 1, vector.clone()) == 'X' {
                                break;
                            } else {
                                index_vertical -= 1;
                            }
                        } else if guard == &'>' {
                            if peek_right(index_vertical as i32, index_horizontal as i32, 1, vector.clone()) == '#' {
                                guard = &'v';
                            } else if peek_right(index_vertical as i32, index_horizontal as i32, 1, vector.clone()) == 'X' {
                                break;
                            } else {
                                index_horizontal += 1;
                            }
                        } else if guard == &'v' {
                            if peek_down(index_vertical as i32, index_horizontal as i32, 1, vector.clone()) == '#' {
                                guard = &'<';
                            } else if peek_down(index_vertical as i32, index_horizontal as i32, 1, vector.clone()) == 'X' {
                                break;
                            } else {
                                index_vertical += 1;
                            }
                        } else if guard == &'<' {
                            if peek_left(index_vertical as i32, index_horizontal as i32, 1, vector.clone()) == '#' {
                                guard = &'^';
                            } else if peek_left(index_vertical as i32, index_horizontal as i32, 1, vector.clone()) == 'X' {
                                break;
                            } else {
                                index_horizontal -= 1;
                            }
                        }

                        if !coordinates.entry(index_vertical as i32).or_default().contains(&(index_horizontal as i32)) {
                            coordinates.entry(index_vertical as i32)
                                .or_default()
                                .push(index_horizontal as i32);

                            result += 1;
                        }


                    }
                }
            }
        }
    }

    println!("result = {:?}", result);
}

fn peek_above (index_vertical: i32, index_horizontal: i32,
                     position: i32, vector: Vec<Vec<char>>) -> char {
    let test = *vector
        .get((index_vertical - position) as usize)
        .unwrap_or(&Vec::new())
        .get(index_horizontal as usize)
        .unwrap_or(&'X');
    return test;
}

fn peek_down (index_vertical: i32, index_horizontal: i32,
                     position: i32, vector: Vec<Vec<char>>) -> char {

    let test = *vector
        .get((index_vertical + position) as usize)
        .unwrap_or(&Vec::new())
        .get(index_horizontal as usize)
        .unwrap_or(&'X');
    return test;
}

fn peek_left (index_vertical: i32, index_horizontal: i32,
                     position: i32, vector: Vec<Vec<char>>) -> char {
    return *vector
        .get(index_vertical as usize)
        .unwrap_or(&Vec::new())
        .get((index_horizontal - position) as usize)
        .unwrap_or(&'X');
}

fn peek_right (index_vertical: i32, index_horizontal: i32,
                     position: i32, vector: Vec<Vec<char>>) -> char {
    return *vector
        .get(index_vertical as usize)
        .unwrap_or(&Vec::new())
        .get((index_horizontal + position) as usize)
        .unwrap_or(&'X');
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

