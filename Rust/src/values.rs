use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
use std::hash::Hash;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueType {
    Nil = 0,
    Bool = 1,
    U8 = 2,
    I8 = 3,
    U16 = 4,
    I16 = 5,
    U32 = 6,
    I32 = 7,
    U64 = 8,
    I64 = 9,
    Varint = 10,
    F32 = 11,
    F64 = 12,
    Str = 13,
    StrIdx = 14,
    Raw = 15,
    Arr = 16,
    Map = 17,
    // Kv = 18,
}

impl From<u8> for ValueType {
    fn from(value: u8) -> Self {
        match value {
            1 => ValueType::Bool,
            2 => ValueType::U8,
            3 => ValueType::I8,
            4 => ValueType::U16,
            5 => ValueType::I16,
            6 => ValueType::U32,
            7 => ValueType::I32,
            8 => ValueType::U64,
            9 => ValueType::I64,
            10 => ValueType::Varint,
            11 => ValueType::F32,
            12 => ValueType::F64,
            13 => ValueType::Str,
            14 => ValueType::StrIdx,
            15 => ValueType::Raw,
            16 => ValueType::Arr,
            17 => ValueType::Map,
            // 18 => ValueType::Kv,

            _ => ValueType::Nil,
        }
    }
}

#[derive(PartialEq, Eq)]
pub struct ValueDescType {
    pub val: &'static str,
}

#[allow(non_upper_case_globals)]
impl ValueDescType {
    pub const fn new(val: &'static str) -> Self {
        Self { val }
    }

    pub const Nil: ValueDescType = ValueDescType::new("nil");
    pub const Bool: ValueDescType = ValueDescType::new("bool");
    pub const U8: ValueDescType = ValueDescType::new("u8");
    pub const I8: ValueDescType = ValueDescType::new("i8");
    pub const U16: ValueDescType = ValueDescType::new("u16");
    pub const I16: ValueDescType = ValueDescType::new("i16");
    pub const U32: ValueDescType = ValueDescType::new("u32");
    pub const I32: ValueDescType = ValueDescType::new("i32");
    pub const U64: ValueDescType = ValueDescType::new("u64");
    pub const I64: ValueDescType = ValueDescType::new("i64");
    pub const Varint: ValueDescType = ValueDescType::new("varint");
    pub const F32: ValueDescType = ValueDescType::new("float");
    pub const F64: ValueDescType = ValueDescType::new("double");
    pub const Str: ValueDescType = ValueDescType::new("str");
    pub const StrIdx: ValueDescType = ValueDescType::new("str_idx");
    pub const Raw: ValueDescType = ValueDescType::new("raw");
    pub const Arr: ValueDescType = ValueDescType::new("arr");
    pub const Map: ValueDescType = ValueDescType::new("map");
    pub const Kv: ValueDescType = ValueDescType::new("kv");
}

impl Display for ValueDescType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.val.fmt(f)
    }
}

impl From<ValueType> for ValueDescType {
    fn from(value: ValueType) -> Self {
        match value {
            ValueType::Nil => ValueDescType::Nil,
            ValueType::Bool => ValueDescType::Bool,
            ValueType::U8 => ValueDescType::U8,
            ValueType::I8 => ValueDescType::I8,
            ValueType::U16 => ValueDescType::U16,
            ValueType::I16 => ValueDescType::I16,
            ValueType::U32 => ValueDescType::U32,
            ValueType::I32 => ValueDescType::I32,
            ValueType::U64 => ValueDescType::U64,
            ValueType::I64 => ValueDescType::I64,
            ValueType::Varint => ValueDescType::Varint,
            ValueType::F32 => ValueDescType::F32,
            ValueType::F64 => ValueDescType::F64,
            ValueType::Str => ValueDescType::Str,
            ValueType::Raw => ValueDescType::StrIdx,
            ValueType::Arr => ValueDescType::Arr,
            ValueType::Map => ValueDescType::Map,
            // ValueType::Kv => ValueDescType::Kv,
            _ => ValueDescType::Nil,
        }
    }
}

