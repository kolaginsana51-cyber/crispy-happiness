[Setup]
AppId={{6B1F3C52-7A4E-4E0B-9D55-2C8A1E7F0A11}
AppName=Potato Please 3D
AppVersion=1.0.0
AppPublisher=Potato Please
DefaultDirName={autopf}\PotatoPlease
DefaultGroupName=Potato Please 3D
OutputDir=installer
OutputBaseFilename=PotatoPlease_Setup
Compression=lzma2
SolidCompression=yes
ArchitecturesInstallIn64BitMode=x64compatible
PrivilegesRequiredOverridesAllowed=dialog
DisableProgramGroupPage=yes

[Languages]
Name: "russian"; MessagesFile: "compiler:Languages\Russian.isl"
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"

[Files]
Source: "target\release\potato_please.exe"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\Potato Please 3D"; Filename: "{app}\potato_please.exe"
Name: "{autodesktop}\Potato Please 3D"; Filename: "{app}\potato_please.exe"; Tasks: desktopicon

[Run]
Filename: "{app}\potato_please.exe"; Description: "{cm:LaunchProgram,Potato Please 3D}"; Flags: nowait postinstall skipifsilent
