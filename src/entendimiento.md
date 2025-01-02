### Para entender todo esto:

& es una referencia a una variable, pero la referencia es inmutable

Ejemplo:

´
let x = 10;
let y = &x;

// si se hace un println! de los dos
// tanto "x" como "y" darán 10, pero "y" no se puede mutar
´


# Funciones

Creación de una función.
Las funciones se crean con la palabra clave "fn".

´
fn main(){}
´

Para crear la función main.


# Importaciones y funciones públicas

Para importar y poder ejecutar una función de otro archivo se debe hacer pública dicha función a usar.

Si en un archivo tenemos la función main y la queremos ejecutar desde el archivo principal (main) debemos hacer esto:

1. Debemos volver pública la función.
   Así se hace una función pública:
   ´
   pub(crate) fn nombre_función(){}
   ´
2. Luego debemos añadir el archivo donde está la función pública.
   ´
   mod nombre_archivo;
   ´
   Si se necesita un archiv que está en una subcarpeta:
   ´
   #[path = "subcarpeta/archivo.rs"] mod nombre_función;
   ´
3. Ahora se debe ejecutar la función.
   Se puede ejecutar la función desde cualquier parte del código.
   ´
   nombre_archivo::nombre_función;
   ´

Aquí te dejo un ejemplo:

´
// se añade el archivo y la función
#[path = "exercises/exercise3.rs"] mod exercise3;
// se ejecuta la función
exercise3::main();
´


# Bucles, flujos de control, etc.

Hay varios bucles que conozco.
Loop, While y For.


Loop: Bucle infinito.

Aunque loop es un bucle infinito, este tiene la opción de parar.
Para parar un loop se usa la instrucción "break;"

Pueden haber varios "break" en todo el loop (aunque no tendrían sentido si no se usan dentro de condicionales)
Si un sólo "break" devuelve un valor (cómo lo hace return, osea "break valor;") entonces todos los otros deben hacerlo.
Si no se devuelve nada de forma explícita, entonces el programa lo hará. El programa devuelve cómo mínimo una tupla vacía, osea ().

La sintaxis de loop es:
´
loop {
    // código
}
´


While: Expresión condicional.

While funciona con condicional, por lo cual se ejecuta si la 

La sintaxis de while es:
´
while condición {
    // código
}
´

For: Iteraciones.

For funciona de dos formas. Siempre se ejecuta un número determinado de veces.
Se puede iterar sobre una colección de elementos, osea sobre una matriz, un vector o un mapa hash.
En estos casos, se usa una variable temporal cómo en otros lenguajes para almacenar el valor de cada iteracion.

Ejemplo 1:
´
let big_birds = ["ostrich", "peacock", "stork", "heron", "flamingo"];
    for bird in big_birds.iter(){
        println!("The bird is: {}", bird);
    }
´
Se usa ".iter()" para iterar entre los elementos.
Y la variable temporal es "bird"


Otra forma de iterar es sobre un rango.
Se usa ".." para crear un rango abierto, osea que no incluye el valor final.
Y se usa "..=" para crear un rango cerrado, el cual sí incluye el valor final.

´
// con ..
for number in 1..6{ // comienza en 1 y termina en 5
        println!("The number is: {}", number);
    }

// con ..=
for number in 1..=6{ // comienza en 1 y termina en 6
    println!("The number is: {}", number);
}
´



Si buscas los bucles for, while y loop están en:
src > control