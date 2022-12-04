mod config_variables;
use colored::Colorize;
use config_variables::*;

struct PrintValue{
    color:String,
    text:String
}


trait PrintFunctions{
    fn print_value(self);
}

impl PrintFunctions for PrintValue {
    fn print_value(self) {
        let values_with_color = match &self.color as &str {
            "blue" => self.text.blue(),
            "red" => self.text.red(),
            "green" => self.text.green(),
            "magenta" => self.text.magenta(),
            "bright_blue" => self.text.bright_blue(),
            "bright_red" => self.text.bright_red(),
            "white" => self.text.white(),
            &_ => self.color.black()
        };
        print!("{} \n", values_with_color);
    }
}



fn main() {
    println!();
    //Include variables config in binary
    let variables = include_str!("../start.start");

    //Color 

    //Startscreen
    let start_screen_color = variables.get_start_print();
    let print_start = PrintValue {color: start_screen_color, text:r"
    +-------------+                     ___        |      |      |    
    |             |                     \ /]       |      |      |    
    |             |        _           _(_)        |      |      |    
    |             |     ___))         [  | \___    |      |      |    
    |             |     ) //o          | |     \   |      |      |    
    |             |  _ (_    >         | |      ]  |      |      |    
    |          __ | (O)  \__<          | | ____/   '------'------'    
    |         /  o| [/] /   \)        [__|/_                          
    |             | [\]|  ( \         __/___\_____                    
    |             | [/]|   \ \__  ___|            |                   
    |             | [\]|    \___E/%%/|____________|_____              
    |             | [/]|=====__   (_____________________)             
    |             | [\] \_____ \    |                  |              
    |             | [/========\ |   |                  |              
    |             | [\]     []| |   |                  |              
    |             | [/]     []| |_  |                  |              
    |             | [\]     []|___) |                  |    MEPH          
    ====================================================================
             ".to_string()};
    print_start.print_value();


    //Welcome message
    let color = variables.get_print_color();
    let welcome_message = variables.welcome_variable();
    let print_welcome  = PrintValue {color:color.to_string(), text:welcome_message};
    print_welcome.print_value(); 

    //Todo: weather message
}
