use std::collections::HashMap;
use std::sync::RwLock;

pub struct Translator {
    locale: RwLock<String>,
    catalogs: RwLock<HashMap<String, HashMap<String, String>>>,
}

impl Translator {
    pub fn new(locale: impl Into<String>) -> Self {
        Self {
            locale: RwLock::new(locale.into()),
            catalogs: RwLock::new(HashMap::new()),
        }
    }

    pub fn set_locale(&self, locale: impl Into<String>) {
        let locale = locale.into();
        let mut current = self
            .locale
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner);

        *current = locale;
    }

    pub fn locale(&self) -> String {
        self.locale
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .clone()
    }

    pub fn register(&self, locale: impl Into<String>, translations: HashMap<String, String>) {
        let locale = locale.into();
        let mut catalogs = self
            .catalogs
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner);

        catalogs.insert(locale, translations);
    }

    pub fn translate(&self, key: &str) -> String {
        let locale = self.locale();

        let catalogs = self
            .catalogs
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner);

        catalogs
            .get(&locale)
            .and_then(|catalog| catalog.get(key))
            .cloned()
            .unwrap_or_else(|| key.to_string())
    }

    pub fn contains(&self, locale: &str, key: &str) -> bool {
        let catalogs = self
            .catalogs
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner);

        catalogs
            .get(locale)
            .is_some_and(|catalog| catalog.contains_key(key))
    }
}

impl Default for Translator {
    fn default() -> Self {
        Self::new("es")
    }
}

#[cfg(test)]
mod tests {
    use super::Translator;
    use std::collections::HashMap;
    use std::panic::{catch_unwind, AssertUnwindSafe};

    #[test]
    fn recovers_after_locale_lock_poisoning() {
        let translator = Translator::new("es");
        let _ = catch_unwind(AssertUnwindSafe(|| {
            let mut locale = translator.locale.write().unwrap();
            *locale = "en".to_owned();
            panic!("simulate a panic while changing locale");
        }));

        translator.register(
            "en",
            HashMap::from([("hello".to_owned(), "Hello".to_owned())]),
        );
        assert_eq!(translator.locale(), "en");
        assert_eq!(translator.translate("hello"), "Hello");
    }
}
