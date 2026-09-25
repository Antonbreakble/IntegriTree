pub mod opc;
pub mod property;
use crate::tag::property::{TagProperty};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tag {
    path: String,
    properties: Vec<TagProperty>,
}

impl Tag {
    pub fn new(path: impl Into<String>) -> Self {
        Self { path: path.into(), properties: Vec::new() }
    }

    pub fn path(&self) -> &str { &self.path }
    pub fn properties(&self) -> &[TagProperty] { &self.properties }


    pub fn add_property(&mut self, property: TagProperty) -> &mut Self {
        self.properties.push(property);
        self
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TagDataType {
    R4,
    Ui16,
    I1,
    String,
}

impl TagDataType {
    fn code(self) -> u16 {
        match self {
            Self::R4 => 4,
            Self::Ui16 => 18,
            Self::I1 => 11,
            Self::String => 8,
        }
    }
}
