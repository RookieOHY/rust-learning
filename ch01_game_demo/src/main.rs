use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::{Error, ErrorKind, Read};
use std::{error, fs, io, vec};

/*
猜数游戏demo分析Rust语法：
    涉及成员：
        println!:
            宏（不是函数）
            格式化输出：
                1. {} 支持基本类
                2. {:?} debug 输出，支持结构体（需要#[derive(Debug)]标记）
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
        变量类型：
            - 默认的整形类型 i32
        借用和所有权
            - 借用 &var_name
            - 所有权 &mut var_name
        方法
            - 方法在结构体的上下文中被定义，第一个参数总是 self，代表结构体的实例
            - 可以选择将方法的名称与结构中的一个字段相同，让属性变成私有，仅提供堆外的同名方法
        关联函数
            - 属于结构体级别
        私有和公有
            -
*/
fn main() {
    // 变量
    // 元组
    let o = (20, 51);

    // 整形
    let w = 20;
    let h = 50;

    // 有参函数（参数为元组）的调用
    println!("面积（元组）：{}", han_shu_yuan_zhu(o));
    // 有参函数的调用
    println!("面积：{}", han_shu_you_can(w, h));
    // 无参函数调用
    han_shu();
    // 有参函数（参数是枚举类型）调用
    han_shu_enum(IpAddrKind::V4(String::from("qqq")));
    han_shu_enum(IpAddrKind::V6(String::from("qqq")));

    // 元组结构体的使用
    let yza = yuan_zu_A(0, 1, 2);
    let yzb = yuan_zu_B(0, 2);
    println!("{}", yza.1);
    println!("{}", yzb.0);
    // 类单元结构体的使用
    let ldy_use = ldy;
    // 普通结构体的使用
    let cfc = CFC {
        width: 20,
        height: 52,
    };
    let cfc_debug = CFC_Debug {
        width: 20,
        height: 53,
    };
    let cfc_debug02 = CFC_Debug {
        width: 19,
        height: 52,
    };
    let cfc_debug03 = CFC_Debug {
        width: 21,
        height: 54,
    };
    println!("面积（普通结构体）：{}", han_shu_ref_struct(&cfc));
    // 普通结构体的格式化输出
    println!("面积（结构体的格式化输出）：{:?}", cfc_debug); // CFC_Debug { width: 20, height: 53 }
                                                             // 普通结构体的格式化输出（易读）
    println!("面积（结构体的格式化输出（易读））：{:#?}", cfc_debug); // 类json展开
                                                                      // 方法的使用
    println!("面积（方法的使用）：{}", cfc_debug.mj());
    if cfc_debug.width() {
        println!("宽：{}", cfc_debug.width);
    }
    println!("比大小:{}", cfc_debug.compare(&cfc_debug02));
    println!("比大小:{}", cfc_debug.compare(&cfc_debug03));

    // 关联函数的使用
    let zfc = CFC_Debug::get_instance(3);
    println!("正方形：{:#?}", zfc);

    // 枚举的使用
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // 利用枚举适当替换结构体
    // let ip_struct_v4 = IpStruct { kind: IpAddrKind::V4, ip: String::from("127.0.0.1") };
    // let ip_struct_v6 = IpStruct { kind: IpAddrKind::V6, ip: String::from("::1") };
    let ip_enum_v4 = IpAddrKind::V4(String::from("127.0.0.1"));
    let ip_enum_v6 = IpAddrKind::V6(String::from("::1"));

    // 枚举方法调用
    let message = Message::Write(String::from("dy"));
    message.enum_method();

    // 内置枚举 Option 使用
    let some_num = Some(5);
    let some_str = Some("str");
    let null: Option<i8> = None;

    //嵌套枚举
    han_shu_match(ZM::D(SZ::ONE));

    //字符串 String

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // 格式化宏不会持有变量的所有权，用于拼接
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s is {}", s);

    //集合 Vector
    let vct_init = vec![1, 2, 3];
    println!("集合（vct_init）:{:?}", vct_init);
    // let v: Vec<i32> = Vec::new();
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(4);
    println!("集合（v）:{:?}", v);

    for me in &mut v {
        *me += 1;
    }
    println!("集合（v）:{:?}", v);

    let x: &i32 = &v[1];
    println!("元素（x）: {}", x);
    match v.get(1) {
        Some(x) => println!("get(1):{}", x),
        None => println!("error"),
    }
    // 读取一个索引的元素后请不要插入元素，会报错
    // Vector存不同类型数据
    // let v_enum = vec![Message::Quit, Message::Move { x: 0,  y: 0 }];

    //集合们 map
    let mut map = HashMap::new();
    map.insert(1, 2);
    println!("集合们（map）:{:?}", map);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("集合们（scores）:{:?}", scores);
    println!("集合们（get）: {:?}", scores.get(&String::from("Blue")));
    for (k, v) in scores {
        println!("{}-{}", k, v);
    }

    let mut scores_insert = HashMap::new();
    scores_insert.insert(String::from("Blue"), 10);
    scores_insert.entry(String::from("Yellow")).or_insert(50);
    scores_insert.entry(String::from("Blue")).or_insert(50);
    println!("集合们（scores_insert）：{:?}", scores_insert);

    let tong_ji_ci_shu = "hello world hello word";
    let mut c_s_map = HashMap::new();
    for e in tong_ji_ci_shu.split_whitespace() {
        let e_count = c_s_map.entry(e).or_insert(0);
        *e_count += 1;
    }
    println!("集合们（map统计单词次数）:{:?}", c_s_map);

    // 异常
    // yi_chang();
    // open_file();
    // open_file_unwrap();
    // open_file_expect();

    // 泛型
    let nums = vec![20, 10, 30];
    let chars = vec!['a', 'c', 'c'];
    println!("nums中最大的是：{}", get_largest(&nums));
    println!("chars中最大的是：{}", get_largest(&chars));

    //接口
    let s4 = S1 {
        x: String::from("zzz"),
        y: String::from("zzz"),
        z: String::from("zzz"),
    };
    let s5 = S2 {
        x: String::from("zzz"),
        y: String::from("zzz"),
        z: String::from("zzz"),
    };
    println!("{}", s4.jkm());
    println!("{}", s5.jkm());
    // 接口参数
    // jk_can_shu(s4);
    // 接口参数（impl转T）
    jk_can_shu_T(s4, s5);
}

