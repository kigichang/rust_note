use std::pin;

fn main() {
    println!("Hello, world!");

    #[cfg(debug_assertions)]
    println!("Debug mode");

    {
        let mut s = "hello";

        println!("{:p}", s);

        s = "world";
        println!("{:p}", s);

        let mut p = pin::Pin::new(&mut s);
        println!("{:?}", p);

        println!("{:?}", ss);
        println!("{:?}", p);
    }
}
