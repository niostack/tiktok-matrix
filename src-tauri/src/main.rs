// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use std::process::{Command, Stdio};

use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
mod ws;

#[derive(serde::Serialize)]
struct Settings {
    proxy_url: String,
    server_url: String,
    country: String,
    wifi_name: String,
    wifi_password: String,
}
fn setup_env() {
    let settings = get_settings().unwrap();
    std::env::set_var("PROXY_URL", &settings.proxy_url);
    std::env::set_var("SERVER_URL", &settings.server_url);
    std::env::set_var("COUNTRY", &settings.country);
    std::env::set_var("WIFI_NAME", &settings.wifi_name);
    std::env::set_var("WIFI_PASSWORD", &settings.wifi_password);
}
#[tauri::command]
fn get_settings() -> Result<Settings, String> {
    let db = PickleDb::load(
        "data/settings.db",
        PickleDbDumpPolicy::AutoDump,
        SerializationMethod::Json,
    )
    .unwrap_or_else(|_| {
        PickleDb::new(
            "data/settings.db",
            PickleDbDumpPolicy::AutoDump,
            SerializationMethod::Json,
        )
    });

    let local_ip = local_ip_address::local_ip().unwrap();
    let proxy_url = db
        .get::<String>("proxy_url")
        .unwrap_or_else(|| format!("{}:7890", local_ip));
    let server_url = db
        .get::<String>("server_url")
        .unwrap_or_else(|| format!("http://{}:8090", local_ip));
    let country = db
        .get::<String>("country")
        .unwrap_or_else(|| "UK".to_string());
    let wifi_name = db
        .get::<String>("wifi_name")
        .unwrap_or_else(|| "ChinaNet-LVBC".to_string());
    let wifi_password = db
        .get::<String>("wifi_password")
        .unwrap_or_else(|| "5bvwmej4".to_string());
    return Ok(Settings {
        proxy_url,
        server_url,
        country,
        wifi_name,
        wifi_password,
    });
}
#[tauri::command]
fn set_settings(
    proxy_url: Option<String>,
    server_url: Option<String>,
    country: Option<String>,
    wifi_name: Option<String>,
    wifi_password: Option<String>,
) {
    let mut db = PickleDb::new(
        "data/settings.db",
        PickleDbDumpPolicy::AutoDump,
        SerializationMethod::Json,
    );
    if let Some(url) = proxy_url {
        db.set("proxy_url", &url).unwrap();
    }
    if let Some(url) = server_url {
        db.set("server_url", &url).unwrap();
    }
    if let Some(country) = country {
        db.set("country", &country).unwrap();
    }
    if let Some(wifi_name) = wifi_name {
        db.set("wifi_name", &wifi_name).unwrap();
    }
    if let Some(wifi_password) = wifi_password {
        db.set("wifi_password", &wifi_password).unwrap();
    }
}
#[tauri::command]
fn start_server() -> u32 {
    setup_env();

    let child = Command::new("bin/tiktok-server")
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to start server");
    return child.id();
}

#[tauri::command]
fn stop_server(pid: i32) {
    let _ = Command::new("taskkill")
        .args(&["/F", "/PID", &pid.to_string()])
        .spawn();
}
#[tauri::command]
fn start_agent() -> u32 {
    setup_env();
    let child = Command::new("bin/tiktok-agent")
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to start agent");
    return child.id();
}
#[tauri::command]
fn stop_agent(pid: i32) {
    //kill adb process
    let _ = Command::new("taskkill")
        .args(&["/F", "/IM", "adb.exe"])
        .status()
        .expect("failed to kill adb processes");
    let _ = Command::new("taskkill")
        .args(&["/F", "/PID", &pid.to_string()])
        .spawn();
}
#[tauri::command]
fn start_adb_server() -> u32 {
    //kill adb process
    let _ = Command::new("taskkill")
        .args(&["/F", "/IM", "adb.exe"])
        .status()
        .expect("failed to kill adb processes");

    let child = Command::new("bin/platform-tools/adb")
        .args(&["-a", "nodaemon", "server", "start"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to start adb server");
    return child.id();
}
#[tauri::command]
fn stop_adb_server(pid: i32) {
    let _ = Command::new("taskkill")
        .args(&["/F", "/PID", &pid.to_string()])
        .spawn();
}
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            start_server,
            stop_server,
            start_agent,
            stop_agent,
            start_adb_server,
            stop_adb_server,
            get_settings,
            set_settings
        ])
        .setup(|app| {
            tauri::async_runtime::spawn(async move {
                let _ = Command::new("taskkill")
                    .args(&["/F", "/IM", "adb.exe"])
                    .status()
                    .expect("failed to kill adb processes");
                //sleep 3 s
                std::thread::sleep(std::time::Duration::from_secs(3));
                ws::start_server(8092).await.unwrap();
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
