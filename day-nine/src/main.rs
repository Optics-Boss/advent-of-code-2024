use std::io;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;


fn main() {
    let mut result = 0;
    let mut free_space = false;
    let mut file_size = 0;
    let mut vector_file_system : Vec<char> = Vec::new();
    let mut vector_file_system_new : Vec<char> = Vec::new();

    if let Ok(lines) = read_lines("test.txt") {
        for line in lines.flatten() {
            for character in line.chars() {
                let number: i64 = character.to_string().parse().unwrap_or(0);

                if free_space {
                    for _ in 0..number {
                        vector_file_system.push('.');
                    }

                    free_space = false;
                } else {
                    free_space = true;

                    for _ in 0..number {
                       let file_size_char : char = file_size.to_string().parse().unwrap_or('0');
                       vector_file_system.push(file_size_char);
                    }

                    file_size += 1;
                }
            }
        }
    }

    for file in vector_file_system.into_iter().rev() {
        if file != '.' {
            vector_file_system_new.push(file);
        } else {
            vector_file_system_new.push(file);
        }
    }

    println!("Result = {:?}", result);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
