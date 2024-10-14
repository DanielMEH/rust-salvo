

struct Punto<T, U> {

    x: T,
    y: U,
}


impl<T,U> Punto<T,U> {
    
    fn x(&self) -> &T{

        &self.x
    }

    fn y(&self) -> &U{
        &self.y
    }
}
// impl Punto<f32,f32> {
    
//     fn implement_decimal(&self) -> f32{
//         (&self.x.powi(2) + &self.y.powi(2)).sqrt()

//     }
// }

fn main() {

    let punt = Punto {x:10,y:1};

    println!("000{}",punt.x());
    println!("000{}",punt.y());
    
    // enum  Options<T> {
    //     Some(T),
    //     None,
    // }   

    // enum Result<T,E>{
    //     Ok(T),
    //     Err(E)
    // }


}