mod english;
mod french;
mod german;
mod hungarian;
mod spanish;

use zsozso_common::Language;
pub use zsozso_common::StoreI18n;
use english::EnglishStore;
use french::FrenchStore;
use german::GermanStore;
use hungarian::HungarianStore;
use spanish::SpanishStore;

/// Factory function to get the appropriate StoreI18n implementation
pub fn store_i18n(lang: Language) -> Box<dyn StoreI18n> {
    match lang {
        Language::English => Box::new(EnglishStore),
        Language::French => Box::new(FrenchStore),
        Language::German => Box::new(GermanStore),
        Language::Hungarian => Box::new(HungarianStore),
        Language::Spanish => Box::new(SpanishStore),
        _ => Box::new(EnglishStore),
    }
}
