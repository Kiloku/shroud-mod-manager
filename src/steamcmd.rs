use std::{process::{Command, self, Stdio, Child}, io::{self, Write, Read}};
use url::{Url};
use webpage::{Webpage, WebpageOptions};
use sysinfo::{Pid, PidExt};
//use duct::{cmd, ReaderHandle};

pub fn generate_mod_download_command(id : String) -> String { 
    return format!("+workshop_download_item 281990 {}", id);
}
pub fn generate_forced_download_command(id : String) -> String { 
    return format!("+download_item 281990 {}", id);
}

#[derive(Clone)]
pub struct ModInfo{
    pub id: String,
    pub name: String
}

impl Default for ModInfo{ 
    fn default() -> ModInfo {
        ModInfo {id: String::new(), name: String::new()}
    }
}

pub fn parse_mod_url(address : String) -> ModInfo {
    let mod_url = match Url::parse(address.as_str()){
        Ok(url) => url,
        Err(_err) => {
            return ModInfo::default()
        }
    };
    
    if mod_url.host_str().unwrap_or_default() == "steamcommunity.com" && mod_url.path() == "/sharedfiles/filedetails/" {
        return mod_url.query_pairs().find_map(|(key, value)| {
            if key == "id" {
                let info = Webpage::from_url(mod_url.as_str(), WebpageOptions::default())
                .expect("Could not read from URL");
                return Some(ModInfo{id: value.to_string(), name: String::from(info.html.title.unwrap_or_default().split("::").nth(1).unwrap_or_default())});
            } else {
                return None;
            }
        } ).unwrap_or_default();
    }
    return ModInfo::default();
}

pub fn download_mod_list(ids : Vec<ModInfo>) {
    
    let mut commands: Vec<String> = Vec::new();
    for v in ids {
        commands.push(generate_mod_download_command(v.id.to_string()));
    }
    call_steamcmd_with_args(commands);
    //return call_steamcmd_with_args_duct(commands);
}

/*pub fn call_steamcmd_with_args_duct(args : Vec<String>) {
    let mut full_args = args.clone();
    full_args.insert(0, "+login anonymous".to_string());
    //full_args.push("+logout".to_string());
    for a in full_args.clone() {
        println!("{}", a);
    }
    cmd("steamcmd", full_args).run();
    //let mut steam = cmd("steamcmd", full_args);
    
    //return steam.reader().unwrap(); 
}*/

pub fn call_steamcmd_with_args(args : Vec<String>)  {
    let mut steam = Command::new("steamcmd")
    .arg("+login anonymous")
    .args(args) 
    .arg("+quit")
    .stdout(Stdio::piped())
    .spawn()
    .expect("Failed to run steamcmd");
    
    steam.wait();

    let mut out = steam.stdout.unwrap();
    let mut res = String::new(); 
    out.read_to_string(&mut res);
    //Do check results outside this function. This should return the results so it can be handled elsewhere.
    check_results(res);
}

fn check_results(results : String) {
    let lines = results.split("\n");
    let mut errored_ids : Vec<String> = Vec::new();
    for l in lines {
        if l.contains("ERROR!"){ 
            for w in l.split(" ") {
                if w.chars().all(char::is_numeric){
                    errored_ids.push(w.to_string());
                }
            }
        }
    }

}