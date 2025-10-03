use serde::Serialize;
use serde_json::ser::{Serializer};
use serde_json::Value;
use anyhow::{anyhow, Result};

pub fn is_valid_json(value: &str) -> bool {
    serde_json::from_str::<serde_json::Value>(value).is_ok()
}

pub fn to_pretty_json(value: &str, with_indent: usize) -> Result<String, Box<dyn std::error::Error>> {
    if with_indent > 32 || with_indent < 1 {
        return Err("缩进只允许1-32位".into());
    }
    let json_str = serde_json::from_str::<serde_json::Value>(value);

    match json_str {
        Ok(json_str) => {
            let mut buffer = Vec::new();
            let indent_position = [b' '; 32];
            let indent = &indent_position[..=with_indent];
            let formatter = serde_json::ser::PrettyFormatter::with_indent(indent);
            let mut ser = Serializer::with_formatter(&mut buffer, formatter);

            json_str.serialize(&mut ser).unwrap();

            Ok(String::from_utf8(buffer).unwrap())
        },
        Err(_) => Err("不是一个有效的json文本".into()),
    }
    // let pretty_string = serde_json::to_string_pretty(&json_str);
    // match pretty_string {
    //     Ok(pretty_string) => pretty_string,
    //     Err(_) => value.to_string(),
    // }
}

pub fn to_minify_json(value: &str) -> Result<String, Box<dyn std::error::Error>> {
    let json_str = serde_json::from_str::<serde_json::Value>(value);

    match json_str {
        Ok(json_str) => {
            match serde_json::to_string(&json_str) {
                Ok(json_str) => Ok(json_str),
                Err(_) => Err("JSON压缩错误".into()),
            }
        },
        Err(_) => Err("不是一个有效的json文本".into()),
    }
}

pub fn parse_json(json: &str) -> Result<Value>
{
    if is_valid_json(json) {
        Ok(serde_json::from_str::<Value>(json)?)
    } else {
        Err(anyhow!("Not a valid JSON"))
    }
}