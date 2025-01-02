fn main(){
    // todo sobre librerias

    // las librerias son archivos que contienen funciones y otros codigos
    // se pueden importar a otros archivos para usar sus funciones


    // para importar una libreria se usa la palabra clave use
    // use std::collections::HashMap;

    // std significa standard library
    // collections es un modulo de la libreria estandar
    // HashMap se encuentra en el modulo collections


    //           Ejemplo 1
    println!("Ejemplo 1");
    // se importa la libreria
    use std::collections::HashMap;

    // se crea un nuevo HashMap
    let mut reviews: HashMap<String, String> = HashMap::new();



}