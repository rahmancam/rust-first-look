fn main() {
    println!("Hello, world!");
    another_function();
    another_function2(5);
    let a =  sum(10 , 20);
    println!("Sum = {}", a);
    let res = sum2(a, 5, 10);
    println!("Sum = {}", res);
}


fn another_function() {
    println!("Another function.");
}

fn another_function2(x: i32) {
    println!("The value of x = {}", x);
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}

fn sum2(x: i32, y: i32, z: i32) -> i32 {
    let mut result = x + y;
    result = result + z;
    return result;
}