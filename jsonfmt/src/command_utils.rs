use std::fs;
use atty::Stream;
use std::io::{self, Read};
use clap::{CommandFactory, Parser};

#[derive(Parser, Debug)]
#[command(version, about, long_about = "
jsonfmt 是一个命令行工具，用于格式化 JSON 数据。
它可以从文件、标准输入（stdin）或直接的字符串输入读取 JSON，
并将其输出为格式化（美化）或压缩（紧凑）的形式。

## 输入源
输入源可以通过位置参数指定：
- **文件路径**：提供一个文件名（如 `data.json`），工具将读取该文件。
- **标准输入**：使用 `-` 作为输入，工具将从标准输入读取数据。
  这在管道（pipe）操作中非常有用。
- **直接字符串**：直接提供 JSON 字符串（注意可能需要引号和转义）。

## 输出形式
- **美化 (Pretty)**：默认行为，以易读的格式输出 JSON，带有缩进和换行。

## 示例

# 美化一个 JSON 文件
jsonfmt data.json

# 将美化后的 JSON 保存到新文件
jsonfmt data.json --output pretty.json

# 从标准输入读取并美化（例如，来自 curl 的响应）
curl -s https://api.example.com/data | jsonfmt -

# 检查 JSON 有效性（不输出，仅验证）
jsonfmt --validate data.json
")]
pub struct Args {
    #[arg(value_name = "INPUT")]
    pub input: Option<String>,

    #[arg(short, long, value_name = "FILENAME")]
    pub file: Option<String>,

    #[arg(long, required = false)]
    pub output: Option<String>,

    #[arg(long, default_value="false", conflicts_with_all= ["validate", "indent"])]
    pub minify: bool,

    #[arg(long, default_value="false", conflicts_with_all= ["minify", "output", "indent"])]
    pub validate: bool,

    #[arg(long, default_value="4", conflicts_with_all = ["minify", "validate"])]
    pub indent: usize,
}

pub fn usage_read(args: Args) -> Args {
    let input = if let Some(data) = args.input.as_ref() {
        if data.to_string() == "-".to_string() {
            read_from_stdin()
        } else {
            data.to_string()
        }
    } else if !atty::is(Stream::Stdin) {
        let mut data = String::new();
        io::stdin().read_to_string(&mut data).unwrap();
        data
    } else {
        if let Some(file) = args.file.as_ref() {
            match fs::read_to_string(&file) {
                Ok(data) => data,
                Err(_) => {
                    eprintln!("File is not found");
                    std::process::exit(1);
                }
            }
        } else {
            Args::command().print_help().unwrap();
            std::process::exit(1);
        }
    };

    Args {input: Some(input), ..args}
}

pub fn read_from_stdin() -> String {
    println!("=== 请输入JSON文本（按Ctrl+D结束输入） ===");
    
    // 方法1：使用lines()迭代器逐行读取
    // println!("\n1. 使用lines()迭代器逐行读取（遇到EOF结束）：");
    // println!("请输入几行文本（按Ctrl+D结束输入）：");
    
    // // 这会创建一个迭代器，但只有在调用next()时才会真正读取输入
    // for line_result in io::stdin().lines() {
    //     match line_result {
    //         Ok(line) => println!("读取到: {}", line),
    //         Err(err) => eprintln!("读取错误: {}", err),
    //     }
    // }
    
    // 方法2：读取全部内容到字符串
    // println!("\n2. 读取全部内容到字符串：");
    // println!("请再次输入一些文本（按Ctrl+D结束输入）：");
    
    let mut buffer = String::new();
    if let Err(err) = io::stdin().read_to_string(&mut buffer) {
        eprintln!("读取全部内容时出错: {}", err);
        Args::command().print_help().unwrap();
    }
    
    // 方法3：使用BufReader进行缓冲读取
    // println!("\n3. 使用BufReader进行缓冲读取：");
    // println!("请输入最后一些文本（按Ctrl+D结束输入）：");
    //
    // let stdin = io::stdin();
    // let reader = io::BufReader::new(stdin.lock());
    //
    // for line_result in reader.lines() {
    //     match line_result {
    //         Ok(line) => println!("Buffered读取到: {}", line),
    //         Err(err) => eprintln!("Buffered读取错误: {}", err),
    //     }
    // }
    //
    // println!("\n所有读取示例已完成。");

    return buffer;
}