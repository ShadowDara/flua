pub mod dir;
pub mod macros;
pub mod print;
pub mod update;

use crate::VERSION;
use crate::helper::print::{BLUE, BOLD, CYAN, END, GREEN, PURPLE, RED, YELLOW};

// Function for a detailed INFO help Message
pub fn print_help() {
    //[GENERALL-OPTIONS]
    let sco = PURPLE;

    //[SCRIPTOPTIONS]
    let sc = YELLOW;

    //[OPTIONS]
    let opt = CYAN;

    //<action>
    let ac = RED;

    //<custom-action-options>
    let aco = CYAN;

    // OptionList
    let op = BLUE;

    // Links
    let link = BLUE;

    // Start
    println!("{}Flua Help:{}", GREEN, END);
    println!("Using Version {}{}v{}{}", BOLD, GREEN, VERSION, END);
    // Lua Scripts
    println!(
        "\nUsage for Lua scripts: {}<flua>{} <script.lua> {}[SCRIPTOPTIONS]{} {}[GENERALL-OPTIONS]{}",
        GREEN, END, sc, END, sco, END
    );
    //[SCRIPTOPTIONS]
    println!("\n{}[SCRIPTOPTIONS]{}", sc, END);
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
        GREEN, END, ac, END, aco, END, sco, END
    );
    println!("\n{}<actions>{}:   'update', 'install'", ac, END);
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
        "{}  -no-config     {}The Programm will not search or load the config file. Standard Option is true, if there is no config file, the step will be skipped. A config file will not be created automaticly!",
        op, END
    );
    // More
    println!("\nFor more info about Flua and the Lua API, see:");
    println!("{}https://github.com/ShadowDara/LuaAPI-Rust{}", link, END);
    println!("{}https://shadowdara.github.io/flua/{}", link, END);
}
