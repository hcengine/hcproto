use hcproto::{from_buffer, to_buffer};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct Test {
    val: u32,
    seq: Vec<String>,
}

fn main() {
    println!("Hello, world!");
    let test = Test {
        val: 1,
        seq: vec!["a".to_string(), "b".to_string()],
    };
    let buffer = to_buffer(&test).unwrap();
    println!("buffer = {:?}", buffer);
    let xx: Test = from_buffer(buffer).unwrap();
    assert_eq!(xx, test);
    println!("value = {:?}", xx);
    // assert_eq!(to_buffer(&test).unwrap(), expected);
}
