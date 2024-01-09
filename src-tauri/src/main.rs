// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    fs::File,
    io::{self, Write},
    process::exit,
};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn set_locale(locale: &str) {
    if let Err(e) = set_locale_inner(locale) {
        eprintln!("{e}");
    }
    exit(0);
}

fn set_locale_inner(locale: &str) -> io::Result<()> {
    let mut f = File::create("/etc/locale.conf")?;
    f.write_all(b"LANG=")?;
    f.write_all(format!("{locale}\n").as_bytes())?;

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![set_locale])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
