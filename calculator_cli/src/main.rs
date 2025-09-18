fn main() {
    let args = std::env::args();

    parse_args(args);
}

fn parse_args(args: std::env::Args) -> Calculator {
    let first_num = args.skip(1).next().take();

    println!("first_num: {:?}", first_num);

    Calculator { first_num: 0, second_num: 0, op: Operation::Add }
}

struct Calculator {
    first_num: i64,
    second_num: i64,
    op: Operation,
}

enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}