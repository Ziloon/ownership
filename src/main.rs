fn main() {
    let mut s1 = String::from("Hello, world!");
    let len = cal_len(&mut s1);
    println!("The length of '{}' is {}.", s1, len);

    let s2: String = String::from("Ziloong");
    let len = cal_len_mv(s2);
    println!("The length {}.", len);

    let mut s = String::from("mutable string value");
    {
        let s1 = &mut s;
        println!("{}", s1);
    }
    let s2 = &mut s;
    println!("{}", s2);
    let s3 = & s; // 可变引用和不可变引用应该是不能在同一个作用域同时存在的，不知道 是不是后续版本修改了。
    println!("{}", s3);
}

fn cal_len(s:&mut String) -> usize {
    s.push_str(" Zhao");
    s.len()
}

fn cal_len_mv(s:String) -> usize {
    s.len()
}