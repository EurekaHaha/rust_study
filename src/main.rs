mod ownership;
// mod reference;
mod generics;
mod slice;
mod error;


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