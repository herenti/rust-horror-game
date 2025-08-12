/*
 * This code might not be functional.
 * existential horror. the characer explores a dark mansion. the mansion is really a projection of the characters ego to protect itself from being without a body. at the end of the game character will realize they are an extention of the players conciousness as the player plays the game. character will see clues as to the seeing eye which is the players eye.
 * game mechanics will include choosing doors to move forward, and moving back to the previous area. items can be found such as keys, clues, notes, story objects, puzzle items. puzzles for obtaining some items.
 * character will have a book item in the beginning that will offer info about game commands to use and what certain items are, and progress info.
 * game interaction through text commmands.
 * keep it simple and have fun. keep it scary! this is not reality, no need to make it not be scary, but consider an ending where the player can consider if the characters fear was false, or maybe it was true doom. or just have an ending of no uncertain doom.
 * monsters to avoid in the game?????? traps? poisons?
 */



struct Commands {


}

impl Commands {
    fn new(command: &str) -> String {
        match command {

         "help" => {
            "Use the tome to look up items and progress info. For example use the game command \"tome itemnamehere\" or \"tome progress\". Check list of game commands with \"tome game commands\", and check a list of the questions the tome will answer with \"tome questions\".".to_String()
        },
    }

    }

}



struct Tome {
    tome: String,
}

impl Tome{
    fn new (term: &str) -> String {
        let info = Iteminfo {
            tome: "A dusty and heavy tome that I found upon first waking up. It has an eye on the front cover that fills me with a strange sense of fear. When I open it with a question in mind it seems to have an answer ready. Is it magic?",
        },
        match term {
            "tome" => {
                info.tome
            }

            }
        }

}


struct User {
    name: String,
    items: Vec<String>,
    health: i32,
}

fn main() {
    println!("Hello, world!");
}
