use std::thread;
use std::time::Duration;

fn main() {
    let intensity = 10;
    let random_number = 7;
    generate_workout(intensity, random_number)
}


fn generate_workout(intensity: u32, random_number: u32) {

    let extensive_closure = |time|{
        println!("calculating....");
        thread::sleep(Duration::from_secs(2));
        time
    };
    if intensity > 25 {
        println!("Do {} pushups.",extensive_closure(intensity));
        println!("Do {} situps.",extensive_closure(intensity));
    }
    else{
        if random_number == 3{
            println!("Climb Moutain in {} mins ",extensive_closure(intensity));
        } 
        else{
            println!("Go run {} kms",extensive_closure(intensity));
        }
    }
}
