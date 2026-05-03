# src/lib.rs

## Purpose

Migrated implementation source for `src/lib.rs`.

## Contract

- Preserve the behavior of the pre-migration Rust source.
- This file is synchronized into `.build/rust/mds/core/src/lib.rs`.

## Source

````rs
mod adapter;
````

````rs
pub mod config;
````

````rs
pub mod descriptor;
````

````rs
pub mod diagnostics;
````

````rs
mod diff;
````

````rs
mod doctor;
````

````rs
mod fs_utils;
````

````rs
mod generation;
````

````rs
mod hash;
````

````rs
mod init;
````

````rs
mod manifest;
````

````rs
pub mod markdown;
````

````rs
pub mod model;
````

````rs
mod new;
````

````rs
pub mod package;
````

````rs
mod package_sync;
````

````rs
mod quality;
````

````rs
mod runner;
````

````rs
pub mod table;

pub use diagnostics::{Diagnostic, RunState, Severity};
pub use model::{
    AgentKitCategory, AiTarget, BuildMode, CliRequest, CliResult, Command, Config, DocKind,
    DoctorFormat, GeneratedFile, GeneratedKind, ImplDoc, InitOptions, InitQualityCommands,
    InitTargetCategories, LabelPreset, Lang, MetadataKind, NewOptions, OutputKind, Package,
    PackageMetadata, PythonTool, QualityConfig, Roots, RustTool, TypeScriptTool,
};
pub use runner::execute;
````