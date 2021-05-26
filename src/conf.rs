
#[derive(Debug)]
pub struct StreamConf {
    pub name: String
}

impl Default for StreamConf {
    fn default() -> Self {
        Self::new("name".to_owned())
    }
}

impl StreamConf {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}


