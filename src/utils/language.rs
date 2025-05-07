use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
    pub fn name(&self) -> &'static str {
        match self {
            Language::English => "angleščina",
            Language::German => "nemščina",
            Language::Latin => "latinščina",
            Language::Slovene => "slovenščina",
        }
    }

    pub fn two_letter_code(&self) -> &'static str {
        match self {
            Language::English => "en",
            Language::German => "de",
            Language::Latin => "la",
            Language::Slovene => "sl",
        }
    }

    pub fn three_letter_code(&self) -> &'static str {
        match self {
            Language::English => "eng",
            Language::German => "deu",
            Language::Latin => "lat",
            Language::Slovene => "slv",
        }
    }
}

impl TryFrom<icu_locale::LanguageIdentifier> for Language {
    type Error = icu_locale::ParseError;

    fn try_from(value: icu_locale::LanguageIdentifier) -> Result<Self, Self::Error> {
        match value.language.as_str() {
            "en" => Ok(Language::English),
            "de" => Ok(Language::German),
            "la" => Ok(Language::Latin),
            "sl" => Ok(Language::Slovene),
            _ => Err(icu_locale::ParseError::InvalidLanguage),
        }
    }
}
