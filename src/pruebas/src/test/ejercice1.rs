use std::io;

fn main() {


    println!("Ingrese un numero");

    let mut number = String::new();
    io::stdin()
    .read_line(&mut number).expect("Error number");

    println!("data {number}");

    let mut  x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("{THREE_HOURS_IN_SECONDS}");

    let difference = 95.5 - 4.3;

    println!("{difference}");
    hello_name(10);


}


fn hello_name (number: i16) -> i16{

    return number;

}