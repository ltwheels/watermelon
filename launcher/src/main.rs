
use gameops::{Game, setup_games};

#[tokio::main]
async fn main() 
{
  let games: Vec<Game> =  setup_games().await;

  for game in games 
  {
    if game.appid.eq("236390")
    {
        println!("Running {}", game.name);
        game.launch()
    }
  }

}