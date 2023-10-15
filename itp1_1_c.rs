use std::io;

fn main(){
    let mut buf = String::new();

    io::stdin()
        .read_line(&mut buf)
        .expect("Failed read");
    let binding = buf.trim().split_whitespace().collect::<Vec<&str>>();
    let mut sum:i32 = 0;
    let mut mul:i32 = 1;
    binding.iter().for_each(|v| {
        let parsed_v  = v.parse::<i32>().unwrap();
        sum = sum+parsed_v;
        mul = mul*parsed_v;
        
    });
   

    println!("{:?} {:?}", mul,sum*2)


}
