# IntegriTree

IntegriTree создаёт файл импорта тегов IntegritySCADA.

## Что подготовить

- Папку проекта, в который добавлен модуль **OPC UA Data Client**. Программа рекурсивно ищет в ней файл `OPC UA Data Client Module_Config.xml`.
- Папку с файлами сигналов в UTF-8. Тип сигнала определяется по префиксу имени переменной: `AI`, `DI`, `DO` или `AO`.

Пример файла сигналов:

```iecst
{attribute 'qualified_only'}
VAR_GLOBAL
    AO_Signal_1 : AnalogOutput;
    AO_Signal_2 : AnalogOutput;
    AO_Signal_3 : AnalogOutput;
END_VAR
```

Все три переменные будут распознаны как AO по префиксу `AO_`. Переменные без поддерживаемого префикса пропускаются.

## Как запустить

Скачайте `integritree.exe` на странице [Releases](https://github.com/Antonbreakble/IntegriTree/releases) и выполните:

```powershell
.\integritree.exe --project_dir "C:\Project" --signal_dir "C:\Signals" --out "C:\Result\signals.csv"
```

Программа обработает файлы из папки сигналов и запишет результат в `signals.csv`.

## Структура тегов

```text
AIs
└── AI_Signal
    ├── Events
    │   ├── Error
    │   ├── HAlarm
    │   ├── HHAlarm
    │   ├── LAlarm
    │   └── LLAlarm
    ├── Info
    │   ├── Channel
    │   ├── Description
    │   ├── Name
    │   └── Units
    ├── OUT
    │   ├── PV
    │   └── Status
    ├── PAR
    │   ├── DeadbandRate
    │   ├── FiltrationRate
    │   ├── H
    │   ├── HH
    │   ├── L
    │   ├── LL
    │   ├── PvHigh
    │   ├── PvLow
    │   └── PvShift
    └── SET
        ├── DeadbandRate
        ├── FiltrationRate
        ├── H
        ├── HH
        ├── L
        ├── LL
        ├── PvHigh
        ├── PvLow
        └── PvShift

AOs
└── AO_Signal
    ├── Info
    │   ├── Channel
    │   ├── Description
    │   ├── Name
    │   └── Units
    └── OUT
        ├── PV
        └── Status

DIs
└── DI_Signal
    ├── Info
    │   ├── Channel
    │   ├── Description
    │   └── Name
    └── OUT
        ├── PV
        └── Error

DOs
└── DO_Signal
    ├── Info
    │   ├── Channel
    │   ├── Description
    │   └── Name
    └── OUT
        └── PV
```

Теги `PAR` предназначены для чтения параметров, `SET` — для их записи.

## Ограничения

- В проекте должен быть файл `OPC UA Data Client Module_Config.xml`.
- Сейчас поддерживается только один модуль **OPC UA Data Client** на проект. Работа с несколькими такими модулями не реализована.