#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_mut)]
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
        
        let quality: (Age, u32) = (Age::New, miles);

        quality
    }

    fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car{
        Car { color, motor, roof, age: car_quality(miles) }
    }

    let colors = ["Red", "Blue", "Green", "Silver"];

    let mut car: Car;
    let mut engine = Transmission::Manual;

    let mut mustang = car_factory(String::from(colors[0]), engine, true, 0);
    println!("My mustang is = {}, {:?} transmission, roof: {}, mileage: {:?}", mustang.color, mustang.motor, mustang.roof, mustang.age.0);


    let mut engine = Transmission::SemiAuto;

    let mut mustang = car_factory(String::from(colors[1]), engine, true, 0);
    println!("My mustang is = {}, {:?} transmission, roof: {}, mileage: {:?}", mustang.color, mustang.motor, mustang.roof, mustang.age.0);

    let mut engine = Transmission::Automatic;

    let mut mustang = car_factory(String::from(colors[2]), engine, true, 0);
    println!("My mustang is = {}, {:?} transmission, roof: {}, mileage: {:?}", mustang.color, mustang.motor, mustang.roof, mustang.age.0);

}