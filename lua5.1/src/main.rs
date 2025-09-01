use std::env;

mod lua_api;

fn main() /* -> Result<(), e> */{
    // get Run Arguments
    let args: Vec<String> = env::args().collect();

    // Check if filename
    if args.len() < 2 {
        eprintln!("No Path!");
        //eprintln!("404");
    }

    let path = &args[1];

    match lua_api::execute_script(&path) {
        Ok(()) => {println!("Script executed!")},
        Err(e) => {eprintln!("Error: {}", e)}
    }
}
