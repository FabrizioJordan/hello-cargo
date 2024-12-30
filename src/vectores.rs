
pub(crate) fn main(){
    vectores1();
    vectores2();
    vectores3();
    vector_error(); 
}

// los vectores son parecidos a los arrays
// pero un vector puede cambiar el tamaño o longitud ( para más o para menos)

// en tiempo de compilación rust no sabe el tamaño de un vector 
// por lo que no saldrá ningún error de "out of bounds" en tiempo de compilación (que se está intentando acceder a un indice que no existe)

fn vectores1(){

    // para crear un vector se puede usar la sintaxis <vector><T>, donde T es un tipo de dato genérico no conocido, sería como un relleno

    // para crear un vector se usa la macro vec!, donde dentro de los siguientes [] se pueden colocar los valores del vector

    // se pueden crear vectores de la misma forma que arrays (<vector>[<tipo de dato>;<longitud>] o <vector>[<datos>])
    let three_nums = vec![15, 6, 95];
    println!("Initial vector: {:?}", three_nums);

    let zeroes = vec![0; 5];
    println!("Zeroes: {:?}", zeroes);
}

fn vectores2(){
    // otra forma de crear un vector es con Vec::new()

    let mut familia = Vec::new();

    // para introducir valores en un vector se usa el método push()
    // y para quitar valores se usa el método pop()

    familia.push("Papá");
    familia.push("Mamá");
    familia.push("Hermano N");
    familia.push("Hermano N+1");
    println!("Familia: {:?}", familia);

    // en un vector no se pueden agregar datos que sean de otro tipo al ya inpuesto

    // para quitar usamos pop()
    println!("Pop: {:?}", familia.pop());
    println!("Familia: {:?}", familia); 
}

fn vectores3(){
    // para acceder a un valor de un vector se usa la misma sintaxis que en un array

    let mut index_vec = vec![4, 20, 3, 304];
    let three = index_vec[2];
    println!("Vector: {:?}, three: {}", index_vec, three);

    

    // dada la mutabilidad del vector se puede cambiar un valor de un vector

    index_vec[2] = index_vec[2] + 10;

    // pero aún mutando un valor 
    // si se declara una variable con el valor de un indice de un vector 
    // y se cambia el valor del vector la variable no cambiará
    println!("Vector: {:?}, three: {}", index_vec, three);
    // aún cambiando la posición 2 del vector, la variable three no cambia y sigue siendo 3
    
    
    // esto es porque three es una copia del valor de la posición 2 del vector
    // y no una referencia a la posición 2 del vector
}

#[allow(unused_mut)]
fn vector_error(){
    // si se intenta acceder a un indice que no existe en un vector
    // rust no lanzará un error en tiempo de compilación
    // pero si en tiempo de ejecución
    
    let mut index_vec = vec![4, 20, 3, 304];
    //index_vec.pop();
    println!("Vector: {:?}, index nro 3 {}", index_vec, index_vec[3]);

    // si se intenta acceder a un indice que no existe en un vector 
    // rust entra en panico y se cierra el programa
}