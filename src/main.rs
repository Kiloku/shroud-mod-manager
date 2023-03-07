use eframe::egui;
use egui::*;
use steamcmd::ModInfo;
mod steamcmd;
mod modfiles;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Shroud Mod Manager", native_options, Box::new(|cc| Box::new(ShroudGUI::new(cc))));

}
#[derive(Default)]
struct ShroudGUI {
    mod_url: String,
    mods: Vec<ModInfo>,
    download_requested: bool
}

impl ShroudGUI {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            mod_url: "".to_owned(),
            mods: Vec::new(),
            download_requested: false
        };
        Self::default()
    }
}

impl eframe::App for ShroudGUI {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.download_requested = false;
            ui.heading("Type in a Stellaris mod ID or Stellaris workshop URL");
            ui.heading("Then click Add, and repeat for each mod you want to install");
            ui.heading("Once your mod list is done, click Download and Install");
            ui.heading("Please note that the app will hang while the download happens. Do not close it.");

            if ui.text_edit_singleline(&mut self.mod_url).lost_focus() && ctx.input(|i| i.key_pressed(Key::Enter))
            {
                self.mods.push(steamcmd::parse_mod_url(self.mod_url.clone()));
                self.mod_url = String::new();
            }
            
            if ui.button("Add").clicked(){
                if self.mod_url.chars().all(char::is_numeric) {
                    let temp_url =  format!("https://steamcommunity.com/sharedfiles/filedetails/?id={}", self.mod_url);
                    self.mods.push(steamcmd::parse_mod_url(temp_url));
                } 
                else {
                    self.mods.push(steamcmd::parse_mod_url(self.mod_url.clone()));
                }
                self.mod_url = String::new();
            }
            self.mods.iter().for_each(|m| {
                ui.label(m.name.as_str());
            });
            if ui.button("Download and Install").clicked() {
                steamcmd::download_mod_list(self.mods.clone());
                modfiles::install_mod_list(self.mods.clone());
            }
            if ui.button("Install mods on workshop folder").clicked(){
                modfiles::install_workshop_mods();
            }
            /*if self.steam_pid.is_some() { 
                self.sys.refresh_processes();
                let steam_proc = self.sys.process(self.steam_pid.unwrap());
                if  steam_proc.is_some() {
                    ui.label("Downloading and installing, please wait.");
                    let mut stdout = String::new();
                    //self.steam_cmd.as_ref().unwrap().read_to_string(&mut stdout).unwrap();
                    //println!("{}", stdout);
                }
                else {
                    ui.label("Download finished."); 

                }
                //modfiles::install_mod_list(self.mods.clone());
            }*/
        });
    }
}
