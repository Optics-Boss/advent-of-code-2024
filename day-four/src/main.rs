use std::io;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("test.txt") {
        for line in lines.flatten() {
            change_letters_to_dot(line);
        }
    }
}

fn change_letters_to_dot (line: String) {
    // println!("{:?}", line);

    let vector : Vec<char> = line.chars().collect();

    println!("{:?}", vector);

    for (i, character) in vector.iter().enumerate() {


        if character == &'X' {
            let character_m = peek(i as i32, 1, vector.clone());
            let character_a = peek(i as i32, 2, vector.clone());
            let character_s = peek(i as i32, 3, vector.clone());
            let mut found_xmas = false;

            if character_m == 'M' && character_a == 'A' && character_s == 'S'{
                found_xmas = true;
            }

            if found_xmas {
                print!("{}{}{}{}", character, character_m, character_a, character_s);
            } else {
                print!(".")
            }
        } else if character == &'S' {
            let character_a = peek(i as i32, 1, vector.clone());
            let character_m = peek(i as i32, 2, vector.clone());
            let character_x = peek(i as i32, 3, vector.clone());
            let mut found_xmas = false;

            if character_m == 'M' && character_a == 'A' && character_x == 'X'{
                found_xmas = true;
            }

            if found_xmas {
                print!("{}{}{}{}", character, character_m, character_a, character_x);
            } else {
                print!(".")
            }
        } else {
            print!(".");
        }
    }

}

fn peek (index: i32, position: i32, vector: Vec<char>) -> char {
    return *vector.get((index + position) as usize).unwrap_or(&'N');
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


