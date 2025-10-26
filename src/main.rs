use std::io;

fn main() {
    let mut summa = 0;

    loop {
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<i32>() {
                    Ok(num) => {
                        if num == -1 {
                            break;
                        }
                        if num > 0 {
                            summa += num;
                        }
                        else {
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
            Err(_) => {
                println!("Nan");
                return;
            }
        }
    }
    println!("{}", summa);
}
