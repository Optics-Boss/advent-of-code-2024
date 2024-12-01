use std::io;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;

fn main() {
    let mut number_list_one: Vec<i32> = Vec::new();
    let mut number_list_two: Vec<i32> = Vec::new();
    let mut total_distance : i32 = 0;

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines.flatten() {
            let split_numbers: Vec<&str> = line.split_whitespace().collect();
            number_list_one.push(split_numbers[0].parse().expect("Must be a number"));
            number_list_two.push(split_numbers[1].parse().expect("Must be a number"));
        }
    }

    number_list_one.sort();
    number_list_two.sort();

    for (list_one, list_two) in number_list_one.into_iter().zip(number_list_two) {
        total_distance += (list_one - list_two).abs();
    }

    println!("{}", total_distance);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
