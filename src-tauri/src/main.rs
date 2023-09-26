// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use rusqlite::{Connection, params};
mod db_utils;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    db_utils::initialize().expect("Failed to initialize the database");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, insert_tutees, insert_tutors, match_tutors_and_tutees])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    db_utils::close();
}

#[tauri::command]
fn insert_tutees(tutees: Vec<String>) -> String {
    db_utils::execute("INSERT INTO tutee (name) VALUES (?)", &[&tutees]).unwrap().to_string()
}

#[tauri::command]
fn insert_tutors(tutors: Vec<String>) -> String {
    db_utils::execute("INSERT INTO tutor (name) VALUES (?)", &[&tutors]).unwrap().to_string()

}

#[tauri::command]
fn match_tutors_and_tutees() -> Vec<String> {
    let tutors_result = db_utils::query_map(
        "SELECT * FROM tutor", 
        params![], 
        |row| row.get::<_, String>(0)
    );

    tutors_result.unwrap()
}

