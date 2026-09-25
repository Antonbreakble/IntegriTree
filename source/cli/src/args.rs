use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    name = "integritree",
    about = "Генератор дерева тегов IntegritySCADA"
)]
pub struct Args {
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