use std::error::Error;
use std::{env, fs};

// Box<dyn Error> 意味着函数会返回实现了 Error trait 的类型，不过无需指定具体将会返回的值的类型
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;
    // println!("读取到的文件的内容：{}",content);

    let result = if config.case_sensitive {
        search(&config.query, &content)
    }else {
        search_case_insensitive(&config.query, &content)
    };

    for line in result {
        println!("查询到的内容:{}", line);
    }

    Ok(())
}


pub struct Config{
    pub query:String,
    pub filename:String,
    pub case_sensitive: bool,
}

impl Config{
    pub fn new(args:&[String]) -> Result<Config, &'static str> {
        if args.len() < 3{
            return Err("没有足够的参数");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config{query,filename,case_sensitive})
    }
}

pub fn search<'a>(query:&str,contents: &'a str) -> Vec<&'a str>{
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.contains(query){
            result.push(line)
        }
    }
    result
}


pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
