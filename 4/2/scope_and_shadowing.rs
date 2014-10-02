fn main() {
    // This variable live in the main function
    let long_lived_variable = 1i;

    // This is a block, and has smaller scope than the main function
    {
        let short_lived_variable = 2i;

        println!("Inner short: {}", short_lived_variable);

        // This variable *shadows* the outer one
        let long_lived_variable = 5_f32;

        println!("Inner long: {}", long_lived_variable);
    }
    // End of block
    //
    // Error! `short_lived_variable` doesn't exist in this scope
    // println!("outer short: {}", short_lived_variable);
    // FIXME ^ Comment out this line
    //
    println!("Outer long: {}", long_lived_variable);
}


// Variable shadowing basically means a variable inside a contained scope
// can have the same name as a variable of a parent scope. Variable in the
// parent scope do not "just appear" inside children.
// Never knew there was a term for this, I just thought this is how things
// always were.
