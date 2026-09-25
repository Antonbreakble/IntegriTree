use super::{info_tags, path, source, OpcConfig};
use crate::tag::property::TagProperty;
use crate::tag::opc::{OpcDirection};
use crate::{Signal, Tag};
use crate::tag::TagDataType;

pub fn generate(signal: &Signal, opc: &OpcConfig, out: &mut Vec<Tag>) {
    let mut tag = Tag::new(path(signal, "OUT.PV"));
    tag.add_property(TagProperty::DataType(TagDataType::I1));
    tag.add_property(TagProperty::ReadWriteDefinition(OpcDirection::Read));
    tag.add_property(TagProperty::OpcStringConnection(source(signal, opc, "OUT.PV"), ));
    tag.add_property(TagProperty::AccessMode(OpcDirection::Read));
    tag.add_property(TagProperty::EventDefinition(2_147_483_744));
    tag.add_property(TagProperty::EventOnMessage("@(#2.Info.Description): True".into(), ));
    tag.add_property(TagProperty::EventOffMessage("@(#2.Info.Description): False".into(), ));
    tag.add_property(TagProperty::EventOnSeverity(500));
    tag.add_property(TagProperty::EventOffSeverity(500));
    out.push(tag);

    out.extend(info_tags(signal, opc, &["Channel", "Description", "Name"]));
}