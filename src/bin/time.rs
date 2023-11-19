use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug)]
struct Time {
    total_milliseconds: u128,
    total_seconds: u128,
    current_seconds: u128,
    total_minutes: u128,
    current_minute: u128,
    total_hours: u128,
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
    fn calc_time(system_time: u128) -> Self {
        let total_milliseconds: u128 = system_time / 1000;
        let total_seconds: u128 = total_milliseconds / 1000;
        let current_seconds: u128 = (total_milliseconds / 1000) % 60;
        let total_minutes: u128 = (total_milliseconds / 1000) / 60;
        let current_minute: u128 = (((total_milliseconds) / 1000) / 60) % 60;
        let total_hours: u128 = (((total_milliseconds) / 1000) / 60) / 60;
        let current_hour: u128 = ((((total_milliseconds) / 1000) / 60) / 60) % 24;
        Self {
            total_milliseconds,
            total_seconds,
            current_seconds,
            total_minutes,
            current_minute,
            total_hours,
            current_hour,
            current_year: 0,
        }
    }
    fn set_period(&mut self, offset: u128) -> String {
        let mut period: String = String::from(" AM");
        if self.current_hour < offset {
            self.current_hour = (self.current_hour + 24) - offset;
        }
        else {
            self.current_hour = self.current_hour - offset;
        }
        if self.current_hour >= 12 {
            period = String::from(" PM");
        }
        return period;
    }
    fn set_timezone() {
    }
    fn format_minute() {}
    fn format_second() {}
    fn print_time() {}
}

fn main() {

    let time_now: Time = Time::calc_time(Time::get_system_time());
    println!("Struct debug: {:#?}", time_now);
}