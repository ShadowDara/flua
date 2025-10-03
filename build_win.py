import os
import shutil
import subprocess

def get_version(file):
    with open(file, "r", encoding="utf-8") as f:
        for line in f:
            line = line.strip()
            if line.startswith("version"):
                # Zeile sieht z.B. so aus: version = "0.1.9"
                # Wir splitten nach '=' und holen das, was in Anführungszeichen steht
                parts = line.split('=', 1)
                if len(parts) == 2:
                    val = parts[1].strip().strip('"').strip("'")
                    return val

if __name__ == "__main__":
    print("LuajitBuilt for Windows")
    
    print("Run cargo Built")
    subprocess.run("cargo build --release", shell=True, capture_output=True, text=True)

    print("Make UserInstaller")
    os.chdir('installer/nsis')
    subprocess.run("makensis installer.nsi", shell=True, capture_output=True, text=True)
    shutil.copy("LuajitSetup.exe", "../../windows_builds")
    
    print("Make AdminInstaller")
    os.chdir('../nsis-admin')
    subprocess.run("makensis installer.nsi", shell=True, capture_output=True, text=True)
    shutil.copy("LuajitSetup_Admin.exe", "../../windows_builds")
    
    # Get the Version for Luajit
    os.chdir('../..')
    version = get_version("Cargo.toml")
    
    os.chdir('windows_builds')
    oldname1 = "LuajitSetup"
    oldname2 = "LuajitSetup_Admin"
    os.rename(oldname1 + ".exe", str(oldname1) + "_v" + str(version) + ".exe")
    os.rename(oldname2 + ".exe", str(oldname2) + "_v" + str(version) + ".exe")
    
    print("Finished")
