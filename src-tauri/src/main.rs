// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::env;

use tauri::{CustomMenuItem, Menu, Submenu};
use tauri::api::dialog;

#[derive(Clone, serde::Serialize)]
struct Payload {
    filename: String,
    content: String,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn has_initial_content() -> Payload {

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Payload {
            filename: "".to_string(),
            content: "".to_string(),
        };
    }

    match read_file_from_command_line() {
        Ok(file_content) => {
            Payload {
                filename: args[1].clone(),
                content: file_content,
            }
        }
        Err(e) => {
            Payload {
                filename: args[1].clone(),
                content: e.to_string(),
            }
        }
    }

}

fn read_file_from_command_line() -> Result<String, std::io::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "File path not provided.",
        ));
    }

    let file_path = &args[1];
    let file_content = fs::read_to_string(file_path)?;

    Ok(file_content)
}

fn main() {
    let menu = Menu::new()
        .add_submenu(Submenu::new("File", Menu::new().add_item(CustomMenuItem::new("open".to_string(), "Open...")).add_item(CustomMenuItem::new("quit".to_string(), "Quit"))));

    tauri::Builder::default()
        .menu(menu)
        .on_menu_event(|event| {
            match event.menu_item_id() {
                "open" => {
                    dialog::FileDialogBuilder::default()
                        .add_filter("Markdown", &["md"])
                        .pick_file(move |path_buf| match path_buf {
                            Some(p) => {
                                let filename = p.clone();
                                if let Ok(file_content) = fs::read_to_string(p) {
                                    event.window().emit("open-file", Payload {
                                        filename: filename.to_str().unwrap().to_string(),
                                        content: file_content,
                                    }).unwrap();
                                }
                            }
                            _ => {}
                        });
                }
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![has_initial_content])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
