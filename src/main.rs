#![allow(unused)]

mod ownership;
// mod reference;
mod lifetime;
mod generics;
mod slice;
mod error;
mod trait_;
mod test;
mod match_;
use std::fmt::Display;
use std::env;

fn main() {
    let args: Vec<String>  = env::args().collect();
    let config = Config::new(&args);
    let file_content = std::fs::read_to_string(config.file_path).expect("file not found");

    println!("With text:\n{file_content}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Self {
        if args.len() < 3 {
            panic!("not enough arguments");
        }
        Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
        }
    }
}



// ************************************************************************************************************** //


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

fn test_1_longest<'a, 'b>(s1: &'a str, s2: &'b str) -> &'a str {
    if s1.len() > s2.len() {
        return s1;
    } else {
        // ! 'b的生命周期可能比'a的生命周期短 所以这里会报错
        // s2
    }
    s1
}