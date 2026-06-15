use colored::*;
use std::fs; // Import fs for file functions

// Use crossterm for terminal manipulation and styling, as similar to C# as possible
use crossterm::{
    cursor::MoveTo,
    cursor::MoveUp,
    execute,                            // Import execute function for terminal commands
    style::{Color, SetForegroundColor}, // Import Color and SetForegroundColor for styling text
    terminal::{Clear, ClearType, size}, // Import Clear and ClearType for clearing the terminal
};

// Use std::io for input and output operations
// Without this, it would need to be std::io::stdin/stdout() every time
use std::io::stdin;
use std::io::stdout;

use std::io::Write;

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
    read_line();
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
    name: String,
    class_type: String,
    ability: String,

    level: i32,
    xp: i32,

    max_hp: i32,
    hp: i32,

    base_attack: i32,
    attack: i32,

    defense: i32,
    speed: i32,
}

/// Levels up a specific character if possible
fn level_up(character: &mut Character) {
    // While the character has enough xp to level up
    while character.xp >= character.level * 100 {
        character.xp -= character.level * 100; // Remove the xp
        character.level += 1; // Add the level
        // Add statisitics for levelling up
        character.max_hp += 10;
        character.hp = character.max_hp; // Heal fully on level up
        character.base_attack += 2;
        character.attack = character.base_attack;
        character.defense += 1;
        character.speed += 1;

        // Output that the character levelled up and info regarding it
        println!(
            "{} levelled up to level {}!",
            character.name, character.level
        );
    }
}

/// Gives xp to a character and attempts to level them up
fn give_xp(character: &mut Character, xp: i32) {
    // Add the xp to the character
    character.xp += xp;
    println!("{} gained {} XP!", character.name, xp); // Output xp change

    // Attempt to level up the character
    level_up(character);
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
                give_xp(character, xp_to_add); // Add the xp and attempt to level up
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
                save_characters(&mut characters);
            }
            7 => {
                load_characters(&mut characters);
            }
            8 => {
                explore_forest(&mut characters);
            }
            9 => {
                character_battle(&mut characters);
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

    let width = 10;
    let height = 10;

    let map = create_map(width, height);

    let mut x: usize = 5;
    let mut y: usize = 5;

    loop {
        // Set terminal text color to dark green
        execute!(stdout(), SetForegroundColor(Color::DarkGreen)).unwrap();
        // Output title
        output_screen("assets/forest_exploration_art.txt", "Exploring the forest!");

        if check_if_dead(character) {
            println!(
                "{} cannot continue as they have 0 hp! Revive them in the search and modify area.",
                character.name
            );
            break;
        }

        render_map(&map, x, y);

        line();
        println!("Exploring as {}:", character.name);
        println!(
            "HP: {}/{} | ATK: {} | DEF: {}",
            character.hp, character.max_hp, character.attack, character.defense
        );
        line();
        let input = get_move();

        let (new_x, new_y) = match input.as_str() {
            "w" => (x, y.saturating_sub(1)),
            "s" => (x, (y + 1).min(height - 1)),
            "a" => (x.saturating_sub(1), y),
            "d" => ((x + 1).min(width - 1), y),
            "q" => break,
            _ => (x, y),
        };

        // prevent walking into walls
        if map[new_y][new_x] == '#' {
            println!("Blocked by wall.");
            wait_for_enter();
            continue;
        }

        x = new_x;
        y = new_y;

        // trigger encounter after movement
        line();
        trigger_encounter(character);

        wait_for_enter();
    }
}

fn create_map(width: usize, height: usize) -> Vec<Vec<char>> {
    let mut map = vec![vec!['.'; width]; height];

    // Add borders
    for x in 0..width {
        map[0][x] = '#';
        map[height - 1][x] = '#';
    }

    for y in 0..height {
        map[y][0] = '#';
        map[y][width - 1] = '#';
    }
    map
}

fn render_map(map: &Vec<Vec<char>>, player_x: usize, player_y: usize) {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if x == player_x && y == player_y {
                print!(" {} ", "P".blue().bold());
            } else {
                print!(" {} ", map[y][x]);
            }
        }
        println!();
    }
}

fn get_move() -> String {
    print!("Move (WASD, Q to Quit): ");
    stdout().flush().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    input.trim().to_lowercase()
}

