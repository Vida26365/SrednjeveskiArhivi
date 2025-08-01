use std::fmt::{Display, Formatter};

use sea_orm::sea_query::StringLen;
use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::{Deserialize, Serialize};

macro_rules! languages {
    ($( $variant:ident { two: $two:expr, three: $three:expr, name: $name:expr } ),+ $(,)?) => {
        #[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, EnumIter, DeriveActiveEnum)]
        #[sea_orm(rs_type = "String", db_type = "String(StringLen::N(2))")]
        pub enum Language {
            $(
                #[sea_orm(string_value = $two)]
                #[serde(rename = $two)]
                $variant,
            )+
        }

        impl Language {
            /// Returns the name of the language in Slovene.
            pub fn as_name(&self) -> &'static str {
                match self {
                    $(Language::$variant => $name,)+
                }
            }

            /// Returns the two-letter code of the language.
            pub fn as_two_letter_code(&self) -> &'static str {
                match self {
                    $(Language::$variant => $two,)+
                }
            }

            /// Returns the three-letter code of the language.
            pub fn as_three_letter_code(&self) -> &'static str {
                match self {
                    $(Language::$variant => $three,)+
                }
            }
        }

        impl Language {
            /// Parses a language from its name in Slovene.
            pub fn from_name(name: &str) -> Option<Self> {
                match name {
                    $($name => Some(Language::$variant),)+
                    _ => None,
                }
            }

            /// Parses a language from its two-letter code.
            pub fn from_two_letter_code(code: &str) -> Option<Self> {
                match code {
                    $($two => Some(Language::$variant),)+
                    _ => None,
                }
            }

            /// Parses a language from its three-letter code.
            pub fn from_three_letter_code(code: &str) -> Option<Self> {
                match code {
                    $($three => Some(Language::$variant),)+
                    _ => None,
                }
            }
        }
    }
}

impl Display for Language {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}", self.as_name())
    }
}

impl TryFrom<&icu_locale::LanguageIdentifier> for Language {
    type Error = icu_locale::ParseError;

    fn try_from(value: &icu_locale::LanguageIdentifier) -> Result<Self, Self::Error> {
        match Language::from_two_letter_code(value.language.as_str()) {
            Some(language) => Ok(language),
            None => Err(icu_locale::ParseError::InvalidLanguage),
        }
    }
}

impl TryFrom<&Language> for icu_locale::LanguageIdentifier {
    type Error = icu_locale::ParseError;

    fn try_from(value: &Language) -> Result<Self, Self::Error> {
        value.as_two_letter_code().parse()
    }
}

impl From<&Language> for sea_orm::Value {
    fn from(value: &Language) -> Self {
        value.as_two_letter_code().into()
    }
}

languages! {
    English { two: "en", three: "eng", name: "angleščina" },
    German { two: "de", three: "deu", name: "nemščina" },
    Latin { two: "la", three: "lat", name: "latinščina" },
    Slovene { two: "sl", three: "slv", name: "slovenščina" },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_letter_codes() {
        assert_eq!(Language::English.as_two_letter_code(), "en");
        assert_eq!(Language::German.as_two_letter_code(), "de");
        assert_eq!(Language::Latin.as_two_letter_code(), "la");
        assert_eq!(Language::Slovene.as_two_letter_code(), "sl");

        assert_eq!(Language::from_two_letter_code("en"), Some(Language::English));
        assert_eq!(Language::from_two_letter_code("de"), Some(Language::German));
        assert_eq!(Language::from_two_letter_code("la"), Some(Language::Latin));
        assert_eq!(Language::from_two_letter_code("sl"), Some(Language::Slovene));

        assert_eq!(Language::from_two_letter_code("xx"), None);
    }

    #[test]
    fn test_three_letter_codes() {
        assert_eq!(Language::English.as_three_letter_code(), "eng");
        assert_eq!(Language::German.as_three_letter_code(), "deu");
        assert_eq!(Language::Latin.as_three_letter_code(), "lat");
        assert_eq!(Language::Slovene.as_three_letter_code(), "slv");

        assert_eq!(Language::from_three_letter_code("eng"), Some(Language::English));
        assert_eq!(Language::from_three_letter_code("deu"), Some(Language::German));
        assert_eq!(Language::from_three_letter_code("lat"), Some(Language::Latin));
        assert_eq!(Language::from_three_letter_code("slv"), Some(Language::Slovene));

        assert_eq!(Language::from_three_letter_code("xxx"), None);
    }

    #[test]
    fn test_language_display() {
        assert_eq!(Language::English.to_string(), "angleščina");
        assert_eq!(Language::German.to_string(), "nemščina");
        assert_eq!(Language::Latin.to_string(), "latinščina");
        assert_eq!(Language::Slovene.to_string(), "slovenščina");
    }
}
