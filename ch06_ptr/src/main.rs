fn main() {
    // println!("Hello, world!");
    use_box();
}

// box<T>的使用
fn use_box() {
    let b = Box::new(5);
    println!("b = {}", b);
}
