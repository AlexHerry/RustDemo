use std::io::{self, BufRead};

fn main() {
    println!("程序开始执行");
    println!("创建stdin().lines()迭代器（这一步不会阻塞）");
    
    // 只是创建迭代器，不会阻塞
    let input = io::stdin().lines();
    
    println!("迭代器已创建: {:?}", input);
    println!("现在尝试从迭代器中读取数据（这一步会阻塞）...");
    
    // 当实际尝试读取数据时才会阻塞
    for line_result in input {
        match line_result {
            Ok(line) => {
                println!("收到输入: {}", line);
                break; // 只读取一行
            },
            Err(e) => {
                println!("读取错误: {}", e);
                break;
            }
        }
    }
    
    println!("程序继续执行并退出");
}
