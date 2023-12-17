use std::io;
use colored::*;
// use std::cmp::Ordering;
fn main() {
    println!("Hello, world!");
    
    loop{
        let mut x = String::new();
        io::stdin().read_line(&mut x).expect("loosu koodhi");
        println!("{}", x);
        let x: u32 = match x.trim().parse()
        {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("{} {}", x, "omale".red());
        break;
    }
}
