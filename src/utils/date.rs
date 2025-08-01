use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

use sea_orm::sea_query::StringLen;
use sea_orm::{DeriveActiveEnum, DeriveValueType, EnumIter};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

macro_rules! calendars {
    ($( $variant:ident { name: $name:expr } ),+ $(,)?) => {
        #[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, EnumIter, DeriveActiveEnum)]
        #[sea_orm(rs_type = "String", db_type = "String(StringLen::None)", rename_all = "lowercase")]
        #[serde(rename_all = "lowercase")]
        pub enum Calendar {
            $($variant,)+
        }

        impl Calendar {
            pub fn as_variant_name(&self) -> &'static str {
                match self {
                    $(Calendar::$variant => paste::paste!{stringify!([<$variant:lower>])},)+
                }
            }
        }

        impl Calendar {
            pub fn from_variant_name(str: &str) -> Option<Self> {
                match str {
                    $(paste::paste!{stringify!([<$variant:lower>])} => Some(Calendar::$variant),)+
                    _ => None,
                }
            }
        }

        impl Display for Calendar {
            fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Calendar::$variant => write!(fmt, "{}", $name),)+
                }
            }
        }
    }
}

impl From<&Calendar> for sea_orm::Value {
    fn from(value: &Calendar) -> Self {
        value.as_variant_name().into()
    }
}

calendars! {
    Gregorian { name: "gregorjanski" },
    Julian { name: "julijanski" },
}

#[derive(Copy, Clone, Debug, DeriveValueType)]
#[sea_orm(
    value_type = "String",
    from_str = "Date::from_serialized",
    to_str = "Date::to_serialized"
)]
pub enum Date {
    Gregorian(icu_calendar::Date<icu_calendar::cal::Gregorian>),
    Julian(icu_calendar::Date<icu_calendar::cal::Julian>),
}

impl Date {
    /// Creates a new date for the specified year, month, and day in the given calendar.
    pub fn new(year: i32, month: u8, day: u8, calendar: &Calendar) -> Result<Self, String> {
        match calendar {
            Calendar::Gregorian => icu_calendar::Date::try_new_gregorian(year, month, day)
                .map(Date::Gregorian)
                .map_err(|error| format!("Invalid Gregorian date: {error}")),
            Calendar::Julian => icu_calendar::Date::try_new_julian(year, month, day)
                .map(Date::Julian)
                .map_err(|error| format!("Invalid Julian date: {error}")),
        }
    }

    /// Parses a date string in the format "DD. MM. YYYY" in the given calendar.
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

        Self::new(year, month, day, calendar)
    }
}

impl Date {
    /// Returns the calendar type of the date.
    pub fn calendar(&self) -> Calendar {
        match self {
            Date::Gregorian(_) => Calendar::Gregorian,
            Date::Julian(_) => Calendar::Julian,
        }
    }

    /// Returns the year of the date.
    ///
    /// Represents the number of years of this date relative to the start of a calendar-specific
    /// epoch year. Usually, year 1 is either the first year of the latest era or the ISO 8601
    /// year 0001. If the epoch is in the middle of the year, that year will have the same value
    /// before and after the start date of the era.
    pub fn year(&self) -> i32 {
        match self {
            Date::Gregorian(date) => date.extended_year(),
            Date::Julian(date) => date.extended_year(),
        }
    }

    /// Returns the month of the date.
    ///
    /// Represents the 1-based month index in the year of this date. The first month of the
    /// year is 1, and the last month may depend on the year and calendar type.
    pub fn month(&self) -> u8 {
        match self {
            Date::Gregorian(date) => date.month().ordinal,
            Date::Julian(date) => date.month().ordinal,
        }
    }

    /// Returns the day of the month of the date.
    ///
    /// Represents the 1-based day index in the month of this date, which is the same day number
    /// you would see on a calendar. Generally starts at 1 and is continuous, but not always.
    pub fn day(&self) -> u8 {
        match self {
            Date::Gregorian(date) => date.day_of_month().0,
            Date::Julian(date) => date.day_of_month().0,
        }
    }
}

