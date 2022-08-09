use std::env;
use std::fs;

fn main() {
    let args:Vec<String> = env::args().collect();
    println!("{:?}", args);

    let query = &args[1];
    println!("query {}", query);

    let filename = &args[2];
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    println!("With contents {}", contents);

}

fn parse_config(args: &[String]) ->(&str, &str) {
    let query = &args[1];
    let filename = &args[2];
    (query, filename)
}

//1、将main单一职责
//2、将相关的命令行参数改成结构体
//3、错误提示对用户没有用
//4、错误处理集中
