use std::env;
use std::process;
use minigrep_tdd;

fn main() {
    let args:Vec<String> = env::args().collect();
    println!("{:?}", args);

    let config = minigrep_tdd::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    //
    // println!("Searching for {}", config.query);
    // println!("In file for {}", config.filename);

   if let Err(e) = minigrep_tdd::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

//1、将main单一职责
//2、将相关的命令行参数改成结构体
//3、错误提示对用户没有用
//4、错误处理集中
