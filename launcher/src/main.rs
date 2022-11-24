use eframe::egui;
use gameops::{Game, setup_games};

fn main() 
{
  let native_options = eframe::NativeOptions::default();
  eframe::run_native("Watermelon Game Launcher", native_options, Box::new(|cc| Box::new(GameLauncher::new(cc))));
}

#[derive(Default)]
struct GameLauncher{
  games: Vec<Game>
}

impl GameLauncher
{
  fn new(_cc: &eframe::CreationContext<'_>) -> GameLauncher
  {
    GameLauncher
    {
      games: setup_games()
    }
  }
}

impl eframe::App for GameLauncher
{

  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame)
  {
    egui::CentralPanel::default().show(ctx, |ui| 
      {
        for g in &self.games 
        {
          let title = g.name.replace("\"", "");
          if ui.button(title).clicked()
          {
            g.launch();
          }
        }
      });   
  }
}
