pub(crate) fn main(){
    // un hash es un tipo de dato que almacena pares de valores
    // se puede acceder a los valores a traves de una clave
    // HashMap<K, V> es un hash que almacena claves de tipo K y valores llamados V

    // los mapa hash se pueden aumentar, osea son mutables


    // se debe importar la libreria para usar HashMap

    use std::collections::HashMap;

    //           Ejemplo 1

    // se crea un nuevo HashMap
    let mut reviews: HashMap<String, String> = HashMap::new();

    // se insertan valores en el HashMap
    // usar insert es fácil, es insert(<clave>, <valor>)

    reviews.insert(String::from("Ancient Roman History"), String::from("Very accurate."));
    reviews.insert(String::from("Cooking with Rhubarb"), String::from("Sweet recipes."));
    reviews.insert(String::from("Programming in Rust"), String::from("Great examples."));


    // para obtener un valor de un HashMap se usa get

    let book: &str = "Ancient Roman History";
    let roman_history_review = reviews.get(book);
    println!("Review for {}: {:?}", book, roman_history_review);


    // para borrar un par clave-valor de un HashMap se usa remove
    // si se borra un valor y se usa get para obtenerlo, se obtiene None

    let obsolete: &str = "Cooking with Rhubarb";
    print!("\n'{}\' removed", obsolete);
    reviews.remove(obsolete);

    let rhubarb_review = reviews.get(obsolete);
    println!("\nReview for {}: {:?}", obsolete, rhubarb_review);


    //              Ejemplo 2

    // se crea un nuevo HashMap
    let mut ages = HashMap::new();

    ages.insert(String::from("Alice"), 30);
    ages.insert(String::from("Bob"), 25);
    ages.insert(String::from("Charlie"), 40);

    let name = String::from("Alice");
    match ages.get(&name) {
        Some(age) => println!("\n La edad de {} es {}", name, age),
        None => println!("\n No se encontró la edad de {}", name),
    }

    ages.insert(String::from("Alice"), 31);
    print!("La nueva edad de Alice es: {}", ages.get("Alice").unwrap());

    ages.remove(&String::from("Bob"));
    println!("\nBob ha sido eliminado. \nAhora hay {} personas en el HashMap", ages.len());
    println!("El HashMap es: {:?}", ages);

    for (key, value) in &ages {
        print!("{}: {}", key, value);
    }

}