mod ai_functions;
mod apis;
mod helpers;
mod models;

use helpers::command_line::get_user_response;

fn main() {
    let user_response = get_user_response("What is your name?");

    println!("Hello, {}!", user_response);
}
