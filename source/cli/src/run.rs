use crate::project::find_opc_config;
use crate::{files::list_files, Args};
use integritree_core::{export, generate, parse_gvl, OpcConfig, Signal, parse_opc_connection, OpcServerProfile};
use std::error::Error;
use std::fs;

pub fn run(args: Args) -> Result<String, Box<dyn Error>> {
    if !args.project_dir.is_dir() {
        return Err(format!(
            "папка проекта не найдена: {}",
            args.project_dir.display()
        )
            .into());
    }

    if !args.signal_dir.is_dir() {
        return Err(format!(
            "папка сигналов не найдена: {}",
            args.signal_dir.display()
        )
            .into());
    }

    let files = list_files(&args.signal_dir)?;
    if files.is_empty() {
        return Err("в папке сигналов нет файлов".into());
    }

    let mut signals = Vec::<Signal>::new();
    for file in files {
        let bytes = fs::read(&file)
            .map_err(|err| format!("{}: {err}", file.display()))?;

        let parsed_signals = parse_gvl(&bytes)
            .map_err(|err| format!("{}: {err}", file.display()))?;

        signals.extend(parsed_signals);
    }

    if signals.is_empty() {
        return Err("в файлах не найдены сигналы AI/AO/DI/DO".into());
    }

    // 2. Поиск конфигурации в проекте и её разбор ядром.
    let config_file = find_opc_config(&args.project_dir)?;
    let xml = fs::read_to_string(&config_file)
        .map_err(|err| format!("{}: {err}", config_file.display()))?;

    let connection = parse_opc_connection(&xml)
        .map_err(|err| format!("{}: {err}", config_file.display()))?;


    // 3. Генерация и запись.
    let profile = match (args.regul, args.codesys, args.elisy) {
        (true, false, false) => OpcServerProfile::Regul,
        (false, true, false) => OpcServerProfile::Codesys,
        (false, false, true) => OpcServerProfile::Elisy,
        _ => return Err("укажите ровно один флаг: --regul, --codesys или --elisy".into()),
    };

    let opc = OpcConfig::for_profile(connection, profile);

    let tags = generate(&signals, &opc)?;
    let content = export(&tags);

    fs::write(&args.out, content)
        .map_err(|err| format!("{}: {err}", args.out.display()))?;

    Ok(format!(
        "Готово: {} сигналов, {} тегов. Файл: {}",
        signals.len(),
        tags.len(),
        args.out.display(),
    ))
}