fn main() {
    // Integer Addition
    println!("1 + 2 = {}", 1u + 2);

    // Integer subtraction
    println!("1 - 2 = {}", 1i - 2 );


    // Short-circuting boolean logic
    println!("true and false is {}", true && false);
    println!("true or  false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operators
    println!("0011 AND 0101 is {:04t}", 0b0011u & 0b0101);
    println!("0011 OR  0101 is {:04t}", 0b0011u | 0b0101);
    println!("0011 XOR 0101 is {:04t}", 0b0011u ^ 0b0101);
    println!("1 << 5 is {}", 1u << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u >> 2);

    // Use underscores to improve readability
    println!("One million is written as {}", 1_000_000u);
}
