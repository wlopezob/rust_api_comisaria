use std::{collections::HashSet, hash::{Hash, Hasher}};

use serde::de::DeserializeOwned;
use serde_json::Value;

pub fn get_value_response<T: DeserializeOwned>(
    response_text: impl Into<String> + Clone,
) -> T {
    let value_json: Value = serde_json::from_str(&response_text.into()).unwrap();
    let value_json = remove_duplicates(&value_json);
    serde_json::from_value::<T>(value_json).unwrap()
}

struct HashableValue<'a>(&'a Value);

pub fn remove_duplicates(value: &Value) -> Value {
    match value {
        Value::Array(arr) => {
            let mut set = HashSet::new();
            let mut array = vec![];
            for a in arr {
                let v = HashableValue(a);
                if set.contains(&v) {
                    continue;
                }
                set.insert(v);
                array.push(remove_duplicates(a));
            }
            return Value::Array(array);
        }
        Value::Object(obj) => {
            let mut map = serde_json::Map::new();
            for x in obj.iter() {
                map.insert(x.0.clone(), remove_duplicates(x.1));
            }
            return Value::Object(map);
        }
        _ => {}
    }
    value.clone()
}

impl<'a> Hash for HashableValue<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match &self.0 {
            Value::Null => 0.hash(state),
            Value::Bool(b) => b.hash(state),
            Value::Number(n) => n.hash(state),
            Value::String(str) => str.hash(state),
            Value::Array(arr) => arr.iter().for_each(|a| HashableValue(a).hash(state)),
            Value::Object(obj) => obj.iter().for_each(|entry| {
                entry.0.hash(state);
                HashableValue(entry.1).hash(state);
            }),
        }
    }
}

impl<'a> PartialEq<Self> for HashableValue<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(other.0)
    }
}

impl<'a> Eq for HashableValue<'a> {}