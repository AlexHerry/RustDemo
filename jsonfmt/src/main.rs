use std::fs;
use clap::Parser;
use jsonfmt::command_utils::{usage_read, Args};
use jsonfmt::json_utils::{is_valid_json, to_minify_json, to_pretty_json};

fn main() {
    let args = usage_read(Args::parse());

    let input = args.input.expect("input should not be None here");
    let with_indent = args.indent;
    let minify = args.minify;
    let output = args.output.unwrap_or("".to_string());
    let validate = args.validate;

    if validate {
        println!("您输入的json文本{}有效json数据", if is_valid_json(&input) {"是"} else {"不是"});
        return;
    }

    let input = if !minify {
        to_pretty_json(&input, with_indent)
    } else {
        to_minify_json(&input)
    }.unwrap_or_else(|err| {
        eprintln!("json解析错误: {}", err);
        std::process::exit(1);
    });

    if output != "".to_string() {
        match fs::write(&output, input) {
            Ok(_) => println!("格式化结构已写入 {}", output),
            Err(e) => eprintln!("{}文件写入失败: {}", output, e),
        }
    } else {
        println!("{}", input);
    }

}
