# src/lib.rs

## Purpose

Migrated implementation source for `src/lib.rs`.

## Contract

- Preserve the behavior of the pre-migration Rust source.
- This file is synchronized into `.build/rust/mds/core/src/lib.rs`.

## Source

````rs
mod adapter;
pub mod config;
pub mod descriptor;
pub mod diagnostics;
mod diff;
mod doctor;
mod fs_utils;
mod generation;
mod hash;
mod init;
mod manifest;
pub mod markdown;
pub mod model;
mod new;
pub mod package;
mod package_sync;
mod quality;
mod runner;
pub mod table;
````

````rs
pub use diagnostics::{Diagnostic, RunState, Severity};
pub use model::{
    AgentKitCategory, AiTarget, BuildMode, CliRequest, CliResult, Command, Config, DocKind,
    DoctorFormat, GeneratedFile, GeneratedKind, ImplDoc, InitOptions, InitQualityCommands,
    InitTargetCategories, LabelPreset, Lang, NewOptions, OutputKind, Package,
    PackageMetadata, PythonTool, QualityConfig, Roots, RustTool, TypeScriptTool,
};
pub use runner::execute;
````