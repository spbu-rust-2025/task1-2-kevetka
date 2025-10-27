use std::io;

fn main() {
    let mut summa = 0;
    let mut false_flag = false;

    loop {
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => match input.trim().parse::<i32>() {
                Ok(num) => {
                    if num == -1 {
                        break;
                    }
                    if num > 0 {
                        summa += num;
                    }
                    else {
                        false_flag = true;
                    }
                }
                Err(_) => {
                    false_flag = true;
                }
            },
            Err(_) => {
                false_flag = true;
            }
        }
    }
    if false_flag {
        println!("NaN");
        return;
    }
    println!("{}", summa);
}
