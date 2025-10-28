use std::io;

fn main() {
    let mut sum = 0;

    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(_) => {
                println!("NaN");
                return;
            }
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
                    println!("NaN");
                    return;
                }
            }
            Err(_) => {
                println!("NaN");
                return;
            }
        }
    }

    println!("{}", sum);
}
