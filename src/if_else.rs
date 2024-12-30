#![allow(unused_variables)]
#![allow(unused_assignments)]
pub(crate) fn main(){
    // if else funciona igual que en JS
    // pero se puede usar en una variable

    let formal = true;

    let greeting = if formal{
        "Good day to you, madam."
    } else {
        "Hey! What's up?"
    };

    println!("{}", greeting);

    //

    // otra prueba es mutando una variable que fue inicializada sin valor
    // usamos else if también igual que en JS

    let num = 500;
    let out_of_range: bool;

    if num < 0 {
        out_of_range = true;
    } else if num == 0 {
        out_of_range = true;
    } else if num > 512 {
        out_of_range = true;
    }else {
        out_of_range = false;
    }
}