use std::io;

fn main() 
{
    // Request and store mathematical operation
    println!("Enter operation: add, sub, mult, div");
    let op = user_op_input();

    // Request and store user input
    println!("Input an integer a: ");
    let a = user_var_input();       
    println!("Input an integer b: ");
    let b = user_var_input();

    // Initialize result as 32-bit signed integer = 0
    let mut result: i32 = 0;
    
    // Do math
    // Check for division by 0
    if op == "div" && b == 0
    {
        println!("Error: This behavior is undefined");
        return; // Exit the program
    }

    // Nominal case
    if op == "add"
    {
        result = add(a,b);
    }
    else if op == "sub"
    {
        result = sub(a,b);
    }
    else if op == "mult"
    {
        result = mult(a,b);
    }
    else if op == "div"
    {
        result = div(a,b);
    }
    // Print answer
    println!("Result: {}", result);

}

// Used for requesting and storing the operation type in memery
fn user_op_input() -> String
{
    loop {
        // Generate a buffer to store the string data
        let mut input1 = String::new();

        // Store using input
        io::stdin().read_line(&mut input1).unwrap();

        // Remove white space and new line
        let input1 = input1.trim();

        // Make sure the operation type makes sense
        match input1 
        {
            // Addition case
            "add" => 
            {
                println!("You selected addition");
                return input1.to_string();
            },
            // Subtraction case
            "sub" =>
            {
                println!("You selected subtraction");
                return input1.to_string();
            },
            // Multiplication case
            "mult" =>
            {
                println!("You selected multiplication");
                return input1.to_string();
            },
            // Division case
            "div" =>
            {
                println!("You selected division");
                return input1.to_string();
            },
            // Errant input case
            _ =>
            {
                println!("You did not select a viable option. Try again.");
            }
        } 
    }
}

// Takes a users integer inputs and stores them in memory
fn user_var_input() -> i32
{
    let mut input2 = String::new();
    // Declare a new modifiable buffer (String) to 
    // store the string of characters coming from the 
    // user's keyboard input.

    io::stdin().read_line(&mut input2).unwrap();
    // io::stdin() - Access standard input from input/output module
    // read_line(&mut input) - Read line into buffer until Enter is pressed
    // unwrap() - Crash if read fails

    let x:i32 = input2.trim().parse().unwrap();
    // x:i32 - Declare x as a 32-bit signed integer
    // trim() - Remove whitespace
    // parse() - In this context, convert to i32 type
    // unwrap() - Crash if error occurs

    x // Return the signed 32-bit integer
}

// Addition function
fn add(a:i32, b:i32) -> i32
{
    a+b // Return addition of user input
}

// Subtraction function
fn sub(a:i32, b:i32) -> i32
{
    a-b // Return subtraction of user input
}

// Multiplication function
fn mult(a:i32, b:i32) -> i32
{
    a*b // Return multiplication of user input
}

// Division function
fn div(a:i32, b:i32) -> i32
{
    a/b // Return division of user input
}