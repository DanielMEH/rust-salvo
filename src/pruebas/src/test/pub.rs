
pub  trait Resumen {
    fn resumir(&mut self) -> String;
}

pub struct  Noticias {
    
    pub titular: String,
    pub localidad: String,
}

impl Resumen for Noticias {

    fn resumir(&mut self) -> String {
        format!("{} por {}", self.titular, self.localidad)
    }
    
}

pub struct Tweet {
   pub user:String,
   pub content:String,
   pub response:bool,
}

// traists interfaces como otros lenguajes como java
fn main() {
    
}