use std::fs::File;
use std::io::ErrorKind;


fn main() {
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
    let f = match f{
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


    a(0);
}

fn a(ch: i32){
    if ch==1{
        panic!("wammale");
    }
}
