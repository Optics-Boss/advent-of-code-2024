use std::io;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;

fn main() {
    let mut safe_reports = 0; 

    if let Ok(lines) = read_lines("test.txt") {
        for line in lines.flatten() {
            let mut prev_number : i32 = 0;
            let mut increase_decrease = "init";
            let mut difference : i32 = 0;
            let mut failed : bool = false;
            let mut dampen : bool = true;

            for number in line.split_whitespace() {
                let number = number.parse().expect("expected a number");

                if prev_number != 0 {

                    let number_abs : i32 = prev_number - number;

                    if number_abs.abs() > difference {
                        difference = number_abs.abs();
                    }

                    if number == prev_number {
                        if dampen {
                            dampen = false;
                        } else {
                            failed = true;
                        }
                    } else if number > prev_number {
                        if increase_decrease == "decrease" { 
                            if dampen {
                                dampen = false;
                            } else {
                                failed = true;
                            }
                        } else {
                            increase_decrease = "increase";
                        }
                    } else if number < prev_number {
                        if increase_decrease == "increase" {
                            if dampen {
                                dampen = false;
                            } else {
                                failed = true;
                            }
                        } else {
                            increase_decrease = "decrease"
                        }
                    }

                }
                prev_number = number;
            }

            if (difference.abs() > 0 && difference.abs() < 4) && !(failed) {
                safe_reports += 1;
            }
        }
    }

    println!("amount of safe reports: {:?}", safe_reports);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
