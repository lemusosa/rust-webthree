fn main() {
    // Variables and simple printing
    let a: i32 = 10;
    let b: i32 = 15;
    println!("Hello, world!, {} {}", a,b);

    // Dat types

    // unsigned integer
    // u8, u16, u32, u64, u128
    let unsigned: u32 = 10;

    // signed integer
    // i8, i186, i32, i64, i128
    let signed: i32 = -10;

    // float is used for decimals: f64 offers more precision
    let float: f64 = 0.32;

    println!("Different numbers => {} {} {}", unsigned, signed, float);


    // char is used for a single character
    let character: char = 'a';
    println!("Caracter => {}", character);

    // boolean is used for true r false
    let boolean: bool = true;
    println!("Boolean => {}", boolean);

    // tuple is a non-homogeneous container that groups/stores
    // different data types.
    // the 'Debug' trat allows for printing data structures contents
    // like tuples, arrays, hashmaps etc
    let tuple: (i32, i32, f64, i32, bool) = (1, -2, 3.0, 4, true);
    println!("Tuple => {:?}", tuple);
}
