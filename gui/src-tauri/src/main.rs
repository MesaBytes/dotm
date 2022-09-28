#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[derive(serde::Serialize)]
struct DotfileEntry {
    pub source: String,
    pub destination: String,
}

#[tauri::command]
fn load_dotfiles() -> Result<Vec<DotfileEntry>, ()> {
    let mut dotfiles: Vec<DotfileEntry> = Vec::new();

    let contents = std::fs::read_to_string("/home/senpai/.config/dotm/dotm.db")
        .expect("Failed to load db path");

    // if contents.is_empty() {
    //     Err("No dotfiles found".into());
    // }

    for line in contents.lines() {
        let dotfile: Vec<_> = line.split(':').collect();

        dotfiles.push(DotfileEntry {
            source: dotfile[0].to_string(),
            destination: dotfile[1].to_string(),
        });
    }

    Ok(dotfiles)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![load_dotfiles])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
