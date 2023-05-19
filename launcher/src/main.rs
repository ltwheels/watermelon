use eframe::egui::{self, Align, Layout, ScrollArea, Separator};
use gameops::{setup_games, Game};

const PADDING: f32 = 5.0;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Watermelon Game Launcher",
        native_options,
        Box::new(|cc| Box::new(GameLauncher::new(cc))),
    );
}

struct GameLauncher {
    games: Vec<Game>,
}

impl GameLauncher {
    fn new(_cc: &eframe::CreationContext<'_>) -> GameLauncher {
        GameLauncher {
            games: setup_games(),
        }
    }
}

impl eframe::App for GameLauncher {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                for g in &self.games {
                    ui.add_space(PADDING);
                    ui.label(&g.name);
                    ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                        if ui.button("Launch Game").clicked() {
                            g.launch();
                        }
                    });
                    ui.add_space(PADDING);
                    ui.add(Separator::default());
                }
            });
        });
    }
}
