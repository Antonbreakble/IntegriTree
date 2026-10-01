use clap::{ArgAction, ArgGroup, Parser};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    name = "integritree",
    version,
    about = "Генератор дерева тегов IntegritySCADA",
    disable_help_flag = true,
    disable_version_flag = true,
    help_template = "IntegriTree {version}\n{about}\n\nИспользование: {usage}\n\nПараметры:\n{options}\n{after-help}",
    after_help = "Примеры:\n  integritree.exe --regul --project_dir C:\\Project --signal_dir C:\\Signals --out C:\\Result\\tags.csv\n  integritree.exe --codesys --project_dir C:\\Project --signal_dir C:\\Signals --out C:\\Result\\tags.csv",
    group(
        ArgGroup::new("opc_server")
            .required(true)
            .args(["regul", "codesys", "elisy"])
    )
)]
pub struct Args {
    /// OPC UA сервер Регул (ns=2, Application)
    #[arg(long)]
    pub regul: bool,

    /// OPC UA сервер CODESYS Control Win V3 x64 (ns=4)
    #[arg(long)]
    pub codesys: bool,

    /// OPC UA сервер ELSYMA (ns=4, |var|ELSYMA.Application)
    #[arg(long)]
    pub elisy: bool,

    /// Папка проекта IntegritySCADA
    #[arg(long = "project_dir", value_name = "ПАПКА")]
    pub project_dir: PathBuf,

    /// Папка с файлами сигналов
    #[arg(long = "signal_dir", value_name = "ПАПКА")]
    pub signal_dir: PathBuf,

    /// Файл для записи результата
    #[arg(long = "out", value_name = "ФАЙЛ")]
    pub out: PathBuf,
    
}