use super::{info_tags, path, source, OpcConfig};
use crate::tag::property::TagProperty;
use crate::tag::{TagDataType};
use crate::{Signal, Tag};
use crate::tag::opc::OpcDirection;

pub fn generate(signal: &Signal, opc: &OpcConfig, out: &mut Vec<Tag>) {
    let mut tag = Tag::new(path(signal, "OUT.PV"));
    tag.add_property(TagProperty::DataType(TagDataType::R4));
    tag.add_property(TagProperty::ReadWriteDefinition(OpcDirection::Read));
    tag.add_property(TagProperty::OpcStringConnection(source(signal, opc, "OUT.PV")));
    tag.add_property(TagProperty::AccessMode(OpcDirection::Read));
    out.push(tag);

    let mut tag = Tag::new(path(signal, "OUT.Status"));
    tag.add_property(TagProperty::DataType(TagDataType::Ui16));
    tag.add_property(TagProperty::ReadWriteDefinition(OpcDirection::Read));
    tag.add_property(TagProperty::OpcStringConnection(source(signal, opc, "OUT.Status")));
    tag.add_property(TagProperty::AccessMode(OpcDirection::Read));
    out.push(tag);

    out.extend(info_tags(signal, opc, &["Channel", "Description", "Name", "Units"]));
}