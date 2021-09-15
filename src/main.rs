struct Hangman {
    head: bool,
    body: bool,
    larm: bool,
    rarm: bool,
    rleg: bool,
    lleg: bool,
}

impl Hangman {
    // returns true if all the body parts have been added
    fn add_part(&mut self) -> bool {
        if self.head == false {
            self.head = true;
            return false;
        }
        if self.body == false {
            self.body = true;
            return false;
        }
        if self.larm == false {
            self.larm = true;
            return false;
        }
        if self.rarm == false {
            self.rarm = true;
            return false;
        }
        if self.rleg == false {
            self.rleg = true;
            return false;
        }
        if self.lleg == false {
            self.lleg = true;
            return true;
        }

        return true;
    }
}

fn main() {
    loop {
        play_game();
        println!("Want to play again? [y/n]?");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Could not read input");
        input = input.chars().take(1).collect();
        if input == "y" {
            ()
        } else if input == "n" {
            break;
        } else {
            println!("Expected \"y\" or \"n\"")
        }
    }
}

fn play_game() {
    let mut hangman = Hangman {
        head: false,
        body: false,
        larm: false,
        rarm: false,
        rleg: false,
        lleg: false,
    };

    let word = String::from("this is the word");
    let mut template = String::new();
    let mut completed_str = String::new();
    for letter in word.chars() {
        completed_str.push_str(" ");
        if letter != ' ' {
            template.push_str("-")
        } else {
            template.push_str(" ")
        }
    }
    let mut used_chars = String::new();

    print_game(
        &hangman,
        &completed_str,
        &template,
        &used_chars,
        "Welcome to hangman! To play, just type in a letter (if you type multiple only the first one will count) and press enter.",
    );
    loop {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Could not read input");
        input = input.chars().take(1).collect();
        if input == "\r" || input == "\n" {
            print_game(
                &hangman,
                &completed_str,
                &template,
                &used_chars,
                "Newlines cannot be inputs",
            );
        } else if used_chars.contains(&input) {
            print_game(
                &hangman,
                &completed_str,
                &template,
                &used_chars,
                "That character has already been tried",
            );
        } else if word.contains(&input) {
            used_chars.push_str(&input);
            for (i, letter) in word.char_indices() {
                if String::from(letter) == input {
                    completed_str.replace_range(i..i + 1, &input);
                }
            }
            if completed_str == word {
                print_game(
                    &hangman,
                    &completed_str,
                    &template,
                    &used_chars,
                    "You won the game! Congratulations!",
                );
                break;
            } else {
                print_game(
                    &hangman,
                    &completed_str,
                    &template,
                    &used_chars,
                    "Got it! Good job!",
                );
            }
        } else {
            used_chars.push_str(&input);
            if hangman.add_part() {
                print_game(
                    &hangman,
                    &completed_str,
                    &template,
                    &used_chars,
                    &["You lost! The correct answer was: \"", &word, "\""].join(""),
                );
                break;
            } else {
                print_game(
                    &hangman,
                    &completed_str,
                    &template,
                    &used_chars,
                    "Incorrect",
                );
            }
        }
    }
}

fn print_game(
    hangman: &Hangman,
    uncovered: &String,
    template: &String,
    used_chars: &String,
    msg: &str,
) {
    println!("\n");
    println!("\n");
    println!("\n");

    println!("   +----         {}", used_chars);
    println!("   |   {}", if hangman.head { "0" } else { " " });
    println!(
        "   |  {}{}{}",
        if hangman.larm { "/" } else { " " },
        if hangman.body { "|" } else { " " },
        if hangman.rarm { "\\" } else { " " }
    );
    println!(
        "   |  {}{}",
        if hangman.lleg { "/" } else { " " },
        if hangman.rleg { "\\" } else { " " }
    );
    println!("   |             {}", uncovered);
    println!("---+---          {}", template);
    println!("{}", msg)
}
