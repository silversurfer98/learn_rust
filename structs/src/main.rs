#[derive(Debug)] // inorder to print the whole struct
struct Data {
    present_val: f32,
    past_val: f32,
    dx: f32,
}

impl Data{
    fn derivative(&mut self)
    {
        self.dx = self.present_val - self.past_val;
    }

    fn _derivative(&self) -> f32{
        self.present_val - self.past_val
    }
}

fn main() {
    let mut d = Data{
        present_val: 10.0,
        past_val: 20.0,
        dx: 0.0
    };
    d.derivative();
    println!("{:?}", d.dx);

    let d2 = Data{
        present_val: 20.0,
        past_val: 20.0,
        ..d
    };
    println!("{}", d2._derivative());
     println!("{:#?}", d)
}
