use colored::*; // Import everything from colored
use std::fs; // Import fs for file functions
// Use crossterm for terminal manipulation and styling, as similar to C# as possible
use crossterm::{
    cursor::MoveTo,                     // Import cursor move to to move the cursor
    cursor::MoveUp, // Import cursor move up to clear certain parts of the console instead of all
    execute,        // Import execute function for terminal commands
    style::{Color, SetForegroundColor}, // Import Color and SetForegroundColor for styling text
    terminal::{Clear, ClearType, size}, // Import Clear and ClearType for clearing the terminal
};
use std::io::Write;
use std::io::stdin; // Use std::io::stdin for input operations
use std::io::stdout; // Use std::io::stdout for output operations // Needed for flush()

/// Reads a line of input from the user, returning it as a string
fn read_line() -> String {
    // Create a mutable String to hold the input
    let mut input: String = String::new();
    stdin() // Allows reading of keyboard input
        .read_line(&mut input) // Read a line of input into the mutable string by borrowing
        .expect("Failed to read line"); // If an error occurs (is not Ok(_)), outputs the message and unwraps
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

    // Set terminal text color to gray
    execute!(stdout(), SetForegroundColor(Color::Grey)).unwrap();
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
    read_line(); // User must press enter to get past this
}

/// Outputs the create a character title
fn create_character_title() {
    // Set terminal text color to green
    execute!(stdout(), SetForegroundColor(Color::Green)).unwrap();

    // Output title
    output_screen("assets/create_a_character_art.txt", "Creating a character!");
}