impl Date {
    /// Converts the date to an ISO calendar date.
    #[allow(clippy::wrong_self_convention)]
    pub fn to_iso(&self) -> icu_calendar::Date<icu_calendar::Iso> {
        match self {
            Date::Gregorian(date) => date.to_iso(),
            Date::Julian(date) => date.to_iso(),
        }
    }

    /// Converts the date to a string in the format "YYYY-MM-DD-CALENDAR".
    ///
    /// The year, month, and day are converted to the ISO calendar date,
    /// and the lowercase calendar type is appended at the end.
    #[allow(clippy::wrong_self_convention)]
    pub fn to_serialized(&self) -> String {
        let calendar = self.calendar().as_variant_name();

        let iso = self.to_iso();
        let year = iso.extended_year();
        let month = iso.month().ordinal;
        let day = iso.day_of_month().0;

        format!("{year:04}-{month:02}-{day:02}-{calendar}")
    }
}

impl Date {
    /// Converts an ISO calendar date to a date in the specified calendar.
    pub fn from_iso(iso: &icu_calendar::Date<icu_calendar::Iso>, calendar: &Calendar) -> Self {
        match calendar {
            Calendar::Gregorian => Date::Gregorian(iso.to_calendar(icu_calendar::cal::Gregorian)),
            Calendar::Julian => Date::Julian(iso.to_calendar(icu_calendar::cal::Julian)),
        }
    }

    /// Converts a serialized date string to a date in the specified calendar.
    ///
    /// The string should be in the format "YYYY-MM-DD-CALENDAR", where the year,
    /// month, and day are in the ISO calendar, and the calendar type is lowercase.
    pub fn from_serialized(serialized: &str) -> Result<Self, String> {
        let parts: Vec<&str> = serialized.split('-').collect();

        if parts.len() != 4 {
            return Err(format!("Invalid date format: {serialized}"));
        }

        let year = parts[0].parse::<i32>().map_err(|_| format!("Invalid year: {}", parts[0]))?;
        let month = parts[1].parse::<u8>().map_err(|_| format!("Invalid month: {}", parts[1]))?;
        let day = parts[2].parse::<u8>().map_err(|_| format!("Invalid day: {}", parts[2]))?;

        let calendar = Calendar::from_variant_name(parts[3])
            .ok_or_else(|| format!("Invalid calendar: {}", parts[3]))?;

        let date = icu_calendar::Date::try_new_iso(year, month, day)
            .map_err(|error| format!("Invalid ISO date: {error}"))?;

        Ok(Self::from_iso(&date, &calendar))
    }
}

impl Serialize for Date {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_serialized())
    }
}

impl<'de> Deserialize<'de> for Date {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let serialized = String::deserialize(deserializer)?;
        Date::from_serialized(&serialized).map_err(serde::de::Error::custom)
    }
}

