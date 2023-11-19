use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug)]
struct Time {
    current_seconds: u128,
    current_minute: u128,
    current_hour: u128,
    current_year: u128,
}
impl Time {
    fn get_system_time() -> u128 {
        let start: SystemTime = SystemTime::now();
        let since_the_epoch: Duration = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let milliseconds: u128 = since_the_epoch.as_millis() as u128;
        return milliseconds;
    }
    fn time_constructor(system_time: u128) -> Self {
        let current_seconds: u128 = system_time / 1000 % 60 % 60; //works
        let current_minute: u128 = system_time / 1000 / 60 % 60; //works
        let current_hour: u128 = system_time / 1000 / 60 / 60 % 24; //all fucked up
        Self {
            current_seconds,
            current_minute,
            current_hour,
            current_year: 0,
        }
    }
    fn set_period(&mut self, offset: &u128) -> String {
        let mut period: String = String::from(" AM");
        if self.current_hour < *offset {
            self.current_hour = (self.current_hour + 24) - offset;
        } else {
            self.current_hour = self.current_hour - offset;
        }
        if self.current_hour >= 12 {
            period = String::from(" PM");
        }
        return period;
    }
    fn set_timezone(&mut self, offset: &u128) -> i32 {
        //let mut updated_hour: i32 = (self.current_hour - offset) as i32;
        let mut updated_hour: i32 = self.current_hour as i32;
        if updated_hour <= 0 {
            updated_hour = updated_hour + 12;
        }
        if updated_hour > 12 {
            updated_hour = updated_hour - 12;
        }
        return updated_hour;
    }
    fn format_minute() {}
    fn format_second() {}
    fn print_time() {}
}

fn main() {
    //Sets the timezone offset from UTC
    let offset: u128 = 8;

    //Prints the time
    let mut time_now: Time = Time::time_constructor(Time::get_system_time());
    //println!("Struct debug: {:#?}", time_now);
    let period: String = Time::set_period(&mut time_now, &offset);
    let hour: i32 = Time::set_timezone(&mut time_now, &offset);
    println!(
        "{}:{}:{}{}",
        hour, time_now.current_minute, time_now.current_seconds, period
    );

    // let time: SystemTime = SystemTime::now();
    // println!("{:#?}", time);
}
