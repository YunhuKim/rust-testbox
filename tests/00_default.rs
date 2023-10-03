//Rust_Testbed
//Duplicate and rename this file to make new test
//to run it
//cargo test --test <your file name> -- --nocapture
//ex) cargo test --test 00_default -- --nocapture

fn main() {
    println!("This is the Tester");
}

#[test]
fn test() {
    main();
}