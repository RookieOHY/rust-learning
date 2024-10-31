// rust的迭代器
fn main() {
    // println!("Hello, world!");
    iter();
}

fn iter(){
    let v1 = vec![1,2,3];
    let mut v1_iter = v1.iter();
    for val in v1_iter{
        println!("{}",val);
    }
    // 断言
    // assert_ne!(v1_iter.next(), Some(&1))
}

