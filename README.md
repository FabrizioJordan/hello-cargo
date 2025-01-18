# Para entender todo esto:

## Compilar y ejecutar 

Rust es un lenguaje de compilación. Por ende se debe compilar antes para poder ejecutarse.

Para compilar:
```
rustc main.rs
```
Esto crea un archivo llamado main sin extensión.

Para ejecutar:
```
./main
```


## Declarar variables

En Rust, todas las variables son inmutables por defecto.
Osea que si se asigna un valor a una variable este valor no se puede volver a cambiar.
 
Variables inmutables por defecto:

```
let valor = 10;
```

¿Cómo se puede volver mutable una variable? Fácil, sólo hay que añadir un "mut" antes del nombre de la variable 

Variables mutables:
```
let mut valor2 = 10;
```


Ahora, algo que usa en Rust son las referencias a otra variable.

& es una referencia a una variable, pero la referencia es inmutable

Ejemplo:

```
let x = 10;
let y = &x;

// si se hace un println! de los dos
// tanto "x" como "y" darán 10, pero "y" no se puede mutar
```

<br>

## Tipos de variables (o dato)

Para aprender sobre inmutabilidad y mutabilidad (importante en Rust) ve al anterior punto (Declarar variables). 

<br>

### Números: 


Los tipos de variables numéricas se dividen en enteros y de punto flotante. 
Hay que recordar que Rust es un lenguaje explícito y extricto, por lo cual sí o sí las variables que usan un tipo de dato número deben usar estas opciones para ser seguro y tener el mejor rendimiento en el código.

<br>
Ejemplos:

<br>
Con "i" ;

<br>
i128 : (el "128" habla de ser un número de máximo 128 bits, no bytes)

<br>
i32 : (el "i" significa que el número puede ser positivo o negativo)

<br>
i32 : ("i32" puede almacenar valores desde -2,147,483,648 hasta 2,147,483,647),

<br>
isize :  (entero con signo cuyo tamaño depende de la arquitectura del sistema, si tu sistema es de 32 bits entonces este número será de 32 bits y lo mismo para 64 bits)

<br>
Con "u" ;

<br>
u32 : (la "u" significa sin signo, osea que sólo puede ser positivo)

u32 : ("u32" puede almacenar valores desde 0 hasta 4,294,967,295),

usize : (entero sin signo cuyo tamaño depende de la arquitectura del sistema, si tu sistema es de 32 bits entonces este número será de 32 bits y lo mismo para 64 bits)

<br>
Para crear variables con estos tipos;

i32:
```
let num1: i32 = -10
let num2: i32 = 10
```

u32:
```
let num3: u32 = 10
```

<br>


### Char y String (también &str):

#### Char:

Char sólo permite crear una variable de un único carácter.

Ejemplo:
```
let emoji: char = '🦀';
```
<br>


#### String y &str, ¿que signfica cada uno?

String (y &str) es un tipo de variable muy complejo.

String es un tipo de dato (aunque en realidad es una referencia) parecida a otros lenguajes, aunque puede "mutar" (en realidad es dinámica), osea que puede crecer y cambiar de tamaño. Por lo cual no es realmente mutable.

Ejemplos;

Para crear una cadena vacía de String:

```
let mut s = String::new();

// se usa mut ya que no tendría sentido crear una cadena de texto vacía inmutable 
```

```
let s = String::from("Hola");

// se convierte una cadena de texto literal a un tipo de dato String
```


#### &str (o string slice):

&str es una referencia a una cadena de texto, pero inmutable por defecto.
&str se utiliza comúnmente para trabajar con literales de cadena y para pasar cadenas de texto como referencias sin necesidad de copiar los datos.


Ejemplos;

```
let saludo: &str = "Hola, mundo!";
```


Acá un ejemplo de lo explicado, que &str es para trabajar con cedenas de texto que son referencias.

```
let s = String::from("Hola, mundo!");
let parte: &str = &s[0..4]; // "Hola"
```


### bool (o también llamado tipo booleano):

Casi igual a otros lenguajes, el tipo booleano puede ser "true" o "false".
Normalmente se usan para if, else, while (obligatorio) y match.


```
let es_verdadero: bool = true;
let es_falso: bool = false;
```

<br>

## Tipos de structs (y tuplas)
Algunos son cómo objetos en JS y otros lenguajes.
En esta parte también te dejo las tuplas.


### Tuplas:
Antes de seguir con el resto de estructuras de datos te dejo las tuplas (parecido a un objeto de JavaScript).

Las tuplas pueden contener elementos de diferentes tipos, estando agrupados juntos. Las tuplas son útiles para agrupar un pequeño número de valores heterogéneos.
Las tuplas se definen cómo cualquier variable, u objeto en caso de JavaScript.

```
let tupla: (i32, f64, bool) = (42, 3.14, true);
```

Las tuplas son sólo pedazos de código que se declaran y se usan (a diferencia de las estructuras de tupla).

Dejo un código como ejemplo de DOS formas que se tienen para acceder a los elementos dentro de una tupla.

```
// Acceder a los elementos de la tupla por su posición
let (x, y, z) = tupla;
println!("x: {}, y: {}, z: {}", x, y, z);

// También se puede acceder a los elementos usando índices
println!("El primer valor es: {}", tupla.0);
println!("El segundo valor es: {}", tupla.1);
println!("El tercer valor es: {}", tupla.2);
```


### Estructuras (structs):

Algunos tipos de structs son, structs clásicas, structs de tupla y structs unitarios. 
Las estructuras serían parecido a usar POO (programación orientada a objetos) en JavaScript u otros lenguajes. Por lo cual se crean instancias de estas structs.


