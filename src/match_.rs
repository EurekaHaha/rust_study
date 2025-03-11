// 匹配模式
use std::array::*;

enum Test {
    A,
    B,
    C
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

fn test_0() {
    let t = Test::A;

    let a = match t {
        Test::A => String::from("A"),
        Test::B => String::from("B"),
        _ => String::from("C")
    };
    // * match表达式是一个表达式 所以可以直接赋值给一个变量
    // if let 是match的语法糖 下方代码等价于上方代码
    let a = if let Test::A = t {
        String::from("A")
    } else if let Test::B = t {
        String::from("B")
    } else {
        String::from("C")
    };
}

// 使用..忽略剩余的值
fn test_1() {
    // @ | i | i 是一个匿名函数 类似于js中的 (i) => i
    let vec_0: [i32; 5] = from_fn(|i| i as i32);
    match vec_0 {
        // 省略了后边的值
        [a, b, ..] => {
            println!("a = {}, b = {}", a, b);
        }
    }

    match vec_0 {
        // 省略了中间的值
        [a, .., b] => {
            println!("a = {}, b = {}", a, b);
        }
    }
}

// 使用@绑定值
fn test_2() {
    let number = 5;
    match number {
        // @绑定值
        n @ 0..=2 => {
            println!("n = {}", n);
        },
        n @ (3 | 4 | 5) => {
            println!("n = {}", n);
        },
        _ => {}
    }
}

// 使用@解构值
fn test_3() {
    // @ 绑定新变量 `p`，同时对 `Point` 进行解构
    let p @ Point {x: px, y: py } = Point {x: 10, y: 23};
    println!("x: {}, y: {}", px, py); // x: 10, y: 23
    println!("{:?}", p); // Point { x: 10, y: 23 }
}