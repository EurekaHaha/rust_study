// @ 类似JAVA的接口和多态

use std::fmt::Debug;

pub struct A {
    pub a: i32,
}

// 实现普通的struct方法
impl A {
    fn a(&self) -> i32 {
        self.a
    }
}

pub trait GetSelf {
    // 默认签名
    fn a(&self) -> i32;
    // 默认实现
    fn b(&self) -> i32 {
        3
    }
}

pub trait GetOther {
    fn c<T: GetSelf>(&self, item: &T) -> i32{
        item.a()
    }
}

// # 实现trait方法需要至少类型或trait在当前作用域
// 实现trait方法
impl GetSelf for A {
    fn a(&self) -> i32 {
        self.a
    }
}

fn main() {

    let a = A { a: 5 };
    // 调用struct方法
    println!("{}", a.a());

    // @ A种包含了重写的trait方法 a 和默认的trait方法 b
    // 调用trait方法
    let a_trait: &dyn GetSelf = &a;
    println!("{}", a_trait.a());
}

// trait 作为参数
fn test_trait_as_param(item: &impl GetSelf) {
    println!("{}", item.a());
    println!("{}", item.b());
}

// trait Bound 可以用泛型来约束
fn test_trait_as_param_bound<T: GetSelf>(item_0: &T, item_1: &T) {
    println!("{}", item_0.a());
    println!("{}", item_1.b());
}

// 制定多个trait约束
fn test_trait_as_param_multi(item: &(impl GetSelf + GetOther)) {
    println!("{}", item.a());
    println!("{}", item.b());
    println!("{}", item.c(item));
}
fn test_trait_as_param_bound_multi<T: GetSelf + GetOther>(item_0: &T, item_1: &T) {
    println!("{}", item_0.a());
    println!("{}", item_1.b());
    println!("{}", item_1.c(item_0));
}

// 使用Where语法
fn test_trait_as_param_without_where<T: GetSelf + Clone, U: GetOther + Debug>(item_0: &T, item_1: &U) {
    println!("{}", item_0.a());
    println!("{}", item_1.c(item_0));
    println!("{:?}", item_1);
}
fn test_trait_as_param_with_where<T, U>(item_0: &T, item_1: &U) -> i32
    where T: GetSelf + Clone,
          U: GetOther + Debug 
{
    println!("{}", item_0.a());
    println!("{}", item_1.c(item_0));
    println!("{:?}", item_1);
    3
}

// trait 作为返回值
// # trait作为返回值的时候只能返回一种类型
fn test_trait_as_return() -> impl GetSelf {
    A { a: 5 }
}

// 使用trait有条件的实现方法
struct B<T> {
    a: T,
}

// 所有类型的B都有new方法
impl<T> B<T> {
    fn new(a: T) -> Self {
        Self { a }
    }
}

// @ 实现了GetSelf和GetOther的类型才有a方法
impl<T: GetSelf + GetOther> B<T> {
    fn a(&self) -> i32 {
        self.a.a()
    }
}

// blanket implementation
// @ 为所有实现GetSelf的类型实现GetOther
impl<T: GetSelf> GetOther for T {}