use std::env;
use std::fs;
use std::process;

fn main() {
    let args:Vec<String> = env::args().collect();
    println!("{:?}", args);

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");
    println!("With contents {}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments")
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config{ query, filename })
    }
}


//1、将main单一职责
//2、将相关的命令行参数改成结构体
//3、错误提示对用户没有用
//4、错误处理集中
