pub enum Language {
    ENGLISH,
    FRENCH,
    GERMAN,
    SPANISH,
    ITALIAN,
    CHINESE_SIMPLIFIED,
    JAPANESE,
    PORTUGESE,
    RUSSIAN,
    KOREAN,
    CHINESE_TRADITIONAL
}

pub fn get_language_id(language: Language) -> u32 {
    match language {
        Language::ENGLISH => 1,
        Language::FRENCH => 2,
        Language::GERMAN => 3,
        Language::SPANISH => 4,
        Language::ITALIAN => 5,
        Language::CHINESE_SIMPLIFIED => 6,
        Language::JAPANESE => 7,
        Language::PORTUGESE => 8,
        Language::RUSSIAN => 9,
        Language::KOREAN => 10,
        Language::CHINESE_TRADITIONAL => 11
    }
}
