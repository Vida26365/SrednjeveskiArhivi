use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, EnumIter)]
pub enum Language {
    #[serde(rename = "en")]
    English,

    #[serde(rename = "de")]
    German,

    #[serde(rename = "la")]
    Latin,

    #[serde(rename = "sl")]
    Slovene,
}

impl Language {
    pub fn as_name(&self) -> &'static str {
        match self {
            Language::English => "angleščina",
            Language::German => "nemščina",
            Language::Latin => "latinščina",
            Language::Slovene => "slovenščina",
        }
    }

    pub fn as_two_letter_code(&self) -> &'static str {
        match self {
            Language::English => "en",
            Language::German => "de",
            Language::Latin => "la",
            Language::Slovene => "sl",
        }
    }

    pub fn as_three_letter_code(&self) -> &'static str {
        match self {
            Language::English => "eng",
            Language::German => "deu",
            Language::Latin => "lat",
            Language::Slovene => "slv",
        }
    }
}

impl Language {
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "angleščina" => Some(Language::English),
            "nemščina" => Some(Language::German),
            "latinščina" => Some(Language::Latin),
            "slovenščina" => Some(Language::Slovene),
            _ => None,
        }
    }

    pub fn from_two_letter_code(code: &str) -> Option<Self> {
        match code {
            "en" => Some(Language::English),
            "de" => Some(Language::German),
            "la" => Some(Language::Latin),
            "sl" => Some(Language::Slovene),
            _ => None,
        }
    }

    pub fn from_three_letter_code(code: &str) -> Option<Self> {
        match code {
            "eng" => Some(Language::English),
            "deu" => Some(Language::German),
            "lat" => Some(Language::Latin),
            "slv" => Some(Language::Slovene),
            _ => None,
        }
    }
}

impl Display for Language {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}", self.as_name())
    }
}

impl TryFrom<icu_locale::LanguageIdentifier> for Language {
    type Error = icu_locale::ParseError;

    fn try_from(value: icu_locale::LanguageIdentifier) -> Result<Self, Self::Error> {
        match Language::from_two_letter_code(value.language.as_str()) {
            Some(language) => Ok(language),
            None => Err(icu_locale::ParseError::InvalidLanguage),
        }
    }
}
