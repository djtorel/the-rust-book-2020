fn main() {
    another_function(5);
    another_function_2(2, 65);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn another_function_2(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