// 接口
// 1. 定义公有接口
pub trait JK {
    fn jkm(&self) -> String;
}

// 2. 实现接口
pub struct S1 {
    pub x: String,
    pub y: String,
    pub z: String,
}

pub struct S2 {
    pub x: String,
    pub y: String,
    pub z: String,
}

impl JK for S1 {
    fn jkm(&self) -> String {
        format!("{}-{}-{}", self.x, self.y, self.z)
    }
}

impl JK for S2 {
    fn jkm(&self) -> String {
        format!("{}:{}:{}", self.x, self.y, self.z)
    }
}

// 结构体
// 1. 元组结构体
struct yuan_zu_A(i32, i32, i32);
struct yuan_zu_B(i32, i32);
// 2. 类单元结构体
struct ldy;
// 3. 普通结构体
struct CFC {
    width: u32,
    height: u32,
}
// 4. 普通结构体（调试标记 #[derive(Debug)]）
#[derive(Debug)]
struct CFC_Debug {
    width: u32,
    height: u32,
}

// 5.泛型结构体
struct TS<T> {
    f1: T,
    f2: T,
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
impl<T> TS<T> {
    fn m(&self) -> &T {
        &self.f1
    }
}

// 方法
impl CFC_Debug {
    fn mj(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn compare(&self, obj: &CFC_Debug) -> bool {
        self.width > obj.width && self.height > obj.height
    }

    // 关联函数
    fn get_instance(size: u32) -> CFC_Debug {
        CFC_Debug {
            width: size,
            height: size,
        }
    }
}

// 枚举
// 1. 利用枚举适当替换结构体
enum IpAddrKind {
    V6(String),
    V4(String),
}

struct IpStruct {
    kind: IpAddrKind,
    ip: String,
}
// 2. 枚举成员任意类型
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// 3.枚举方法
impl Message {
    fn enum_method(&self) {
        println!("我是enum_method output")
    }
}

// 4.枚举和match
enum ZM {
    A,
    B,
    C,
    D(SZ),
}
#[derive(Debug)]
enum SZ {
    ONE,
    TWO,
}

// 函数
// 1. 无参函数
fn han_shu() {
    println!("我是函数")
}
// 2. 有参函数
fn han_shu_you_can(w: u32, h: u32) -> u32 {
    w * h
}

// 3.有参函数（参数为元组）
fn han_shu_yuan_zhu(args: (u32, u32)) -> u32 {
    args.0 * args.1
}

// 4.有参函数（参数为结构体引用（只读不可变））
fn han_shu_ref_struct(args: &CFC) -> u32 {
    args.width * args.height
}

// 5.有参函数（结构体debug标记）
fn han_shu_struct_debug(args: &CFC_Debug) -> u32 {
    args.width * args.height
}

// 6.有参函数（参数是枚举类型）
fn han_shu_enum(args: IpAddrKind) {}

// 7.有参函数（和match）
fn han_shu_match(args: ZM) -> u8 {
    match args {
        ZM::A => {
            println!("对了");
            1
        }
        ZM::B => 2,
        ZM::C => 3,
        ZM::D(sz) => {
            println!("sz is {:?}!", sz);
            4
        }
    }
}

// 8. 异常宏
fn yi_chang() {
    panic!("boom!")
}

fn open_file() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("打开文件异常");
        }
    };
}

