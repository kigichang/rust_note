fn main() {
    println!("Hello, world!");

    #[cfg(debug_assertions)]
    println!("Debug mode");
}
