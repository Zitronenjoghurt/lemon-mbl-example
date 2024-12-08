use std::path::PathBuf;
use lemon_mbl::get_game_data;

#[cfg(test)]
mod tests;

fn main() {
    let game_data = get_game_data();
    
    let test_path = PathBuf::from("./").join("test.png"); 
    game_data.monster_images.save_to_file(0, test_path).unwrap()
}
