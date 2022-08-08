use integration;
mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, integration::add_two(2))
}

//Doc-tests 之前是集成测试的输出

//cargo test --test integration_test //执行特定文件下的集成测试函数