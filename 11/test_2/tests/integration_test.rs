// 7. 集成测试

// 我们可以使用 cargo test --test integration_test 来运行集成测试
// 表示指定测试模块 integration_test.rs 运行测试


// 7.1 集成测试中的子模块
// 创建 tests/common/mod.rs 
// 这样命名告诉 Rust 不要将 common 看作一个集成测试文件

use test_2;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, test_2::add_two2(2));
}
