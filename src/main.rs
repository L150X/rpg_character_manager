use std::fs; // Import fs for file functions

// Use crossterm for terminal manipulation and styling, as similar to C# as possible
use crossterm::{
    cursor::MoveTo,
    execute,                            // Import execute function for terminal commands
    style::{Color, SetForegroundColor}, // Import Color and SetForegroundColor for styling text
    terminal::{Clear, ClearType, size}, // Import Clear and ClearType for clearing the terminal
};

// Use std::io for input and output operations
// Without this, it would need to be std::io::stdin/stdout() every time
use std::io::stdin;
use std::io::stdout;

/// Reads a line of input from the user, returning it as a string
fn read_line() -> String {
    // Return type is String

    // Create a mutable String to hold the input
    let mut input: String = String::new();

    stdin() // Allows reading of keyboard input
        .read_line(&mut input) // Read a line of input into the mutable string by borrowing
        .expect("Failed to read line"); // Handle potential errors with expect

    // Trim empty space and newlines, convert it to a string, and return the input
    input.trim().to_string()
}

/// Reads a line and returns it as a integer. Loops if incorrect input. Upper and lower bound are inclusive.
fn get_int(lower_bound: i32, upper_bound: i32) -> i32 {
    // Loop around the asking and verification phase
    loop {
        let base_input = read_line(); // Read the user input

        // Convert the input to an integer safely
        let choice: i32 = match base_input.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Please enter a valid number.");
                continue; // Skip rest of loop, go back to asking phase
            }
        };

        // Only continue if the choice is one of the available options
        if !(lower_bound..upper_bound + 1).contains(&choice) {
            println!("Please enter a valid number between {lower_bound} and {upper_bound}");
            continue; // Skip rest of loop, go back to asking phase
        }
        break choice; // Break the loop and return choice
    }
}

/// Reads the art.txt to get title ASCII art
fn get_art(path: &str) -> String {
    // Read the contents of art.txt and put it into a string
    let art = fs::read_to_string(path).expect(&format!("Failed to read {path}")); // Handle errors. Format macro builds new string
    art // Return art
}

/// Simply clears the terminal
fn clear_terminal() {
    // Clear the terminal
    execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0)).expect("Issue clearing terminal");
}

/// Outputs a screen with money information
fn output_screen(art_path: &str, extra_info: &str) {
    // Clear the terminal
    clear_terminal();

    // Get and output title art
    line();
    let title_art = get_art(art_path);
    println!("{title_art}");
    line();

    // Output extra information
    println!("{extra_info}");
    line();
    
    // Set terminal text color to white
    execute!(stdout(), SetForegroundColor(Color::White)).unwrap();
}

/// Output a line across the screen
fn line() {
    let (width, _) = size().unwrap(); // Set width equal to the width of the console
    println!("{}", "-".repeat(width as usize)); // Repeat repeats a string n times, so fills the screen
}

/// Waits for the user to press enter
fn wait_for_enter() {
    line();
    println!("Press enter to continue");
    read_line();
}

/// Outputs the create a character title
fn create_character_title(){
    // Set terminal text color to green
    execute!(stdout(), SetForegroundColor(Color::Green)).unwrap();

    // Output title
    output_screen("assets/create_a_character_art.txt", "Creating a character!");
}

/// Create a character function
fn create_character(characters: &Vec<Character>) -> Character{
    // Output title
    create_character_title();

    // Get character name
    println!("Enter character name:");
    let name = loop { // Ensure the character name entered is not a duplicate name
        let name = read_line(); // Get name
        let mut duplicate = false; // Set duplicate bool

        // For every character
        for character in characters{
            // If the character has the same name as the name to be made
            if character.name.to_lowercase() == name.to_lowercase(){
                // Output that the name already exists as a character
                println!("A character with that name already exists");
                println!("Please enter a different name");
                duplicate = true; // Set duplicate to true
                break; // Break the for loop
            }
        }

        // If it was not a duplicate, break the loop and return name
        if !duplicate{
            break name;
        }
    };
    line();

    // Get character class
    println!("Choose a class:");
    println!("1. Warrior");
    println!("2. Mage");
    println!("3. Archer");
    println!("4. Tank");
    let class_choice = get_int(1,4);
    let (class_type, ability, hp, attack, defense) = match class_choice{ // Choose class type based on the class choice
        1 => ("Warrior","Power Strike", 100, 10, 1,),
        2 => ("Mage", "Fireball", 90, 12, 1,),
        3 => ("Archer", "Piercing Shot", 90, 14, 0,),
        4 => ("Tank", "Shield Bash", 120, 6, 4,),
        _ => unreachable!(), // Unreachable code as the int is verified
    };

    // Output that the character is successfully being made
    line();
    println!("The character is being made!");
    wait_for_enter();

    // Create and return the character
    Character{
        name,
        class_type: class_type.to_string(),
        ability: ability.to_string(),
        level: 1,
        hp,
        attack,
        defense,
    }
}

