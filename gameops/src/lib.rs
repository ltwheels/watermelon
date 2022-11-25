use open;
use rayon::prelude::*;
use serde_json::{Value};
use std::{
    fs::{self, DirEntry, ReadDir},
    process,
};

const STEAM_PATH_WINDOWS: &str = "C:\\Program Files (x86)\\Steam\\steamapps";
const STEAM_RUNGAME_STRING: &str =  "steam://rungameid/";
const STEAM_API_PATH: &str = "http://store.steampowered.com/api/appdetails/";


const EPIC_PATH_WINDOWS: &str = "C:\\Program Files\\Epic Games";
const EPIC_GAMES_LAUNCH_PREFIX: &str = "com.epicgames.launcher://apps/";
const EPIC_GAMES_LAUNCH_SUFFIX: &str = "?action=launch&silent=true";

pub struct Game
{
    pub name: String,
    pub appid: String, 
    pub store: String,
}
impl Game {
    pub fn  launch(&self)
    {
        if self.store == "Steam"
        {
            let run_string: String = format!("{}{}",STEAM_RUNGAME_STRING, self.appid);
            open::that(run_string).unwrap();
        }
        else if self.store == "EGS"
        {
            let run_string: String = format!("{}{}{}", EPIC_GAMES_LAUNCH_PREFIX, self.appid, EPIC_GAMES_LAUNCH_SUFFIX);
            open::that(run_string).unwrap();
        }
        process::exit(0);
    }
}

fn get_egsgames() -> Vec<Game>
{
    let mut games: Vec<Game> = Vec::new();
    let folders: ReadDir = fs::read_dir(EPIC_PATH_WINDOWS).unwrap();

    for f in folders
    {
        let dir: DirEntry = f.unwrap();
        let name: String = dir.file_name().into_string().unwrap();
        println!("{}", name);
        let store: String = String::from("EGS");
        let mut appid: String = String::from("NaN");
        let next_path: String = dir.path().as_os_str().to_owned().into_string().unwrap();

        let files = fs::read_dir(next_path).unwrap().next().unwrap().unwrap().path().as_os_str().to_owned().into_string().unwrap();
        let id_path = fs::read_dir(files).unwrap();
        for file in id_path
        {
            let entry: DirEntry = file.unwrap();
            let isdir: bool = entry.file_type().unwrap().is_dir();
            if !isdir
            {
                
                let ext: String = entry.path().extension().unwrap().to_owned().into_string().unwrap();
                if ext == "mancpn"
                {
                    let file_to_read: String = entry.path().as_os_str().to_owned().into_string().unwrap();
                    let file_contents: String = fs::read_to_string(file_to_read).unwrap();
                    let jsn: Value = serde_json::from_str(&file_contents).unwrap();
                    let mut namespace = jsn["CatalogNamespace"].to_string();
                    namespace.pop();
                    namespace.remove(0);
                    let mut itemid = jsn["CatalogItemId"].to_string();
                    itemid.pop();
                    itemid.remove(0);
                    let mut app_name = jsn["AppName"].to_string();
                    app_name.pop();
                    app_name.remove(0);
                    appid = format!("{}%3A{}%3A{}", namespace, itemid, app_name);

                }
            }
        }

         let game = Game {
             name: name,
             store: store,
             appid: appid,
         };

         games.push(game);
    }
    games
}

fn get_steamids() -> Vec<String>
{   
    let mut id_list: Vec<String> = Vec::new();
    let files: ReadDir = fs::read_dir(STEAM_PATH_WINDOWS).unwrap();
    for file in  files 
    {
        let file_entry: DirEntry = file.unwrap();
        let name: String = file_entry.file_name().into_string().unwrap();
        let id: String = name.chars().filter(|c| c.is_digit(10)).collect();
        if id.len() > 0 && id != "228980"
        {
            id_list.push(id);
        }
    }
    id_list
}

fn get_steamgame(id: String) -> Game
{
    let client = reqwest::blocking::Client::new();

    let res: String = client.get(STEAM_API_PATH.to_owned())
                            .query(&[("appids", &id)])
                            .send().unwrap().text().unwrap();
    let jsn: Value = serde_json::from_str(&res).unwrap();
    
    let mut name: String = jsn[id.to_owned()]["data"]["name"].to_string();
    name = name[1..name.len()-1].to_owned();

    let game = Game {
        name: name,
        appid: id,
        store: String::from("Steam"),
    };
    game
}

pub fn setup_games() -> Vec<Game>
{
    let mut games: Vec<Game> = Vec::new();
    let steamids: Vec<String> = get_steamids();

    let mut steamgames: Vec<Game> = steamids.into_par_iter().map(|i| get_steamgame(i)).collect();
    games.append(&mut steamgames);

    let mut epicgames: Vec<Game> = get_egsgames();
    games.append(&mut epicgames);

    games
}