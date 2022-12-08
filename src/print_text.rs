use colored::Colorize;

pub trait PrintFunctions{
    fn print_value(self);
}

pub struct PrintValue{
    pub color:String,
    pub text:String
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