impl Display for Date {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}. {}. {}", self.day(), self.month(), self.year())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calendar_variants() {
        assert_eq!(Calendar::Gregorian.as_variant_name(), "gregorian");
        assert_eq!(Calendar::Julian.as_variant_name(), "julian");

        assert_eq!(Calendar::from_variant_name("gregorian"), Some(Calendar::Gregorian));
        assert_eq!(Calendar::from_variant_name("julian"), Some(Calendar::Julian));

        assert_eq!(Calendar::from_variant_name("invalid"), None);
    }

    #[test]
    fn test_calendar_display() {
        assert_eq!(Calendar::Gregorian.to_string(), "gregorjanski");
        assert_eq!(Calendar::Julian.to_string(), "julijanski");
    }

    #[test]
    fn test_date_creation() {
        let gregorian = Date::new(2025, 2, 24, &Calendar::Gregorian).unwrap();
        assert_eq!(gregorian.calendar(), Calendar::Gregorian);
        assert_eq!(gregorian.year(), 2025);
        assert_eq!(gregorian.month(), 2);
        assert_eq!(gregorian.day(), 24);

        let julian = Date::new(2025, 2, 24, &Calendar::Julian).unwrap();
        assert_eq!(julian.calendar(), Calendar::Julian);
        assert_eq!(julian.year(), 2025);
        assert_eq!(julian.month(), 2);
        assert_eq!(julian.day(), 24);
    }

    #[test]
    fn test_date_parsing() {
        let expected = Date::new(2000, 1, 20, &Calendar::Gregorian).unwrap();
        assert_eq!(expected, Date::parse("20. 1. 2000", &Calendar::Gregorian).unwrap());
        assert_eq!(expected, Date::parse("20.1.2000", &Calendar::Gregorian).unwrap());
        assert_eq!(expected, Date::parse("020.   0001.2000", &Calendar::Gregorian).unwrap());

        let expected = Date::new(2000, 2, 29, &Calendar::Julian).unwrap();
        assert_eq!(expected, Date::parse("29. 2. 2000", &Calendar::Julian).unwrap());
        assert_eq!(expected, Date::parse("29.2.2000", &Calendar::Julian).unwrap());
        assert_eq!(expected, Date::parse("029.   0002.2000", &Calendar::Julian).unwrap());

        assert!(Date::parse("2020-01-01", &Calendar::Gregorian).is_err());
        assert!(Date::parse("30. 2. 2000", &Calendar::Gregorian).is_err());
        assert!(Date::parse("DD. MM. YYYY", &Calendar::Gregorian).is_err());
    }

    #[test]
    fn test_date_display() {
        let date = Date::new(1271, 11, 17, &Calendar::Gregorian).unwrap();
        assert_eq!(date.to_string(), "17. 11. 1271");

        let date = Date::new(1271, 11, 17, &Calendar::Julian).unwrap();
        assert_eq!(date.to_string(), "17. 11. 1271");
    }

    #[test]
    fn test_date_serialization() {
        let date = Date::new(2025, 10, 5, &Calendar::Gregorian).unwrap();
        assert_eq!(serde_json::to_string(&date).unwrap(), "\"2025-10-05-gregorian\"");
        assert_eq!(date.to_serialized(), "2025-10-05-gregorian");

        let date = Date::new(2025, 10, 5, &Calendar::Julian).unwrap();
        assert_eq!(serde_json::to_string(&date).unwrap(), "\"2025-10-18-julian\"");
        assert_eq!(date.to_serialized(), "2025-10-18-julian");
    }

    #[test]
    fn test_date_deserialization() {
        let date: Date = serde_json::from_str("\"2025-10-05-gregorian\"").unwrap();
        assert_eq!(date.calendar(), Calendar::Gregorian);
        assert_eq!(date.year(), 2025);
        assert_eq!(date.month(), 10);
        assert_eq!(date.day(), 5);

        let date: Date = serde_json::from_str("\"2025-10-05-julian\"").unwrap();
        assert_eq!(date.calendar(), Calendar::Julian);
        assert_eq!(date.year(), 2025);
        assert_eq!(date.month(), 9);
        assert_eq!(date.day(), 22);

        assert!(Date::from_serialized("invalid-date").is_err());
    }

    #[test]
    fn test_date_comparison() {
        let date1 = Date::new(1243, 4, 13, &Calendar::Julian).unwrap();
        let date2 = Date::new(1243, 4, 13, &Calendar::Gregorian).unwrap();
        assert!(date1 > date2);

        let date1 = Date::new(1582, 10, 5, &Calendar::Julian).unwrap();
        let date2 = Date::new(1582, 10, 15, &Calendar::Gregorian).unwrap();
        assert_eq!(date1, date2);

        let date1 = Date::new(2025, 8, 1, &Calendar::Gregorian).unwrap();
        let date2 = Date::new(2025, 8, 1, &Calendar::Julian).unwrap();
        assert_ne!(date1, date2);
    }
}
