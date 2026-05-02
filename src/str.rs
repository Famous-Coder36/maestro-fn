pub struct Str {
    value: String,
}

pub fn str(s: &str) -> Str {
    Str {
        value: s.to_string(),
    }
}

impl Str {
    pub fn trim(mut self) -> Self {
        self.value = self.value.trim().to_string();
        self
    }

    pub fn ltrim(mut self) -> Self {
        self.value = self.value.trim_start().to_string();
        self
    }

    pub fn rtrim(mut self) -> Self {
        self.value = self.value.trim_end().to_string();
        self
    }

    pub fn upper(mut self) -> Self {
        self.value = self.value.to_uppercase();
        self
    }

    pub fn lower(mut self) -> Self {
        self.value = self.value.to_lowercase();
        self
    }

    pub fn replace(mut self, from: &str, to: &str) -> Self {
        self.value = self.value.replace(from, to);
        self
    }

    pub fn explode(self, delimiter: &str) -> StrVec {
        let items = self
            .value
            .split(delimiter)
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        StrVec { items }
    }

    pub fn get(self) -> String {
        self.value
    }
}


pub struct StrVec {
    items: Vec<String>,
}

impl StrVec {
    pub fn implode(self, delimiter: &str) -> String {
        self.items.join(delimiter)
    }

    pub fn map<F>(self, f: F) -> Self
    where
        F: Fn(String) -> String,
    {
        let items = self.items.into_iter().map(f).collect();
        StrVec { items }
    }

    pub fn filter<F>(self, f: F) -> Self
    where
        F: Fn(&String) -> bool,
    {
        let items = self.items.into_iter().filter(f).collect();
        StrVec { items }
    }

    pub fn each<F>(self, f: F) -> Self
    where
        F: Fn(&String),
    {
        for item in &self.items {
            f(item);
        }
        self
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn to_vec(self) -> Vec<String> {
        self.items
    }
}
