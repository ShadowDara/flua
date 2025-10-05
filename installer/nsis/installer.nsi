# Import
!include LogicLib.nsh
!include StrFunc.nsh
${StrStr}
${StrRep}
${UnStrStr}

# Name des Installers
Outfile "LuajitSetup.exe"

# Verzeichnis, in das installiert wird
InstallDir "$LOCALAPPDATA\@shadowdara\luajit"

RequestExecutionLevel user   ; <<< Wichtig! Kein Admin

# Standardseite für Benutzer
Page directory
Page instfiles

# -------------------------------
# Uninstaller
# -------------------------------
Section "Uninstall"

  # Dateien löschen
  Delete "$INSTDIR\luajit.exe"
  Delete "$INSTDIR\README.md"
  Delete "$INSTDIR\CHANGELOG.md"
  Delete "$INSTDIR\LICENSE"

  # Verzeichnis löschen
  RMDir "$INSTDIR"

  Delete "$SMPROGRAMS\Luajit\Uninstall.lnk"
  RMDir "$SMPROGRAMS\Luajit"

  # PATH aus Umgebungsvariablen entfernen
  ReadRegStr $0 HKCU "Environment" "Path"
  ${UnStrStr} $1 $0 "$INSTDIR"
  ${If} $1 != ""
    ${StrRep} $2 $0 "$INSTDIR;" ""
    ${StrRep} $2 $2 ";$INSTDIR" ""
    ${StrRep} $2 $2 "$INSTDIR" ""
    WriteRegExpandStr HKCU "Environment" "Path" "$2"
    System::Call 'user32::SendMessageTimeoutA(i 0xffff, i 0x1A, i 0, t "Environment", i 0, i 1000, *i .r0)'
  ${EndIf}

  # Entferne Uninstaller-Eintrag aus der Registry
  DeleteRegKey HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\LuajitInstaller"

SectionEnd

# -------------------------------
# Funktion: AddToPath
# -------------------------------
Function AddToPath
  Exch $1 ; new path
  Exch
  Exch $0 ; old path
  Push $2
  Push $3
  Push $4

  StrCpy $2 "$0"
  StrCpy $3 "$1"

  ${StrStr} $4 $2 $3

  ${If} $4 == ""
    StrCpy $2 "$2;$3"
    WriteRegExpandStr HKCU "Environment" "Path" "$2"
    System::Call 'user32::SendMessageTimeoutA(i 0xffff, i 0x1A, i 0, t "Environment", i 0, i 1000, *i .r0)'
  ${EndIf}

  Pop $4
  Pop $3
  Pop $2
  Pop $0
  Pop $1
FunctionEnd

# -------------------------------
# Sektion: Installiere Programm
# -------------------------------
Section "Install"

  # Erstelle Installationsverzeichnis
  CreateDirectory "$INSTDIR"

  # Kopiere Datei(en) ins Zielverzeichnis
  SetOutPath "$INSTDIR"
  File "..\..\target\release\luajit.exe"
  File "..\..\README.md"
  File "..\..\CHANGELOG.md"
  File "..\..\LICENSE"

  # ---- PATH hinzufügen ----
  ReadRegStr $0 HKCU "Environment" "Path"
  StrCpy $1 "$INSTDIR"
  Push $0
  Push $1
  Call AddToPath

  # Uninstaller Shortcut
  CreateShortCut "$SMPROGRAMS\Luajit\Uninstall.lnk" "$INSTDIR\Uninstall.exe"

  # ---- Uninstaller schreiben ----
  WriteUninstaller "$INSTDIR\Uninstall.exe"

  # ---- In Windows-Softwareliste eintragen ----
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\LuajitInstaller" "DisplayName" "Luajit (Benutzerinstallation)"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\LuajitInstaller" "UninstallString" "$INSTDIR\Uninstall.exe"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\LuajitInstaller" "DisplayIcon" "$INSTDIR\luajit.exe"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\LuajitInstaller" "InstallLocation" "$INSTDIR"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\LuajitInstaller" "Publisher" "@shadowdara"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\LuajitInstaller" "DisplayVersion" "1.0.0"

SectionEnd
