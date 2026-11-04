use std::error::Error;
use std::fs;
use std::env;


// 创建结构体
pub struct Config {
    pub query: String,
    pub filename: String,
    /** 是否区分大小写 */
    pub case_sensitive: bool,
}

// 创建一个 Config 的构造函数
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str>  {
        // 检查参数数量是否足够
        if args.len() < 3 {
            // panic 一般处理程序上的错误，而不是使用上的错误
            // panic!("Not enough arguments provided");
            return Err("Not enough arguments provided");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        // 检查叫做 CASE_INSENSITIVE 的环境变量
        // powershell 运行 $env:CASE_INSENSITIVE=1 来设置环境变量，值为 1 表示不区分大小写
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

// 修改 run 函数返回 Result
// Box<dyn Error> 意味着函数会返回实现了 Error trait 的类型，不过无需指定具体将会返回的值的类型。
// ? 会从函数中返回错误值并让调用者来处理它
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    for line in results {
        println!("{}", line);
    }
    // println!("The file contents of the file is:\n{}", contents);
    Ok(())
}



// 测试代码会失败
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> { 
    let mut results = Vec::new();

    for line in contents.lines() {
        // do something with line
        if line.contains(query) {
            results.push(line);
            // println!("{}", line);
        }
    }
    results
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> { 
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        // do something with line
        if line.to_lowercase().contains(&query) {
            results.push(line);
            // println!("{}", line);
        }
    }
    results
}



#[cfg(test)]
mod tests {
    use super::*;


        #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

        #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}


