fn main() {
    let mut x = 5;
    println!("The value of x = {}", x);
    x = 6;
    println!("The value of x = {}", x);

    // integer
    let mut a:i64 = 2147483647;
    a = a + 1;
    println!("A = {}", a);

    // floats
    let _y1 = 2.0; // f64
    let _y2: f32 = 3.0; // f32

    // boolean
    let _z1 = true;
    let _z2:bool = false;

    // char
    let _c = 'z';
    let _z = 'Z';

    // tuples
    let tup: (i32, f64, u8) = (500, 3.5, 1);
    let (x, y, z) = tup;
    println!("x = {}, y ={}, z= {}", x, y, z);

    let v0 = tup.0;
    let v1 = tup.1;
    let v2 = tup.2;
    
    println!("v0 = {}, v1 = {}, v2= {}", v0, v1, v2);

    // Arrays (In Rust elements must have same type and fixed length)
    let days = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];

    println!("First month = {}", days[0]);

}
