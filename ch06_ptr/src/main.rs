use std::ops::Deref;
use std::rc::Rc;

fn main() {
    // println!("Hello, world!");
    // use_box();
    // use_list()
    // reference();
    // box_and_deref();
    // test_my_box();
    // let drop_struct = DropStruct { data: String::from("hello") };
    // let drop_struct2 = DropStruct { data: String::from("drop") };
    // println!("作用域要结束了")

    test_rc();

}

// 1. box<T>的使用
fn use_box() {
    // 在堆上分配一个值5
    let b = Box::new(5);
    println!("b = {}", b);
} // 作用域结束，b被释放，堆上的数据也释放


// 2. box<T>和递归变量
// 枚举
#[derive(Debug)]
enum List {
    // 这里的Cons包含一个指针，指向下一个元素
    // 元组结构体，可以包含任意多个元素
    // !不能直接使用
    Cons(i32, Box<List>),
    Nil,
}

// 函数：初始化和打印List
fn use_list() {
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));
    println!("{:?}", list)
}

// 3. 引用和解引用
fn reference() {
    // 引用
    let x = 5;
    // &x是引用，它不拥有x的所有权
    let y = &x;

    assert_eq!(5, x);

    // 解引用
    assert_eq!(5, *y);
}

// Box<T> 和 解引用
fn box_and_deref() {
    let x = 5;
    let y = Box::new(x);
    // 解引用
    assert_eq!(5, x);
    assert_eq!(5, *y);
}

// 4. 自定义Box类型
struct MyBox<T>(T);

// 实现 Deref
impl<T> Deref for MyBox<T> {
    // 指向T
    type Target = T;
    // 返回一个指向内部数据 `T` 的引用
    fn deref(&self) -> &T {
        &self.0
    }
}

// 测试结构体 MyBox

fn test_my_box(){
    let x = 5;
    let y = MyBox(x);
    assert_eq!(*y,5);
}

// 5. Drop接口
struct DropStruct{
    data: String,
}
impl Drop for DropStruct{
    fn drop(&mut self){
        println!("作用域结束，drop方法执行");
    }
}

// 6. Rc<T>
#[derive(Debug)]
enum RCList {
    Cons(i32,Rc<RCList>),
    Nil,
}
fn test_rc(){
    let a = Rc::new(RCList::Cons(5, Rc::new(RCList::Cons(10, Rc::new(RCList::Nil)))));
    // b c 引用 a
    let b = RCList::Cons(3, Rc::clone(&a));
    let c = RCList::Cons(4, Rc::clone(&a));
    // 打印 bc
    println!("b:{:?}", b);
    println!("c:{:?}", c);
}