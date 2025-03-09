#![allow(unused)]

// * 泛型需要再函数名后直接声明
fn generics<T>(list: &Vec<T>) -> &T {
    &list[0]
}

struct Point<T> {
    x: T,
    y: T,
}


// 只给String类型的Point实现x方法
impl Point<String> {
    fn x(&self) -> &String {
        &self.x
    }
}

// 给所有类型的Point实现y方法
impl<T> Point<T> {
    fn y(&self) -> &T {
        &self.y
    }
}

// * 不能为泛型和非泛型实现相同的方法

// @ rust中泛型不会比具体类型执行慢 因为rust在编译时会将泛型替换为具体类型

fn test_0() {
    let p_0 = Point { x: 5, y: 10 };
    let p_1 = Point {
        x: String::from("hello"),
        y: String::from("world")
    };

    // p_0.x();
    // ! 报错 no method named `x` found for struct `Point<{integer}>` in the current scope field
    p_0.y();
    p_1.x();
    p_1.y();
}