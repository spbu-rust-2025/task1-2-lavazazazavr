use std::io;

fn main() {
    let mut sum = 0;
    let mut has_invalid_data = false;

    loop {
        let mut input = String::new();

        if io::stdin().read_line(&mut input).is_err() {
            has_invalid_data = true;
            break;
        }

        let input = input.trim();

        if input == "-1" {
            break;
        }

        match input.parse::<i32>() {
            Ok(num) => {
                if num > 0 {
                    sum += num;
                } else {
                    has_invalid_data = true;
                }
            }
            Err(_) => {
                has_invalid_data = true;
            }
        }
    }

    if has_invalid_data {
        println!("NaN");
    } else {
        println!("{}", sum);
    }
}
