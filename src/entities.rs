use crate::util::Printable;

pub struct Player {
    pub name: String,
    pub money: i32,
    pub position: i32,
}
pub struct Property{
    pub name: String,
    pub price: i32,
}
pub struct Board {
    pub spaces: Vec<Property>
}
impl Player {
    //Player definition
    pub fn new(name: String) -> Self{
        Player{
            name,
            money: 1500,
            position: 0,
        }
    }

    //Move the player
    pub fn move_player(&mut self, spaces:i32){
        self.position = (self.position + spaces).rem_euclid(40);
        /*
        Some explanation about this code for those who may read it as this was my first time using this function as well:
        Normally we would just do a mod on 40(40 cuz thats how many spaces there are on a monopoly board) but rem_euclid is a fun function to get a euclidean remainder which is a slightly more fun way to do
        it and hey, maybe you learned a new function.

        I avoided going the u32 route here cuz some Chance or Community Chest cards move you back spaces, and in order to make game logic a bit easier in the future I just pre-empltively made the change here
        */
    }

    // Add money to player account
    pub fn add_money(&mut self, amount:i32){
        self.money += amount;
    }

    // Remove money from player accunt
    pub fn spend_money(&mut self, amount:i32){
        self.money -= amount;
    }

    // Check if the player went bankrupt
    pub fn is_bankrupt(&self) -> bool{
        self.money < 0
        // This will return True if the player has less than 0 credits left which means GG
    }
}

impl Property{
    pub fn new(name: String, price: i32) -> Self{
        Property {name, price}
    }
}

impl Board{
    pub fn new() -> Self{
        let mut spaces = Vec::<Property>::new();
        spaces.push(Property::new(String::from("GO"), 0));
        spaces.push(Property::new(String::from("Mediterranean Avenue"), 60));
        spaces.push(Property::new(String::from("Community Chest"), 0));
        spaces.push(Property::new(String::from("Baltic Avenue"), 60));
        spaces.push(Property::new(String::from("Income Tax"), 0));
        spaces.push(Property::new(String::from("Reading Railroad"), 200));
        Board {spaces}
    }
    
    pub fn get_space(&self, position:i32) -> &Property{
        &self.spaces[position as usize]
    }
}
impl Printable for Player {
    fn print_state(&self) {
        println!("name: {}, money: {}, position: {}", self.name, self.money, self.position);
    }
}

