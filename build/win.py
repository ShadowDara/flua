# Requirements
# cargo
# NSIS

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

    print("Clear Build Folder")

    try: shutil.rmtree("windows_builds")
    except: pass

    os.makedirs("windows_builds", exist_ok=True)

    print("Run cargo build --release")
    result = subprocess.run("cargo build --release", shell=True, capture_output=True, text=True)
    print("STDOUT:", result.stdout)
    print("STDERR:", result.stderr)
    print("Returncode:", result.returncode)

    # Copy the Executable to the root
    shutil.copy("target/release/flua.exe", "flua.exe")

    os.chdir('installer')

    print("Make Installer")
    os.chdir('nsis')

    result = subprocess.run("makensis installer.nsi", shell=True, capture_output=True, text=True)
    print("STDOUT:", result.stdout)
    print("STDERR:", result.stderr)
    print("Returncode:", result.returncode)

    shutil.copy("LuajitSetup.exe", "../../windows_builds")

    os.chdir('../..')

    # Get the Version for Luajit
    version = get_version("Cargo.toml")

    os.chdir('windows_builds')
    oldname1 = "LuajitSetup"
    os.rename(oldname1 + ".exe", str(oldname1) + "_v" + str(version) + ".exe")

    print("Finished Build for Windows")
