fn main() {
    println!("ownership");

    {
        fn print_padovan() {
            let mut padovan = vec![1, 1, 1]; // allocate memory here.
            for i in 3..10 {
                let next = padovan[i - 3] + padovan[i - 2];
                padovan.push(next);
            }

            println!("{:?}", padovan);
            // dropped here.
        }

        print_padovan();
    }
}
