use std::io::Write;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::{io, thread};

#[derive(Debug)]
struct Time {
    _current_second: u128,
    _current_minute: u128,
    _current_hour: u128,
    _current_year: u128,
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
        let _current_second: u128 = system_time / 1000 % 60;
        let _current_minute: u128 = &system_time / 1000 / 60 % 60;
        let _current_hour: u128 = &system_time / 1000 / 60 / 60 % 24;
        let _current_year: u128 = &system_time / 31557600000 + 1970;
        Self {
            _current_second,
            _current_minute,
            _current_hour,
            _current_year,
        }
    }
    //TODO: fix UB with mutable reference offset
    fn set_12h_period(&mut self, offset: &u128) -> String {
        let mut period: String = String::from("AM");
        if self._current_hour < *offset {
            self._current_hour = (&self._current_hour + 24) - offset;
        } else {
            self._current_hour = &self._current_hour - offset;
        }
        if self._current_hour >= 12 {
            period = String::from("PM");
        }
        return period;
    }
    fn set_timezone(&mut self) -> i32 {
        //let mut updated_hour: i32 = (self.current_hour - offset) as i32;
        //let mut updated_hour: i32 = self.current_hour as i32;
        let mut updated_hour: i32 = self._current_hour.clone() as i32;
        if updated_hour <= 0 {
            updated_hour = updated_hour + 12;
        }
        if updated_hour > 12 {
            updated_hour = updated_hour - 12;
        }
        return updated_hour;
    }
    fn format_minute(&self) -> String {
        return if self._current_minute < 10 {
            let i: String = self._current_minute.to_string();
            let mut formatted_minute: String = String::from("0");
            formatted_minute.push_str(&i);
            formatted_minute
        } else {
            self._current_minute.to_string()
        };
    }
    fn format_second(&self) -> String {
        return if self._current_second < 10 {
            let i: String = self._current_second.to_string();
            let mut formatted_second: String = String::from("0");
            formatted_second.push_str(&i);
            formatted_second
        } else {
            self._current_second.to_string()
        };
    }
}
pub fn loop_time(offset: u128) {
    loop {
        let mut time_now: Time = Time::time_constructor(Time::get_system_time());
        let period: String = Time::set_12h_period(&mut time_now, &offset);
        let hour: i32 = Time::set_timezone(&mut time_now);
        let minute: String = time_now.format_minute();
        let second: String = time_now.format_second();
        print!("\r{}:{}:{} {} ", hour, minute, second, period);
        //println!("{}", time_now.current_year);
        io::stdout().flush().unwrap();
        let wait = Duration::from_millis(1000);
        thread::sleep(wait);
    }
}
pub fn static_time(offset: u128) -> String {
    //TODO: Create logic to enter actual offset, which is -7/8 for PT
    let mut time_now: Time = Time::time_constructor(Time::get_system_time());
    let period: String = Time::set_12h_period(&mut time_now, &offset);
    let hour: i32 = Time::set_timezone(&mut time_now);
    let minute: String = time_now.format_minute();
    let second: String = time_now.format_second();
    //println!("\r{}:{}:{}{} ", hour, minute, second, period);
    //println!("{}", time_now.current_year);
    return format!("{hour}:{minute}:{second} {period}");
}
