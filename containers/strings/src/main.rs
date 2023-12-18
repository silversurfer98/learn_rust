// strings are complex, because it uses UTF-8 not ASCII
// UTF-8 is backword compatible with ASCII

/* ASCII uses one byte to store a character
which means we can only recogonize 128 characters
BUT -->
UTF8 uses variable length of bytes to store a character which means
size of a character can range from 1 to 4 bytes
so using 4 bytes we can have lot more charaters 
--------- IMPORTANT ----------
Since we are dealing with bigger and variable byte sizes
we cannot index string with data[0] as the first byte might be 1 byte length and second may be 3 byte length*/
use unicode_segmentation::UnicodeSegmentation;
fn main() {
    let x = String::from("ஒத்தா");

    println!("String {} as bytes", x);
    // intrepreting the string as bytes
    for i in x.bytes(){
        println!("{}",i);
    }


    println!("String {} as scalar chars", x);
    // intrepreting string as scalar values
    for i in x.chars(){
        println!("{}",i);
    }

    println!("String {} as human readable char", x);
    // to reperesent as human readable char we need a external lib
    // unicode-segmentation="1.7.1"
    // and seperate char as human readable is grapheme

    for i in x.graphemes(true){
        println!("{}",i);
    }

}
