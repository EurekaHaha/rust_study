#![allow(unused)]

// * 泛型需要再函数名后直接声明
fn generics<T>(list: &Vec<T>) -> &T {
    &list[0]
}