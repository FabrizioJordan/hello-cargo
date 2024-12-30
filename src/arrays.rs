pub(crate) fn main(){
    array1();
    array_days();
}

#[allow(unused_variables)]
fn array1(){
    // hay dos formas de declarar arrays en rust (siempre con [] y con ;) 
    // los arrays son siempre de un solo tipo de dato a diferencia de la tupla
    // el tamaño de un array es fijo, no se puede cambiar

    // 1. 
    let days = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];

    // 2.
    // se crea el valor inicial,luego un punto y como y al final la longitud del array
    let bytes = [0; 5];
}

#[allow(unused_variables)]
fn array_days(){

    let days = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];

    let first_day = days[0];

    let second_day = days[1];
}