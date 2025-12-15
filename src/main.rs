mod util;
mod entities;

use util::Printable;
use entities::Player;
use crate::entities::{Board, Property};

fn main() {
    println!("In main function");

    let mut player = Player::new(String::from("Alex"));
    player.print_state();

    println!("Adding 200 credits to {}", player.name);
    player.add_money(200);
    player.print_state();
    
    println!("Removing 2000 credits from {}", player.name);
    player.spend_money(2000);
    
    if player.is_bankrupt() {
        player.print_state();
        println!(" Player is Bankrupt");
    }
    else{
        println!("Player is not Bankrupt");
    }
    
    // println!("Moving player {} 10 spaces", player.name);
    // player.move_player(10);
    // player.print_state();
    // 
    // println!("Player {} moved back 5 spaces", player.name);
    // player.move_player(-5);
    // player.print_state();

    let mut board = Board::new();
    println!("Board has {} spaces", board.spaces.len());

    for (i, property) in board.spaces.iter().enumerate() {
        println!(" {}: {} (${})", i , property.name, property.price);
    }
    
    let current_space = board.get_space(player.position);
    println!("Current Player {} is on space: {}",player.name, current_space.name);
    player.move_player(3);
    let new_space = board.get_space(player.position);
    
    println!("Player {} moved 3 spaces to {} and its cost is: {}", player.name, new_space.name, new_space.price);
}

