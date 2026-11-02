pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // can_hold 方法返回一个布尔值，这意味着它完美符合 assert! 宏的使用场景
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}


pub fn greeting(name: &str) -> String {
    String::from("Hello!")
}


// 5
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        // if value < 1 {
        //     panic!("Guess value must be greater than or equal to 1, got {}.",
        //            value);
        // } else if value > 100 {
        //     panic!("Guess value must be less than or equal to 100, got {}.",
        //            value);
        // }

        if value < 1 {
            panic!("Guess value must be less than or equal to 100, got {}.",
                   value);
        } else if value > 100 {
            panic!("Guess value must be greater than or equal to 1, got {}.",
                   value);
        }

        Guess {
            value
        }
    }
}
// 5



// cargo test

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // 每一个测试都在一个新线程中运行，当主线程发现测试线程异常了，就将对应测试标记为失败。
    #[test]
    fn another() {
        // panic!("Make this test fail");
    }

    // 2. 使用 assert! 宏来检查结果
    // 需要向 assert! 宏提供一个求值为布尔值的参数。
    // 如果值是 true，assert! 什么也不做，同时测试会通过。
    // 如果值为 false，assert! 调用 panic! 宏，这会导致测试失败。

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }


    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(!smaller.can_hold(&larger));
    }


    // 3. 使用 assert_eq! 和 assert_ne! 宏来测试相等
    // assert_ne!：确定值绝对 不会 是什么的时候，这个宏最有用处


    #[test]
    fn it_adds_two() {
        assert_eq!(add_two(2), 4);
    }



    // 4. 自定义失败信息
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        // assert!(
        //     result.contains("Carol"),
        //     "Greeting did not contain name, value was `{}`", result
        // );
    }



    // 5. 使用 should_panic 检查 panic
    #[test]
    // #[should_panic]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn guess_is_between_1_and_100() {
        Guess::new(200);
    }




    // 6. 将 Result<T, E> 用于测试
    #[test]
    fn it_works_with_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
