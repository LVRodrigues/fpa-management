#[derive(Debug, Clone)]
pub struct Context {
    id: String,
    name: String,
}

impl Context {
    pub fn new(id: String, name: String) -> Self {
        Self{id, name}
    }

    pub fn id(&self) -> &str {
        &self.id.as_str()
    }

    pub fn name(&self) -> &str {
        &self.name.as_str()
    }
}