/// Create a character function
fn create_character(characters: &Vec<Character>) -> Character {
    // Output title
    create_character_title();

    // Get character name
    println!("Enter character name:");
    let name = loop {
        // Ensure the character name entered is not a duplicate name
        let name = read_line(); // Get name
        let mut duplicate = false; // Set duplicate bool

        // For every character
        for character in characters {
            // If the character has the same name as the name to be made
            if character.name.to_lowercase() == name.to_lowercase() {
                // Output that the name already exists as a character
                line();
                println!("A character with that name already exists");
                println!("Please enter a different name");
                line();
                duplicate = true; // Set duplicate to true
                break; // Break the for loop
            }
        }

        // If it was not a duplicate, break the loop and return name
        if !duplicate {
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
    let class_choice = get_int(1, 4);
    let (class_type, ability, hp, attack, defense, speed) = match class_choice {
        // Choose class type based on the class choice
        1 => ("Warrior", "Power Strike", 100, 10, 2, 10),
        2 => ("Mage", "Fireball", 90, 12, 1, 12),
        3 => ("Archer", "Piercing Shot", 90, 14, 0, 16),
        4 => ("Tank", "Shield Bash", 120, 6, 5, 6),
        _ => unreachable!(), // Unreachable code as the int is verified
    };

    // Output that the character is successfully being made
    line();
    println!("The character is being made!");
    wait_for_enter();

    // Create and return the character
    Character {
        name,
        class_type: class_type.to_string(),
        ability: ability.to_string(),
        level: 1,
        xp: 0,
        max_hp: hp,
        hp,
        base_attack: attack,
        attack,
        defense,
        speed,
    }
}

/// Character struct
struct Character {
    // String values
    name: String,
    class_type: String,
    ability: String,

    // Level related
    level: i32,
    xp: i32,

    // HP related
    max_hp: i32,
    hp: i32,

    // Attack related
    base_attack: i32,
    attack: i32,

    // Other
    defense: i32,
    speed: i32,
}

/// Functions for the character struct
impl Character {
    /// Gives xp to the character and attempts to level them up
    fn give_xp(&mut self, xp: i32) {
        // Add the xp to the character
        self.xp += xp;
        println!("{} gained {} XP!", self.name, xp); // Output xp change
        // Attempt to level up the character
        self.level_up();
    }

    /// Levels up a specific character if possible
    fn level_up(&mut self) {
        // While the character has enough xp to level up
        while self.xp >= self.level * 100 {
            self.xp -= self.level * 100; // Remove the xp
            self.level += 1; // Add the level
            // Add statisitics for levelling up
            self.max_hp += 10;
            self.hp = self.max_hp; // Heal fully on level up
            self.base_attack += 2;
            self.attack = self.base_attack;
            self.defense += 1;
            self.speed += 1;

            // Output that the character levelled up and info regarding it
            println!("{} levelled up to level {}!", self.name, self.level);
        }
    }
}

/// View all characters
fn view_characters(characters: &Vec<Character>) {
    // If there are no characters, output that there are none and go back to main menu
    if characters.is_empty() {
        println!("No characters created.");
        wait_for_enter();
        return;
    }

    // Set terminal text color to yellow
    execute!(stdout(), SetForegroundColor(Color::Yellow)).unwrap();
    // Output title
    output_screen("assets/view_characters_art.txt", "Viewing Characters");

    // Output the characters
    for character in characters {
        line();
        println!(
            "{} | {} | Ability: {} | Level: {} | XP: {}/{} | HP: {}/{} | ATK: {} | DEF: {} | SPD: {} |",
            character.name,
            character.class_type,
            character.ability,
            character.level,
            character.xp,
            character.level * 100,
            character.hp,
            character.max_hp,
            character.attack,
            character.defense,
            character.speed,
        );
    }

    // Enter to continue back to main menu
    wait_for_enter();
}

/// Sorts the characters vector by what the user requests
fn sort_characters(characters: &mut Vec<Character>) {
    // Set terminal text color to magenta
    execute!(stdout(), SetForegroundColor(Color::Magenta)).unwrap();
    // Output title
    output_screen(
        "assets/search_and_modify_art.txt",
        "Searching characters...",
    );

    // If there are no characters
    if characters.is_empty() {
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
    let choice = get_int(1, 5); // Get choice

    // Based on the choice
    match choice {
        1 => {
            characters.sort_by(|a, b| a.name.cmp(&b.name)); // Sort by name
        }
        2 => {
            characters.sort_by(|b, a| a.level.cmp(&b.level)); // Sort by level
        }
        3 => {
            characters.sort_by(|b, a| a.attack.cmp(&b.attack)); // Sort by attack
        }
        4 => {
            characters.sort_by(|b, a| a.hp.cmp(&b.hp)); // Sort by hp
        }
        5 => {
            characters.sort_by(|b, a| a.defense.cmp(&b.defense)); // Sort by defense
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
    // Set terminal text color to cyan
    execute!(stdout(), SetForegroundColor(Color::Cyan)).unwrap();
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
    line(); // Spacing

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
fn save_characters(characters: &mut Vec<Character>) {
    // Get confirmation
    line();
    println!("Are you sure? Your current characters save file will be overwritten.");
    println!("Enter 1 for yes, 2 for no");
    let answer = get_int(1, 2); // Get input
    if answer == 2 {
        // If they do not want to,
        return; // Exit the loop
    }
    line();

    // Set output string
    let mut output = String::new();

    // For each character
    for character in characters {
        // Add each character to the output string
        output.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{},{}\n",
            character.name,
            character.class_type,
            character.ability,
            character.level,
            character.xp,
            character.max_hp,
            character.hp,
            character.base_attack,
            character.attack,
            character.defense,
            character.speed
        ));
    }

    // Write output to the characters file
    match fs::write("characters/characters.txt", output) {
        Ok(_) => {
            // If it works, output that it was a success
            println!("Characters saved successfully!");
        }
        Err(error) => {
            // If it encounters an error, output that there was an error
            println!("Error saving file: {}", error);
        }
    }

    // Wait for enter key
    wait_for_enter();
}

/// Loads characters from a text file
fn load_characters(characters: &mut Vec<Character>) {
    // Get confirmation
    line();
    println!("Are you sure? Your current characters will be cleared.");
    println!("Enter 1 for yes, 2 for no");
    let answer = get_int(1, 2); // Get input
    if answer == 2 {
        return; // Exit the loop
    }
    line();

    // Clear old characters
    characters.clear();

    // Load contents of the file
    let contents = match fs::read_to_string("characters/characters.txt") {
        Ok(text) => text, // If everything works, use the text that was read
        Err(error) => {
            // If there is an error
            println!("Error loading file: {}", error); // Output error message
            wait_for_enter(); // Wait for input, then exit function
            return;
        }
    };

    // For each line of the read contents
    for line in contents.lines() {
        // Split it into parts while removing the commas inbetween
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 11 {
            continue;
        }

        // Rebuild each character
        let character = Character {
            // For string values, convert them to being a string
            name: parts[0].to_string(),
            class_type: parts[1].to_string(),
            ability: parts[2].to_string(),
            // For int32 values, convert it to int32. If it fails, unwrap to default (ex. 1 for level)
            level: parts[3].parse().unwrap_or(1),
            xp: parts[4].parse().unwrap_or(0),
            max_hp: parts[5].parse().unwrap_or(100),
            hp: parts[6].parse().unwrap_or(100),
            base_attack: parts[7].parse().unwrap_or(10),
            attack: parts[8].parse().unwrap_or(10),
            defense: parts[9].parse().unwrap_or(1),
            speed: parts[10].parse().unwrap_or(10),
        };

        // Add the built character
        characters.push(character);
    }

    // Output it was a success
    println!("Characters loaded successfully!");
    wait_for_enter(); // Wait for enter key
}

/// Searches for a character that the user names and allowes the user to modify the character
fn search_and_modify_character(characters: &mut Vec<Character>) {
    // Set terminal text color to blue
    execute!(stdout(), SetForegroundColor(Color::Blue)).unwrap();
    // Output title
    output_screen(
        "assets/search_and_modify_art.txt",
        "Searching characters...",
    );

    // If there are no created characters, output there are none
    if characters.is_empty() {
        println!("No characters exist.");
        return; // Return to main
    }

    // Get name to search
    println!("Enter character name to find:");
    let search_name = read_line();
    line();

    // Create found variable
    let mut found = false;

    // For all characters, attempt to find the named character
    for character in characters.iter_mut() {
        if character.name.to_lowercase() == search_name.to_lowercase() {
            // Set them to lowercase
            println!("Character Found!");
            line();

            // Output the named character
            println!("Name: {}", character.name);
            println!("Class: {}", character.class_type);
            println!("Ability: {}", character.ability);
            println!("Level: {}", character.level);
            println!("XP: {}/{}", character.xp, character.level * 100);
            println!("HP: {}/{}", character.hp, character.max_hp);
            println!("Attack: {}", character.attack);
            println!("Defense: {}", character.defense);
            println!("Speed: {}", character.speed);

            // Revive them if they have 0 hp
            if check_if_dead(character) {
                println!("{} has been revived!", character.name);
                character.hp = character.max_hp; // Set their hp to their max hp
            }

            // Ask the user if they want to add xp
            line();
            println!("Would you like to add XP?");
            println!("1. Yes");
            println!("2. No");
            let choice = get_int(1, 2); // Get user choice

            // If they wanted to add xp
            if choice == 1 {
                // Clear the previous asking phase
                execute!(stdout(), MoveUp(4)).unwrap();
                execute!(stdout(), Clear(ClearType::FromCursorDown)).unwrap();
                // Get amount to add
                println!("How much XP would you like to add?");
                let xp_to_add = get_int(1, 100000);
                line();
                character.give_xp(xp_to_add); // Add the xp and attempt to level up
            }

            // Update found variable
            found = true;
            break; // Break the for loop
        }
    }

    // If there were no characters found with that name
    if !found {
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

        // Set terminal text color to grey
        execute!(stdout(), SetForegroundColor(Color::Grey)).unwrap();

        // Output list of what the user could do
        println!("What would you like to do?");
        let choices = [
            "Create a Character",
            "View Characters",
            "Search and Modify a Character",
            "Sort Characters",
            "Delete a Character",
            "Save Characters",
            "Load Characters",
            "Explore Forest",
            "Character Duel",
            "Exit",
        ]; // Create an array of choices

        // For each choice, output it with formatting (eg., 1. Create character)
        let mut count = 1;
        for choice in choices {
            println!("{count}. {choice}.");
            count += 1;
        }

        // Get user input
        line();
        println!("Enter a number corresponding to the action you want to do:");
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
                search_and_modify_character(&mut characters); // Call search and modify function
            }
            4 => {
                sort_characters(&mut characters); // Call sort characters function
            }
            5 => {
                delete_character(&mut characters); // Call delete character function
            }
            6 => {
                save_characters(&mut characters); // Call save characters function
            }
            7 => {
                load_characters(&mut characters); // Call load characters function
            }
            8 => {
                explore_forest(&mut characters); // Call explore forest function
            }
            9 => {
                character_battle(&mut characters); // Call character battle function
            }
            10 => {
                break; // Exit the terminal by breaking the loop
            }
            _ => unreachable!(), // Could not happen
        }
    }
}

/// Allows the user to choose a character for an adventure. Returns the character index
fn choose_character(characters: &Vec<Character>) -> usize {
    // Ask the user to choose a character
    line();
    println!("Choose a character:");
    // Output the characters
    for i in 0..characters.len() {
        println!("{}. {}", i + 1, characters[i].name);
    }
    // Get choice
    let choice = get_int(1, characters.len() as i32);
    (choice - 1) as usize // Return choice index
}

/// Allows the user to explore the forest which has random encounters, a map, etc., with a character.
fn explore_forest(characters: &mut Vec<Character>) {
    // If there are no characters
    if characters.is_empty() {
        // Output that there are no characters
        println!("No characters exist.");
        wait_for_enter();
        return; // Return to main
    }
    // Get character index of a chosen character
    let character_index = choose_character(characters);
    let character = &mut characters[character_index];

    // Set map size and create the map
    let width = 10;
    let height = 10;
    let map = create_map(width, height);
    let mut x: usize = 5; // Player x location
    let mut y: usize = 5; // Player y location

    // Main loop for exploration
    loop {
        // Set terminal text color to dark green
        execute!(stdout(), SetForegroundColor(Color::DarkGreen)).unwrap();
        // Output title
        output_screen("assets/forest_exploration_art.txt", "Exploring the forest!");

        // Check to see if the character is dead
        if check_if_dead(character) {
            // If so, output that they cannot continue
            println!(
                "{} cannot continue as they have 0 hp! Revive them in the search and modify area.",
                character.name
            );
            break; // Return to main
        }

        // Output the map
        output_map(&map, x, y);

        // Output details of the character that is currently exploring
        line();
        println!("Exploring as {}:", character.name);
        println!(
            "HP: {}/{} | ATK: {} | DEF: {}",
            character.hp, character.max_hp, character.attack, character.defense
        );
        line();

        // Get input
        let input = get_move();
        let mut player_moved = true;
        // Match the input to movement
        let (new_x, new_y) = match input.as_str() {
            "w" => (x, y.saturating_sub(1)),
            "s" => (x, (y + 1).min(height - 1)),
            "a" => (x.saturating_sub(1), y),
            "d" => ((x + 1).min(width - 1), y),
            "q" => break, // If the user wants to leave, break the loop
            _ => {
                // If the input was not a "WASD" input, do not move the player
                player_moved = false;
                (x, y)
             } 
        };

        // Prevent walking in walls
        if map[new_y][new_x] == '#' { // If the location to go to is a wall
            println!("Blocked by wall."); // Output that the character is blocked by a wall
            wait_for_enter(); // Wait for enter
            continue; // Restart loop
        }

        // Set new location
        x = new_x;
        y = new_y;

        // Trigger encounter after movement
        line();
        if player_moved{
            trigger_encounter(character); // Trigger event only if the player moved
        }

        // Wait for enter before continuing next loop
        wait_for_enter();
    }
}

/// Creates the map for forest exploring based on size
fn create_map(width: usize, height: usize) -> Vec<Vec<char>> {
    let mut map = vec![vec!['.'; width]; height]; // Use a "2d vector" for the map

    // Add borders around the edges
    for x in 0..width {
        map[0][x] = '#';
        map[height - 1][x] = '#'; // All top tiles become walls
    }
    for y in 0..height {
        map[y][0] = '#';
        map[y][width - 1] = '#'; // All side tiles become walls
    }
    map // Return the created map
}

/// Outputs the map
fn output_map(map: &Vec<Vec<char>>, player_x: usize, player_y: usize) {
    // For each tile
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            // If the player is on that tile
            if x == player_x && y == player_y {
                print!(" {} ", "P".blue().bold()); // Output a bold blue P to represent the player
            } else {
                print!(" {} ", map[y][x]); // Else, output regularly what it is (wall or grass '.')
            }
        }
        println!(); // Make a new line after each row
    }
}

