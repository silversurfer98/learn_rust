#[derive(Debug)]
enum data{
    present(f32),
    past(f32),
    dx(f32)
}

// enums are just variants, not like struct
// its just for labelling data and grouping it under a common name

fn main() {
    let x = data::present(20.0);
    let y = data::past(200.0);
    let z = data::dx(2.5);
    
    println!("{:#?}", x);

    // using a enum option, it is used to have null
    /*
    enum Option<T>{
        Some(T),
        None,
    }
     */

    let y = Some(5);
    let z: Option<i32> = None;

    // uisng this with sanity
    let a = 20;
    // let b = Some(10);
    let b = None;
    // not possible because b is a Option and can have a None
    let c = a+b.unwrap_or(0);
    println!("{}", c);

    // match with option
    let d: Option<f32> = Some(2.5);
    match d{
        Some(num) => {
            println!("omale");
        }
        // all other than above will get directed here
        _ => {
            println!("otha");
        }

    }
}

