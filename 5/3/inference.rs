fn main() {
    // Using local inference; the compiler know that `elem` has type u8
    let elem = 5u8;

    // Create and empty vector ( a growable array) (not my words)
    let mut vec = Vec::new();
    // At this point the compiler doesn't know the exact type of `vec`, it
    // just knows that it's a vector of something (`Vec<_>`)

    // Insert `elem` in the vector
    vec.push(elem);
    // Aha! Now the compiler knows that `vec` is a vector
    // TODO ^ Try commenting out the `vec.push(elem)` line

    println!("{}", vec);
}