impl From<ValueDescType> for ValueType {
    fn from(value: ValueDescType) -> Self {
        match value {
            ValueDescType::Nil => ValueType::Nil,
            ValueDescType::Bool => ValueType::Bool,
            ValueDescType::U8 => ValueType::U8,
            ValueDescType::I8 => ValueType::I8,
            ValueDescType::U16 => ValueType::U16,
            ValueDescType::I16 => ValueType::I16,
            ValueDescType::U32 => ValueType::U32,
            ValueDescType::I32 => ValueType::I32,
            ValueDescType::U64 => ValueType::U64,
            ValueDescType::I64 => ValueType::I64,
            ValueDescType::Varint => ValueType::Varint,
            ValueDescType::F32 => ValueType::F32,
            ValueDescType::F64 => ValueType::F64,
            ValueDescType::Str => ValueType::Str,
            ValueDescType::Raw => ValueType::Raw,
            ValueDescType::Arr => ValueType::Arr,
            ValueDescType::Map => ValueType::Map,
            // ValueDescType::Kv => ValueType::Kv,
            _ => ValueType::Nil,
        }
    }
}

#[derive(Clone)]
pub enum Value {
    Nil,
    Bool(bool),
    U8(u8),
    I8(i8),
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64),
    Varint(i64),
    F32(f32),
    F64(f64),
    Str(String),
    Raw(Vec<u8>),
    Arr(Vec<Value>),
    Map(HashMap<Value, Value>),
    // Kv(String, Vec<Value>),
}

