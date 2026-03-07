use std::io;

fn main() {
    println!("Prime Check Number");
    println!("Enter a number to check if it's prime:");

    let number = match get_input_as_u32() {
        Some(value) => value,
        None => {
            println!("Please enter a valid positive number.");
            return;
        }
    };

    if number<= 1 {
        println!("Numbers less than or equal to 1 are not prime.");
        return;
    }
    if is_prime(number) {
        println!("{} is a prime number.", number);
    } else {
        println!("{} is not a prime number.", number);
    }
}

fn get_input_as_u32() -> Option<u32> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    match input.trim().parse::<u32>() {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}

fn is_prime(n: u32) -> bool {
   if n<=1{
       return false;
   }
    if n==2 {
        return true;
    }
    if n%2==0 {
        return false;
    }

    let limit = (n as f64).sqrt() as u32+1;
    for i in (3..limit).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}