/// Gets the movement from the user (WASD)
fn get_move() -> String {
    print!("Move (WASD, Q to Quit): "); // Output options
    stdout().flush().unwrap(); // Ensure it prints correctly

    // Get the input
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap(); // Unwrap if error

    // Trim empty space and put it into lowercase
    input.trim().to_lowercase()
}

/// Triggers a random encounter, intended for the forest
fn trigger_encounter(character: &mut Character) {
    // Roll a random number 0-100
    let roll = rand::random_range(0..=100);

    // 40% Chance for nothing to happen
    if roll < 40 {
        println!("Nothing happened!");
    } 
    // 20% Chance for a random goblin enemy
    else if roll < 60 {
        // Create a goblin that scales with character level
        let mut enemy = Enemy {
            name: "Goblin".to_string(),
            hp: 25 + character.level * 12,
            attack: 3 + character.level * 2,
            defense: character.level / 3,
        };
        // Begin combat with the goblin
        combat_loop(character, &mut enemy);
    } 
    // 15% Chance for the player to find a +5 hp potion
    else if roll < 75 {
        println!("Potion found (+5 hp)"); // Output that the player found it
        character.hp += 5; // Add the hp
    } 
    // 15% Chance to find a +50 XP orb
    else if roll < 90 {
        println!("XP orb found (+50 XP)"); // Output that the player found it
        character.give_xp(50) // Add the xp
    } 
    // 10% Chance for the player to find a +1 attack sword
    else {
        println!("Sword found (+1 attack)"); // Output that the player found it
        character.attack += 1; // Add temporary attack (does not go past the forest)
    }
}

