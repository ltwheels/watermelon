    use open;
    use serde_json::{Value};
    use std::{
        fs::{self, DirEntry},
    };

    const STEAM_PATH_WINDOWS: &str = "C:\\Program Files (x86)\\Steam\\steamapps";
    const STEAM_RUNGAME_STRING: &str =  "steam://rungameid/";
    const STEAM_API_PATH: &str = "http://store.steampowered.com/api/appdetails/";

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
                let run_string = format!("{}{}",STEAM_RUNGAME_STRING, self.appid);
                open::that(run_string).unwrap();
            }
            else if self.store == "EGS"
            {
                todo!();
            }
        }
    }

    fn get_egsids() -> Vec<String>
    {
        //TODO
        let v: Vec<String> = Vec::new();
        v
    }

    fn get_steamids() -> Vec<String>
    {   
        let mut id_list: Vec<String> = Vec::new();
        let files = fs::read_dir(STEAM_PATH_WINDOWS).unwrap();
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

   fn get_game(id: String, store: String) -> Game
    {
        let client = reqwest::blocking::Client::new();
        if store == "Steam"
        {
            let res: String = client.get(STEAM_API_PATH.to_owned())
                                    .query(&[("appids", &id)])
                                    .send().unwrap().text().unwrap();
            let jsn: Value = serde_json::from_str(&res).unwrap();
            
            let mut name: String = jsn[id.to_owned()]["data"]["name"].to_string();
            name = name[1..name.len()-1].to_owned();

            let g = Game {
                name: name,
                appid: id,
                store: store
            };

            g
        }
        else 
        {
                todo!();
        }
    }

    pub fn setup_games() -> Vec<Game>
    {
        let mut games: Vec<Game> = Vec::new();
        let steamids: Vec<String> = get_steamids();
        let egsids: Vec<String> = get_egsids();

        for id in steamids
        {
        let g: Game =  get_game(id,   String::from("Steam"));
        games.push(g);
        }

        for _id in egsids
        {
            //let g: Game = get_game(id, String::from("EGS")).await;
            //games.push(g);
        }

        games
    }