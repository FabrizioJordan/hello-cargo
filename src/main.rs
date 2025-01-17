#![allow(dead_code)]
mod arrays;
mod vectores;
mod if_else;
#[path = "exercises/exercise1.rs"] mod exercise1;
#[path = "exercises/exercise2.rs"] mod exercise2;
#[path = "exercises/exercise3.rs"] mod exercise3;
mod hash;

fn main() {

    // los tres primeros ejercicios son horribles

    println!("\n \n \n");
    //num();
    //emoji();
    //more_num();
    //tuple();
    //structs();
    //my_enum();

    // se crea la variable casual y se le asigna el valor "Lucas"
    //let casual = "Lucas";
    // se llama a la funcion goodbye y se le pasa la variable casual
    //goodbye(casual);

    //let num1: i8 = 2;
    //let num2: i8 = 3;
    //println!("La suma de {} + {} es: {}", num1, num2, calculadora_mas(num1, num2));

    //let num3: u32 = 4;
    //let num4: u32 = 3;
    //println!("La suma de {} + {} es: {}", num3, num4, si_suma_vale_5(num3, num4));

    // se llama a la funcion de otro archivo
    //exercise1::main();
    //arrays::main();
    //vectores::main();
    //exercise2::main();
    //if_else::main();
    //exercise3::main();
    hash::main();

}

fn num() {
    let shadow_num: i32 = 32;

    let shadow_num: i32 = shadow_num + 64;

    println!("The value of shadow_num is: {}", shadow_num);
}

fn emoji() {
    let emoji: char = '🦀';
    println!("The value of emoji is: {}", emoji);
}

fn more_num() {
    println!("The value of 1 + 9 is: {}", 1u32 + 9u32);
}

fn tuple() {
    let tuple = ("element", 2 ,true, 4.5);
    println!("The second value of the tuple is: {}", tuple.1);
}

fn structs(){

    // classic struct
    struct Student {
        name: String,
        level: u8,
        remote: bool
    } 

    // tuple struct
    struct Grades(char, char, char, f32);

    // unit struct
    struct Unit;

    let estudiante = Student {
        name: String::from("Lucas"),
        level: 3,
        remote: true
    };

    let mis_grados = Grades('A', 'B', 'C', 3.75);

    println!("My Student level {}
    My Grades first type {}
    And the unit struct
    ", estudiante.level, mis_grados.0)


}

fn my_enum(){
    #[derive(Debug)]
    struct KeyPress(String, char);

    #[derive(Debug)]
    struct MouseClick(i32, i32);

    #[derive(Debug)]
    enum WebEvent {
        WELoad(bool),
        WEClick(MouseClick),
        WEKeys(KeyPress)
    }

    // LOAD
    let we_load = WebEvent::WELoad(true);

    println!("The web event load is: {:#?}", we_load);

    // CLICK

    let click = MouseClick(120, 320);
    let we_click = WebEvent::WEClick(click);
    
    println!("The web event click is: {:#?}", we_click);

    // KEYS

    let keys = KeyPress(String::from("Ctrl + C"), 'C');
    let we_keys = WebEvent::WEKeys(keys);

    println!("The web event keys is: {:#?}", we_keys);
}


fn otro_enum(){
/* ¿Qué es un enum? 

Un enum es un tipo de dato, sirve para tener sus posibles variantes, o también llamados conjuntos de valores fijos.

Es un tipo de dato personalizado, por lo cual todas sus variantes pueden ser personalizadas.

Ejemplo: */

enum TipoDeComputador {
    PC,
    Laptop,
}

/* Vamos a seguir este ejemplo un poco más, como si tuvieramos dos opciones en una venta. */

struct VentaDeComputador {
    tipo: TipoDeComputador,
    valor: String,
}

let computador_sobremesa = VentaDeComputador{
    tipo: TipoDeComputador::PC,
    valor: String::from("12000")
};

let computador_laptop = VentaDeComputador{
    tipo: TipoDeComputador::Laptop,
    valor: String::from("6200")
};

}

fn goodbye(message: &str) {
    println!("Goodbye, {}", message);
}

// se asigna el tipo (ej i8) obligatoriamente en los parametros de la funcion
fn calculadora_mas(num1: i8, num2: i8) -> i8 {
    num1 + num2
}

// se asigna el tipo (ej u32) obligatoriamente para decir que la funcion devuelve un u32
fn si_suma_vale_5(num: u32, num2: u32) -> u32{
    // crear una variable que sume los dos numeros
    // si la suma es 5, imprimir "El numero es 5"
    // si la suma no es 5, imprimir "El numero no es 5"
    let suma = num + num2;
    if suma == 5 {
        println!("El numero es 5");
        return suma;
    } else {
        println!("El numero no es 5");
        return suma;
    }
}

// como llamo a otro archivo en rust?
// mod nombre_archivo;