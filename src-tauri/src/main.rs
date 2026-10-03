// Prevents additional console window on Windows in release, do not remove!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Serialize, Deserialize};
use std::fs::{self, File, OpenOptions};
use std::io::{Write, BufRead, BufReader};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit};
use rand::RngCore;
use base64::{engine::general_purpose, Engine as _};
use std::env;

fn get_file_name() -> String {
    env::var("FILE_NAME").expect("FILE_NAME must be set in .env")
}

fn get_master_key() -> [u8; 32] {
    let key_str = env::var("MASTER_KEY").expect("MASTER_KEY must be set in .env");
    let bytes = key_str.as_bytes();
    let mut key = [0u8; 32];
    let len = if bytes.len() >= 32 { 32 } else { bytes.len() };
    key[..len].copy_from_slice(&bytes[..len]);
    key
}

#[derive(Serialize, Deserialize, Clone)]
struct AppData {
    name: String,
    uname: String,
    passwd: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct AppEntry {
    app: String,
    data: AppData,
}

// 1. SAVE COMMAND
#[tauri::command]
fn save_entry(app: String, uname: String, passwd: String) -> String {
    let encrypted_pass = encrypt(&passwd);

    let new_entry = AppEntry {
        app: app.clone(),
        data: AppData {
            name: app,
            uname,
            passwd: encrypted_pass,
        },
    };
    let json_string = serde_json::to_string(&new_entry).expect("Serialize error");
    
    let mut json_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(get_file_name())
        .expect("File open error");

    if let Err(_) = writeln!(json_file, "{}", json_string) {
        return "Failed to write to file".to_string();
    }
    "Entry Saved Successfully!".to_string()
}

// 2. SEARCH COMMAND
#[tauri::command]
fn search_entry(target: String) -> Result<AppEntry, String> {
    let file = File::open(get_file_name()).map_err(|_| "No entries found.")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(content) = line {
            if let Ok(entry) = serde_json::from_str::<AppEntry>(&content) {
                if entry.app == target {
                    let mut found_entry = entry; 
                    found_entry.data.passwd = decrypt(&found_entry.data.passwd)?;
                    return Ok(found_entry);
                }
            }
        }
    }
    Err(format!("No entry found for '{}'", target))
}

// 3. DELETE COMMAND
#[tauri::command]
fn delete_entry(target: String) -> String {
    let file_name = get_file_name();
    let file = match File::open(&file_name) {
        Ok(f) => f,
        Err(_) => return "File not found.".to_string(),
    };

    let reader = BufReader::new(file);
    let temp_name = "temp_passwords.json";
    let temp_file = File::create(temp_name).unwrap();
    let mut writer = std::io::BufWriter::new(temp_file);
    let mut found = false;

    for line in reader.lines() {
        let content = line.unwrap();
        if let Ok(entry) = serde_json::from_str::<AppEntry>(&content) {
            if entry.app == target {
                found = true;
                continue;
            }
        }
        writeln!(writer, "{}", content).unwrap();
    }

    drop(writer);

    if found {
        fs::remove_file(&file_name).unwrap();
        fs::rename(temp_name, &file_name).unwrap();
        format!("'{}' removed successfully.", target)
    } else {
        fs::remove_file(temp_name).ok();
        format!("'{}' not found.", target)
    }
}




fn encrypt(data: &str) -> String {
    let master_key = get_master_key();
    let key = Key::<Aes256Gcm>::from_slice(&master_key);
    let cipher = Aes256Gcm::new(key);
    
    // Nonce must be unique per encryption to stay secure
    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher.encrypt(nonce, data.as_bytes()).expect("encryption failure!");
    
    // We store the Nonce + Ciphertext together so we can decrypt it later
    let mut combined = nonce_bytes.to_vec();
    combined.extend(ciphertext);
    
    general_purpose::STANDARD.encode(combined)
}

fn decrypt(encoded_data: &str) -> Result<String, String> {
    let combined = general_purpose::STANDARD.decode(encoded_data).map_err(|_| "Decode error")?;
    let (nonce_bytes, ciphertext) = combined.split_at(12);
    
    let master_key = get_master_key();
    let key = Key::<Aes256Gcm>::from_slice(&master_key);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(nonce_bytes);

    let plaintext = cipher.decrypt(nonce, ciphertext).map_err(|_| "Decryption failed (wrong key?)")?;
    
    String::from_utf8(plaintext).map_err(|_| "UTF8 error".to_string())
}

fn main() {
    dotenvy::dotenv().ok();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            save_entry, 
            search_entry, 
            delete_entry
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}