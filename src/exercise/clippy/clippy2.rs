fn main() {
    let mut res = 42;
    let option: Option<i32> = Some(12);
    for x in option{
        println!("{}",x);
        res += x;

    }
    println!("{}",res);
}
