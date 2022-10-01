#[derive(Default)]
pub struct Config {
    pub ident: usize,
}

impl Config {
    pub fn default() -> Self {
        Self { ident: 2 }
    }
}
