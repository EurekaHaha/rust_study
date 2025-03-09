
// 测试
use super::*;
// # 需要单独引入trait 不然就算引入了实现该trait的struct 也无法使用该trait
use crate::trait_::{A, GetSelf};

// * asset_eq! 宏用于比较两个值是否相等
// * assert_ne! 宏用于比较两个值是否不相等
// * assert! 宏用于判断一个表达式是否为true

// @ 使用assert类的宏时 其实调用的是 == 和 != 运算符 所以需要实现PartialEq和Debug trait

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn it_works() {
    let result = add(1, 2);
    assert_eq!(result, 3);
}

#[test]
fn test_0() {
    let get_self: A = A { a: 5 };
    assert_eq!(get_self.b(), 3);
}

#[test]
#[should_panic]
fn test_1() {
    error::is_panic(0);
}

#[test]
// * 期望panic
#[should_panic(expected = "a is negative")]
fn test_2() {
    error::is_panic_1(-1);
}
