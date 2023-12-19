use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand,
};
use std::io::{stdin, stdout};

#[derive(PartialEq, Debug)]
pub enum PrintCommand {
    AICall,
    Unittest,
    Issue,
}

impl PrintCommand {
    pub fn print_agent_message(&self, agent_pos: &str, agent_statement: &str) {
        let mut stdout = stdout();

        // Decide the print color
        let statement_color = match self {
            PrintCommand::AICall => Color::Cyan,
            PrintCommand::Unittest => Color::Magenta,
            PrintCommand::Issue => Color::Red,
        };

        // Print the agent statement
        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        print!("Agent: {}", agent_pos);

        // Make selected color
        stdout.execute(SetForegroundColor(statement_color)).unwrap();
        println!(" {}", agent_statement);

        // Reset color
        stdout.execute(ResetColor).unwrap();
    }
}

pub fn get_user_response(question: &str) -> String {
    let mut stdout = stdout();
    stdout.execute(SetForegroundColor(Color::Cyan)).unwrap();

    println!("");

    println!("`{}`", question);

    stdout.execute(ResetColor).unwrap();

    let mut user_response = String::new();
    stdin()
        .read_line(&mut user_response)
        .expect("Did not enter a correct string");

    user_response.trim_end().to_string()
}
