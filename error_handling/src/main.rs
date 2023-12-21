use std::fs::File;
use std::io::{ErrorKind, Read};
use std::io;


fn main() {
    // how we can use panic macro
    a(0);

    // open a file
    let f = File::open("test.txt");
    
    // this will give file not found error
    // match f{
    //     Err(err) => {
    //         println!("the error is {}", err.to_string());
    //         panic!("cholamutha pocha");
    //     },
    //     _ => println!("watha"),
    // }
    
    // higher lvl rust baby
    let _f = match f{
        // returns the file to f
        Ok(file) => file,
        Err(err) => match err.kind(){
            // easy way is to call unwrap()
            // ErrorKind::NotFound => File::create("test.txt").unwrap(),
            ErrorKind::NotFound => match File::create("test.txt"){
                Ok(fi) => fi,
                Err(e) => panic!("cant even creata a file fuck {:?}", e),
            },
            other_err => panic!("IDK {:?}", other_err)
        }
    };

    // we can do all the prev things with unwrap or use except
    let _f = File::open("test.txt").unwrap();
    let _f = File::open("test.txt").expect("thorakka mudiyala");

    // moving on to error peopagation, how we get errors from a func
    let _ans = read_file("test.txt".to_string());
    println!("{:?}", _ans);


}

// same example -- opening a file
fn read_file(a :String) -> Result<String, io::Error>{
    let mut s = String::new();
    let mut f = File::open(a)?;
    // instead of having a match and returning we can put 
    // ? so error is automatically gets returned
    f.read_to_string(&mut s)?;

    // this ? operator can only be used in func
    return Ok(s);
}







fn a(ch: i32){
    if ch==1{
        panic!("wammale");
    }
}
