// RcCell<T> 例子

// 消息接口
pub trait Messager{
    fn send(&self,msg: &str);
}

// 限制结构体（不同值实现的send不同，持有接口）
pub struct Checker<'a,T: Messager>{
    Messager:&'a T,
    value: usize,
    max: usize,
}

impl <'a,T: Messager> Checker<'a,T>
where T: Messager {
    // 初始化
    pub fn new (messager: &T,max:usize ) -> Checker<T>{
        Checker{
            Messager: messager,
            value: 0,
            max,
        }
    }

    // setValue
    pub fn set_value(&mut self,value: usize){
        self.value = value;
        let percent = self.value as f64 / self.max as f64;

        if percent >= 1.0{
            self.Messager.send("api使用次数已满");
        }else if percent >= 0.9{
            self.Messager.send("api使用次数已大于90%");
        }else if percent >= 0.75 {
            self.Messager.send("api使用次数已大于75%");
        }
    }
}

// 测试mod
#[cfg(test)]
mod tests{
    use std::cell::RefCell;
    use super::*;

    // 测试消息接口
    struct MockMessager{
        sent_msgs: RefCell<Vec<String>>,
    }

    impl MockMessager{
        fn new() -> MockMessager{
            MockMessager{
                sent_msgs: RefCell::new(vec![]),
            }
        }
    }
    impl Messager for MockMessager{
        fn send(&self,msg: &str){
            // msg 放到 sent_msgs
            self.sent_msgs.borrow_mut().push(String::from(msg));
        }
    }

    // 测试
    #[test]
    fn test_checker(){
        let mock = MockMessager::new();
        // 传入检查器
        let mut checker = Checker::new(&mock, 100);
        checker.set_value(50);
    }
}