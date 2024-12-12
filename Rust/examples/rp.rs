use algorithm::buf::Bt;
use hcproto::{Buffer, Value};
use std::time::SystemTime;

mod test_data;
use std::collections::HashMap;

fn test_level4_json() {
    let mut now = SystemTime::now();
    let parsed = test_data::get_json();
    println!("解析JSON用时 = {:?}", now.elapsed());
    now = SystemTime::now();
    // println!("ok!!! parsed= {:?}", parsed);
    let mut buffer = Buffer::new();
    hcproto::encode_proto(&mut buffer, &"cmd_level4_full".to_string(), vec![parsed]).unwrap();
    println!(
        "用tunm_proto压缩test_level4_json的长度 = {}k",
        buffer.remaining() / 1024
    );

    println!("压缩JSON耗时 = {:?}", now.elapsed());
    now = SystemTime::now();
    let read = hcproto::decode_proto(&mut buffer).unwrap();
    println!("解析buffer耗时 = {:?}", now.elapsed());
    match read {
        (name, _val) => {
            assert_eq!(name, "cmd_level4_full".to_string());
            // println!("value === {:?}", val);
        }
    }
}

fn main() {
    println!("welcome to tickdream rust protocol");
    test_level4_json();
    let mut hash_value = HashMap::<Value, Value>::new();
    hash_value.insert(
        Value::Str("name".to_string()),
        Value::Str("tunm_proto".to_string()),
    );
    hash_value.insert(Value::Str("tunm_proto".to_string()), Value::U16(1 as u16));

    {
        let mut buffer = Buffer::new();
        hcproto::encode_proto(
            &mut buffer,
            &"cmd_test_op".to_string(),
            vec![Value::Map(hash_value.clone())],
        )
        .unwrap();
        let just_str = "
        [\"cmd_test_op\", [\"tunm_proto\", {\"name\": \"tunm_proto\", \"tunm_proto\": 1}]]
        ";
        println!("普通文本的长度 = {}", just_str.len());
        println!("用tunm_proto的长度 = {}", buffer.remaining());
        // just read field
        let read = hcproto::decode_proto(&mut buffer).unwrap();
        match read {
            (name, val) => {
                assert_eq!(name, "cmd_test_op".to_string());
                assert_eq!(val[0], Value::Map(hash_value));
                assert_eq!(val.len(), 1);
            }
        }
    }
}
