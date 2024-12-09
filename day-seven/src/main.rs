use std::io;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;


fn main() {
    let mut result = 0;

    if let Ok(lines) = read_lines("test.txt") {
        for line in lines.flatten() {
            let total_number : Vec<&str> = line.split(':').collect();
            let secondary_numbers : Vec<i128> = total_number[1]
                .split_whitespace()
                .map(|v| v.parse::<i128>().unwrap_or(0))
                .collect();
            let total_number : i128 = total_number[0].parse().unwrap_or(0); 
            let mut times_number : i128 = 1;
            
            for number in secondary_numbers {
                times_number = calculate(number, times_number);
            }

            println!("Total = {:?}", total_number);
            println!("Times = {:?}", times_number);

            result += total_number;
        }
    }

    println!("{:?}", result);
}

fn calculate(number: i128, times_number: i128) -> i128 {
    if number == 0 || number == 1 {
        return number;
    } else {
       return number + calculate(number + times_number, times_number);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
