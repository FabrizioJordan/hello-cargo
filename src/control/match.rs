fn main(){
    // es como un switch en otros lenguajes

    // match es una construccion de control de flujo que permite comparar un valor con una serie de patrones
    // cada patron se compara con el valor, si el patron coincide con el valor, se ejecuta el bloque de codigo asociado
    // si no hay coincidencias, se ejecuta el bloque de codigo de la rama final

    //                  Ejemplo 1

    enum Color{
        Red,
        Green,
        Blue,
        RGB(u8, u8, u8)
    }

    fn colors(){
        let color = Color::RGB(122, 17, 40);

        match color{
            Color::Red => println!("El color es Rojo"),
            Color::Green => println!("El color es Verde"),
            Color::Blue => println!("El color es Azul"),
            Color::RGB(r, g, b) => println!("El color es RGB({}, {}, {})", r, g, b)
        }   
    }



    //                 Ejemplo 2


    enum Message{
        Quit,
        Move{x: i32, y: i32},
        Write(String),
        ChangeColor(i32, i32, i32)
    }

    fn process_message(msg: Message){
        match msg{
            Message::Quit => {
                println!("El mensaje es de salida. Saliendo...")
            }
            Message::Move{x, y} => {
                println!("Moviendose a las coords: {}, y: {}", x, y)
            }
            Message::Write(text) => {
                println!("El mensaje es: {}", text)
            }
            Message::ChangeColor(r, g, b) => {
                println!("Cambiando el color a RGB -> r: {}, g: {}, b: {}", r, g, b)
            }
        }
    }
    
    fn executor(){
        let messages = vec![
            Message::Quit,
            Message::Move{x: 10, y: 20},
            Message::Write(String::from("Hola")),
            Message::ChangeColor(0, 255, 0),        
        ];

        for msg in messages{
            process_message(msg);
        }
    }


    //                 Ejemplo 3

    // un ejemplo tal vez más real

    enum Event {
        Start,
        Stop,
        Pause,
        Resume,
        Quit,
        Custom(String)
    }

    fn handle_event(event: Event){
        match event{
            Event::Start => {
                println!("Iniciando...")
                // Lógica para iniciar la aplicación
            },
            Event::Stop => {
                println!("Deteniendo...")
                // Lógica para detener la aplicación
            },
            Event::Pause => {
                println!("Pausando...")
                // Lógica para pausar la aplicación
            },
            Event::Resume => {
                println!("Reanudando...")
                // Lógica para reanudar la aplicación
            },
            Event::Quit => {
                println!("Saliendo...")
                // Lógica para salir de la aplicación
            },
            Event::Custom(msg) => {
                println!("Evento especial: {}", msg)
                // Lógica para manejar un evento especial
            }
        }
    }

    fn new_main(){
        let events = vec![
            Event::Start,
            Event::Pause,
            Event::Resume,
            Event::Stop,
            Event::Quit,
            Event::Custom(String::from("Evento especial"))
        ];

        for event in events{
            handle_event(event);
        }
    }


    // ejectuar funciones
    colors();

    println!("\n");

    executor();

    println!("\n");

    new_main();

}