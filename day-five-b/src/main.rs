use std::collections::HashMap;
use std::io;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;

fn main() {
    let mut hash_map: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut result = 0;

    if let Ok(lines) = read_lines("test.txt") {
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
                let mut vector : Vec<i32> = line
                    .split(",")
                    .map(|x| -> i32 { x.parse().unwrap() })
                    .collect();

                let mut check_vector : Vec<bool> = Vec::new();

                println!("{:?}", line);

                for (i, number) in vector.iter().enumerate() {
                    for (i_2, number_two) in vector.iter().enumerate() {
                        let mut flagged = false;
                        if i != i_2 {
                            if i_2 > i {
                                let check_if_number_after = &hash_map
                                    .get(&number)
                                    .unwrap_or(&Vec::new())
                                    .contains(number_two);

                                if check_if_number_after == &false {
                                    flagged = true
                                }
                                check_vector.push(*check_if_number_after);
                            } else if i_2 < i {
                                let check_if_number_before = &hash_map
                                    .get(&number_two)
                                    .unwrap_or(&Vec::new())
                                    .contains(number);

                                if check_if_number_before == &false {
                                    flagged = true
                                }

                                check_vector.push(*check_if_number_before);
                            }
                        }

                        if flagged {
                            println!("{:?} - {:?} - {:?}", number, check_in_hashmap(hash_map.clone(), *number_two, *number), vector.iter().position(|&r| &r == number).unwrap());
                            println!("{:?} - {:?} - {:?}", number, check_in_hashmap(hash_map.clone(), *number, *number_two), vector.iter().position(|&r| &r == number_two).unwrap());
                            let vector_index_one = vector.iter().position(|&r| &r == number).unwrap();
                            let vector_index_two = vector.iter().position(|&r| &r == number_two).unwrap();
                            vector.swap(vector_index_one, vector_index_two);
                        }

                    }
                }

                println!("{:?}", check_vector);

                if !check_vector.contains(&false) {
                    result += vector.get(vector.len()/2).unwrap_or(&0);
                }
            }
        }

        println!("{:?}", result);
    }
}

fn check_in_hashmap(hash_map: HashMap<i32, Vec<i32>>, get_value: i32, value: i32) -> bool {
    hash_map
        .get(&get_value)
        .unwrap_or(&Vec::new())
        .contains(&value)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

