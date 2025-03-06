//! Processing date information; this is set aside as it usually requires
//! additional parsing and extra packages.

use core::fmt;

use biblatex::{Chunks, DateValue, Datetime, Entry, PermissiveType};
use chrono::NaiveDate;
use log::error;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct Date {
    pub year: Option<i32>,
    pub month: Option<u8>,
    pub day: Option<u8>,
}

impl Date {
    pub fn empty() -> Date {
        Date {
            year: None,
            month: None,
            day: None,
        }
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result: String = "".to_string();
        if let Some(y) = self.year {
            result.push_str(&y.to_string());
        } else {
            return write!(f, "unknown");
        }
        if let Some(m) = self.month {
            result.push('/');
            result.push_str(&format!("{:0>2}", m.to_string()));
        } else {
            return write!(f, "{}", result);
        }
        if let Some(d) = self.day {
            result.push('/');
            result.push_str(&format!("{:0>2}", d.to_string()));
        }
        write!(f, "{}", result)
    }
}

impl fmt::Debug for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{}", &self))
    }
}

impl Into<Date> for &Entry {
    fn into(self) -> Date {
        match self.date() {
            Ok(permissive_date) => {
                match permissive_date {
                    PermissiveType::Chunks(chunks) => {
                        error!("unimplemented {:?}", chunks);
                    }
                    PermissiveType::Typed(date) => match date.value {
                        DateValue::At(datetime) => {
                            let year = Some(datetime.year).into();
                            let mut month = datetime.month;
                            if let Some(m) = month {
                                month = Some(m + 1);
                            }
                            let mut day = datetime.day;
                            if let Some(d) = day {
                                day = Some(d + 1);
                            }
                            return Date { year, month, day };
                        }
                        _ => {
                            error!("unimplemented date format");
                        }
                    },
                }
                Date::empty()
            }
            Err(_) => Date::empty(),
        }
    }
}
