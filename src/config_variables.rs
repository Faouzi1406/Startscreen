use std::collections::HashMap;

pub trait Variables {
    fn parse_variables(self) -> (String,String);
    fn get_variables(self) -> HashMap<String,String>;
    
}

//Get variables from self 
pub trait GetVariables {
    fn get_variable(self, needle:&str) ->String;
    fn get_print_color(self) -> String;
    fn welcome_variable(self) ->String; 
    fn get_start_print(self) -> String;
}

pub trait Actions {
    fn weather_action();
}



impl Variables for &str{
    fn parse_variables(self) -> (String,String){
        let values = self.split_once(':');
        let mut variables:(String, String) = ("".to_string(), "".to_string());
        
        if values.is_some(){
            let value = values.unwrap();
            variables =  (value.0.to_string(), value.1.to_string());
        }

        variables
    }

    fn get_variables(self) -> HashMap<String,String>{
        let mut variables:HashMap<String, String> = HashMap::new();

        for line in self.split("\n") {
            let variable = line.parse_variables();
            variables.insert(variable.0.to_string(), variable.1.to_string());
        };

        variables
    }

}




impl GetVariables for &str{
    //Get variable method
    fn get_variable(self, needle:&str) ->String{
        let variables = self.get_variables();
        let color = variables.get(needle); 
        let is_some = color.is_some();

        let unwrap = if is_some {
            color.unwrap()
        }
        else{
            return "null".to_string()
        };

        unwrap.to_string()
    }
    
   //Get print color from variables
    fn get_print_color(self) -> String{
        let variables = self.get_variables();
        let color = variables.get("print_color");

        if color.is_some() { return color.unwrap().to_string() }
        else {return "black".to_string()};
    }

    //Welcome message setup if turned on
    fn welcome_variable(self) ->String{
        let variables = self.get_variables();
        let welcome_text = variables.get("welcome_text");

        if welcome_text.is_some() { return welcome_text.unwrap().to_string() }
        else {return " 😟 Wow you don't seem to have a welcome message :(, You are always welcome to add it to you config file :)!!".to_string()};
    }

    //Get print color of assci
    fn get_start_print(self) -> String{
        let variables = self.get_variables();
        let color = variables.get("assci_color");

        if color.is_some() { return color.unwrap().to_string() }
        else {return "black".to_string()};
    }


    
}


