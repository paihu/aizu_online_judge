use std::io;

fn main(){
    let mut s_s =  String::new();
    io::stdin()
        .read_line(&mut s_s)
        .expect("failed to read");
    let s:i32 = s_s.trim().parse().unwrap();
    println!("{:?}:{:?}:{:?}",(s/3600),(s/60)%60,s%60)
}

