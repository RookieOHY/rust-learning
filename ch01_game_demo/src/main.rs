use std::io;
use rand::Rng;
/**
猜数游戏demo分析Rust语法：
    涉及成员：
        println!:
            宏（不是函数）
        use std::io：
            引入标准库std的io库（默认Rust会把Prelude模块导入每一个函数的作用域中）
        String::new()：
            类比Java 静态类的静态方法调用。
        read_line():
            参数是一个可变字符串的引用（取值地），因为引用默认也是不可变的，也是需要mut关键字修饰使得可变.
            io::Result<usize> 这是函数的返回值。
    知识点：
        Rust中的所有变量默认都是不可变。
        修改变量的值：
            申明变量用mux修饰。
        Rust中库为：
            crate (https://crates.io/crates/rand)
            其中rand为具体的库名字
        运行案例：
            cargo run
        接口：
            trait
*/
fn main() {
    println!("猜数...");
    // let a = 1;
    // a = 2 //Cannot assign twice to immutable variable [E0384]
    //生成随机数（1-100）
    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("神秘数字：{}",secret_number);
    let mut guess = String::new();
    //读取输入值
    io::stdin().read_line(&mut guess).expect("无法读取行");
    // std::io::stdin().read_line(&mut guess).expect("无法读取行");
    println!("你猜测的数：{}",guess)
}
