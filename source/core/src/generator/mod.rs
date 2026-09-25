mod ai;
mod ao;
mod di;
mod do_;

use crate::tag::property::TagProperty;
use crate::tag::{TagDataType};
use crate::{IntegritreeError, Signal, SignalKind, Tag};
use crate::opc_config::OpcConnection;
use crate::tag::opc::{OpcDirection, OpcSource};

#[derive(Debug, Clone)]
pub struct OpcConfig {
    pub connection: OpcConnection,
    pub namespace: u16,
    pub application: String,
    pub data_type: String,
}

impl OpcConfig {
    pub fn new(connection: OpcConnection) -> Self {
        Self {
            connection,
            namespace: 2,
            application: "Application".into(),
            data_type: "String".into(),
        }
    }
}

pub fn generate(
    signals: &[Signal],
    opc: &OpcConfig,
) -> Result<Vec<Tag>, IntegritreeError> {
    if opc.application.is_empty()
        || !opc.application.chars().all(|ch| {
        ch.is_ascii_alphanumeric() || ch == '_' || ch == '.'
    })
    {
        return Err(IntegritreeError(
            "некорректный корень OPC-пути".into()
        ));
    }

    if opc.data_type.is_empty()
        || opc.data_type.chars().any(|ch| {
        matches!(ch, '|' | ',' | '\n' | '\r')
    })
    {
        return Err(IntegritreeError(
            "некорректный тип OPC-источника".into()
        ));
    }

    let mut tags = Vec::new();

    for signal in signals.iter().filter(|s| s.kind == SignalKind::Ai) {
        ai::generate(signal, opc, &mut tags);
    }

    for signal in signals.iter().filter(|s| s.kind == SignalKind::Di) {
        di::generate(signal, opc, &mut tags);
    }

    for signal in signals.iter().filter(|s| s.kind == SignalKind::Do) {
        do_::generate(signal, opc, &mut tags);
    }

    for signal in signals.iter().filter(|s| s.kind == SignalKind::Ao) {
        ao::generate(signal, opc, &mut tags);
    }

    Ok(tags)
}

fn path(signal: &Signal, suffix: &str) -> String {
    format!(
        "{}.{}.{}",
        signal.kind.group(),
        signal.name,
        suffix,
    )
}

fn source(
    signal: &Signal,
    opc: &OpcConfig,
    suffix: &str,
) -> OpcSource {
    OpcSource::Node {
        connection: opc.connection,
        namespace: opc.namespace,
        data_type: opc.data_type.clone(),
        node_id: format!(
            "{}.{}.{}.{}",
            opc.application,
            signal.kind.group(),
            signal.name,
            suffix,
        ),
    }
}

fn info_tags(
    signal: &Signal,
    opc: &OpcConfig,
    names: &[&str],
) -> Vec<Tag> {
    names
        .iter()
        .map(|name| {
            let suffix = format!("Info.{name}");
            let mut tag = Tag::new(path(signal, &suffix));

            tag.add_property(TagProperty::DataType(
                TagDataType::String
            ));
            tag.add_property(TagProperty::ReadWriteDefinition(
                OpcDirection::Read
            ));
            tag.add_property(TagProperty::OpcStringConnection(
                source(signal, opc, &suffix)
            ));
            tag.add_property(TagProperty::AccessMode(
                OpcDirection::Read
            ));

            tag
        })
        .collect()
}