/// Struct for a basic enemy
struct Enemy {
    name: String,
    hp: i32,
    attack: i32,
    defense: i32,
}

/// Main combat loop for PVE
fn combat_loop(player: &mut Character, enemy: &mut Enemy) {
    // Set terminal text color to dark red
    execute!(stdout(), SetForegroundColor(Color::DarkRed)).unwrap();
    // Output title
    output_screen("assets/combat_art.txt", "Fighting!");

    // Output that an enemy has appeared
    println!("A wild {} appears!", enemy.name);
    wait_for_enter();

    // Start combat loop
    loop {
        // Set terminal text color to dark red
        execute!(stdout(), SetForegroundColor(Color::DarkRed)).unwrap();
        // Output title
        output_screen("assets/combat_art.txt", "Fighting!");

        // Print current hp of the enemy and player
        println!("{} HP: {}/{}", player.name, player.hp, player.max_hp);
        println!("{} HP: {}", enemy.name, enemy.hp);

        // Output possible actions for the user
        line();
        println!("Choose action:");
        println!("1. Attack");
        println!("2. Use ability");
        let choice = get_int(1, 2); // Get choice

        // Match the choice to the action
        match choice {
            1 => { // If the player wanted to do a basic attack
                let damage = (player.attack - enemy.defense).max(1);
                enemy.hp -= damage;
            }

            2 => { // If the player wanted to do a special attack
                // Calculate damage
                let damage = (player.attack * 2 - enemy.defense).max(1);
                enemy.hp -= damage; // Reduce enemy hp by that damage
            }

            _ => { // Unreachable code due to the get_int function
                unreachable!();
            }
        }

        // Check if the enemy HP is below or equal to 0
        if enemy.hp <= 0 {
            // Set terminal text color to dark red
            execute!(stdout(), SetForegroundColor(Color::DarkRed)).unwrap();
            // Output title
            output_screen("assets/combat_art.txt", "Fighting!");

            // Output that the enemy has been defeated
            println!("Enemy defeated!");
            line();
            player.give_xp(50); // Give xp to the player
            break; // Break the combat loop
        }

        // Enemy damage calculation
        let enemy_damage = (enemy.attack - player.defense).max(1);
        player.hp -= enemy_damage; // Minus the players current hp by the enemy damage

        // Check if the player is dead
        if check_if_dead(player) {
            break; // If so, break the loop. It will be outputted at the start of the forest loop.
        }
    }
}

