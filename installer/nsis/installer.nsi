# Flua Installer with NSIS for Windows

!include "LogicLib.nsh"
!include "StrFunc.nsh"

${Using:StrFunc} StrStr
!insertmacro STRFUNC_MAKEFUNC StrStr "un."
!insertmacro STRFUNC_MAKEFUNC StrRep "un."

; Danach in Funktionen kannst du:
; Call un.StrStr
; Call un.StrRep

# Name des Installers
Outfile "LuajitSetup.exe"

# Verzeichnis, in das installiert wird
InstallDir "$LOCALAPPDATA\@shadowdara\luajit"

# NO ADMIN
RequestExecutionLevel user

# Standardseite für Benutzer
Page directory
Page instfiles

# -------------------------------
# Uninstaller
# -------------------------------
Section "Uninstall"

  # Dateien löschen
  Delete "$INSTDIR\flua.exe"
  Delete "$INSTDIR\README.md"
  Delete "$INSTDIR\CHANGELOG.md"
  Delete "$INSTDIR\LICENSE"
  Delete "$INSTDIR\Uninstall.exe"
  Delete "$INSTDIR\luajitdocs"
  Delete "$INSTDIR\luajitdocs.cmd"
  Delete "$INSTDIR\luajitdocs.lua"
  Delete "$INSTDIR\luajit"
  Delete "$INSTDIR\luajit.cmd"
  Delete "$INSTDIR\flua"

  # Verzeichnis löschen
  RMDir "$INSTDIR"

  ; Löscht das docs rekursiv
  RMDir /r "$INSTDIR\docs"

  # Verzeichnis löschen
  RMDir "$INSTDIR"

  # Delete Both Directories
  RMDir "$LOCALAPPDATA\@shadowdara"

  Delete "$DESKTOP\lua.lnk"
  Delete "$SMPROGRAMS\Luajit\Uninstall.lnk"
  Delete "$SMPROGRAMS\Luajit\flua.lnk"
  RMDir "$SMPROGRAMS\Luajit"

  ; Read current PATH from registry
  ReadRegStr $0 HKCU "Environment" "Path"

  ; Call un.StrStr to check if $INSTDIR is in PATH
  Push "$INSTDIR"
  Push $0
  Call un.StrStr
  Pop $1 ; $1 = result of un.StrStr (position or empty)

  ${If} $1 != ""
    ; Remove $INSTDIR from PATH with un.StrRep

    Push ""             ; replacement string (empty)
    Push "$INSTDIR;"    ; string to remove with trailing semicolon
    Push $0             ; original PATH
    Call un.StrRep
    Pop $2              ; first replacement result

    Push ""             ; replacement string (empty)
    Push ";$INSTDIR"    ; string to remove with leading semicolon
    Push $2             ; previous result
    Call un.StrRep
    Pop $2              ; second replacement result

    Push ""             ; replacement string (empty)
    Push "$INSTDIR"     ; string to remove (no semicolon)
    Push $2             ; previous result
    Call un.StrRep
    Pop $2              ; final result

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
  CreateDirectory "$INSTDIR\docs"

  ; Rekursiv alle Dateien und Ordner einbinden
  SetOutPath "$INSTDIR\docs"
  File /r "..\..\site\*.*"

  # Kopiere Datei(en) ins Zielverzeichnis
  SetOutPath "$INSTDIR"
  File "..\..\flua.exe"
  File "..\..\README.md"
  File "..\..\CHANGELOG.md"
  File "..\..\LICENSE"
  File "..\docs\luajitdocs"
  File "..\docs\luajitdocs.cmd"
  File "..\docs\luajitdocs.lua"
  File "..\wrapper\luajit"
  File "..\wrapper\luajit.cmd"
  File "..\wrapper\flua"
  File "..\..\build\heart.ico"

  # Nutzer fragen, ob .lua-Dateien damit verknüpft werden sollen
  MessageBox MB_YESNO "Do you want to use luajit as the standard program for .lua files?" IDNO skip_assoc

  # Dateityp- und ProgID-Verknüpfung setzen
  WriteRegStr HKCR ".lua" "" "MyLuaFile"
  WriteRegStr HKCR "MyLuaFile" "" "Lua-Skript"
  WriteRegStr HKCR "MyLuaFile\DefaultIcon" "" "$INSTDIR\heart.ico,0"
  WriteRegStr HKCR "MyLuaFile\shell\open\command" "" '"$INSTDIR\lua.exe" "%1"'

  # Info für Windows 10/11 Standardprogramme-UI (nur teilweise effektiv)
  # Optional: ApplicationCapabilities setzen (mehr Aufwand)

skip_assoc:

  # ---- PATH hinzufügen ----
  ReadRegStr $0 HKCU "Environment" "Path"
  StrCpy $1 "$INSTDIR"
  Push $0
  Push $1
  Call AddToPath

  # Create Startmenu Directory
  CreateDirectory "$SMPROGRAMS\Luajit"

  # Shortcuts
  CreateShortCut "$SMPROGRAMS\Luajit\flua.lnk" "$INSTDIR\flua.exe"
  CreateShortCut "$SMPROGRAMS\Luajit\Uninstall.lnk" "$INSTDIR\Uninstall.exe"

  ; Desktop Shortcut
  CreateShortCut "$DESKTOP\flua.lnk" "$INSTDIR\flua.exe" "" "$INSTDIR\heart.ico"

  # ---- Uninstaller schreiben ----
  WriteUninstaller "$INSTDIR\Uninstall.exe"

  # ---- In Windows-Softwareliste eintragen ----
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\LuajitInstaller" "DisplayName" "Flua (Benutzerinstallation)"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\LuajitInstaller" "UninstallString" "$INSTDIR\Uninstall.exe"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\LuajitInstaller" "DisplayIcon" "$INSTDIR\flua.exe"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\LuajitInstaller" "InstallLocation" "$INSTDIR"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\LuajitInstaller" "Publisher" "@shadowdara"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\LuajitInstaller" "DisplayVersion" "0.2.0"

SectionEnd
