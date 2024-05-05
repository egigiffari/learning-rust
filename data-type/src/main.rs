fn main() {
    println!("Hello, world!");
}

#[test]
fn test_number_integer() {
    let integer8: i8 = 10;
    let integer16: i16 = 100;
    let integer32: i32 = 1000;
    let integer64: i64 = 10000;
    let integer128: i128 = 100000;

    println!("integer8 {}", integer8);
    println!("integer16 {}", integer16);
    println!("integer32 {}", integer32);
    println!("integer64 {}", integer64);
    println!("integer128 {}", integer128);
}

#[test]
fn test_number_uinteger() {
    let uinteger8: u8 = 10;
    let uinteger16: u16 = 100;
    let uinteger32: u32 = 1000;
    let uinteger64: u64 = 10000;
    let uinteger128: u128 = 100000;

    println!("uinteger8 {}", uinteger8);
    println!("uinteger16 {}", uinteger16);
    println!("uinteger32 {}", uinteger32);
    println!("uinteger64 {}", uinteger64);
    println!("uinteger128 {}", uinteger128);
}

#[test]
fn test_number_convertion() {
    let a: i8 = 10;
    let b: i16 = a as i16;
    let c: i32 = b as i32;
    println!("c is {}", c);
}

#[test]
fn test_number_failed_convertion() {
    let a: i16 = 32243;
    let b: i8 = a as i8;
    println!("b is {}", b); // overflow range from i16 to i8
}
