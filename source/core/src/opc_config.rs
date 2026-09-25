use crate::IntegritreeError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OpcConnection {
    pub module_number: u16,
    pub server_number: u16,
}

pub fn parse_opc_connection(xml: &str, ) -> Result<OpcConnection, IntegritreeError> {
    let document = roxmltree::Document::parse(xml).map_err(|err| IntegritreeError(
            format!("некорректный XML конфигурации OPC: {err}")
        ))?;

    let parameters = document
        .descendants()
        .find(|node| {
            node.has_tag_name("Node") && node.attribute("Name") == Some("Parameters")
        })
        .ok_or_else(|| IntegritreeError("в OPC XML не найден узел Parameters".into()))?;

    let property = |id: &str| {
        parameters.children().find(|node| {
            node.has_tag_name("Prop_BSTR") && node.attribute("Id") == Some(id)
        })
    };

    let module_number = match property("0") {
        None => 1,
        Some(node) => {
            let raw = node.text().unwrap_or("");
            let value = raw
                .split_once(';')
                .map_or(raw, |(before, _)| before)
                .trim();

            value.parse::<u16>().map_err(|_| { IntegritreeError("некорректный номер OPC-модуля в Id=0".into()) })?
        }
    };

    let server_number = match property("1") {
        None => {
            return Err(IntegritreeError("в OPC XML отсутствует номер сервера в Id=1".into()));
        }
        Some(node) => {
            let raw = node.text().unwrap_or("");
            let value = raw
                .split_once(';')
                .map_or(raw, |(before, _)| before)
                .trim();

            value.parse::<u16>().map_err(|_| {IntegritreeError("некорректный номер OPC-сервера в Id=1".into()) })?
        }
    };

    Ok(OpcConnection {module_number, server_number})
}