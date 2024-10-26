use hcproto::{from_buffer, to_buffer};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
struct Test {
    int: u32,
    seq: Vec<String>,
}


fn main() {
    println!("Hello, world!");

        let test = Test {
            int: 1,
            seq: vec!["a".to_string(), "b".to_string()],
        };
        let expected = r#"{"int":1,"seq":["a","b"]}"#;
        let mut buffer = to_buffer(&test).unwrap();
        println!("xxx = {:?}", buffer);
        let xx: Test = from_buffer(&mut buffer).unwrap();
        println!("value = {:?}", xx);
        // assert_eq!(to_buffer(&test).unwrap(), expected);
}
