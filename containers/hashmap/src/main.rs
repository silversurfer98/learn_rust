use std::collections::HashMap;
fn main() {
    let mut hasm = HashMap::new();
    {
        // this string a only live within this scope
        let a = String::from("omale");
        
        // If I did this a ownership will be transfered to hashmap but the a goes out of scope
        // so error 
        // hasm.insert(1,a);

        //If I did this (giving a reference) and a goes out of scope hashmap will have no data
        // so error again
        // hasm.insert(1,&a);

        // the safest way is to clone and pass the value to hashmap
        hasm.insert(1,a.clone());
    }
    
    // now a interesting thing try to print a
    // out of scope
    // println!("{:#?}", a);
    
    // --- moving on to hashmap ---
    // let mut a = String::new();
    let a = hasm.entry(2).or_insert("otha".to_string());
    *a = "othaotha".to_string();
    // hasm.insert(2,"otha".to_string());

    println!("{:#?}", hasm);
}
