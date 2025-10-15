// Code to Update the application

// Function to check if the correct Version of Flua is used
//
// Rules in strictmode
// The third number is allowed to be different
// The 2nd and 1st number must be equal !!!
pub fn version_checker(version: &str, this_version: &str) -> bool {
    // Wanted Version
    let teile: Vec<&str> = version.split('.').collect();
    let v_major;
    let v_minor;
    let v_patch;

    if teile.len() == 3 {
        v_major = teile[0].parse::<u32>().unwrap_or(0);
        v_minor = teile[1].parse::<u32>().unwrap_or(0);
        v_patch = teile[2].parse::<u32>().unwrap_or(0);
    } else {
        return false;
    }

    // Used Version
    let teile2: Vec<&str> = this_version.split('.').collect();
    let u_major;
    let u_minor;
    let u_patch;

    if teile2.len() == 3 {
        u_major = teile2[0].parse::<u32>().unwrap_or(0);
        u_minor = teile2[1].parse::<u32>().unwrap_or(0);
        u_patch = teile2[2].parse::<u32>().unwrap_or(0);
    } else {
        return false;
    }

    // Vergleichslogik
    if v_major != u_major {
        return false;
    }

    if v_minor != u_minor {
        return false;
    }

    if v_patch > u_patch {
        return false;
    }

    true
}

pub fn update() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(windows)]
    {
        // Windows-specific update logic can go here
    }

    #[cfg(not(windows))]
    {
        // Non-Windows update logic can go here
    }

    // Placeholder for update logic
    println!("Checking for updates...");
    // Simulate update process
    println!("No updates available.");

    Ok(())
}

pub fn install() -> Result<(), Box<dyn std::error::Error>> {
    // Download ZIP from GitHub Releases and extract it
    // to APPDATA Local

    #[cfg(windows)]
    {
        // Windows-specific installation logic can go here
    }

    #[cfg(not(windows))]
    {
        // Non-Windows installation logic can go here
    }

    // Placeholder for installation logic
    println!("Installing the application...");
    // Simulate installation process
    println!("Installation complete.");

    Ok(())
}
