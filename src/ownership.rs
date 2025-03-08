#![allow(unused_variables)]
#![allow(unused_doc_comments)]

fn main_0() {
    let s0 = String::from("hello");
    let s1 = s0;

    // 赋值的时候会移动所有权 移动所有权后之前的变量的指针会被释放
    // println!("{} {}", s0, s1);
}


fn main_1() {
    let i0 = 5;
    let i1 = i0;

    /**
     * 可复制类型不会移交所有权 会直接复制
     * 可复制类型有: 整数类型 布尔类型 浮点数类型 字符类型 和内部都是可复制类型的元组
     */
    println!("i0 = {}, i1 = {}", i0, i1);
}

fn main_2() {
    let s = String::from("hello");

    // s的所有权移动到函数中 所以s在之后不能被使用
    temp_0(s);
}

fn main_3() {
    // temp_1 将返回值转移给s1
    let s1 = temp_1();

    let s2 = String::from("hello");

    // s2的所有权转移到temp_2中 temp_2返回一个所有权给到s3
    let s3 = temp_2(s2);
}
// 赋值给一个变量时，所有权转移到新变量上。 当拥有队中数据值的变量离开作用域时，其值将被丢弃

fn main_4() {

}



// **************************************//
fn temp_0(s: String) {
    println!("{}", s);
}

fn temp_1() -> String {
    String::from("hello")
}

fn temp_2(s: String) -> String {
    s
}