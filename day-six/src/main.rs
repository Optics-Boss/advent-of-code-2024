use std::{io, usize};
use std::fs::File;
use std::io::BufRead;
use std::path::Path;

fn main() {
    let mut result = 0;
    if let Ok(lines) = read_lines("test.txt") {
        let mut vector : Vec<Vec<char>> = Vec::new();
        let mut visited_gred_vec : Vec<VisitedGrid> = Vec::new();

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

                        let test: Vec<> = visited_gred_vec.into_iter().filter(|x| x.vertical_index == index_vertical && x.horizontal_index == index_horizontal);

                        // if visited_gred_vec.into_iter().filter(|x| x.vertical_index == index_vertical && x.horizontal_index == index_horizontal).collect(){
                        //     visited_gred_vec.push(VisitedGrid {vertical_index: index_vertical, horizontal_index: index_horizontal});
                        //     result += 1;
                        // }

                    }
                }
            }
        }
    }

    println!("result = {:?}", result);
}

struct VisitedGrid {
    vertical_index: i32,
    horizontal_index: i32,
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

