use crate::{Signal, SignalKind, IntegritreeError};
pub fn parse_gvl(bytes: &[u8]) -> Result<Vec<Signal>, IntegritreeError> {
    let content = std::str::from_utf8(bytes)
        .map_err(|err| IntegritreeError(format!("GVL должен быть в UTF-8: {err}")))?;

    let mut signals = Vec::new();

    for (_, raw_line) in content.lines().enumerate() {
        let line = raw_line.trim_start();

        let Some(kind) = SignalKind::from_name_prefix(line) else { continue };
        let Some(end) = line.find(char::is_whitespace) else { continue };
        let name = &line[..end];
        let description = line.split_once("//").map_or("", |(_, comment)| comment.trim());

        signals.push(Signal { name: name.to_string(), kind, description: description.to_string() });
    }
    Ok(signals)
}

