pub mod config;
pub mod dir;
pub mod logger;
pub mod macros;
pub mod print;
pub mod update;

use crate::VERSION;
use crate::helper::print::{BLUE, BOLD, BRIGHT_BLUE, CYAN, END, GREEN, PURPLE, RED, YELLOW};

// TODO
// add no color argument for no color output

//[GENERALL-OPTIONS]
const SCO: &str = PURPLE;

//[SCRIPTOPTIONS]
const SC: &str = YELLOW;

//[OPTIONS]
const OPT: &str = CYAN;

//<action>
const AC: &str = RED;

//<custom-action-options>
const ACO: &str = CYAN;

// OptionList
const OP: &str = BLUE;

// LINKs
const LINK: &str = BLUE;

// CONFIG
const CONFIG: &str = BRIGHT_BLUE;

// Function for a detailed INFO help Message
pub fn help() {
    // Start
    println!("{}Flua Help:{}", GREEN, END);
    println!("Using Version {}{}v{}{}", BOLD, GREEN, VERSION, END);
    // Lua Scripts
    println!(
        "\nUsage for Lua scripts: {}<flua>{} <script.lua> {}[SCRIPTOPTIONS]{} {}[GENERALL-OPTIONS]{}",
        GREEN, END, SC, END, SCO, END
    );
    //[SCRIPTOPTIONS]
    println!("\n{}[SCRIPTOPTIONS]{}", SC, END);
    println!(
        "{}  --safe:        {}Run in safe mode (limited API, no OS access)",
        OP, END
    );
    println!(
        "{}  --no-info:     {}Suppress start and end info messages",
        OP, END
    );
    println!(
        "{}  l, lua-args    {}submit arguments after lua-args for the lua file which will be run, stop the collectiong when it sees an argument which starts with: {}'-'{}",
        OP, END, RED, END
    );
    // Modules
    println!(
        "\nUsage for Modules: {}<flua>{} run {}<actions>{} {}<custom-action-options>{} {}[GENERALL-OPTIONS]{}",
        GREEN, END, AC, END, ACO, END, SCO, END
    );
    println!("\n{}<actions>{}:   'update', 'install'", AC, END);
    println!(
        "{}  module         {}Argument to run a {}dlm13{} Module",
        OP, END, RED, END
    );
    println!(
        "\n{}<custom-action-options>{}: Ends the arg collecting when a path starts with #",
        ACO, END
    );
    println!("{}  -path=    {}Add the Module path after the '='", OP, END);
    // Other
    println!(
        "\nOther Usage: {}<flua>{} {}[OPTIONS]{} {}[GENERALL-OPTIONS]{}",
        GREEN, END, OPT, END, SCO, END
    );
    //[OPTIONS]
    println!("\n{}[OPTIONS]{}", OPT, END);
    println!(
        "{}  -v, --version  {}Function which prints the version in the terminal",
        OP, END
    );
    println!(
        "{}  h, -h, --help  {}Function which prints this help message in the terminal",
        OP, END
    );
    //[GENERALL-OPTIONS]
    println!("\n{}[GENERALL-OPTIONS]{}", SCO, END);
    println!(
        "{}  -nw            {}No exit -> The Programm closes instantly after an error and showing an error message without waiting for some Time.",
        OP, END
    );
    println!(
        "{}  -no-config     {}The Programm will not search or load the Config file. Standard Option is true, if there is no Config file, the step will be skipped. A Config file will not be created automaticly!",
        OP, END
    );
    // CONFIG
    println!("\n{}[MORE-OPTIONS]{}", CONFIG, END);
    println!("  --help-config");
    // More
    println!("\nFor more info about Flua and the Lua API, see:");
    println!("{}https://github.com/ShadowDara/LuaAPI-Rust{}", LINK, END);
    println!("{}https://shadowdara.github.io/flua/{}", LINK, END);
}

// CONFIG
// println!("generate, open, check");
pub fn config_help() {
    println!("{}[CONFIG]{} Help Info for Flua v{}", CONFIG, END, VERSION);
    println!(
        "\nUsage: flua conifg {}[CONFIGOPTIONS]{} {}[GENERALL-OPTIONS]{}",
        CONFIG, END, SCO, END
    );
    println!("\n{}[CONFIGOPTIONS]{}", CONFIG, END);
    println!(
        "{}    generate{}    Be careful! This will create a new config file, the old one will be overritten",
        OP, END
    );
    println!(
        "{}    open{}        to open the config file in the default programm",
        OP, END
    );
    println!(
        "{}    opendir{}     to open the config file in the default programm",
        OP, END
    );
    println!(
        "{}    dir{}         to open the config file in the default programm",
        OP, END
    );
    println!(
        "{}    check{}       to check if the config file is correct",
        OP, END
    );
    println!(
        "{}    clean{}       to delete the current configfile",
        OP, END
    );
}

// Function to wait some to read the command in an open Terminal Window
// when an Error appears
//
// TODO
// Make this Interuptable with pressing Enter
pub fn exit(wait: bool, error: bool) {
    if wait {
        std::thread::sleep(std::time::Duration::from_secs(3));
    }
    if error {
        std::process::exit(1);
    } else {
        std::process::exit(0);
    }
}
