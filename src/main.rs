mod math_service;
use math_service::Service;

fn main() {
    let s = Service::new();
    println!("sum of {} and {} is: {}", 1, 2, s.sum(1, 2));
    println!("sum static is: {}", Service::sum_static(10, 20));
    match Service::div_static(10.1, 0.0) {
        Ok(v) => {
            println!("value is {}", v)
        }
        Err(e) => {
            println!("error is {:?}", e)
        }
    }
    match Service::div_static(10.0, 20.0) {
        Ok(v) => {
            println!("value is {}", v)
        }
        Err(e) => {
            println!("error is {:?}", e)
        }
    }

    println!("{}", math_service::four::gen_four());
}
