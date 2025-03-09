use std::fs::File;

pub fn set_fs_error() {
    let file = File::open("hello.txt");
    match file {
        Ok(_) => {
            println!("get file!");
        },
        Err(err) => {
            panic!("Error: {:?}", err);
        }
    }
    // todo ? operator 还没有学 需要学完trait之后再来学习
}

pub fn is_panic(a: i32) {
    if a == 0 {
        panic!("a is zero");
    }
    println!("a is not zero");
}

pub fn is_panic_1(a: i32) {
    if a < 0 {
        panic!("a is negative");
    }
    if a > 0 {
        panic!("a is positive");
    }
    print!("a is zero");
}