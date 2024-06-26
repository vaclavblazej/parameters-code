//! Processing date information; this is set aside as it usually requires
//! additional parsing and extra packages.

use biblatex::{Chunks, DateValue, Datetime, Entry, PermissiveType};
use chrono::NaiveDate;

use crate::data::data::Date;

impl Into<Date> for &Entry {
    fn into(self) -> Date {
        match self.date() {
            Ok(permissive_date) => {
                match permissive_date {
                    PermissiveType::Chunks(chunks) => {
                        eprintln!("unimplemented {:?}", chunks);
                    },
                    PermissiveType::Typed(date) => {
                        match date.value{
                            DateValue::At(datetime) => {
                                let year = Some(datetime.year).into();
                                let mut month = datetime.month;
                                if let Some(m) = month {
                                    month = Some(m+1);
                                }
                                let mut day = datetime.day;
                                if let Some(d) = day {
                                    day = Some(d+1);
                                }
                                return Date { year, month, day, };
                            },
                            _ => {
                                eprintln!("unimplemented date format");
                            },
                        }
                    },
                }
                Date::empty()
            }
            Err(_) => {
                Date::empty()
            }
        }
    }
}
