// Use the text_io crate
use text_io::scan;

enum InputType {
    Float(f64),
    Char(char),
}

fn main() {
    println!("Enter number one: ");
    let mut num_one: f64;
    println!("Enter number two: ");
    let mut num_two: f64;
    println!("Enter operator used: ");
    let mut operator: String;
    scan!("{}", operator);
    
    let result = math(num_one, num_two, operator);

    //debug
    println!("The result of: {} {} {} is {}", num_one, operator, num_two, result);
}

fn math(num_one: f64, num_two: f64, operator: String) -> f64 {
    let mut output: f64;

    match operator {
        "+" => {
            output = num_one + num_two;
            output
        },
        "-" => {
            output = num_one + num_two;
            output
        },
        "/" => {
            output = num_one / num_two;
            output
        },
        "*" => {
            output = num_one * num_two;
            output
        }
    }
}
