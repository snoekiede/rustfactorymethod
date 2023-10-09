use std::fmt::Error;

enum VehicleBrand {
    Volkswagen,
    Renault
}

trait AbstractCar {
    fn description(&self)->String;
}

trait AbstractBike {
    fn description(&self)->String;
}

trait VehicleFactory {
    fn create_car(&self,color:String)->Box<dyn AbstractCar>;
    fn create_bike(&self,number_of_wheels:i8)->Box<dyn AbstractBike>;
}

struct VolksWagenCar {
    make: String,
    color: String,
}

impl VolksWagenCar {
    fn new(color:String)->Self {
        VolksWagenCar {
            color,
            make: "Volkswagen".to_string()
        }
    }
}

impl AbstractCar for VolksWagenCar {
    fn description(&self) -> String {
        format!("make: {}, color: {}",self.make,self.color)
    }
}

struct VolkswagenBike {
    make: String,
    number_of_wheels: i8
}

impl VolkswagenBike {
    fn new(number_of_wheels: i8)->Self {
        VolkswagenBike {
            number_of_wheels,
            make: "Volkswagen".to_string()
        }
    }
}

impl AbstractBike for VolkswagenBike {
    fn description(&self) -> String {
        format!("make: {}, number of wheels: {}",self.make,self.number_of_wheels)
    }
}

struct RenaultCar {
    make: String,
    color: String,
}

impl RenaultCar {
    fn new(color:String)->Self {
        RenaultCar {
            color,
            make: "Renault".to_string()
        }
    }
}

impl AbstractCar for RenaultCar {
    fn description(&self) -> String {
        format!("make: {}, color: {}",self.make,self.color)
    }
}

struct RenaultBike {
    make: String,
    number_of_wheels: i8
}

impl RenaultBike {
    fn new(number_of_wheels: i8)->Self {
        RenaultBike {
            number_of_wheels,
            make: "Volkswagen".to_string()
        }
    }
}

impl AbstractBike for RenaultBike {
    fn description(&self) -> String {
        format!("make: {}, number of wheels: {}",self.make,self.number_of_wheels)
    }
}

struct VolkswagenFactory;

impl VehicleFactory for VolkswagenFactory {
    fn create_car(&self,color: String) -> Box<dyn AbstractCar> {
        Box::new(VolksWagenCar::new(color))
    }

    fn create_bike(&self, number_of_wheels: i8) -> Box<dyn AbstractBike> {
        Box::new(VolkswagenBike::new(number_of_wheels))
    }
}

struct RenaultFactory;

impl VehicleFactory for RenaultFactory {
    fn create_car(&self,color: String) -> Box<dyn AbstractCar> {
        Box::new(RenaultCar::new(color))
    }

    fn create_bike(&self,number_of_wheels: i8) -> Box<dyn AbstractBike> {
        Box::new(RenaultBike::new(number_of_wheels))
    }
}

trait VehicleCreator {
    fn create_car(&self,brand: VehicleBrand,color:String)->Result<Box<dyn AbstractCar>,Error>;
    fn create_bike(&self,brand:VehicleBrand,number_of_wheels:i8)->Result<Box<dyn AbstractBike>,Error>;
}

struct VehicleCreatorImpl;

impl VehicleCreator for VehicleCreatorImpl {
    fn create_car(&self, brand: VehicleBrand, color: String) -> Result<Box<dyn AbstractCar>, Error> {
        return match brand {
            VehicleBrand::Volkswagen => {
                let factory = VolkswagenFactory;
                let car = factory.create_car(color);
                Ok(car)
            }
            VehicleBrand::Renault => {
                let factory = RenaultFactory;
                let car = factory.create_car(color);
                Ok(car)
            }
        }
    }

    fn create_bike(&self, brand: VehicleBrand, number_of_wheels: i8) -> Result<Box<dyn AbstractBike>, Error> {
        return match brand {
            VehicleBrand::Volkswagen => {
                let factory = VolkswagenFactory;
                let bike = factory.create_bike(number_of_wheels);
                Ok(bike)
            }
            VehicleBrand::Renault => {
                let factory = RenaultFactory;
                let bike = factory.create_bike(number_of_wheels);
                Ok(bike)
            }
        }
    }
}
fn main() {
    let creator=VehicleCreatorImpl;
    let car=creator.create_car(VehicleBrand::Volkswagen,"Red".to_string());
    match car {
        Ok(x)=>{
            println!("{}",x.description());
        }
        Err(e)=>{
            println!("Something went wrong: {}",e);
        }
    }
}
