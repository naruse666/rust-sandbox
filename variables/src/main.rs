fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x); // xの値は{}です
    x = 6;
    println!("The value of x is: {}", x);
    let guess: u32 = "42".parse().expect("not a number");
    let y = add(3);
    println!("Add result y is: {}", y);
}

fn add(x: i32) -> i32 {
    x
}
