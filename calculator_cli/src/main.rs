fn main() {
    let args: Vec<String> = std::env::args().into_iter().collect();

    // 参数数量检查
    if args.len() != 4 {
        print_usage();
        return;
    }

    if args[1] == "--help" {
        print_usage();
        return;
    }

    let calc = parse_args(args.into_iter());

    calc.calculate_print();
}

fn print_usage() {
    println!("Usage: calc <num1> <op> <num2>");
    println!("  <op> can be: +, -, *, /");
}

fn parse_args(mut args: impl Iterator<Item = String>) -> Calculator {
    args.next();// 跳过第一个参数，因为它是程序名
    let first_num: i64 = args.next().expect("first num is required").parse().expect("first num is not a number");
    let arg_op = args.next().expect("op is required");
    let op = match arg_op.as_str() {
        "+" => Operation::Add,
        "-" => Operation::Subtract,
        "*" => Operation::Multiply,
        "/" => Operation::Divide,
        _ => {
            eprintln!("op is not valid");
            print_usage();
            std::process::exit(1);
        },
    };
    let second_num: i64 = args.next().expect("second num is required").parse().expect("second num is not a number");

    // println!("first_num: {}", first_num);
    // println!("op: {:?}", op);
    // println!("second_num: {}", second_num);
    Calculator { first_num, second_num, op }
}

struct Calculator {
    first_num: i64,
    second_num: i64,
    op: Operation,
}

impl Calculator {
    fn calculate(&self) -> i64 {
        match self.op {
            Operation::Add => self.first_num + self.second_num,
            Operation::Subtract => self.first_num - self.second_num,
            Operation::Multiply => self.first_num * self.second_num,
            Operation::Divide => {
                if self.second_num == 0 {
                    eprintln!("divide by zero");
                    std::process::exit(1);
                }
                self.first_num / self.second_num
            },
        }
    }

    fn calculate_print(&self) {
        let result = self.calculate();
        let sym = match self.op {
            Operation::Add => "+",
            Operation::Subtract => "-",
            Operation::Multiply => "*",
            Operation::Divide => "/",
        };
        println!("{} {} {} = {}", self.first_num, sym, self.second_num, result);
    }
}

#[derive(Debug)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}