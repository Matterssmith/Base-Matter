
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
        let mut current = self
            .locale
            .write()
            .expect("Translator lock poisoned");

        *current = locale.into();
    }

    pub fn locale(&self) -> String {
        self.locale
            .read()
            .expect("Translator lock poisoned")
            .clone()
    }

    pub fn register(
        &self,
        locale: impl Into<String>,
        translations: HashMap<String, String>,
    ) {
        let mut catalogs = self
            .catalogs
            .write()
            .expect("Translator lock poisoned");

        catalogs.insert(locale.into(), translations);
    }

    pub fn translate(&self, key: &str) -> String {
        let locale = self.locale();

        let catalogs = self
            .catalogs
            .read()
            .expect("Translator lock poisoned");

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
            .expect("Translator lock poisoned");

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
