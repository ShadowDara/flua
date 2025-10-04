[Setup]
AppName=Luajit
AppVersion=0.1.10
AppPublisher=@shadowdara
DefaultDirName={userappdata}\@shadowdara\Luajit
OutputDir=output
OutputBaseFilename=LuajitSetupInno
Compression=lzma2
SolidCompression=yes
DisableProgramGroupPage=yes
WizardStyle=modern

[Files]
Source: "..\..\target\release\luajit.exe"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\Luajit"; Filename: "{app}\luajit.exe"
Name: "{userdesktop}\Luajit"; Filename: "{app}\luajit.exe"

[Run]
Filename: "{app}\luajit.exe"; Description: "Programm starten"; Flags: nowait postinstall skipifsilent
