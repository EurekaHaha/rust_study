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