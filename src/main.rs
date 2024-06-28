use clap::{command, Arg, Command, ArgGroup};



fn main() {
    let match_result = command!()
    .subcommand(
    Command::new("register-person")
            .arg(
                Arg::new( "firstname")
            //       .group("person-register")
                    .short('f')
                    .long("first-name")
                    //provee otras alternativas para el parametro
                    .aliases(["fname","firstname"])
                    //si queremos que el parametro se obligatorio
                    .required(true)
                    .help("The person' s First name 👨‍🦱")
                    //para especificar que el argumento entra en conflicto con otr
                    //.conflicts_with("fluffy")
        
            
            )
            .arg(
                Arg::new( "lastname")
            //     .group("person-register")
                    .short('l')
                    .long("last-name")
                    .aliases(["lname", "lastname"])
                    //.required(true)
                    .help("The person' s Last name 👨‍🦱 ")
        
            )
            
        
    )
    .subcommand(
        Command::new("register-pet")
            .arg(
                Arg::new( "pet-name")
                    .long("pet-name")
                    .short('n')
                    .require_equals(true)
                    
            )
    )    
    .arg(
        Arg::new( "fluffy")
            .long("fluffy")
            .aliases(["ffluffy"])
            .help("Is rhe person wearing a fluffy coat 🎍 or not")

    )
    .get_matches();

    //Obtenemos los parametros en el global option
    
    println!("fluffy is: {}", match_result.get_one::<String>("fluffy").unwrap_or(&"No pet name on Main command .".to_string()));
    //Obtenemos los parametros de los sub comandos
let pet_arg = match_result.subcommand_matches("register-pet");
if !pet_arg.is_none(){

    println!("Does pet name exist on sub-command : {} ", pet_arg.unwrap().contains_id("pet-name"));
    println!("Pet name : {} ", pet_arg.unwrap().get_one::<String>("pet-name").unwrap());
    
}
let person_arg = match_result.subcommand_matches("register-person").unwrap();
//person_arg.unwrap().get_one::<String>("Person-name").unwrap_or(&"No name person".to_string());
println!("Name : {} last name : {}", person_arg.get_one::<String>("firstname").unwrap(),person_arg.get_one::<String>("lastname").unwrap());
}
