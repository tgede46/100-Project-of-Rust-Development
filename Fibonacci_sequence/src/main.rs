use std::io;

fn main() {
    println!("Fibonacci Sequence");
    println!("Enter number of terms:");

    let num_terms = match get_input_as_u32() {
        Some(value) => value,
        None => {
            println!("Please enter a positive number.");
            return;
        }
    };

    if num_terms == 0 {
        println!("Number of terms must be greater than zero.");
        return;
    }

    let sequence = generate_fibonacci(num_terms);
    let formatted = sequence.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(", ");
    println!("Fibonacci Sequence ({} terms): {}", num_terms, formatted);
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

fn generate_fibonacci(n: u32) -> Vec<u32> {
    let mut sequence = Vec::new();

    if n >= 1 {
        sequence.push(0);
    }
    if n >= 2 {
        sequence.push(1);
    }
    for i in 2..n {
        let next = sequence[i as usize - 1] + sequence[i as usize - 2];
        sequence.push(next);
    }
    sequence
}