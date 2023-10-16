// Data types

fn main(){
    let _integer: u32 = 42;

    // 8, 16, 32, 64, 128, isize
    // -(2^(n-1)) to 2^(n-1) for signed

    // 0 to (2^n)-1 for unsigned
    // 2^16 -1 = 65.535 for u16 (2 bytes)

    let _float = 2.0; // f64

    let _float: f32 = 3.0; // f32

    // Nothing special, basic arithmetic operations..
    let _sum = 5 + 10;
    let _difference = 10 - 5; 
    let _product = 4 * 10; 
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3; 
    let _remainder = 43 % 5;

    // bool 
    let _t: bool  = true; // or false

    let _c = 'z'; // char is with single quotes.
    let _c: char = 'x';

    // compound types are lists and such.. a var that can hold multiple elements.

    let tup: (i32, f64, u8) = (500, 6.6, 5);

    // tuples can hold different types.
    let (x,y,z) = tup;

    println!("{x},{y},{z}");

    // arrays can only hold a single datatype.

    let arr: [u32; 3] = [32,32,32];
    println!("{:?}", arr);

    let arr_str = ["Hello", "world"];
    println!("{}", arr_str[0]);
}