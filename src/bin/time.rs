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
        let milliseconds: u128 = since_the_epoch.as_millis();
        return milliseconds;
    }
    fn time_constructor(system_time: u128) -> Self {
        let current_seconds: u128 = system_time / 1000 % 60;
        let current_minute: u128 = system_time / 1000 / 60 % 60;
        let current_hour: u128 = system_time / 1000 / 60 / 60 % 24;
        let current_year: u128 = system_time / 31557600000 + 1970;
        Self {
            current_seconds,
            current_minute,
            current_hour,
            current_year,
        }
    }
    //TODO: fix UB with mutable reference offset
    fn set_12h_period(&mut self, offset: &u128) -> String {
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
    fn set_timezone(&mut self) -> i32 {
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
    //fn format_minute() -> String {}
    //fn format_second() -> String {}
    fn print_time() {}
}

fn main() {
    //Sets the timezone offset from UTC
    let offset: u128 = 8;

    //Prints the time
    let mut time_now: Time = Time::time_constructor(Time::get_system_time());
    //println!("Struct debug: {:#?}", time_now);
    let period: String = Time::set_12h_period(&mut time_now, &offset);
    let hour: i32 = Time::set_timezone(&mut time_now);
    println!(
        "{}:{}:{}{}",
        hour, time_now.current_minute, time_now.current_seconds, period
    );
    println!("{}", time_now.current_year);
}