fn trigger_encounter(character: &mut Character) {
    let roll = rand::random_range(0..=100);

    if roll < 40 {
        println!("Nothing happened!");
    } else if roll < 60 {
        let mut enemy = Enemy {
            name: "Goblin".to_string(),
            hp: 25 + level * 12.
            attack: 3 + level * 2,
            defense: level / 3,
        };

        combat_loop(character, &mut enemy);
    } else if roll < 75 {
        println!("Potion found (+5 hp)");
        character.hp += 5;
    } else if roll < 90 {
        println!("XP orb found (+50 XP)");
        give_xp(character, 50)
    } else {
        println!("Sword found (+1 attack)");
        character.attack += 1;
    }
}

struct Enemy {
    name: String,
    hp: i32,
    attack: i32,
    defense: i32,
}

fn combat_loop(player: &mut Character, enemy: &mut Enemy) {
    // Set terminal text color to dark red
    execute!(stdout(), SetForegroundColor(Color::DarkRed)).unwrap();
    // Output title
    output_screen("assets/combat_art.txt", "Fighting!");
    println!("A wild {} appears!", enemy.name);
    wait_for_enter();

    loop {
        // Set terminal text color to dark red
        execute!(stdout(), SetForegroundColor(Color::DarkRed)).unwrap();
        // Output title
        output_screen("assets/combat_art.txt", "Fighting!");

        println!("{} HP: {}/{}", player.name, player.hp, player.max_hp);
        println!("{} HP: {}", enemy.name, enemy.hp);

        line();
        println!("Choose action:");
        println!("1. Attack");
        println!("2. Use ability");

        let choice = get_int(1, 2);

        // PLAYER TURN
        match choice {
            1 => {
                let damage = (player.attack - enemy.defense).max(1);
                enemy.hp -= damage;
            }

            2 => {
                let damage = (player.attack * 2 - enemy.defense).max(1);
                enemy.hp -= damage;
            }

            _ => {
                unreachable!();
            }
        }

        // CHECK WIN
        if enemy.hp <= 0 {
            // Set terminal text color to dark red
            execute!(stdout(), SetForegroundColor(Color::DarkRed)).unwrap();
            // Output title
            output_screen("assets/combat_art.txt", "Fighting!");
            println!("Enemy defeated!");
            line();
            give_xp(player, 50);
            break;
        }

        // ENEMY TURN
        let enemy_damage = (enemy.attack - player.defense).max(1);
        player.hp -= enemy_damage;

        // CHECK LOSS
        if check_if_dead(player) {
            break;
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

fn choose_two_characters(characters: &Vec<Character>) -> (usize, usize) {
    println!("Choose first fighter:");
    let fighter1 = choose_character(characters);

    // Set terminal text color to Dark Yellow
    execute!(stdout(), SetForegroundColor(Color::DarkYellow)).unwrap();
    // Output title
    output_screen(
        "assets/duel_art.txt",
        "Duel to the death! (HP is not changed outside the arena)",
    );

    println!("Choose second fighter:");

    loop {
        let fighter2 = choose_character(characters);

        if fighter2 != fighter1 {
            return (fighter1, fighter2);
        }

        println!("Cannot fight the same character.");
    }
}

fn character_battle(characters: &mut Vec<Character>) {
    // Set terminal text color to Dark Yellow
    execute!(stdout(), SetForegroundColor(Color::DarkYellow)).unwrap();
    // Output title
    output_screen(
        "assets/duel_art.txt",
        "Duel to the death! (HP is not changed outside the arena)",
    );

    if characters.len() < 2 {
        println!("You need at least two characters.");
        wait_for_enter();
        return;
    }

    let (fighter1_index, fighter2_index) = choose_two_characters(characters);

    let fighter1 = &characters[fighter1_index];
    let fighter2 = &characters[fighter2_index];

    let mut hp1 = fighter1.hp;
    let mut hp2 = fighter2.hp;

    // Set terminal text color to Dark Yellow
    execute!(stdout(), SetForegroundColor(Color::DarkYellow)).unwrap();
    // Output title
    output_screen(
        "assets/duel_art.txt",
        "Duel to the death! (HP is not changed outside the arena)",
    );

    println!("{} challenges {}!", fighter1.name, fighter2.name);

    wait_for_enter();

    loop {
        // Set terminal text color to red
        execute!(stdout(), SetForegroundColor(Color::Red)).unwrap();
        // Output title
        output_screen(
            "assets/duel_art.txt",
            "Duel to the death! (HP is not changed outside the arena)",
        );

        println!("{} HP: {}/{}", fighter1.name, hp1, fighter1.max_hp);

        println!("{} HP: {}/{}", fighter2.name, hp2, fighter2.max_hp);

        line();

        // Roll initiative
        let initiative1 = fighter1.speed + rand::random_range(1..=20);
        let initiative2 = fighter2.speed + rand::random_range(1..=20);

        if initiative1 >= initiative2 {
            // Fighter 1 attacks first

            let multiplier = rand::random_range(80..=120);

            let mut damage1 = (fighter1.attack * multiplier / 100 - fighter2.defense).max(1);

            let critical = rand::random_range(1..=100) <= 10;

            if critical {
                damage1 *= 2;
            }

            hp2 -= damage1;

            if critical {
                println!(
                    "{} attacks {} for {} damage! {}",
                    fighter1.name,
                    fighter2.name,
                    damage1,
                    "CRITICAL HIT!".green().bold()
                );
            } else {
                println!(
                    "{} attacks {} for {} damage!",
                    fighter1.name, fighter2.name, damage1
                );
            }

            if hp2 <= 0 {
                line();
                println!("{} wins!", fighter1.name);

                give_xp(&mut characters[fighter1_index], 100);

                wait_for_enter();
                break;
            }

            // Fighter 2 attacks second

            let multiplier = rand::random_range(80..=120);

            let mut damage2 = (fighter2.attack * multiplier / 100 - fighter1.defense).max(1);

            let critical = rand::random_range(1..=100) <= 10;

            if critical {
                damage2 *= 2;
            }

            hp1 -= damage2;

            if critical {
                println!(
                    "{} attacks {} for {} damage! {}",
                    fighter2.name,
                    fighter1.name,
                    damage2,
                    "CRITICAL HIT!".green().bold()
                );
            } else {
                println!(
                    "{} attacks {} for {} damage!",
                    fighter2.name, fighter1.name, damage2
                );
            }

            if hp1 <= 0 {
                line();
                println!("{} wins!", fighter2.name);

                give_xp(&mut characters[fighter2_index], 100);

                wait_for_enter();
                break;
            }
        } else {
            // Fighter 2 attacks first

            let multiplier = rand::random_range(80..=120);

            let mut damage2 = (fighter2.attack * multiplier / 100 - fighter1.defense).max(1);

            let critical = rand::random_range(1..=100) <= 10;

            if critical {
                damage2 *= 2;
            }

            hp1 -= damage2;

            if critical {
                println!(
                    "{} attacks {} for {} damage! {}",
                    fighter2.name,
                    fighter1.name,
                    damage2,
                    "CRITICAL HIT!".green().bold()
                );
            } else {
                println!(
                    "{} attacks {} for {} damage!",
                    fighter2.name, fighter1.name, damage2
                );
            }

            if hp1 <= 0 {
                line();
                println!("{} wins!", fighter2.name);

                give_xp(&mut characters[fighter2_index], 100);

                wait_for_enter();
                break;
            }

            // Fighter 1 attacks second

            let multiplier = rand::random_range(80..=120);

            let mut damage1 = (fighter1.attack * multiplier / 100 - fighter2.defense).max(1);

            let critical = rand::random_range(1..=100) <= 10;

            if critical {
                damage1 *= 2;
            }

            hp2 -= damage1;

            if critical {
                println!(
                    "{} attacks {} for {} damage! {}",
                    fighter1.name,
                    fighter2.name,
                    damage1,
                    "CRITICAL HIT!".green().bold()
                );
            } else {
                println!(
                    "{} attacks {} for {} damage!",
                    fighter1.name, fighter2.name, damage1
                );
            }

            if hp2 <= 0 {
                line();
                println!("{} wins!", fighter1.name);

                give_xp(&mut characters[fighter1_index], 100);

                wait_for_enter();
                break;
            }
        }
        wait_for_enter();
    }
}
