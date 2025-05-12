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

#[derive(Copy, Clone, Debug, PartialEq, Eq, FromJsonQueryResult)]
pub enum Date {
    Gregorian(icu_calendar::Date<icu_calendar::cal::Gregorian>),
    Julian(icu_calendar::Date<icu_calendar::cal::Julian>),
}

impl Date {
    pub fn to_iso(&self) -> icu_calendar::Date<icu_calendar::Iso> {
        match self {
            Date::Gregorian(date) => date.to_iso(),
            Date::Julian(date) => date.to_iso(),
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
            Date::Gregorian(ref d) => {
                state.serialize_field("calendar", &Calendar::Gregorian)?;
                state.serialize_field("year", &d.year().extended_year)?;
                state.serialize_field("month", &d.month().ordinal)?;
                state.serialize_field("day", &d.day_of_month().0)?;
            }
            Date::Julian(ref d) => {
                state.serialize_field("calendar", &Calendar::Julian)?;
                state.serialize_field("year", &d.year().extended_year)?;
                state.serialize_field("month", &d.month().ordinal)?;
                state.serialize_field("day", &d.day_of_month().0)?;
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
                    .map_err(|e| serde::de::Error::custom(format!("Invalid Gregorian date: {}", e)))
            }
            Calendar::Julian => {
                icu_calendar::Date::try_new_julian(helper.year, helper.month, helper.day)
                    .map(Date::Julian)
                    .map_err(|e| serde::de::Error::custom(format!("Invalid Julian date: {}", e)))
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
