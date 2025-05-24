#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::File;
use std::io::{Read, Write};

use aes_gcm::{
    aead::{Aead, KeyInit, OsRng, rand_core::RngCore},
    Aes256Gcm, Nonce,
};

use base64::{engine::general_purpose, Engine as _};

const SECRET_KEY: &[u8; 32] = b"01234567012345670123456701234567"; // à changer pour une vraie clé secrète

#[tauri::command]
fn save_note(content: String) -> Result<(), String> {
    let key =  aes_gcm::Key::<aes_gcm::Aes256Gcm>::from_slice(SECRET_KEY);
    let cipher = Aes256Gcm::new(key);

    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let encrypted = cipher
        .encrypt(nonce, content.as_bytes())
        .map_err(|e| e.to_string())?;

    let data = format!(
        "{}:{}",
        general_purpose::STANDARD.encode(nonce_bytes),
        general_purpose::STANDARD.encode(encrypted)
    );

    let mut file = File::create("note.txt").map_err(|e| e.to_string())?;
    file.write_all(data.as_bytes()).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn load_note() -> Result<String, String> {
    let mut file = File::open("note.txt").map_err(|e| e.to_string())?;
    let mut content = String::new();
    file.read_to_string(&mut content).map_err(|e| e.to_string())?;

    let parts: Vec<&str> = content.split(':').collect();
    if parts.len() != 2 {
        return Err("Format de fichier invalide".into());
    }

    let nonce_bytes = general_purpose::STANDARD
        .decode(parts[0])
        .map_err(|_| "Nonce invalide")?;
    let nonce = Nonce::from_slice(&nonce_bytes);

    let encrypted_bytes = general_purpose::STANDARD
        .decode(parts[1])
        .map_err(|_| "Données chiffrées invalides")?;

    let key = aes_gcm::Key::<aes_gcm::Aes256Gcm>::from_slice(SECRET_KEY);
    let cipher = Aes256Gcm::new(key);

    let decrypted = cipher
        .decrypt(nonce, encrypted_bytes.as_ref())
        .map_err(|_| "Erreur de déchiffrement")?;

    String::from_utf8(decrypted).map_err(|_| "Texte non UTF-8".into())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .invoke_handler(tauri::generate_handler![save_note, load_note])
        .run(tauri::generate_context!())
        .expect("Erreur au lancement de l'app");
}


// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// #[tauri::command]
// fn process_note(note: &str) -> String {
//     // Ici, on emprunte la note sans en prendre possession.
//     // Rust évite de copier inutilement la donnée.
//     format!("Note reçue (empruntée) : {}", note)
// }

// fn main() {
//     tauri::Builder::default()
//         .invoke_handler(tauri::generate_handler![process_note])
//         .run(tauri::generate_context!())
//         .expect("Erreur au lancement de Tauri");
// }