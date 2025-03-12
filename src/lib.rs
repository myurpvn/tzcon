use std::num::ParseFloatError;
use std::num::ParseIntError;

// pub struct Date {
//     pub year: i32,
//     pub month: i32,
//     pub date: i32,
// }

// impl Date {
//     pub fn build(dstring: &String) -> Time {
//         let time_vec: Vec<i32> = dstring
//             .split(":")
//             .map(|item| item.parse::<i32>().unwrap())
//             .collect();

//         Time {
//             hour: time_vec[0],
//             minute: time_vec[1],
//             second: time_vec[2],
//             next_day: false,
//         }
//     }
// }

#[derive(Debug, PartialEq)]
enum Day {
    Today,
    PrevDay,
    NextDay,
}

#[derive(Debug, PartialEq)]
struct Time {
    hour: i32,
    minute: i32,
    second: i32,
}

impl Time {
    fn build(tstring: &String) -> Result<Time, &'static str> {
        let tstring_vec = tstring.split(":").collect();

        let Ok(time_vec) = Time::parse_vec(tstring_vec) else {
            return Err("Unable to parse time");
        };

        let hour = time_vec[0];
        let minute = time_vec[1];
        let second = time_vec[2];

        if hour < 0 || hour > 23 {
            return Err("Invalid hour value!");
        }

        if minute < 0 || minute > 59 {
            return Err("Invalid minute value!");
        }

        if second < 0 || second > 59 {
            return Err("Invalid second value!");
        }

        Ok(Time {
            hour,
            minute,
            second,
        })
    }

    fn parse_vec(tstring_vec: Vec<&str>) -> Result<Vec<i32>, ParseIntError> {
        let mut return_vec: Vec<i32> = Vec::new();

        for stime in tstring_vec {
            let parsed_time = stime.parse::<i32>()?;
            return_vec.push(parsed_time);
        }

        Ok(return_vec)
    }

    fn parse_tz(arg: &str) -> Result<f32, ParseFloatError> {
        Ok(arg.parse::<f32>()?)
    }
}

#[derive(Debug, PartialEq)]
pub struct Config {
    time: Time,
    tz: f32,
    tztc: f32,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // if args.len() < 4 {
        //     return Err("Not enough arguments");
        // }

        args.next();

        let tstring = match args.next() {
            Some(arg) => arg,
            None => return Err("time is empty!"),
        };
        let tz_string = match args.next() {
            Some(arg) => arg,
            None => return Err("timezone is empty!"),
        };
        let tztc_string = match args.next() {
            Some(arg) => arg,
            None => return Err("timezone to convert is empty!"),
        };

        let time = Time::build(&tstring)?;
        let Ok(tz) = Time::parse_tz(&tz_string) else {
            return Err("Invalid timezone values!");
        };
        let Ok(tztc) = Time::parse_tz(&tztc_string) else {
            return Err("Invalid timezone values!");
        };

        if tz.abs() > 14.0 || tztc.abs() > 14.0 {
            return Err("Invalid timzone values!");
        }

        Ok(Config { time, tz, tztc })
    }
}

#[derive(Debug, PartialEq)]
pub struct Output {
    time: Time,
    day: Day,
}

impl Output {
    pub fn format_output(&self) {
        match self.day {
            Day::Today => println!(
                "Today: {:0>2}:{:0>2}:{:0>2}",
                self.time.hour, self.time.minute, self.time.second
            ),

            Day::NextDay => println!(
                "Next Day: {:0>2}:{:0>2}:{:0>2}",
                self.time.hour, self.time.minute, self.time.second
            ),

            Day::PrevDay => println!(
                "Previous Day: {:0>2}:{:0>2}:{:0>2}",
                self.time.hour, self.time.minute, self.time.second
            ),
        }
    }
}

pub fn convert(config: Config) -> Output {
    let Time {
        hour,
        minute,
        second,
    } = config.time;

    let tz_diff = config.tztc - config.tz;

    let mut day = Day::Today;
    let mut converted_hour = hour + tz_diff.trunc() as i32;
    let mut converted_minute = (minute as f32 + tz_diff.fract() * 60.0) as i32;

    if converted_minute >= 60 {
        converted_minute -= 60;
        converted_hour += 1;
    }

    if converted_minute < 0 {
        converted_minute += 60;
        converted_hour -= 1;
    }

    if converted_hour >= 24 {
        converted_hour -= 24;
        day = Day::NextDay;
    }

    if converted_hour < 0 {
        converted_hour += 24;
        day = Day::PrevDay;
    }

    Output {
        time: Time {
            hour: converted_hour,
            minute: converted_minute,
            second,
        },
        day,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_convert() {
        let config = Config {
            time: Time {
                hour: 00,
                minute: 00,
                second: 30,
            },
            tz: 3.5,
            tztc: 5.0,
        };

        assert_eq!(
            Output {
                time: Time {
                    hour: 01,
                    minute: 30,
                    second: 30,
                },
                day: Day::Today
            },
            convert(config)
        );
    }

    #[test]
    fn incremental_convert() {
        assert_eq!(
            Output {
                time: Time {
                    hour: 02,
                    minute: 00,
                    second: 30,
                },
                day: Day::Today
            },
            convert(Config {
                time: Time {
                    hour: 00,
                    minute: 30,
                    second: 30,
                },
                tz: 3.5,
                tztc: 5.0
            })
        );
        assert_eq!(
            Output {
                time: Time {
                    hour: 01,
                    minute: 00,
                    second: 00,
                },
                day: Day::NextDay
            },
            convert(Config {
                time: Time {
                    hour: 23,
                    minute: 00,
                    second: 00,
                },
                tz: 3.0,
                tztc: 5.0
            })
        );
    }
}
