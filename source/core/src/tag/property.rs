use std::fmt;
use crate::tag::opc::{OpcDirection, OpcSource};
use crate::tag::TagDataType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TagProperty{
    AccessMode(OpcDirection), // 5
    OpcStringConnection(OpcSource), // 7996
    ReadWriteDefinition(OpcDirection), // 7997
    DataType(TagDataType), // 1
    Formula(String), // 3000000
    EventDefinition(u32), // 11000
    EventOnMessage(String), // 11150
    EventOnSeverity(u16), // 11154
    EventOffMessage(String), // 11151
    EventOffSeverity(u16), // 11155
}

impl TagProperty {
    pub fn encode(&self) -> (u32, PropertyValue) {
        match self {
            TagProperty::AccessMode(direction) => (5, PropertyValue::vt_ui4(direction.code())),
            TagProperty::OpcStringConnection(source) => (7996, PropertyValue::vt_bstr(source.serialize())),
            TagProperty::ReadWriteDefinition(direction) => (7997, PropertyValue::vt_i4(direction.code() as i32)),
            TagProperty::DataType(tag_data_type) => (1, PropertyValue::vt_ui2(tag_data_type.code())),
            TagProperty::Formula(value) => (3_000_000, PropertyValue::vt_bstr(value.clone())),
            TagProperty::EventDefinition(value) => (11000, PropertyValue::vt_ui4(*value)),
            TagProperty::EventOnMessage(message) => (11150, PropertyValue::vt_bstr(message.clone())),
            TagProperty::EventOnSeverity(severity) => (11154, PropertyValue::vt_ui2(*severity)),
            TagProperty::EventOffMessage(message) => (11151, PropertyValue::vt_bstr(message.clone())),
            TagProperty::EventOffSeverity(severity) => (11155, PropertyValue::vt_ui2(*severity)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropertyValue {
    vt_ui2(u16),
    vt_ui4(u32),
    vt_i4(i32),
    vt_bstr(String),
}

impl PropertyValue {
    pub fn variant_name(&self) -> &'static str {
        match self {
            Self::vt_ui2(_) => "vt_ui2",
            Self::vt_ui4(_) => "vt_ui4",
            Self::vt_i4(_) => "vt_i4",
            Self::vt_bstr(_) => "vt_bstr",
        }
    }
}

impl fmt::Display for PropertyValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::vt_ui2(value) => write!(f, "{value}"),
            Self::vt_ui4(value) => write!(f, "{value}"),
            Self::vt_i4(value) => write!(f, "{value}"),
            Self::vt_bstr(value) => f.write_str(value),
        }
    }
}