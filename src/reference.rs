fn main() {
    //  引用和借用的区别
    let mut s = String::from("hello");

    // 获得变量指针就是引用 用&符号 之前的引用还有读取权限
    let t = &s;

    // 通过&mut获得可变引用 当获得可变引用时就会变成独占引用 之前的引用就会失效
    let mt = &mut s;

    println!("{}", s);

    // 将引用传递给函数作为参数是借用 借用只临时提供读取权限 而不会移交所有权

    println!("{}", t);

    let v = vec![String::from("A"), String::from("B")];
    let v_0 = &v[0];
}

fn main_0() {
    let s1 = String::from("hello");

    // &s1 是s1的引用 通过引用传递给函数作为参数是借用 借用只提供读取权限 而不会移交所有权
    let len = get_len(&s1);

    // 由于s1的所有权还在s1上 所以s1还能继续使用
    println!("The length of '{}' is {}.", s1, len);
}

fn main_1() {
    let mut s1 = String::from("hello");

    // 如果需要修改变量的值 可以通过可变引用传递给函数
    change(&mut s1);
}

fn main_2() {
    let mut s = String::from("hello");

    // 在特定作用域中某个数据的可变引用只能有一个
    let r1 = &mut s;
    // 此处在编译编译时会报错
    let r2 = &mut s;

    println!("{} and {}", r1, r2);

}

fn main_3() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    }

    let r2 = &mut s;

    // 在定义r2的时候 r1已经被释放 所以此处不会报错
    println!("{} and {}", r2, s);
}

fn main_4() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s;

    // 此时报错 不能同时存在可变和不可变的引用
    println!("{} , {} and {}", r1, r2, r3);

    {
        let r1 = &s;
        let r2 = &s;

        let r3 = &mut s;

        // 此时正确 因为不再使用r1 和 r2
        println!("{}", r3);
    }
}

// 悬垂引用
fn main_5() -> &String {

    // 此处返回一个引用 但是s在函数结束后会被释放 所以会报错
    let s = String::from("hello");
    &s
}

// * 引用必须是有效的
// * 在任意给定的时间内 要么只能有一个可变引用 要么只能有多个不可变引用

// *********************************************** //

fn get_len(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}