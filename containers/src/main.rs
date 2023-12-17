use std::io;
fn main() {
    println!("Hello, world!");

    // create a static array
    let a = [1.23, 1.32, 2.23, 5.65];
    // or this
    let a: [f32; 4] = [1.23, 1.32, 2.23, 5.65];
    println!("{:#?}",a);

    // vector
    // always mut for empty vec because we will push values to it
    let mut b:Vec<u32> = Vec::new();
    b.push(12);
    b.push(13);
    b.push(14);
    println!("{:#?}",b);

    // initializing vector using macro
    let c = vec![1,2,3,4,5];
    println!("{:#?}",c);

    println!("2nd element of array using unsafe cond -> {}", c[1]);

    // safe traverse of vector
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("correct ah adra");
    let x: usize = match x.trim().parse()
    {
        Ok(num) => num,
        Err(_) => 0,
    };
    match c.get(x){
        Some(num) => println!("the number at {} is {}", x, num),
        _ => println!("loosu koodhi"),
    }

    // looping through vector
    for i in &c{
        println!("{}", i);
    }

    // now a complex vector -> enum
    #[derive(Debug)]
    enum gas_prop{
        dewpt_pressure(f32),
        dewpt_temp(f32),
    }

    let gas1 = vec![gas_prop::dewpt_pressure(12.3), gas_prop::dewpt_temp(273.15), gas_prop::dewpt_temp(300.0)];
    println!("{:#?}", gas1);

    // use match staements to print only dewpt_temp
    for i in &gas1{
        match i{
            gas_prop::dewpt_temp(num) => println!("{}", num),
            _ => continue,
        }
    }

}