/// Character struct
struct Character {
    name: String,
    class_type: String,
    ability: String,
    level: i32,
    hp: i32,
    attack: i32,
    defense: i32,
}

/// View all characters
fn view_characters(characters: &Vec<Character>){
    // If there are no characters, output that there are none and go back to main menu
    if characters.is_empty(){
        println!("No characters created.");
        wait_for_enter();
        return;
    }

    // Set terminal text color to red
    execute!(stdout(), SetForegroundColor(Color::Red)).unwrap();
    // Output title
    output_screen("assets/view_characters_art.txt", "Viewing Characters");

    // Output the characters
    for character in characters{
        line();
        println!(
            "{} | {} | Ability: {} | Level: {} | HP: {} | ATK: {} | DEF: {} |",
            character.name,
            character.class_type,
            character.ability,
            character.level,
            character.hp,
            character.attack,
            character.defense,
        );
    }

    // Enter to continue back to main menu
    wait_for_enter();
}

/// Sorts the characters vector by what the user requests
fn sort_characters(characters: &mut Vec<Character>){
    // Set terminal text color to red
    execute!(stdout(), SetForegroundColor(Color::Red)).unwrap();
    // Output title
    output_screen("assets/search_and_modify_art.txt", "Searching characters...");
    
    // If there are no characters
    if characters.is_empty(){
        println!("No characters to sort.");
        return;
    }

    // Output options
    println!("Sort By:");
    println!("1: Name");
    println!("2. Level");
    println!("3. Attack");
    println!("4. HP");
    println!("5. Defense");
    let choice = get_int(1,5); // Get choice

    // Based on the choice
    match choice {
        1 => {
            characters.sort_by(|a,b| a.name.cmp(&b.name)); // Sort by name
        }
        2 => {
            characters.sort_by(|b,a| a.level.cmp(&b.level)); // Sort by level
        }
        3 => {
            characters.sort_by(|b,a| a.attack.cmp(&b.attack)); // Sort by attack
        }
        4 => {
            characters.sort_by(|b,a| a.hp.cmp(&b.hp)); // Sort by hp
        }
        5 => {
            characters.sort_by(|b,a| a.defense.cmp(&b.defense)); // Sort by defense
        }
        _ => {
            unreachable!();
        }
    }

    // Output that they are sorted
    line();
    println!("Characters Sorted!");
    wait_for_enter(); // Wait for enter key
}

/// Function to delete a character by name
fn delete_character(characters: &mut Vec<Character>) {
    // Set terminal text color to red
    execute!(stdout(), SetForegroundColor(Color::Red)).unwrap();
    // Output title
    output_screen("assets/delete_character_art.txt", "Deleting a character");

    // If there are no characters, output that
    if characters.is_empty() {
        println!("No characters exist.");
        return; // Return to main 
    }

    // Get character name to delete
    println!("Enter character name to delete:");
    let target = read_line();
    let mut index = None; // Create index variable

    // For all characters
    for i in 0..characters.len() {
        // If the character has the name of the target character
        if characters[i].name.to_lowercase() == target.to_lowercase() {
            index = Some(i); // Set the index
            break;
        }
    }

    // Match the index
    match index {
        // If the character was found
        Some(i) => {
            // Delete the character and output that
            println!("Deleted {}.", characters[i].name);
            characters.remove(i);
        }

        // If the character was not found
        None => {
            // Output that the character was not found
            println!("Character not found.");
        }
    }

    // Wait for enter press
    wait_for_enter();
}

/// Saves all characters as a text file
fn save_characters(characters: &mut Vec<Character>){
    // Get confirmation
    line();
    println!("Are you sure? Your current characters save file will be overwritten.");
    println!("Enter 1 for yes, 2 for no");
    let answer = get_int(1, 2); // Get input
    if answer == 2{ // If they do not want to,
        return; // Exit the loop
    }
    line();

    // Set output string
    let mut output = String::new();

    // For each character
    for character in characters{
        // Add each character to the output string
        output.push_str(&format!(
            "{},{},{},{},{},{},{}\n",
            character.name,
            character.class_type,
            character.ability,
            character.level,
            character.hp,
            character.attack,
            character.defense
        ));
    }

    // Write output to the characters file
    match fs::write("characters/characters.txt", output){
        Ok(_) => { // If it works, output that it was a success
            println!("Characters saved successfully!");
        }
        Err(error) => { // If it encounters an error, output that there was an error
            println!("Error saving file: {}", error);
        }
    }

    // Wait for enter key
    wait_for_enter();
}

