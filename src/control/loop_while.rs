fn main(){
    // Bucles loop y while.
    loops();
    sobre_while();
}

fn loops(){
                // Loop

    // loop es un bucle infinito, 
    // no tiene forma de parar menos con break
    // o con continue, que hace parecido a break, pero en vez de parar todo el loop
    // lo que hace es parar la actual iteración e ir a la siguiente

    // loop simple, no para nunca
    loop{
        println!("Esto es un bucle infinito");
    }

    // loop más complejo, para si se llega a break;

    let mut counter = 1;
    let stop_loop = loop{
        counter *= 2;
        if counter > 100 {
            break counter;
        }
    };

    println!("El bucle se detuvo en: {}", stop_loop);

    // loop medio simple (con continue)

    // las dos siguientes líneas son para conseguir un número al azar
    use rand::Rng;
    let azar_numero = rand::thread_rng().gen_range(1..=100);
    
    let mut num_actual = 0;
    loop { 
        println!("El número actual es: {}", num_actual);
        if num_actual == azar_numero {
            continue;
        }
        num_actual += 1;
    }

}

fn sobre_while(){

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