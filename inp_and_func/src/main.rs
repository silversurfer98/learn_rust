use std::io;
fn main() 
{
    println!("Hello, world!");
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("omale");
    let y: u32 = x.trim().parse().unwrap();
    println!("{}", y);
    if y==1{
        println!("{}", func(1,2,true));
    }
}

fn func(x:u32,y:u32, z:bool) -> u32{
    if z{
        x+y
    }
    else{
        return x;
    }
}
