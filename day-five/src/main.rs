use std::collections::HashMap;
use std::io;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;

fn main() {
    let mut hash_map: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut result = 0;

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines.flatten() {
            if line.contains("|") {
                let numbers : Vec<i32> = line
                    .split("|")
                    .map(|x| -> i32 { x.parse().unwrap() })
                    .collect();

                let number_one = &numbers.get(0).unwrap_or(&0);
                let number_two = &numbers.get(1).unwrap_or(&0);

                hash_map.entry(**number_one)
                    .or_default()
                    .push(**number_two);
            }

            if line.contains(",") {
                let vector : Vec<i32> = line
                    .split(",")
                    .map(|x| -> i32 { x.parse().unwrap() })
                    .collect();

                let mut check_vector : Vec<bool> = Vec::new();

                println!("{:?}", line);

                for (i, number) in vector.iter().enumerate() {
                    for (i_2, number_two) in vector.iter().enumerate() {
                        if i != i_2 {
                            if i_2 > i {
                                let check_if_number_after = &hash_map
                                    .get(&number)
                                    .unwrap_or(&Vec::new())
                                    .contains(number_two);

                                check_vector.push(*check_if_number_after);
                            } else if i_2 < i {
                                let check_if_number_before = &hash_map
                                    .get(&number_two)
                                    .unwrap_or(&Vec::new())
                                    .contains(number);

                                check_vector.push(*check_if_number_before);
                            }
                        }
                    }
                }


                if !check_vector.contains(&false) {
                    result += vector.get(vector.len()/2).unwrap_or(&0);
                }
            }

        }

        println!("{:?}", result);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

