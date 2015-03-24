fn main() {
    let n = 50i;

    if n < 0 {
        print!("{} is negitive", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else { 
        print!("{} is zero", n);
    }

    let big_n = 
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");

            // This expression returns an `int`
            10 * n
        } else {
            println!(", and is a big number, cut in half");

            // This expression must return an `int` as well
            n / 2
            // TODO expirament with supressing the output with a semi-colon
            // You end up with an incompatible types error. The semicolon causes
            // the expression to "return" () since the last line is nothing
        };

    println!("{} -> {}", n, big_n);
}
