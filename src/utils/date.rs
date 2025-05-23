use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

use sea_orm::FromJsonQueryResult;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use strum::EnumIter;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, EnumIter)]
#[serde(rename_all = "lowercase")]
pub enum Calendar {
    Gregorian,
    Julian,
}

impl Calendar {
    pub fn as_variant_name(&self) -> &'static str {
        match self {
            Calendar::Gregorian => "gregorian",
            Calendar::Julian => "julian",
        }
    }
}

impl Calendar {
    pub fn from_variant_name(str: &str) -> Option<Self> {
        match str {
            "gregorian" => Some(Calendar::Gregorian),
            "julian" => Some(Calendar::Julian),
            _ => None,
        }
    }
}

impl Display for Calendar {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Calendar::Gregorian => write!(fmt, "gregorjanski"),
            Calendar::Julian => write!(fmt, "julijanski"),
        }
    }
}

#[derive(Copy, Clone, Debug, FromJsonQueryResult)]
pub enum Date {
    Gregorian(icu_calendar::Date<icu_calendar::cal::Gregorian>),
    Julian(icu_calendar::Date<icu_calendar::cal::Julian>),
}

impl Date {
    pub fn parse(str: &str, calendar: &Calendar) -> Result<Self, String> {
        let parts: Vec<&str> = str.split('.').collect();

        if parts.len() != 3 {
            return Err(format!("Invalid date format: {str}"));
        }

        let day =
            parts[0].trim().parse::<u8>().map_err(|_| format!("Invalid day: {}", parts[0]))?;
        let month =
            parts[1].trim().parse::<u8>().map_err(|_| format!("Invalid month: {}", parts[1]))?;
        let year =
            parts[2].trim().parse::<i32>().map_err(|_| format!("Invalid year: {}", parts[2]))?;

        match calendar {
            Calendar::Gregorian => icu_calendar::Date::try_new_gregorian(year, month, day)
                .map(Date::Gregorian)
                .map_err(|error| format!("Invalid Gregorian date: {error}")),
            Calendar::Julian => icu_calendar::Date::try_new_julian(year, month, day)
                .map(Date::Julian)
                .map_err(|error| format!("Invalid Julian date: {error}")),
        }
    }
}

impl Date {
    pub fn to_iso(self) -> icu_calendar::Date<icu_calendar::Iso> {
        match self {
            Date::Gregorian(date) => date.to_iso(),
            Date::Julian(date) => date.to_iso(),
        }
    }
}

impl Date {
    pub fn calendar(&self) -> Calendar {
        match self {
            Date::Gregorian(_) => Calendar::Gregorian,
            Date::Julian(_) => Calendar::Julian,
        }
    }

    pub fn year(&self) -> i32 {
        match self {
            Date::Gregorian(date) => date.year().extended_year,
            Date::Julian(date) => date.year().extended_year,
        }
    }

    pub fn month(&self) -> u8 {
        match self {
            Date::Gregorian(date) => date.month().ordinal,
            Date::Julian(date) => date.month().ordinal,
        }
    }

    pub fn day(&self) -> u8 {
        match self {
            Date::Gregorian(date) => date.day_of_month().0,
            Date::Julian(date) => date.day_of_month().0,
        }
    }
}

impl Serialize for Date {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Date", 4)?;
        match self {
            Date::Gregorian(ref date) => {
                state.serialize_field("calendar", &Calendar::Gregorian)?;
                state.serialize_field("year", &date.year().extended_year)?;
                state.serialize_field("month", &date.month().ordinal)?;
                state.serialize_field("day", &date.day_of_month().0)?;
            }
            Date::Julian(ref date) => {
                state.serialize_field("calendar", &Calendar::Julian)?;
                state.serialize_field("year", &date.year().extended_year)?;
                state.serialize_field("month", &date.month().ordinal)?;
                state.serialize_field("day", &date.day_of_month().0)?;
            }
        }
        state.end()
    }
}

impl<'de> Deserialize<'de> for Date {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct DateHelper {
            calendar: Calendar,
            year: i32,
            month: u8,
            day: u8,
        }

        let helper = DateHelper::deserialize(deserializer)?;

        match helper.calendar {
            Calendar::Gregorian => {
                icu_calendar::Date::try_new_gregorian(helper.year, helper.month, helper.day)
                    .map(Date::Gregorian)
                    .map_err(|error| {
                        serde::de::Error::custom(format!("Invalid Gregorian date: {error}"))
                    })
            }
            Calendar::Julian => {
                icu_calendar::Date::try_new_julian(helper.year, helper.month, helper.day)
                    .map(Date::Julian)
                    .map_err(|error| {
                        serde::de::Error::custom(format!("Invalid Julian date: {error}"))
                    })
            }
        }
    }
}

impl Display for Date {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Date::Gregorian(date) => write!(
                fmt,
                "{}. {}. {}",
                date.day_of_month().0,
                date.month().ordinal,
                date.year().extended_year
            ),
            Date::Julian(date) => write!(
                fmt,
                "{}. {}. {}",
                date.day_of_month().0,
                date.month().ordinal,
                date.year().extended_year
            ),
        }
    }
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Date {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_iso().cmp(&other.to_iso())
    }
}

impl PartialEq for Date {
    fn eq(&self, other: &Self) -> bool {
        self.to_iso() == other.to_iso()
    }
}

impl Eq for Date {}
