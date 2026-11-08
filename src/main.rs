pub mod inputs;
use inputs::user_input_fn;


fn main() {
    let mut win_streak = 0;
    let possibilities = ["paper", "rock", "scissors", "leave"];

    loop {
        println!("input either: Rock, Paper, Scissors or Leave");

        let user_input = user_input_fn().to_lowercase();
        if !possibilities.contains(&user_input.as_str()) {
            println!("Invalid Input");
            continue;
        }
        if user_input == "leave" {
            println!("You left");
            break;
        }

        let random_choice = match rand::random_range(0..3) {
            0 => "rock",
            1 => "paper",
            _ => "scissors",
        };

        if user_input == random_choice {
            println!("AI chose {}", random_choice);
            println!("Tie. Your win streak resets");
            win_streak = 0;

            continue;
        } else if user_input == "rock" && random_choice == "scissors"
            || user_input == "scissors" && random_choice == "paper"
            || user_input == "paper" && random_choice == "rock"
        {
            win_streak += 1;
            println!("AI chose {}", random_choice);
            println!("You win! Your win streak is: {}", win_streak);
            continue;
        } else {
            println!("AI chose {}", random_choice);
            println!("The AI wins! Your win streak resets.");
            win_streak = 0;
            continue;
        }
    }
}
