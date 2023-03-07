use std::fs;
use egui::ModifierNames;
use fs_extra::dir::{self, CopyOptions};
use home::{self, home_dir};
use std::path::Path;
use std::io::Write;
use std::collections::{HashSet, HashMap};

use crate::steamcmd::ModInfo;
static STELLARIS_WORKSHOP_PATH: &str = "/.steam/steam/steamapps/workshop/content/281990";
static STELLARIS_MODS_FOLDER: &str = "/.local/share/Paradox Interactive/Stellaris/mod";

fn get_stellaris_workshop_folder() -> String {
    return format!("{}{}", home_dir().unwrap().display(), STELLARIS_WORKSHOP_PATH);
}

fn get_stellaris_mods_folder() -> String {
    return format!("{}{}", home_dir().unwrap().display(), STELLARIS_MODS_FOLDER);
}

pub fn convert_steam_mod_to_stellaris(mod_id: String) {
    let steam_mod_path = format!("{}/{}", get_stellaris_workshop_folder(), mod_id);
    let stellaris_mod_path = format!("{}/{}", get_stellaris_mods_folder(), mod_id);

    if let Err(e) = fs::create_dir_all(&stellaris_mod_path) {
        eprintln!("Error creating directory {}: {}", &stellaris_mod_path, e);
        return;
    }

    if let Err(e) = dir::copy(&steam_mod_path, &get_stellaris_mods_folder(), &CopyOptions::new()){
        eprintln!("Error copying directory {}: {}", &steam_mod_path, e);
        return;
    }

    let descriptor_path = Path::new(&stellaris_mod_path).join("descriptor.mod");
    let new_descriptor_path = Path::new(&stellaris_mod_path).parent().unwrap().join(format!("{}.mod", mod_id));

    if let Err(e) = fs::rename(&descriptor_path, &new_descriptor_path) {
        eprintln!("Error renaming file {}: {}", &descriptor_path.display(), e);
        return;
    }

    let mut mod_file = fs::OpenOptions::new()
        .append(true)
        .open(&new_descriptor_path)
        .unwrap();
    let line = format!("\npath=\"{}\"", stellaris_mod_path);
    writeln!(mod_file, "{}", line).unwrap();
    mod_file.sync_all();
}

pub fn install_workshop_mods() {
    let workshop_mods = fs::read_dir(get_stellaris_workshop_folder()).unwrap().filter_map(|e| e.ok()).map(|e| e.file_name()).collect::<HashSet<_>>();
    let installed_mods = fs::read_dir(get_stellaris_mods_folder())
    .unwrap()
    .filter_map(|e| {
        if let Ok(entry) = e{ 
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_dir() {
                    return Some(entry.file_name());
                }
            }
        }
        return None
    })
    .collect::<HashSet<_>>();
    
    let missing : HashSet<_> = workshop_mods.difference(&installed_mods).collect();
    let mut to_install : Vec<ModInfo> = Vec::new();
    for entry in missing {
        let converted = entry.clone().into_string();
        match converted {
            Ok(string) => to_install.push(ModInfo { id:string, name: String::new() }),//println!("{}", string), //convert_steam_mod_to_stellaris(string.clone()),
            Err(entry) => println!("Failed to install {:?}", entry),
        }
    }
    install_mod_list(to_install);
}

pub fn install_mod_list(mods : Vec<ModInfo>)
{
    for info in mods {
        convert_steam_mod_to_stellaris(info.id);
    }
}

pub fn get_mods_on_workshop() {
    let mut default_steam_stellaris_path = format!("{}{}", home_dir().unwrap_or_default().display(), STELLARIS_WORKSHOP_PATH);  
    for file in fs::read_dir(default_steam_stellaris_path).unwrap(){
        println!("{}", file.unwrap().path().display());
    }
}

pub fn get_mods_on_stellaris() {
    let mut default_gog_stellaris_path = format!("{}{}", home_dir().unwrap_or_default().display(), STELLARIS_MODS_FOLDER);
    for file in fs::read_dir(default_gog_stellaris_path).unwrap(){
        println!("{}", file.unwrap().path().display());
    }
}
