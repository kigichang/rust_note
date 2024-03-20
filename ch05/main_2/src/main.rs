fn test() -> Result<i32, std::io::Error> {
    std::fs::File::open("foo.txt")?;
    Ok(0)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    test()?;
    Ok(())
}
