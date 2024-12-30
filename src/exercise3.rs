//#[allow(dead_code)]
//#[allow(unused_variables)]
//#[allow(unused_mut)]
pub(crate) fn main(){

    #[derive(Debug)]
    enum Age {
        New,
        Used
    }

    #[derive(Debug)]
    struct Car{
        color: String,
        motor: Transmission,
        roof: bool,
        age: (Age, u32),
    }

    #[derive(Debug, PartialEq)]
    enum Transmission{
        Manual,
        SemiAuto,
        Automatic
    }

    fn car_quality(miles: u32) -> (Age, u32){
        
        if miles > 0 {
            return (Age::Used, miles);
        } else {
            return (Age::New, miles);
        }
    }

    fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car{

        if miles > 0 {
            if roof == true {
                println!("Prepare a used car: {:?}, {}, Hard top, {} miles\n", motor, color, miles);
            }
        }
        
        Car { color, motor, roof, age: car_quality(miles) }
    }

    car_factory(String::from("Orange"), Transmission::Manual, true, 0);
    
    car_factory(String::from("Red"), Transmission::SemiAuto, false, 565);

    car_factory(String::from("White"), Transmission::Automatic, true,3000);
}