fn main() {
    let mut count = 0u;

    println!("Let's count until infinity!");

    // Infinite loop

    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // Skip the rest of this iteration
            continue;
        }

        println!("{}", count);

        if count == 10 {
            println!("OK, that's enough");

            // Exit this loop
            break;
        }
    }
}