impl Value {
    pub fn is_nil(&self) -> bool {
        match self {
            Value::Nil => true,
            _ => false
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Bool(l0), Self::Bool(r0)) => l0 == r0,
            (Self::U8(l0), Self::U8(r0)) => l0 == r0,
            (Self::I8(l0), Self::I8(r0)) => l0 == r0,
            (Self::U16(l0), Self::U16(r0)) => l0 == r0,
            (Self::I16(l0), Self::I16(r0)) => l0 == r0,
            (Self::U32(l0), Self::U32(r0)) => l0 == r0,
            (Self::I32(l0), Self::I32(r0)) => l0 == r0,
            (Self::U64(l0), Self::U64(r0)) => l0 == r0,
            (Self::I64(l0), Self::I64(r0)) => l0 == r0,
            (Self::Varint(l0), Self::Varint(r0)) => l0 == r0,

            (Self::Varint(l0), Self::U8(r0)) => l0 == &(*r0 as i64),
            (Self::Varint(l0), Self::I8(r0)) => l0 == &(*r0 as i64),
            (Self::Varint(l0), Self::U16(r0)) => l0 == &(*r0 as i64),
            (Self::Varint(l0), Self::I16(r0)) => l0 == &(*r0 as i64),
            (Self::Varint(l0), Self::U32(r0)) => l0 == &(*r0 as i64),
            (Self::Varint(l0), Self::I32(r0)) => l0 == &(*r0 as i64),
            (Self::Varint(l0), Self::U64(r0)) => l0 == &(*r0 as i64),
            (Self::Varint(l0), Self::I64(r0)) => l0 == &(*r0 as i64),

            (Self::U8(r0), Self::Varint(l0)) => l0 == &(*r0 as i64),
            (Self::I8(r0), Self::Varint(l0)) => l0 == &(*r0 as i64),
            (Self::U16(r0), Self::Varint(l0)) => l0 == &(*r0 as i64),
            (Self::I16(r0), Self::Varint(l0)) => l0 == &(*r0 as i64),
            (Self::U32(r0), Self::Varint(l0)) => l0 == &(*r0 as i64),
            (Self::I32(r0), Self::Varint(l0)) => l0 == &(*r0 as i64),
            (Self::U64(r0), Self::Varint(l0)) => l0 == &(*r0 as i64),
            (Self::I64(r0), Self::Varint(l0)) => l0 == &(*r0 as i64),

            (Self::F32(l0), Self::F32(r0)) => l0 == r0,
            (Self::F64(l0), Self::F64(r0)) => l0 == r0,
            (Self::Str(l0), Self::Str(r0)) => l0 == r0,
            (Self::Raw(l0), Self::Raw(r0)) => l0 == r0,
            (Self::Arr(l0), Self::Arr(r0)) => l0 == r0,
            (Self::Map(l0), Self::Map(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl Eq for Value {}

impl Hash for Value {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Nil => write!(fmt, "nil"),
            Value::Bool(val) => write!(fmt, "bool({:?})", val),
            Value::U8(val) => write!(fmt, "u8({:?})", val),
            Value::I8(val) => write!(fmt, "i8({:?})", val),
            Value::U16(val) => write!(fmt, "u16({:?})", val),
            Value::I16(val) => write!(fmt, "i16({:?})", val),
            Value::U32(val) => write!(fmt, "u32({:?})", val),
            Value::I32(val) => write!(fmt, "i32({:?})", val),
            Value::U64(val) => write!(fmt, "u64({:?})", val),
            Value::I64(val) => write!(fmt, "i64({:?})", val),
            Value::Varint(val) => write!(fmt, "varint({:?})", val),
            Value::F32(val) => write!(fmt, "float({:?})", val),
            Value::F64(val) => write!(fmt, "double({:?})", val),
            Value::Str(ref val) => write!(fmt, "str({:?})", val),
            Value::Raw(ref val) => write!(fmt, "str({:?})", val),
            Value::Arr(ref val) => write!(fmt, "arr({:?})", val),
            Value::Map(ref val) => write!(fmt, "str({:?})", val),
            // Value::Kv(ref key, ref val) => write!(fmt, "key:{:?}, str({:?})", key, val),
        }
    }
}

impl From<bool> for Value {
    fn from(val: bool) -> Value {
        Value::Bool(val)
    }
}

impl From<u8> for Value {
    fn from(val: u8) -> Value {
        Value::U8(val)
    }
}

impl From<i8> for Value {
    fn from(val: i8) -> Value {
        Value::I8(val)
    }
}

impl From<u16> for Value {
    fn from(val: u16) -> Value {
        Value::U16(val)
    }
}

impl From<i16> for Value {
    fn from(val: i16) -> Value {
        Value::I16(val)
    }
}

impl From<u32> for Value {
    fn from(val: u32) -> Value {
        Value::U32(val)
    }
}

impl From<i32> for Value {
    fn from(val: i32) -> Value {
        Value::I32(val)
    }
}

impl From<u64> for Value {
    fn from(val: u64) -> Value {
        Value::U64(val)
    }
}

impl From<i64> for Value {
    fn from(val: i64) -> Value {
        Value::I64(val)
    }
}

impl From<f32> for Value {
    fn from(val: f32) -> Value {
        Value::F32(val)
    }
}

impl From<f64> for Value {
    fn from(val: f64) -> Value {
        Value::F64(val)
    }
}

impl From<String> for Value {
    fn from(val: String) -> Value {
        Value::Str(val)
    }
}

impl From<Vec<u8>> for Value {
    fn from(val: Vec<u8>) -> Value {
        Value::Raw(val)
    }
}

impl From<Vec<Value>> for Value {
    fn from(val: Vec<Value>) -> Value {
        Value::Arr(val)
    }
}

impl From<HashMap<Value, Value>> for Value {
    fn from(val: HashMap<Value, Value>) -> Value {
        Value::Map(val)
    }
}


// impl From<(String, Vec<Value>)> for Value {
//     fn from(val: (String, Vec<Value>)) -> Value {
//         Value::Kv(val.0, val.1)
//     }
// }

impl Into<bool> for Value {
    fn into(self) -> bool {
        match self {
            Value::Bool(val) => val,
            Value::U8(val) => val != 0,
            Value::I8(val) => val != 0,
            Value::Varint(val) => val != 0,
            _ => panic!(
                "into error type {}",
                ValueDescType::from(get_type_by_value(&self))
            ),
        }
    }
}

impl Into<u8> for Value {
    fn into(self) -> u8 {
        match self {
            Value::U8(val) => val,
            Value::Varint(val) => val as u8,
            _ => panic!(
                "into error type {}",
                ValueDescType::from(get_type_by_value(&self))
            ),
        }
    }
}


impl Into<ValueType> for Value {
    fn into(self) -> ValueType {
        match self {
            Value::U8(val) => ValueType::from(val),
            Value::Varint(val) =>  ValueType::from(val as u8),
            _ => panic!(
                "into error type {}",
                ValueDescType::from(get_type_by_value(&self))
            ),
        }
    }
}

impl Into<i8> for Value {
    fn into(self) -> i8 {
        match self {
            Value::I8(val) => val,
            Value::Varint(val) => val as i8,
            _ => panic!(
                "into error type {}",
                ValueDescType::from(get_type_by_value(&self))
            ),
        }
    }
}

impl Into<u16> for Value {
    fn into(self) -> u16 {
        match self {
            Value::U16(val) => val,
            Value::Varint(val) => val as u16,
            _ => panic!(
                "into error type {}",
                ValueDescType::from(get_type_by_value(&self))
            ),
        }
    }
}

impl Into<i16> for Value {
    fn into(self) -> i16 {
        match self {
            Value::I16(val) => val,
            Value::Varint(val) => val as i16,
            _ => panic!(
                "into error type {}",
                ValueDescType::from(get_type_by_value(&self))
            ),
        }
    }
}

impl Into<u32> for Value {
    fn into(self) -> u32 {
        match self {
            Value::U32(val) => val,
            Value::Varint(val) => val as u32,
            _ => panic!(
                "into error type {}",
                ValueDescType::from(get_type_by_value(&self))
            ),
        }
    }
}

impl Into<i32> for Value {
    fn into(self) -> i32 {
        match self {
            Value::I32(val) => val,
            Value::Varint(val) => val as i32,
            _ => panic!(
                "into error type {}",
                ValueDescType::from(get_type_by_value(&self))
            ),
        }
    }
}

impl Into<u64> for Value {
    fn into(self) -> u64 {
        match self {
            Value::U64(val) => val,
            Value::Varint(val) => val as u64,
            _ => panic!(
                "into error type {}",
                ValueDescType::from(get_type_by_value(&self))
            ),
        }
    }
}

impl Into<i64> for Value {
    fn into(self) -> i64 {
        match self {
            Value::I64(val) => val,
            Value::Varint(val) => val as i64,
            _ => panic!(
                "into error type {}",
                ValueDescType::from(get_type_by_value(&self))
            ),
        }
    }
}

impl Into<f32> for Value {
    fn into(self) -> f32 {
        match self {
            Value::F32(val) => val,
            Value::Varint(val) => val as f32 / 1000.0,
            _ => panic!(
                "into error type {}",
                ValueDescType::from(get_type_by_value(&self))
            ),
        }
    }
}

impl Into<f64> for Value {
    fn into(self) -> f64 {
        match self {
            Value::F64(val) => val,
            Value::Varint(val) => val as f64 / 1000000.0,
            _ => panic!(
                "into error type {}",
                ValueDescType::from(get_type_by_value(&self))
            ),
        }
    }
}

impl Into<String> for Value {
    fn into(self) -> String {
        match self {
            Value::Str(val) => val,
            _ => panic!(
                "into error type {}",
                ValueDescType::from(get_type_by_value(&self))
            ),
        }
    }
}

impl Into<Vec<u8>> for Value {
    fn into(self) -> Vec<u8> {
        match self {
            Value::Raw(val) => val,
            _ => panic!(
                "into error type {}",
                ValueDescType::from(get_type_by_value(&self))
            ),
        }
    }
}

impl Into<Vec<Value>> for Value {
    fn into(self) -> Vec<Value> {
        match self {
            Value::Arr(val) => val,
            _ => panic!(
                "into error type {}",
                ValueDescType::from(get_type_by_value(&self))
            ),
        }
    }
}

impl Into<HashMap<Value, Value>> for Value {
    fn into(self) -> HashMap<Value, Value> {
        match self {
            Value::Map(val) => val,
            _ => panic!(
                "into error type {}",
                ValueDescType::from(get_type_by_value(&self))
            ),
        }
    }
}

pub fn get_type_by_value(value: &Value) -> ValueType {
    match *value {
        Value::Bool(_) => ValueType::Bool,
        Value::U8(_) => ValueType::U8,
        Value::I8(_) => ValueType::I8,
        Value::U16(_) => ValueType::U16,
        Value::I16(_) => ValueType::I16,
        Value::U32(_) => ValueType::U32,
        Value::I32(_) => ValueType::I32,
        Value::U64(_) => ValueType::U64,
        Value::I64(_) => ValueType::I64,
        Value::Varint(_) => ValueType::Varint,
        Value::F32(_) => ValueType::F32,
        Value::F64(_) => ValueType::F64,
        Value::Str(_) => ValueType::Str,
        Value::Raw(_) => ValueType::Raw,
        Value::Arr(_) => ValueType::Arr,
        Value::Map(_) => ValueType::Map,
        // Value::Kv(_, _) => ValueType::Kv,
        _ => ValueType::Nil,
    }
}
