fn main(){
    // Bucles loop y while.


                // Loop

    // loop es un bucle infinito, 
    // no tiene forma de parar menos con break

    loop{
        println!("Esto es un bucle infinito");
    }

    //              Ejemplo

    let mut counter = 1;
    let stop_loop = loop{
        counter *= 2;
        if counter > 100 {
            break counter;
        }
    };

    println!("El bucle se detuvo en: {}", stop_loop);


                // While

    // while es un bucle que se ejecuta mientras una condicion sea verdadera
    // para while se usa una condición, true o false
    // osea que es siempre booleana. (aunque se puede usar cualquier tipo de dato, para while es booleano)

    let mut count = 0;
    while count < 5 {
        println!("El while sigue corriendo");
        println!("Contador: {}", count);
        count += 1;
    }


}