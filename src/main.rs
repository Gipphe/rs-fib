use std::io::stdin;

fn fibonacci(num: u64) -> u64 {
    if num <= 1 {
        return 1
    }
    fibonacci(num - 1) + fibonacci(num - 2)
}
fn read_line_positive_number() -> u64 {
    let num: u64;
    
    let mut input = String::new();
    stdin().read_line(&mut input)
        .expect("Error reading input");
    
    num = match input.trim().parse() {
        Ok(num) => {
            num
        },
        Err(_) => {
            println!("Invalid number!");
            return read_line_positive_number();
        },
    };
    return num
}

fn main() {
    println!("Calculate the fibonacci number for:");

    let num = read_line_positive_number();
    let res = fibonacci(num);
    println!("Result: {}", res);
}
