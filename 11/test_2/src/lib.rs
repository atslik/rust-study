// 1. 如果你不希望测试并行运行，或者想要更加精确的控制线程的数量，可以传递 --test-threads 参数和希望使用线程的数量给测试二进制文件。
// 例如，要使用 2 个线程运行测试，你可以运行：
// cargo test --test-threads 2


// 2. 如果你希望也能看到通过的测试中打印的值，可以通过在末尾增加 --show-output 参数来告知 Rust 显示通过测试的输出：
// cargo test -- --show-output


// 3. 可运行单个测试
// 可以向 cargo test 传递任意测试的名称来只运行某个测试
// ex: cargo test one_hundred


// 4. 过滤运行多个测试
// 可以指定部分测试的名称，任何名称匹配这个名称的测试会被运行
// ex: cargo test add   （运行了所有名字中带有 add 的测试）


// 5. 忽略某些测试
// 在 #[test] 之后增加了 #[ignore] 行



fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}


fn add_two(a: i32) -> i32 {
    a + 2
}


//6. 测试私有函数
pub fn add_two2(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}



#[cfg(test)]
// 告诉 Rust 这是一个测试模块，应该在测试时编译
mod tests {
    use super::*;

    // #[test]
    // fn this_test_will_pass() {
    //     let result = prints_and_returns_10(4);
    //     assert_eq!(result, 10);
    // }


    // #[test]
    // fn this_test_will_fail() {
    //     let result = prints_and_returns_10(8);
    //     assert_eq!(result, 5);
    // }

// ============================ 示例2
    // #[test]
    // fn add_two_and_two() {
    //     let result = add_two(2);
    //     assert_eq!(result, 4);
    // }
    
    // #[test]
    // fn add_three_and_two() {
    //     assert_eq!(5, add_two(3));
    // }

    // #[test]
    // fn one_hundred() {
    //     assert_eq!(102, add_two(100));
    // }



    // ============================ 示例3
    // #[test]
    // fn add_two_and_two_private() {
    //     let result = internal_adder(2, 2);
    //     assert_eq!(result, 4);
    // }
}
