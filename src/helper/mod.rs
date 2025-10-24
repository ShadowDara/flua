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
const sco: &str = PURPLE;

//[SCRIPTOPTIONS]
const SC: &str = YELLOW;

//[OPTIONS]
const opt: &str = CYAN;

//<action>
const AC: &str = RED;

//<custom-action-options>
const aco: &str = CYAN;

// OptionList
const op: &str = BLUE;

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
        GREEN, END, SC, END, sco, END
    );
    //[SCRIPTOPTIONS]
    println!("\n{}[SCRIPTOPTIONS]{}", SC, END);
    println!(
        "{}  --safe:        {}Run in safe mode (limited API, no OS access)",
        op, END
    );
    println!(
        "{}  --no-info:     {}Suppress start and end info messages",
        op, END
    );
    println!(
        "{}  l, lua-args    {}submit arguments after lua-args for the lua file which will be run, stop the collectiong when it sees an argument which starts with: {}'-'{}",
        op, END, RED, END
    );
    // Modules
    println!(
        "\nUsage for Modules: {}<flua>{} run {}<actions>{} {}<custom-action-options>{} {}[GENERALL-OPTIONS]{}",
        GREEN, END, AC, END, aco, END, sco, END
    );
    println!("\n{}<actions>{}:   'update', 'install'", AC, END);
    println!(
        "{}  module         {}Argument to run a {}dlm13{} Module",
        op, END, RED, END
    );
    println!(
        "\n{}<custom-action-options>{}: Ends the arg collecting when a path starts with #",
        aco, END
    );
    println!("{}  -path=    {}Add the Module path after the '='", op, END);
    // Other
    println!(
        "\nOther Usage: {}<flua>{} {}[OPTIONS]{} {}[GENERALL-OPTIONS]{}",
        GREEN, END, opt, END, sco, END
    );
    //[OPTIONS]
    println!("\n{}[OPTIONS]{}", opt, END);
    println!(
        "{}  -v, --version  {}Function which prints the version in the terminal",
        op, END
    );
    println!(
        "{}  h, -h, --help  {}Function which prints this help message in the terminal",
        op, END
    );
    //[GENERALL-OPTIONS]
    println!("\n{}[GENERALL-OPTIONS]{}", sco, END);
    println!(
        "{}  -nw            {}No exit -> The Programm closes instantly after an error and showing an error message without waiting for some Time.",
        op, END
    );
    println!(
        "{}  -no-config     {}The Programm will not search or load the Config file. Standard Option is true, if there is no Config file, the step will be skipped. A Config file will not be created automaticly!",
        op, END
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
        CONFIG, END, sco, END
    );
    println!("\n{}[CONFIGOPTIONS]{}", CONFIG, END);
    println!(
        "{}    generate{}    Be careful! This will create a new config file, the old one will be overritten",
        op, END
    );
    println!(
        "{}    open{}        to open the config file in the default programm",
        op, END
    );
    println!(
        "{}    check{}       to check if the config file is correct",
        op, END
    );
}
