fn main(){
    // for es un bucle que se ejecuta un numero determinado de veces
    // se usa para iterar sobre una coleccion de elementos


    // en rust se puede iterar sobre una matriz, un vector o un mapa hash

    // se usa una variable temporal cómo en otros lenguajes para almacenar el valor de cada iteracion

    // se puede usar iter() para iterar sobre un vector o una matriz
    let big_birds = ["ostrich", "peacock", "stork", "heron", "flamingo"];
    for bird in big_birds.iter(){
        println!("The bird is: {}", bird);
    }

    // se puede usar iter_mut() para iterar sobre un vector o una matriz y modificar los valores




    // otra forma de crear un iterador es con un rango (notación de rango)
    
    // se usa .. para crear un rango abierto
    // rango abierto = no incluye el valor final

    // se usa ..= para crear un rango cerrado
    // rango cerrado = incluye el valor final

    for number in 1..6{ // comienza en 1 y termina en 5
        println!("The number is: {}", number);
    }
    // con ..=
    for number in 1..=6{ // comienza en 1 y termina en 6
        println!("The number is: {}", number);
    }


    
}