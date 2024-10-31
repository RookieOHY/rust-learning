use std::thread;
use std::time::Duration;

// 程序的主入口点
fn main() {
    // 用户参数 + 随机数
    let td_value = 10;
    let random_num = 7;
    // 核心业务逻辑
    service05(td_value,random_num);
}

// 再次优化 ：定义存放闭包和存放闭包执行结果的结构体(惰性执行)
fn service05(args: u32, random: u32) {
    // 创建实例
    let mut instance = Cacher::new(|args| {
        println!("计算执行了...");
        thread::sleep(Duration::from_secs(2));
        args
    });
    // 闭包调用：闭包变量(实参)
    if args < 25 {
        println!("小于25：调用2次，第一次：{}",instance.call(args));
        println!("小于25：调用2次，第二次：{}",instance.call(args));
    } else {
        if random == 3 {
            println!("等于3，不做任何事！")
        }else {
            println!("不等于3，调用1次：{}",instance.call(args))
        }
    }
}


struct Cacher<T>
    where T: Fn(u32)->u32
{
    js: T, //闭包
    res: Option<u32>, // 闭包结果
}

// 新建 和 调用
impl<T> Cacher<T>
    where T: Fn(u32)->u32
{
    fn new(js: T) -> Cacher<T> {
        Cacher {
            js,
            res: None,
        }
    }

    fn call(&mut self, args: u32) -> u32 {
        match self.res {
            Some(v) => v, // 直接返回结果
            None => {
                let v = (self.js)(args); // 实例对象.闭包(实参)
                self.res = Some(v); // 接收结果
                v // 返回结果
            }
        }
    }
}



// 优化为闭包，但还是存在被调用多次
fn service04(args: u32, random: u32) {
    // 闭包
    let i = |args|{
        println!("计算执行了...");
        thread::sleep(Duration::from_secs(2));
        args
    };
    // 闭包调用：闭包变量(实参)
    if args < 25 {
        println!("小于25：调用2次，第一次：{}",i(args));
        println!("小于25：调用2次，第二次：{}",i(args));
    } else {
        if random == 3 {
            println!("等于3，不做任何事！")
        }else {
            println!("不等于3，调用1次：{}",i(args))
        }
    }
}

// 闭包重构 service02
fn service03(args: u32, random: u32) {
    let i = js(args);
    if args < 25 {
        println!("小于25：调用2次，第一次：{}",i);
        println!("小于25：调用2次，第二次：{}",i);
    } else {
        if random == 3 {
            println!("等于3，不做任何事！")
        }else {
            println!("不等于3，调用1次：{}",i)
        }
    }
}


// 有个语句块本来不需要等待该函数执行完毕的，但还是依旧等待了
fn service02(args: u32, random: u32) {
    let i = js(args);
    if args < 25 {
        println!("小于25：调用2次，第一次：{}",i);
        println!("小于25：调用2次，第二次：{}",i);
    } else {
        if random == 3 {
            println!("等于3，不做任何事！")
        }else {
            println!("不等于3，调用1次：{}",i)
        }
    }
}

fn service(args: u32, random: u32) {
    if args < 25 {
        println!("小于25：调用2次，第一次：{}",js(args));
        println!("小于25：调用2次，第二次：{}",js(args));
    } else {
        if random == 3 {
            println!("等于3，不做任何事！")
        }else {
            println!("不等于3，调用1次：{}",js(args))
        }
    }
}

// 函数：健身经历计算
fn js(args:u32)->u32{
    println!("计算执行了...");
    thread::sleep(Duration::from_secs(2));
    args
}

// let nm_fn = |args|{
//     println!("计算执行了...");
//     thread::sleep(Duration::from_secs(2));
//     args
// }
