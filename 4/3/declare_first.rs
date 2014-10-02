fn main() {
    //Declare a variable
    let a_variable;

    {
        let x = 2i;

        // Initialize the variable
        a_variable = x * x;
    }

    println!("A variable: {}", a_variable);

    let another_variable;

    // Error! use of uninitialized variable
    // println!("Another variable: {}"; another_variable);
    // Fixme ^ Comment out this line

    another_variable = 1i;

    println!("Another variable: {}", another_variable);
}

// Interesting to see here how the variable is accesible when
// not redeclared/shadowed
