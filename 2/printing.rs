fn main() {
    // `print!` is like `println` but it doesn't add a newline at the end of the print
    print!("Janurary has ");

    // `{}` are placeholders for arguments that will be stringified
    println!("{} days", 31i);
    // The `i` indicates to the compiler that this is a literal type:
    // signed pointer size integer, see the next chapter for more details

    // The positional arguments can be reused along the template
    println!("{0}, this is {1}. {1} this is {0}", "Alice", "Bob");

    // Name arguments can also be used
    println!("{subject} {verb} {predicate}",
             predicate="over the lazy dog",
             subject="the quick brown fox",
             verb="jumps");

    // Special formatting can be specified in the placeholder after a `:`
    println!("{} of {:t} people know binary, the other half don't", 1i, 2i);

    // Error! You are missing an argument
    // println!("My name is {0}, {1} {0}", "Bond");
    println!("My name is {0}, {1} {0}", "Bond", "James");
}
