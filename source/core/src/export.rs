use crate::Tag;
use std::fmt::Write;

pub fn export(tags: &[Tag]) -> String {
    let mut result = String::new();

    for tag in tags {
        for property in tag.properties() {
            let (id, value) = property.encode();

            writeln!(
                result,
                "{},{},{},{}",
                tag.path(),
                id,
                value.variant_name(),
                value,
            )
                .expect("запись в String");
        }
    }
    result
}