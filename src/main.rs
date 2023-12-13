// Stephen Lee Belden
// 2023-12-12

fn main() {
    // Unused intentionally; starts with underscore
    let _three = 3;

    // Note the function called before the line on which it's declared
    println!("{}", value());

    let name = "Lee";

    // Mutability must be explicitly declared
    let mut _name_length = name.len();

    // Types are namespaces. Methods can be called like functions.
    _name_length = str::len(name);
}

// Blocks are expressions, they return a value
fn value() -> i32 {
    4
}

