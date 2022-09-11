pub struct Scanner {}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {}
    }

    pub fn scan_tokens(&self) -> String {
        String::from("called from scan_tokens")
    }
}
