# Requirements
# cargo
# mkdocs
# Inno Setup
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
    windows_export_folder = "windows_builds"
    
    print("LuajitBuilt for Windows")
    
    print("Clear Build")
    try: shutil.rmtree("windows_builds")
    except: pass
    
    # Build Mkdocs
    # print("Build Mkdocs")
    # subprocess.run("mkdocs build", shell=True, capture_output=True, text=True)
    # shutil.copytree("site/", "windows_builds/docs/", dirs_exist_ok=True)
    # shutil.copytree("installer/win", "windows_builds/", dirs_exist_ok=True)
    
    os.makedirs("windows_builds", exist_ok=True)
    
    print("Run cargo build --release")
    subprocess.run("cargo build --release", shell=True, capture_output=True, text=True)
    
    os.chdir('installer/nsis')

    print("Make UserInstaller")
    os.chdir('../nsis')
    subprocess.run("makensis installer.nsi", shell=True, capture_output=True, text=True)
    shutil.copy("LuajitSetup.exe", "../../windows_builds")
    
    os.chdir('../..')
    
    # Get the Version for Luajit
    version = get_version("Cargo.toml")
    
    os.chdir('windows_builds')
    oldname1 = "LuajitSetup"
    os.rename(oldname1 + ".exe", str(oldname1) + "_v" + str(version) + ".exe")
    
    print("Finished")