Sobre todo voy a usar ejemplos para estos casos.


### Estructura de tipo Tupla:
Se acceden a los valores de la instancia por posición (tuple.0 = primer valor).

```
struct Color(u8, u8, u8);
```
Son cómo las tuplas pero tienen nombre y se definen cómo el resto de estructuras (struct).
Los campos de esta estructura no tienen nombre, sólo posiciones cómo las tuplas.

Son bastante útiles para usar un grupo de elementos (del mismo tipo) sin necesidad de nombres para cada campo.

Ejemplo 
```
struct Color(u8, u8, u8);

let rojo = Color(255, 0, 0);

// Acceder a los elementos de la estructura de tupla por su posición
println!("Rojo: {}, Verde: {}, Azul: {}", rojo.0, rojo.1, rojo.2);
```


### Estructura clásicas:
Campos con nombres. Muy parecidas a los objetos en otros lenguajes.
Se acceden a los valores de la instancia por campos con nombre.

Así se definen:
```
struct Persona {
    nombre: String,
    edad: u8,
    es_estudiante: bool,
}
```

Así se usan:
```
let persona = Persona {
    nombre: String::from("Alice"),
    edad: 30,
    es_estudiante: true,
};

println!("Nombre: {}", persona.nombre);
```


### Estructuras unitarias:
No tienen campos. Se utilizan sobre todo para implementar traits en tipos que no tienen datos asociados. Cómo esto es más complejo lo dejo para otra guía.

Se definen así:
```
struct Unit_Struct;
```

Ejemplo
Básicamente se usan para por ejemplo darles comportamientos específicos a diferentes roles de usuario.
```
struct Admin;
struct Guest;

// código más complejo dando compotamiento usando traits.
```

### Enum

<a id="item_enum"></a>

¿Qué es un enum? 

Un enum es un tipo de dato, sirve para tener sus posibles variantes, o también llamados conjuntos de valores fijos.

Es un tipo de dato personalizado, por lo cual todas sus variantes pueden ser personalizadas.

Ejemplo:

```
enum TipoDeComputador {
    PC,
    Laptop,
}
```


## Funciones

Creación de una función.
Las funciones se crean con la palabra clave "fn".

```
fn main(){}
```

Para crear la función main.


### Importaciones y funciones públicas

Para importar y poder ejecutar una función de otro archivo se debe hacer pública dicha función a usar.

Si en un archivo tenemos la función main y la queremos ejecutar desde el archivo principal (main) debemos hacer esto:

1. Debemos volver pública la función.
   Así se hace una función pública:
   ```
   pub(crate) fn nombre_función(){}
   ```
2. Luego debemos añadir el archivo donde está la función pública.
   ```
   mod nombre_archivo;
   ```
   Si se necesita un archiv que está en una subcarpeta:
   ```
   #[path = "subcarpeta/archivo.rs"] mod nombre_función;
   ```
3. Ahora se debe ejecutar la función.
   Se puede ejecutar la función desde cualquier parte del código.
   ```
   nombre_archivo::nombre_función;
   ```

Aquí te dejo un ejemplo:

```
// se añade el archivo y la función
#[path = "exercises/exercise3.rs"] mod exercise3;
// se ejecuta la función
exercise3::main();
```


## Bucles, flujos de control, etc.

Hay varios bucles que conozco.
Loop, While y For.


### Loop: Bucle infinito.

Aunque loop es un bucle infinito, este tiene la opción de parar.
Para parar un loop se usa la instrucción "break;"

Pueden haber varios "break" en todo el loop (aunque no tendrían sentido si no se usan dentro de condicionales)
Si un sólo "break" devuelve un valor (cómo lo hace return, osea "break valor;") entonces todos los otros deben hacerlo.
Si no se devuelve nada de forma explícita, entonces el programa lo hará. El programa devuelve cómo mínimo una tupla vacía, osea ().

La sintaxis de loop es:

```
loop {
    // código
}
```

Otro agregado de ```loop``` es el uso del ```continue```.

Cuando necesitas que en medio de una iteración del "loop" se pase de largo el resto del código pero que no pare de iterar (cómo lo hace ```break;```), entonces se puede usar ```continue;```

Osea básicamente si uno quiere en medio de una iteración parar la actual, e ir a la siguiente, se puede usar ```continue```.

Acá te dejo un ejemplo:

```
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
```


### While: Expresión condicional.

While funciona con condicional, por lo cual se ejecuta si la 

La sintaxis de while es:
```
while condición {
    // código
}
```

### For: Iteraciones.

For funciona de dos formas. Siempre se ejecuta un número determinado de veces.
Se puede iterar sobre una colección de elementos, osea sobre una matriz, un vector o un mapa hash.
En estos casos, se usa una variable temporal cómo en otros lenguajes para almacenar el valor de cada iteracion.

Ejemplo 1:
```
let big_birds = ["ostrich", "peacock", "stork", "heron", "flamingo"];
    for bird in big_birds.iter(){
        println!("The bird is: {}", bird);
    }
```
Se usa ".iter()" para iterar entre los elementos.
Y la variable temporal es "bird"


Otra forma de iterar es sobre un rango.
Se usa ".." para crear un rango abierto, osea que no incluye el valor final.
Y se usa "..=" para crear un rango cerrado, el cual sí incluye el valor final.

```
// con ..
for number in 1..6{ // comienza en 1 y termina en 5
        println!("The number is: {}", number);
    }

// con ..=
for number in 1..=6{ // comienza en 1 y termina en 6
    println!("The number is: {}", number);
}
```


<br>
<br>

Si buscas los bucles for, while y loop están en:

src > control
