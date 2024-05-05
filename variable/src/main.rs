fn main() {
    println!("Hello, world!");
}

#[test]
fn test_variable() {
    let name = "Giffari";
    println!("Hello {}!", name);
}

#[test]
fn test_variable_mutable() {
    let mut name = "Giffari";
    println!("Hello {}!", name);

    name = "Sipayung";
    println!("Hello {}!", name)
}

#[test]
fn test_variable_static_type() {
    let mut name = "Giffari";
    println!("Hello {}!", name);

    // name = 10; // failed cause name is string
    // println!("Hello {}", name);
}

#[test]
fn test_variable_shadowing() {
    let name = "Giffari";
    println!("Hello {}", name);

    let name = 10;
    println!("Age is {}", name);
}
