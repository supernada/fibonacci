use std::io;

fn main() {
    println!("welcome to the fibonacci finder calculator 30000");
    println!("type the num u want top find"); //"type sum thing

    let mut n_input = String::new();

    io::stdin()
        .read_line(&mut n_input)
        .expect("Failed to read");

    let n: u32 = match n_input.trim().parse() {
        Ok(num) => num,
            Err(_) => {
                println!("Hey, we can't Fibonacci words yet!");
                return;
            }
    };    


        let num2 = 2;
    for num1 in 3..n+1{
        let num3 = num2 + num1;
        println!("{num3}");      
    }



}


//fn n_conv() {

//}
