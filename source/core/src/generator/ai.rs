use super::{info_tags, path, source, OpcConfig};
use crate::tag::property::TagProperty;
use crate::tag::{TagDataType};
use crate::{Signal, Tag};
use crate::tag::opc::{OpcDirection, OpcSource};

const PAR_PARAMETERS: &[&str] = &[
    "DeadbandRate",
    "FiltrationRate",
    "H",
    "HH",
    "L",
    "LL",
    "PvHigh",
    "PvLow",
    "PvShift",
];

const SET_PARAMETERS: &[&str] = &[
    "DeadbandRate",
    "FiltrationRate",
    "H",
    "HH",
    "L",
    "LL",
    "PvHigh",
    "PvLow",
    "PvShift",
];

pub fn generate(signal: &Signal, opc: &OpcConfig, out: &mut Vec<Tag>) {
    let mut tag = Tag::new(path(signal, "Events.Error"));
    tag.add_property(TagProperty::DataType(TagDataType::I1));
    tag.add_property(TagProperty::Formula("(#2.OUT.Status & 1) != 0".into()));
    tag.add_property(TagProperty::EventDefinition(0));
    out.push(tag);

    let mut tag = Tag::new(path(signal, "Events.HAlarm"));
    tag.add_property(TagProperty::DataType(TagDataType::I1));
    tag.add_property(TagProperty::Formula("(#2.OUT.Status & 16) != 0".into()));
    tag.add_property(TagProperty::OpcStringConnection(OpcSource::Empty{connection: opc.connection}));
    tag.add_property(TagProperty::ReadWriteDefinition(OpcDirection::Read));
    tag.add_property(TagProperty::EventDefinition(2_147_483_744));
    tag.add_property(TagProperty::EventOnMessage(
        "[Пришло] @(#2.Info.Description.Value) : Превышена верхняя предупредительная уставка".into(),
    ));
    tag.add_property(TagProperty::EventOffMessage(
        "[Ушло] @(#2.Info.Description.Value) : Превышена верхняя предупредительная уставка".into(),
    ));
    tag.add_property(TagProperty::EventOnSeverity(500));
    tag.add_property(TagProperty::EventOffSeverity(500));
    out.push(tag);

    let mut tag = Tag::new(path(signal, "Events.HHAlarm"));
    tag.add_property(TagProperty::DataType(TagDataType::I1));
    tag.add_property(TagProperty::Formula("(#2.OUT.Status & 8) != 0".into()));
    tag.add_property(TagProperty::OpcStringConnection(OpcSource::Empty{connection: opc.connection}));
    tag.add_property(TagProperty::ReadWriteDefinition(OpcDirection::Read));
    tag.add_property(TagProperty::EventDefinition(2_147_483_744));
    tag.add_property(TagProperty::EventOnMessage(
        "[Пришло] @(#2.Info.Description.Value) : Превышена верхняя критическая уставка".into(),
    ));
    tag.add_property(TagProperty::EventOffMessage(
        "[Ушло] @(#2.Info.Description.Value) : Превышена верхняя критическая уставка".into(),
    ));
    tag.add_property(TagProperty::EventOnSeverity(500));
    tag.add_property(TagProperty::EventOffSeverity(500));
    out.push(tag);

    let mut tag = Tag::new(path(signal, "Events.LAlarm"));
    tag.add_property(TagProperty::DataType(TagDataType::I1));
    tag.add_property(TagProperty::Formula("(#2.OUT.Status & 32) != 0".into()));
    tag.add_property(TagProperty::OpcStringConnection(OpcSource::Empty{connection: opc.connection}));
    tag.add_property(TagProperty::ReadWriteDefinition(OpcDirection::Read));
    tag.add_property(TagProperty::EventDefinition(2_147_483_744));
    tag.add_property(TagProperty::EventOnMessage(
        "[Пришло] @(#2.Info.Description.Value) : Превышена нижняя предупредительная уставка".into(),
    ));
    tag.add_property(TagProperty::EventOffMessage(
        "[Ушло] @(#2.Info.Description.Value) : Превышена нижняя предупредительная уставка".into(),
    ));
    tag.add_property(TagProperty::EventOnSeverity(500));
    tag.add_property(TagProperty::EventOffSeverity(500));
    out.push(tag);

    let mut tag = Tag::new(path(signal, "Events.LLAlarm"));
    tag.add_property(TagProperty::DataType(TagDataType::I1));
    tag.add_property(TagProperty::Formula("(#2.OUT.Status & 64) != 0".into()));
    tag.add_property(TagProperty::OpcStringConnection(OpcSource::Empty{connection: opc.connection}));
    tag.add_property(TagProperty::ReadWriteDefinition(OpcDirection::Read));
    tag.add_property(TagProperty::EventDefinition(2_147_483_744));
    tag.add_property(TagProperty::EventOnMessage(
        "[Пришло] @(#2.Info.Description.Value) : Превышена нижняя критическая уставка".into(),
    ));
    tag.add_property(TagProperty::EventOffMessage(
        "[Ушло] @(#2.Info.Description.Value) : Превышена нижняя критическая уставка".into(),
    ));
    tag.add_property(TagProperty::EventOnSeverity(500));
    tag.add_property(TagProperty::EventOffSeverity(500));
    out.push(tag);


    out.extend(info_tags(signal, opc, &["Channel", "Description", "Name", "Units"]));

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

    // PAR: теги чтения
    for &name in PAR_PARAMETERS {
        let suffix = format!("PAR.{name}");
        let mut tag = Tag::new(path(signal, &suffix));

        tag.add_property(TagProperty::DataType(TagDataType::R4));
        tag.add_property(TagProperty::ReadWriteDefinition(OpcDirection::Read));
        tag.add_property(TagProperty::OpcStringConnection(source(signal, opc, &suffix)));
        tag.add_property(TagProperty::AccessMode(OpcDirection::Read));

        out.push(tag);
    }

    // SET: теги записи
    for &name in SET_PARAMETERS {
        let suffix = format!("SET.{name}");
        let suffix_plc = format!("PAR.{name}");

        let mut tag = Tag::new(path(signal, &suffix));

        tag.add_property(TagProperty::DataType(TagDataType::R4));
        tag.add_property(TagProperty::ReadWriteDefinition(OpcDirection::Write));
        tag.add_property(TagProperty::OpcStringConnection(source(signal, opc, &suffix_plc)));
        tag.add_property(TagProperty::AccessMode(OpcDirection::Write));

        out.push(tag);
    }
}