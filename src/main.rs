//use regex::Regex;

use regex::Regex;

fn main() {

    //Pido la entrada
    print!("Ingrese sus operaciones:\n  ");
    let mut input_user:String = "".to_string();
    std::io::stdin().read_line(&mut input_user).unwrap();


    //declaro regex and loop captures variables
    let regular_expresion_master = Regex::new(r"(\d+)\s*?(\*)\s*?(\d+)").unwrap();

    loop{
        let captures_regular_expresion = regular_expresion_master.captures(&input_user).unwrap();

        //Body code math operations
        //loop
        //multipication, divide, add and minus
        let operator = captures_regular_expresion.get(2).unwrap().as_str();
        let mut value_c_resulted:i64= 0; //value println!

        if operator == "*"|| operator =="/"|| operator == "+"|| operator == "-"{

            let value_a:i64 = captures_regular_expresion.get(1).unwrap().as_str().parse().unwrap();
            let value_b:i64 = captures_regular_expresion.get(3).unwrap().as_str().parse().unwrap();

            //multiplication
            if operator == "*"{
                value_c_resulted = value_a * value_b;
                
            //divide    
            }else if operator == "/"{
                value_c_resulted = value_a / value_b;

            //add
            }else if operator == "+"{
                value_c_resulted = value_a + value_b;

            //minus
            }else if operator == "-"{
                value_c_resulted = value_a - value_b;
            }

            print!("\nResultado: {}", value_c_resulted);
        }

        let replaced_capture = captures_regular_expresion.get(0).unwrap().as_str();
        let value_c_replacer:&str = &value_c_resulted.to_string();
        input_user = input_user.replace(replaced_capture, value_c_replacer);
    }

    //devuelvo salida


}
