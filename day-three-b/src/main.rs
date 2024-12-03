use std::io;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use regex::Regex;

fn main() {
    if let Ok(lines) = read_lines("input.txt") {
        let mut result = 0;
        let mut doing = true;
        for line in lines.flatten() {
            let re = Regex::new("(mul[(]\\d*[,]\\d*[)])|(do[(][)])|(don[']t[(][)])").unwrap();
            let caps : Vec<&str> = re.find_iter(&line)
                .map(|hay| hay.as_str())
                .collect(); 

            for test in caps {
                let mut testing = test.to_string();
                if testing == "don't()" {
                    doing = false;
                } else if testing == "do()" {
                    doing = true;
                } else {
                    if doing {
                        testing.pop();

                        for _i in 0..4 {
                            testing.remove(0);
                        }

                        let numbers : Vec<i32> = testing.split(",")
                            .filter_map(|f| f.parse::<i32>().ok())
                            .collect();

                        let final_number = numbers.get(0).unwrap_or(&0) * numbers.get(1).unwrap_or(&0);
                        result += final_number
                    }
                } 
            }
        }

        println!("Result = {:?}", result);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

