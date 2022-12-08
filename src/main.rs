mod config_variables;
mod print_text;
mod config_files;

use print_text::*;
use config_variables::*;
use config_files::include_files::include_files;



fn main() {

    let include_files = include_files().expect("Error coulnd't find files");
    let variables = include_files.variables;
    let assci = include_files.assci;

    //Startscreen
    let start_screen_color = variables.get_start_print();
    let print_start = PrintValue {color: start_screen_color, text:assci};
    print_start.print_value();


    //Welcome message
    let color = variables.get_print_color();
    let welcome_message = variables.welcome_variable();
    let print_welcome  = PrintValue {color:color.to_string(), text:welcome_message};
    print_welcome.print_value(); 

    //Todo: weather message
}
