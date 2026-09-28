use clap::{ArgGroup, Parser};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    name = "integritree",
    about = "Генератор дерева тегов IntegritySCADA",
    group(
        ArgGroup::new("opc_server")
                    .required(true)
                    .args(["regul", "codesys"])
        )
)]



pub struct Args {

    /// OPC UA сервер Регул
    #[arg(long)]
    pub regul: bool,

    /// OPC UA сервер CODESYS Control Win V3 x64
    #[arg(long)]
    pub codesys: bool,

    /// Папка проекта IntegritySCADA
    #[arg(long = "project_dir", value_name = "DIR")]
    pub project_dir: PathBuf,

    /// Папка с файлами сигналов
    #[arg(long = "signal_dir", value_name = "DIR")]
    pub signal_dir: PathBuf,

    /// Файл для записи результата
    #[arg(long = "out", value_name = "FILE")]
    pub out: PathBuf,
}