# Import
!include LogicLib.nsh
!include StrFunc.nsh
${StrStr}

# Name des Installers
Outfile "LuajitSetup.exe"

# Verzeichnis, in das installiert wird
InstallDir "$LOCALAPPDATA\@shadowdara\luajit"

RequestExecutionLevel user   ; <<< Wichtig! Kein Admin

# Standardseite für Benutzer
Page directory
Page instfiles

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

  # ---- PATH hinzufügen ----
  ReadRegStr $0 HKCU "Environment" "Path"
  StrCpy $1 "$INSTDIR"
  Push $0
  Push $1
  Call AddToPath

SectionEnd
