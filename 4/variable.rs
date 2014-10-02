fn main() {
    let an_integer = 1u;
    let a_boolean = true;
    let unit = ();

    // copy `an_integer` into `copied_integer`
    let copied_integer = an_integer;

    println!("An Integer: {}", copied_integer);
    println!("A  Boolean: {}", a_boolean);
    println!("Meet the unit value: {}", unit);

    // The compiler warns you about unused variables; these warnings can be
    // silenced by prefixing the variable name with an underscore
    let _unused_variable = 3u;
    let _noisy_unused_variable = 2u;
    // let noisy_unused_variable = 2u;
    // FIXME ^ Prefix with an underscore to supress the warning
}

