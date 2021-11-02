pub enum TrafficLight {
    Red,
    Green,
    Yellow,
}
pub trait LightTime {
    fn last_time(&self) -> u32;
}

impl LightTime for TrafficLight {
    fn last_time (&self) -> u32 {
        match self {
            TrafficLight::Red => 50,
            TrafficLight::Green => 30,
            TrafficLight::Yellow => 3,
        }
    }
} 

fn main () {
    let red = TrafficLight::Red;
    let green = TrafficLight::Green;
    let yellow = TrafficLight::Yellow;
    println!("Red light lasts {} seconds", red.last_time());
    println!("Green light lasts {} seconds", green.last_time());
    println!("Yellow light lasts {} seconds", yellow.last_time());
}
