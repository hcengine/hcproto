# hcproto
simple binary proto
一种对标JSON的二进制数据协议, 支持JSON的所有类型的动态组合

## 支持的数据类型
基本支持的类型 "u8",   "i8",   "u16",   "i16",   "u32",   "i32", "u64",   "i64", "varint", "f32", "f64", "string",  "raw", "array",  "map"

### 各种数值类型格式说明
- u8/i8 用一个字节进行写入
- u16/i16/u32/i32/u64/i64 分别对应大小的数据写入, 小端模式
- f32 按IEEE754进行写入
- f64 按IEEE754进行写入
- varint 可变长的整型数据
> 如果是正数则*2, 如果是负数则-(x + 1) * 2, 相当于0->0, -1->1, 1->2,-2->3,2->4来做处理, 因为是小子节的数比较多, 每bit里的第一位则表示是否是最后一位, 如果10000001, 则表示还要继续往下读如果是00000001则表示这是最后一位
- str 字符串类型, 则先用varint表示str的长度, 然后再写入str的值
- str_idx 字符串索引值, 在str的arr表中的第几位, 重复的str则在同一个位置, 用varint表示
- array 数组类型, 先用varint表示array的长度, 然后再写入各个value的数值
- map map类型, 先用varint表示map的长度, 然后先写入key, 再写入value, 依次循环到结束

## 与protobuf差异
> 相对protobuf, 无需预先定义任何的数据格式, 更好的适应多变的场景, 或者客户端不好更新的情况, 拥有更好的自适应性, 简单开封即用, 和JSON一样, 在可支持的数据类型里, 可以自由的进行转换
## 与JSON的差异
> 可以把这个看做是二进制的JSON格式, 有更好的压缩率和更快的解析速度


## 数据使用, 以Rust为例
```rust
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

```

### 格式说明
数据协议分为三部分(协议名称, 字符串索引区, 数据区(默认为数组))
如数据协议名为cmd_test_op, 数据为["tunm_proto", {"name": "tunm_proto", "tunm_proto": 1}]
1. 那么数据将先压缩协议名cmd_test_op, 将先写下可变长度(varint)值为11占用1字节, 然后再写入cmd_test_op的utf8的字节数
2. 接下来准备写入字符串索引区, 索引数据用到的字符串为["tunm_proto", "name"]两个字符串, 即将写入可变长度(varint)值为2占用一字节, 然后分别写入字符串tunm_proto和name两个字符串, 这样子字符串相接近有利于压缩, 且如果有相同的字符串可以更好的进行复用
3. 接下来准备写入数据区, 
首先判断为一个数组, 写入类型u8(TYPE_ARR=16), 写入数组长度varint(2), 准备开始写第一个数据, 字符串tunm_proto, 已转成id, 则写入类型u8(TYPE_STR_IDX=14), 查索引号0, 则写入varint(0), 第一个字段写入完毕, 接下来第二个字段是一个map数据, 写入map长度varint(2), 然后进行遍历得到key值为name, 则写入写入类型u8(TYPE_STR_IDX=14),查索引号1, 则写入varint(1), 然后开始写name对应的值tunm_proto, 写入TYPE_STR_IDX类型的0值, 则这组key写入完毕, 依此类推写入第二组数据

测试打印的结果
用完整的level-full4.json

```
解析JSON用时 = Ok(3.104869s)
用tunm_proto压缩test_level4_json的长度 = 370k
压缩JSON耗时 = Ok(22.2712ms)
解析buffer耗时 = Ok(16.9738ms)
普通文本的长度 = 90
```
解析速度约为JSON的68倍, 符合预期, 大小为明文的0.16倍, 符合压缩比

#### 相关连接
[协议地址https://github.com/hcengine/hcproto/tree/main/Rust](https://github.com/hcengine/hcproto/tree/main/Rust)