/// Loads characters from a text file
fn load_characters(characters: &mut Vec<Character>){
    // Get confirmation
    line();
    println!("Are you sure? Your current characters will be cleared.");
    println!("Enter 1 for yes, 2 for no");
    let answer = get_int(1, 2); // Get input
    if answer == 2{
        return; // Exit the loop
    }
    line();

    // Clear old characters
    characters.clear();

    // Load contents of the file
    let contents = match fs::read_to_string("characters/characters.txt"){
        Ok(text) => text, // If everything works, use the text that was read
        Err(error) => { // If there is an error
            println!("Error loading file: {}", error); // Output error message
            wait_for_enter(); // Wait for input, then exit function
            return;
        }
    };

    // For each line of the read contents
    for line in contents.lines(){
        // Split it into parts while removing the commas inbetween
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 7{
            continue;
        }

        // Rebuild each character
        let character = Character{
        // For string values, convert them to being a string
        name: parts[0].to_string(), 
        class_type: parts[1].to_string(),
        ability: parts[2].to_string(),
        // For int32 values, convert it to int32. If it fails, unwrap to default (ex. 1 for level)
        level: parts[3].parse().unwrap_or(1),
        hp: parts[4].parse().unwrap_or(100),
        attack: parts[5].parse().unwrap_or(10),
        defense: parts[6].parse().unwrap_or(1),
        };

        // Add the built character
        characters.push(character);
    }

    // Output it was a success
    println!("Characters loaded successfully!");
    wait_for_enter(); // Wait for enter key
}

/// Searches for a character that the user names and allowes the user to modify the character
fn search_and_modify_character(characters: &Vec<Character>){
    // Set terminal text color to red
    execute!(stdout(), SetForegroundColor(Color::Red)).unwrap();
    // Output title
    output_screen("assets/search_and_modify_art.txt", "Searching characters...");

    // If there are no created characters, output there are none
    if characters.is_empty(){
        println!("No characters exist.");
        return; // Return to main
    }

    // Get name to search
    println!("Enter character name to find:");
    let search_name = read_line();

    // Create found variable
    let mut found = false;

    // For all characters, attempt to find the named character
    for character in characters{
        if character.name.to_lowercase() == search_name.to_lowercase(){ // Set them to lowercase
            println!("Character Found!");
            line();

            // Output the named character
            println!("Name: {}", character.name);
            println!("Class: {}", character.class_type);
            println!("Ability: {}", character.ability);
            println!("Level: {}", character.level);
            println!("HP: {}", character.hp);
            println!("Attack: {}", character.attack);
            println!("Defense: {}", character.defense);

            // Update found variable
            found = true;
            break; // Break the for loop
        }
    }

    // If there were no characters found with that name
    if !found{
        println!("Character not found."); // Output that there were none found
    }

    // Wait for enter to continue
    wait_for_enter();
}

/// Main program
fn main() {
    // Create characters vector
    let mut characters: Vec<Character> = Vec::new();

    // Loop around main cycle
    loop {
        // Set terminal text color to red
        execute!(stdout(), SetForegroundColor(Color::Red)).unwrap();

        // Output title
        output_screen("assets/rpg_character_manager_art.txt", "Welcome!");

        // Set terminal text color to white
        execute!(stdout(), SetForegroundColor(Color::White)).unwrap();

        // Output list of what the user could do
        println!("What would you like to do?");
        let choices = ["Create a Character", "View Characters", "Search and Modify a Character", 
        "Sort Characters", "Delete a Character", "Save Characters", "Load Characters", "Exit"]; // Create an array of choices

        // For each choice, output it with formatting (eg., 1. Create character)
        let mut count = 1;
        for choice in choices {
            println!("{count}. {choice}.");
            count += 1;
        }

        // Get user input
        line();
        println!("Enter a number corresponding to the action you want to:");
        let choice = get_int(1, choices.len() as i32);

        match choice {
            // Call a different method based on the input.
            1 => {
                let character = create_character(&characters); // Create the character
                characters.push(character); // Add the character to the vector
            }
            2 => {
                view_characters(&characters) // View the characters and pass a reference to the vector
            } 
            3 => {
                search_and_modify_character(&characters); // Call search and modify function
            }
            4 => {
                sort_characters(&mut characters); // Call sort characters function
            }
            5 => {
                delete_character(&mut characters); // Call delete character function
            }
            6 => {
                save_characters(&mut characters);
            }
            7 => {
                load_characters(&mut characters);
            }
            8 => {
                break; // Exit the terminal by breaking the loop
            }
            _ => unreachable!(), // Could not happen
        }
    }
}