fn open_file_unwrap() {
    let f = File::open("hello.txt").unwrap();
}

fn open_file_expect() {
    let f = File::open("hello.txt").expect("打开异常");
}

// 判断类型处理
// fn error_solve(){
//     let f = File::open("hello.txt");
//     let f = match f {
//         Ok(file) => file,
//         Err(error) => match error.kind() {
//            ErrorKind::NotFound => match File::create("hello.txt") {
//                 Ok(fc) => fc,
//                Err(e) => panic!("其他错误类型:{:?}",e),
//            },
//             other_error => panic!("其他异常"),
//         },
//     };
// }

fn error_solve_jj() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("创建文件异常：{:?}", error);
            })
        } else {
            panic!("打开文件异常：{:?}", error);
        }
    });
}

// 异常传播
fn open_and_read_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut s = String::new();
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_fs() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// 泛型
fn get_largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut laegest = list[0];
    for &e in list.iter() {
        if e > laegest {
            laegest = e;
        }
    }
    laegest
}

fn jk_can_shu_T<T1: JK, T2: JK>(p0: T1, p1: T2) {
    println!("{}", p0.jkm());
    println!("{}", p1.jkm());
}

fn jk_can_shu(args: impl JK) {
    println!("{}", args.jkm());
}

// 泛型 where简化接口入参写法
fn fu_zha_duo_jk<T: Display + Clone, U: Clone + Debug>(t: T, u: U) {}

// 模块
mod w_mod_name {
    mod l_mod_1 {
        fn method() {}
        fn method2() {}
    }
    mod l_mod_2 {
        fn method() {}
        fn method2() {}
    }
}

// 函数调用模块
mod ren {
    pub mod a {
        pub fn method() {}
    }
}
pub fn ren_ren() {
    //绝对
    crate::ren::a::method();
    //相对
    ren::a::method();
}

// use 关键字 解决调用链写法过长，use引入模块
mod m1 {
    pub mod m11 {
        pub fn m11_method() {}
    }
}

// use crate::m1::m11;
use m1::m11;
pub fn diao_yong_zhe() {
    m11::m11_method();
}

fn main_01() {
    // println!("猜数...");
    println!("读取一个命令行输入的内容");
    let rand_number = rand::thread_rng().gen_range(1..101);
    println!("rand生成一个随机数：{}", rand_number);
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {}", guess);

        match guess.cmp(&rand_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