/// Checks if a character is dead. If so, it returns true. Else, it returns false.
fn check_if_dead(character: &mut Character) -> bool {
    // If the character has 0 or less hp
    if character.hp <= 0 {
        // Output that the character is dead
        println!("{} is dead!", character.name);
        character.hp = 0; // Set hp to 0
        true // Return true
    } else {
        // Otherwise, the character must be alive
        false // Return false
    }
}

/// Gets the user to choose two characters, intended for the character battles
fn choose_two_characters(characters: &Vec<Character>) -> (usize, usize) {
    // Get first fighter
    println!("Choose first fighter:");
    let fighter1 = choose_character(characters); // Let the user choose a character

    // Set terminal text color to Dark Yellow
    execute!(stdout(), SetForegroundColor(Color::DarkYellow)).unwrap();
    // Output title
    output_screen(
        "assets/duel_art.txt",
        "Duel to the death! (HP is not changed outside the arena)",
    );

    // Output text to prompt for the second fighter
    println!("Choose second fighter:");
    // Ensure the second fighter is not the same as the first
    loop {
        let fighter2 = choose_character(characters); // Get a choice from the user
        if fighter2 != fighter1 { // If the character is not the first as well
            return (fighter1, fighter2); // Return both selectedfighters
        }

        // Otherwise, output that the fighters cannot be the same
        println!("Cannot fight the same character.");
    }
}

