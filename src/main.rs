#![allow(unused)]

mod ownership;
// mod reference;
mod generics;
mod slice;
mod error;
mod trait_;
use std::fmt::Display;

fn main() {
    // let mut s = String::from("hello");
    // let t = &mut s;
    //
    // t.push('2');
    //
    // println!("{}", t);

    // let mut s = String::from("hello");
    // let s2 = &s;
    // let s3 = &s;

    // let mut hash_map = 

    // s3.push_str(" world");

    // println!("{}", s2);
    error::set_fs_error();

}

fn get_first(v: &Vec<String>) -> &str {
    &v[0]
}

fn test() {
    let mut str = vec![String::from("A"), String::from("B")];

    let first = get_first(&str);

    if first.len() > 0 {
        str.push(String::from(first));
    }
}

fn displayable<T: Display>(t: T) -> impl Display {
    t
}

fn test_0() {
    let s = String::from("hello");
    let mut s1 = displayable(s);
    // ! s1是displayable的返回值 只实现了Display trait 所以只能调用Display trait的方法
    // s1.push_str(" world");
    println!("{}", s1);
}