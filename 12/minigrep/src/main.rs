
use std::env; 
use std::process;

use minigrep::Config;

fn main() {
    // args 这个函数返回一个传递给程序的命令行参数的 迭代器（iterator）。
    // 1. 将命令行参数收集到一个 vector 中并打印出来
    let args: Vec<String> = env::args().collect();
    // println!("args: {:?}", args);

    // 2. 创建变量来存放查询参数和文件名参数
    // unwrap_or_else 函数会返回一个 Result<T, E> 类型的值，如果 Result 的值为 Ok(T)，则返回 Ok(T) 中的值，否则返回第二个参数的值。
    // 出错时，值会传递给闭包中位于两道竖线间的参数 err。闭包中的代码在其运行时可以使用这个 err 值。
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });
    
    // println!("Searching for {}", config.query);
    // println!("In file {}", config.filename);

    // 处理 run 返回的错误
    if let Err(e) = minigrep::run(config) {
        // 使用 eprintln! 将错误信息写入标准错误而不是标准输出
        // 使用 cargo run > output.txt
        // cargo run to poem.txt > output.txt 并查看 output.txt 文件内容
        eprintln!("Application error: {}", e);
        process::exit(1);
    }

}






// 抽离函数 parse_config，用于解析命令行参数
// 返回值是一个 Config 结构体实例
// fn parse_config(args: &[String]) -> Config {
//     // 返回的数据拥有所有权，因此使用 clone 拷贝一份，无需管理引用的生命周期，但会牺牲一小部分性能
//     let query = args[1].clone();
//     let filename = args[2].clone();

//     Config { query, filename }
// }