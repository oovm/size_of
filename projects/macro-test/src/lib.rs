use std::ops::Range;
use name_of::{function_name, name_of};

#[test]
fn main() {
    let span = Range {
        start: 1,
        end: 2,
    };
    // start
    println!("name: {}", name_of!(span.start));
}


#[test]
fn main2() {
    println!("{}", function_name!());
}
