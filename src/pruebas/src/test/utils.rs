




fn main() {

    const PI:f64 = 3.142334465656565656;
    let tupla: (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = (10,13,1,2,3,4,5,5,6,77);
    let matriz: [&str; 2] =  ["1","3"];
    let valid: i32 = if true { 5 } else { 6 };

    println!("Ingrese un numero {PI} {tupla:?} {matriz:?} {valid}");

    loop {
        println!("Hola amigos");

        if  valid == 5 {
            break;
        }

    }

    let array:[i32; 9]= [1,2,3,4,5,6,7,8,9];
    let mut counter:usize = 0;


    for it in (-10..=10).rev() {
        println!("gg {}",it)
    }
     for n in array.iter() {

        println!("{}",n)
    }

    while counter < 9 {

        println!("El valor es: {}", array[counter]);
        counter +=1 ;

    }

    //loop bucles rompe con break


    //propiedad
    let mut longitud = String::from("Hollaa mundo");

    let y: usize = view_data(&mut longitud);
    println!("longitud {} len {}",longitud,y);


    let mut cadena_2 = String::from("Rust properties");
    let hola: &str = &cadena_2[0..4];
    let hol2: &str = &cadena_2[4..9];
    println!("vvv{} uuuu{}",hola,hol2);

    assert_eq!(hola,hol2);

   let _ =  number_byte(&cadena_2);
   cadena_2.clear();

   // tipos de datos genericos



   let vec_list_number: Vec<i32> = vec![1,2,3,4,5,6,7,8];
   
   let result1: i32 =  mayor_i32(&vec_list_number);
   println!("kkk {}",result1); 
   
   let lista_char: Vec<&str> = vec!["m","e","j","g","d"];
   let result2 =  mayor_char(&lista_char);


}



fn mayor_i32(list: &[i32]) -> i32{

    let mut  mayor_zero = list[0];

    for &item in list {
        if item > mayor_zero {
            mayor_zero = item;
        }
    }

    mayor_zero

}

fn mayor_char(lista_char: &[char]) -> char {

    let mut mayor = lista_char[0];

    
    for &item in lista_char {
        if item > mayor {
            mayor = item;
        }
    }

    mayor
}

fn view_data(cadena: &mut String) -> usize {

    println!("{}",cadena);
    let aux = cadena.len();
    return aux;
}



fn number_byte( cadena: &String) -> usize {

    let bytes = cadena.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {

        if item == b' ' {
            return i;
        }

    }

    cadena.len()
}