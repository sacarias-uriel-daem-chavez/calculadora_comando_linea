use regex::Regex;                                               
fn main() {


    loop{ //Pido la entrada
        print!("Ingrese sus operaciones: (presione <s> y <enter> para salir)\n  ");
        let mut input_user:String = "".to_string();
        std::io::stdin().read_line(&mut input_user).unwrap();

        //option program ending                                                                           
        if input_user == "s\n"||input_user == "S\n"{ break; }

        //operations mathematics
        input_user = prioridad_multdiv(input_user);
        input_user = prioridad_multdiv(input_user);
        input_user= add(input_user);
        input_user= minus(input_user);
        println!("Resultado --> {}", input_user);
    }
}


fn multiplication(mut input_user: String)->String{                  
    loop{ 
        //declaro regex and loop captures variable
        let regular_expresion_master = Regex::new(r"(\d+)\s*?(\*)\s*?(\d+)").unwrap();                              
        let captures_regular_expresion = regular_expresion_master.captures(&input_user);

        if captures_regular_expresion.is_none(){
            return input_user;
        }

        let captures_regular_expresion = captures_regular_expresion.unwrap();
        let operator = captures_regular_expresion.get(2).unwrap().as_str();
        let mut value_c_resulted:i64= 0; //value println!

        if operator == "*"{                                       
            let value_a:i64 = captures_regular_expresion.get(1).unwrap().as_str().parse().unwrap();                     
            let value_b:i64 = captures_regular_expresion.get(3).unwrap().as_str().parse().unwrap();
            value_c_resulted = value_a * value_b;
        }


        let replaced_capture = captures_regular_expresion.get(0).unwrap().as_str();                                 
        let value_c_replacer:&str = &value_c_resulted.to_string();                                                  
        input_user = input_user.replace(replaced_capture, value_c_replacer);
    }
}




fn division(mut input_user: String)->String{
    loop{
        //declaro regex and loop captures variable
        let regular_expresion_master = Regex::new(r"(\d+)\s*?(\/)\s*?(\d+)").unwrap();                              
        let captures_regular_expresion = regular_expresion_master.captures(&input_user);

        if captures_regular_expresion.is_none(){                            
            return input_user;                                          
        }

        let captures_regular_expresion = captures_regular_expresion.unwrap();
        let operator = captures_regular_expresion.get(2).unwrap().as_str();
        let mut value_c_resulted:i64= 0; //value println!

        if operator == "/"{                                       
            let value_a:i64 = captures_regular_expresion.get(1).unwrap().as_str().parse().unwrap();                     
            let value_b:i64 = captures_regular_expresion.get(3).unwrap().as_str().parse().unwrap();
            value_c_resulted = value_a / value_b;               
        }

        let replaced_capture = captures_regular_expresion.get(0).unwrap().as_str();                                 
        let value_c_replacer:&str = &value_c_resulted.to_string();                                                  
        input_user = input_user.replace(replaced_capture, value_c_replacer);
    }
}




fn add(mut input_user: String)->String{
    loop{
        //declaro regex and loop captures variable
        let regular_expresion_master = Regex::new(r"(\d+)\s*?(\+)\s*?(\d+)").unwrap();                              
        let captures_regular_expresion = regular_expresion_master.captures(&input_user);

        if captures_regular_expresion.is_none(){                            
            return input_user;                                          
        }

        let captures_regular_expresion = captures_regular_expresion.unwrap();
        let operator = captures_regular_expresion.get(2).unwrap().as_str();
        let mut value_c_resulted:i64= 0; //value println!

        if operator == "+"{                                       
            let value_a:i64 = captures_regular_expresion.get(1).unwrap().as_str().parse().unwrap();                     
            let value_b:i64 = captures_regular_expresion.get(3).unwrap().as_str().parse().unwrap();
            value_c_resulted = value_a + value_b;

        }

        let replaced_capture = captures_regular_expresion.get(0).unwrap().as_str();                                 
        let value_c_replacer:&str = &value_c_resulted.to_string();                                                  
        input_user = input_user.replace(replaced_capture, value_c_replacer);
    }
}




fn minus(mut input_user: String)->String{
    loop{
        //declaro regex and loop captures variable
        let regular_expresion_master = Regex::new(r"(\d+)\s*?(-)\s*?(\d+)").unwrap();                              
        let captures_regular_expresion = regular_expresion_master.captures(&input_user);

        if captures_regular_expresion.is_none(){                            
            return input_user;                                          
        }

        let captures_regular_expresion = captures_regular_expresion.unwrap();
        let operator = captures_regular_expresion.get(2).unwrap().as_str();
        let mut value_c_resulted:i64= 0; //value println!

        if operator == "-"{                                       
            let value_a:i64 = captures_regular_expresion.get(1).unwrap().as_str().parse().unwrap();                     
            let value_b:i64 = captures_regular_expresion.get(3).unwrap().as_str().parse().unwrap();
            value_c_resulted = value_a - value_b;

        }

        let replaced_capture = captures_regular_expresion.get(0).unwrap().as_str();                                 
        let value_c_replacer:&str = &value_c_resulted.to_string();                                                  
        input_user = input_user.replace(replaced_capture, value_c_replacer);
    }
}

fn prioridad_multdiv(input: String)->String{
    let regular_expresion_master = Regex::new(r"(\d+)\s*?(\*||\/)\s*?(\d+)").unwrap();
    let captures_regular_expresion = regular_expresion_master.captures(&input);
    
    if captures_regular_expresion.is_none(){ return input; }

    let operator= captures_regular_expresion.unwrap().get(2).unwrap().as_str();

    //le damos prioridad a las operaciones
    if operator == "*" {
        return multiplication(input);
    }
    else if operator == "/" {
        return division(input);
    }
    else {
        return input;
    }

}