/// Character PVP function. Handles everything to do with the PVP.
fn character_battle(characters: &mut Vec<Character>) {
    // Set terminal text color to Dark Yellow
    execute!(stdout(), SetForegroundColor(Color::DarkYellow)).unwrap();
    // Output title
    output_screen(
        "assets/duel_art.txt",
        "Duel to the death! (HP is not changed outside the arena)",
    );

    // If there are not two characters available
    if characters.len() < 2 {
        // Output that the user needs at least two characters
        println!("You need at least two characters.");
        wait_for_enter();
        return; // Return to main
    }

    // Get the user to choose both fighters
    let (fighter1_index, fighter2_index) = choose_two_characters(characters);
    // Borrow each fighter from the characters list
    let fighter1 = &characters[fighter1_index]; 
    let fighter2 = &characters[fighter2_index]; 

    // Ensure the hp does not carry back into the character list. HP will stay the same.
    let mut hp1 = fighter1.hp;
    let mut hp2 = fighter2.hp;

    // Set terminal text color to Dark Yellow
    execute!(stdout(), SetForegroundColor(Color::DarkYellow)).unwrap();
    // Output title
    output_screen(
        "assets/duel_art.txt",
        "Duel to the death! (HP is not changed outside the arena)",
    );

    // Output that the first fighter challenges the second
    println!("{} challenges {}!", fighter1.name, fighter2.name);
    wait_for_enter(); // Wait for enter press

    // Main combat loop
    loop {
        // Set terminal text color to red
        execute!(stdout(), SetForegroundColor(Color::Red)).unwrap();
        // Output title
        output_screen(
            "assets/duel_art.txt",
            "Duel to the death! (HP is not changed outside the arena)",
        );

        // Print the statistics of each fighter
        println!("{} HP: {}/{}", fighter1.name, hp1, fighter1.max_hp);
        println!("{} HP: {}/{}", fighter2.name, hp2, fighter2.max_hp);
        line();

        // Roll initiative for speed
        let initiative1 = fighter1.speed + rand::random_range(1..=20);
        let initiative2 = fighter2.speed + rand::random_range(1..=20);

        // If the first fighter had higher initiative
        if initiative1 >= initiative2 {
            // Get random damage multiplier between effectively 80%-120%
            let multiplier = rand::random_range(80..=120);
            let mut damage1 = (fighter1.attack * multiplier / 100 - fighter2.defense).max(1); // Calculate the damage

            // 10% Chance that the attack is a critical
            let critical = rand::random_range(1..=100) <= 10;

            // If it is a critical
            if critical {
                damage1 *= 2; // Mutliply damage by 2
            }
            hp2 -= damage1; // Minus the health of fighter 2 by fighter 1's damage

            // If it was a critical hit
            if critical {
                // Output the damage fighter 1 did to 2, and with special crit text
                println!(
                    "{} attacks {} for {} damage! {}",
                    fighter1.name,
                    fighter2.name,
                    damage1,
                    "CRITICAL HIT!".green().bold()
                );
            } 

            // Else, output it normally
            else {
                println!(
                    "{} attacks {} for {} damage!",
                    fighter1.name, fighter2.name, damage1
                );
            }

            // If fighter 2 has no HP left
            if hp2 <= 0 {
                // Output that fighter 1 wins
                line();
                println!("{} wins!", fighter1.name);
                characters[fighter1_index].give_xp(100); // Give the fighter 100 xp
                wait_for_enter();
                break; // Break back to main
            }

            // Get multiplier for fighter 2's attack damage
            let multiplier = rand::random_range(80..=120); // Again, between 80%-120%
            let mut damage2 = (fighter2.attack * multiplier / 100 - fighter1.defense).max(1); // Calculate damage
            let critical = rand::random_range(1..=100) <= 10; // 10% Chance to be critical

            // If it is a critical hit
            if critical {
                damage2 *= 2; // Mutilply the damage by 2
            }
            hp1 -= damage2; // Minus the health of fighter 2 by fighter 1's damage

            // If it was a critical hit
            if critical {
                // Output the damage fighter 1 did to 2, and with special crit text
                println!(
                    "{} attacks {} for {} damage! {}",
                    fighter2.name,
                    fighter1.name,
                    damage2,
                    "CRITICAL HIT!".green().bold()
                );
            } 
            // Else, output it normally
            else {
                println!(
                    "{} attacks {} for {} damage!",
                    fighter2.name, fighter1.name, damage2
                );
            }

            // If fighter 1 has no HP left
            if hp1 <= 0 {
                // Output that fighter 2 wins
                line();
                println!("{} wins!", fighter2.name);
                characters[fighter2_index].give_xp(100); // Give fighter 2 XP
                wait_for_enter();
                break; // Return back to main
            }
        } 
        
        // Otherwise, character 2 attacks first
        else {
            // Get multiplier for fighter 2's attack damage
            let multiplier = rand::random_range(80..=120); // Again, between 80%-120%
            let mut damage2 = (fighter2.attack * multiplier / 100 - fighter1.defense).max(1); // Calculate damage
            let critical = rand::random_range(1..=100) <= 10; // 10% Chance to be critical

            // If it is a critical hit
            if critical {
                damage2 *= 2; // Mutilply the damage by 2
            }
            hp1 -= damage2; // Minus the health of fighter 2 by fighter 1's damage

            // If it was a critical hit
            if critical {
                // Output the damage fighter 1 did to 2, and with special crit text
                println!(
                    "{} attacks {} for {} damage! {}",
                    fighter2.name,
                    fighter1.name,
                    damage2,
                    "CRITICAL HIT!".green().bold()
                );
            } 
            // Else, output it normally
            else {
                println!(
                    "{} attacks {} for {} damage!",
                    fighter2.name, fighter1.name, damage2
                );
            }

            // If fighter 1 has no HP left
            if hp1 <= 0 {
                // Output that fighter 2 wins
                line();
                println!("{} wins!", fighter2.name);
                characters[fighter2_index].give_xp(100); // Give fighter 2 XP
                wait_for_enter();
                break; // Return back to main
            }
            
            // Fighter 1 attacks now
            // Get random damage multiplier between effectively 80%-120%
            let multiplier = rand::random_range(80..=120);
            let mut damage1 = (fighter1.attack * multiplier / 100 - fighter2.defense).max(1); // Calculate the damage

            // 10% Chance that the attack is a critical
            let critical = rand::random_range(1..=100) <= 10;

            // If it is a critical
            if critical {
                damage1 *= 2; // Mutliply damage by 2
            }
            hp2 -= damage1; // Minus the health of fighter 2 by fighter 1's damage

            // If it was a critical hit
            if critical {
                // Output the damage fighter 1 did to 2, and with special crit text
                println!(
                    "{} attacks {} for {} damage! {}",
                    fighter1.name,
                    fighter2.name,
                    damage1,
                    "CRITICAL HIT!".green().bold()
                );
            } 

            // Else, output it normally
            else {
                println!(
                    "{} attacks {} for {} damage!",
                    fighter1.name, fighter2.name, damage1
                );
            }

            // If fighter 2 has no HP left
            if hp2 <= 0 {
                // Output that fighter 1 wins
                line();
                println!("{} wins!", fighter1.name);
                characters[fighter1_index].give_xp(100); // Give the fighter 100 xp
                wait_for_enter();
                break; // Break back to main
            }
        }
        // Wait for enter between each combat step
        wait_for_enter();
    }
}