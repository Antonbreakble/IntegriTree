use crate::opc_config::OpcConnection;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpcDirection {
    Read,
    Write,
}

impl OpcDirection {
    pub fn code(&self) -> u32 {
        match self {
            Self::Read => 1,
            Self::Write => 2,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpcSource {
    Empty {
        connection: OpcConnection,
    },
    Node {
        connection: OpcConnection,
        namespace: u16,
        data_type: String,
        node_id: String,
    },
}

impl OpcSource {
    pub fn serialize(&self) -> String {
        match self {
            Self::Empty { connection } => format!(
                "{}#{}###-1#",
                connection.module_number,
                connection.server_number,
            ),
            Self::Node {
                connection,
                namespace,
                data_type,
                node_id,
            } => format!(
                "{}#{}###-1#NS{namespace}|{data_type}|{node_id}",
                connection.module_number,
                connection.server_number,
            ),
        }
    }
}