use std::io;

fn main(){
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("failed to read");
    let binding = buf.trim().split_whitespace().collect::<Vec<&str>>();
    let mut iter = binding.iter();
    let a  = iter.next().expect("0").parse::<i32>().unwrap();
    let b  = iter.next().expect("0").parse::<i32>().unwrap();

    if a<b {
        println!("a < b")
    } else if a>b {
        println!("a > b")
    }else{
        println!("a == b")
    }

    
}
