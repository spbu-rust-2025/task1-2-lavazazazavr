use std::io;

fn main() {
    let mut sum = 0;
    loop {
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);

        let n: Result<i32, _> = input.trim().parse();
        match n {
            Ok(n) => {
                if n == -1 {
                    break;
                }
                if n < 0 {
                    println!("NaN");
                    return;
                }
                sum += n;
            }
            Err(_) => {
                println!("NaN");
                return;
            }
        }
    }
    println!("{sum}");
}
