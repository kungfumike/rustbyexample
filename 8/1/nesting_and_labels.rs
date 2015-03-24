fn main() {
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // This would only break us out of the inner loop
            // break;
            //
            // This breaks us out out of the outer loop
            break 'outer;
        }

        println!("This point will never be reached");

    }

    println!("Exited the outer loop");
}
