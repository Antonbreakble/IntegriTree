/// Signal group used both in the input name and in the exported tree.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum SignalKind { Ai, Ao, Di, Do }

impl SignalKind {
    pub fn group(self) -> &'static str {
        match self {
            Self::Ai => "AIs",
            Self::Ao => "AOs",
            Self::Di => "DIs",
            Self::Do => "DOs"
        }
    }

    pub(crate) fn from_name_prefix(name: &str) -> Option<Self> {
        if name.starts_with("AI") { Some(Self::Ai) }
        else if name.starts_with("AO") { Some(Self::Ao) }
        else if name.starts_with("DI") { Some(Self::Di) }
        else if name.starts_with("DO") { Some(Self::Do) }
        else { None }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signal {
    pub name: String,
    pub kind: SignalKind,
    pub description: String,
}
