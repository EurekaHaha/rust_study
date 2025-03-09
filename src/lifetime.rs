// Borrow Checker
// 确保数据存活时间比引用长 如果短于引用则会出现悬垂引用 报错

use std::fmt::Display;

// 生命周期注解
fn t() {
//    let r;              // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;   // -+-- 'b  |  
//        r = &x;         //  |       |
    }                     // -+       |
                          //          |
    println!("r");        //          |
}                         // ---------+

// 生命周期注解的语法

// 生命周期注解不会改变引用的存活时间 只是描述多个引用之间的生命周期关系
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// struct 添加生命周期注解
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn t_1() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .unwrap();
    let i = ImportantExcerpt { part: first_sentence };
}
/*
报错 不知道x和y的生命周期关系
fn t_2(x: &str, y: &str) -> &str {
}
*/

// 静态生命周期
fn t_2() {
    // 整个程序的持续时间内都存活
    // @ 字符串字面值是一个拥有静态生命周期的引用
    let s: &'static str = "I have a static lifetime.";
}

// 泛型、生命周期和trait一起使用的例子
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}