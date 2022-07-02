use std::io;

fn main() {
    let mut num = String::new();

    let condition = true;
    let number = if condition { 5 } else {6 };
    println!("The number is {}", number);
    loop{

           
        println!("Please input your number...");

        io::stdin()
            .read_line(&mut num)
            .expect("Failed to read line");

        let num: u32 = match num.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Please insert a number!");
                continue;
                    }
                };

        
            let value = num%2;
            if value==0{
                println!("The number is even!");
            }else{
                println!("The number is odd!");
            }

            break
        }
}
