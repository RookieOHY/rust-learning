

/*
熟悉一些 api:
*/
use std::{env, fs, process};
use std::error::Error;
// use ch02_os;
fn main() {
    // let args:Vec<String> = env::args().collect();
    // println!("{:?}",args); // ["E:\\RustProjects\\rust-learning\\target\\debug\\ch02_os.exe", "a", "b"]
    // let query = &args[1];
    // let filename = &args[2];
    let config = ch02_os::Config::new(env::args()).unwrap_or_else(|err|{
        println!("输入参数有误:{}",err);
        process::exit(1)
    });
    // println!("搜索内容： {}", query);
    // println!("搜索的文件： {}", filename);

    if let Err(e) = ch02_os::run(config){
        println!("执行检索有误:{}",e);
        process::exit(1);
    }




}



