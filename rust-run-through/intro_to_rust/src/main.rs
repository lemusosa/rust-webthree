fn main() {
    // INTRO TO RUST SYNTAX

    let message = "Hello, world";

    println!("{}", message);

    // Other datat types integer, floating numbers, booleans and charcters

    let x: i32 = 42;
    let pi: f64 = 3.14159;
    let is_rust_fun: bool = true;
    let letter_a: char = 'a';

    // functions
    // simple function with two params and return
    fn add(x: i32, y: i32) -> i32 {
        x + y
    }

    // Control flow statements in rust

    let x = 42;

    if x >= 0 {
        println!("x is non-negative");
    } else {
        println!("x is negative");
    }

    // While loop

    let mut i = 1;
    while i <= 5 {
        println!("{}", i);
        i += 1;
    }

    // BASIC DATA TYPES AND VARIABLES

    // Booleans
    let is_rust_great = true;
    let is_rust_hard: bool = false;

    // Integers i.e. whole numbers
    let x: i32 = 42;

    // Min and max values f integers
    let min_i32 = i32::MIN;
    let max_i32 = i32::MAX;

    println!(
        "The minimum and maximum values of an i32 are {} and {} respectively",
        min_i32, max_i32
    );

    // Beyond std arithmetic operators (+, -, /, *, %), Rust interger types also support bitwise
    // operators (&, |, ^, <<,>>) for manipulating individual bits

    // Floating point numbers in rust support both std arithmetic (+, -,/, %) and
    // comparison operators (<, >, <=, >=, ==, !=).

    let pi: f64 = 3.14159;

    // Characters ecnlosed in single quotes  can hold ann Unicode character
    let letter_a: char = 'a';

    // Strings represent a sequence of Unicode characters.
    // &str: is a reference to a string slice. Are immutable by default
    // String: is a growable string type. Can be mutable as they a re growable.
    // a string slice is decalred using the & operator followed by the string literal
    let message: &str = "Hello, world!";

    // String declared using the String::from method
    let mut name = String::from("Alice");
    // String is stored on the heap because its size is non-fixed or growable
    // A string slice, is a reference to a fixed-size sequence of characters that can be stored
    // either on the heap or on the program's binary: the stack.

    // ARRAYS: A fixed-size homogeenous type container. Known size at compile time.
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    let third_number = numbers[2];
    print!("the htird number in the array is {}", third_number);

    // SLICES: represents a variable-size view into a contiguous sequence of
    // elements of the same type.

    let slice = &numbers[1..3];

    let first_element = slice[0];
    println!("The first element of the lsice is {}.", first_element);

    // Tuples: represents a fixed-sized hetereogenoeus container. Are immutable
    // by default.

    let person = ("Alice", 30);

    let name = person.0;
    let age = person.1;
    println!("The person's name is {} and their age is {}.", name, age);

    // Nested tuples

    let persons = (("Alice", "Bob"), 30);
    println!(
        "The person's name is {} {} and their age is {}.",
        person.0 .0, person.0 .1, person.1
    );

    // Unit type: a data type that ha no meaningful infromation

    let result = do_something();
    println!("The result is {}.", result);

    // Variables: are immuatble by default unless you prepend the mut keyword
    // if immutable it means you cannot change the value after declaring it.

    let xy: i32 = 42;

    let xz: i32 = 42; // immutable variable/owner
    xz = 10; // error: cannot assign twice to immutable variable unless you shadow

    // making a variable mutable
    let mut xa: i32 = 42; // mutable variable
    xa = 10; // Correct!

    // type inference example: implicit types
    let y = 3.14; // Rust infers the type as f64

    // Shadowing in rust
    let xb = 42;
    let xb = xb + 1;

    // FUNCTIONS IN RUST

    // Syntax and Structure of Functions
    fn add_numbers(x: i32, y: i32) -> i32 {
        let result = x + y;
        return result;
    }

    // Function Arguments and Return Types

    fn get_greeting() -> String {
        return String::from("Hello, Rust!");
    }

    // Passing Parameters to a Function
    fn add_numbers_two(a: i32, b: i32) -> i32 {
        let result = a + b;
        return result;
    }

    // calling the above function
    let sum = add_numbers_two(3, 5);

    // Default parameters: not support in rust function sbut an Option can be used to
    // simulate default parameter behaviour
    fn greet(name: Option<&str>) {
        match name {
            Some(n) => println!("Hello, {}!", n),
            None => println!("Hello, Rust!"),
        }
    }
    // Returning Values from a function
    fn get_new_greeting(name: &str) -> String {
        let greeting = format!("Hello, {}!", name);
        return greeting;
    }

    // Control Flow: lthree loops: while, for and loop

    // Condtional Statements: if, else if, else used for creating conditional statements
    let x = 5;

    // if conditonal statement: if this do that, else if do that and if not then just do the rest

    if x > 10 {
        println!("x is greater than 10");
    } else if x < 10 {
        println!("x is less than 10");
    } else {
        print!("x is equal to 10");
    }

    // While Loop: execute while a specific condition is true, stop when it is not.

    let mut counter = 0;

    while counter < 10 {
        print!("Counter value is {}", counter);
        counter += 1;
    }

    // For Loop: Ussed to iterate over a collection of items
    let numbers = vec!["1,2,3,4,5,6,7"];

    for number in numbers {
        print!("Number is {}", number);
    }

    // Loop: creates an ifninte loop which can be exited using a break statement.
    // the following just reads: increment ounter by one until 7 is reached.
    let mut counter = 0;

    loop {
        println!("Counter value is {}", counter);
        counter += 1;

        if counter == 7 {
            break;
        }
    }

    // Match:: a control flow construct in rust that allows us to match a value
    // agaisnt a series of patterns and execute corresponding code based on pattern matching
    // it provides exaustive pattenr matching which gv=ives it the ability to handle all 
    // possibilities

    let any_number: i8 = 5;

    match any_number {
        // each statement elow is a match arm
        1 => println!("The number is one!"),
        2 => println!("The number is two!"),
        3 => println!("The number is three!"),
        _ => println!("The number is something else!"),
    }

    let new_num = 10;

    let result = match new_num {
        1 => "The number is one!",
        2 => "The number is two!",
        3 => "The number is three!",
        _ => "The number is something else!",
    };

    println!("{}", result);

    // CODING CONVENTIONS AND STYLE GUDIELINES (RUST STYLE GUIDE)

    // Use snake_case for variable and function names.
    // use spaces around operators and after commas
    // use two spaces for indentations
    // use doc ommentions for documenting your code
    // Kepp your lines short (less than 80 characters) to improve readability

    // WRITING REUSABLE AND MAINTAINABLE CODE

    // Keep your fnctions short and focused on a single task
    // Use descriptive function and variable names to make your code more readable
    // Avoid using global variables whenever possible. Instea, pass data as function
    // or use closures.
    // Use enums and mach statements to handle complex control flow scent=arios
}
