
fn main() {
    println!("{}", working_items_per_minute(10));
}

const ratePerSpeedInAnHour : u32 = 221;

pub fn production_rate_per_hour(speed: u8) -> f64 {


    let produced_per_hour : f64 =  (ratePerSpeedInAnHour * (speed as u32)) as f64;

    discount_fails(speed, produced_per_hour)

   
}

pub fn working_items_per_minute(speed: u8) -> u32 {

    let minutes : u32 = 60;
    let produced_per_minute : f64  = ((ratePerSpeedInAnHour * speed as u32)  /minutes ) as f64;

    
    discount_fails(speed, produced_per_minute).round() as u32
    
}

fn discount_fails(speed : u8 , mut produced :  f64) -> f64 {

    if speed <= 8 && speed >= 5{ //90% rate success

        produced = produced * 0.9;

   }else if speed >= 9{
    produced = produced*0.77;
   }

   produced

}