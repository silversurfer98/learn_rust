// https://www.youtube.com/watch?v=T0Xfltu4h3A&list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8&index=11

#[derive(Debug)]
enum PresUom{
    barG(f32),
    barA(f32),
    atmG(f32),
    atmA(f32),
    kgfG(f32),
    kgfA(f32),
}

#[derive(Debug)]
enum tempUom{
    kelvin(f32),
    cel(f32),
    fuck(f32),
}

#[derive(Debug)]
struct gas<T, U, V>{
    name: T,
    pres: U,
    temp: V,
}

impl<T, U, V> gas<T, U, V>{
    fn change_UOM(&mut self){
        println!("omale");
    }
}

fn main() {
    let mut hydrogen = gas{
        name: "Hydrogen".to_string(),
        pres: PresUom::barG(1.2),
        temp: tempUom::kelvin(280.0),
    };

    hydrogen.pres = PresUom::barA(1.22);
    hydrogen.change_UOM();

    println!("{:#?}", hydrogen);
}
