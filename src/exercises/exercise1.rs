pub(crate) fn main(){
    struct Car {
        color: String,
        transmission: Transmission,
        convertible: bool,
        mileage: u32
    }

    #[derive(Debug)]
    #[derive(PartialEq)]
    enum Transmission {
        Manual,
        SemiAuto,
        Automatic,
    }


    fn car_factory(color: String, transmission: Transmission, convertible:bool) -> Car{
        let new_car: Car = Car{
            color,
            transmission,
            convertible,
            mileage: 100
        };
        return new_car;
    }

    let mut car = car_factory(String::from("Green"), Transmission::Manual, false);
    println!("Car 1 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);

    car = car_factory(String::from("Silver"), Transmission::Automatic, true);
    println!("Car 2 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);

    car = car_factory(String::from("Yellow"), Transmission::SemiAuto, false);
    println!("Car 3 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);

}