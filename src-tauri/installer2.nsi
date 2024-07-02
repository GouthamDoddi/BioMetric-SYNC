; Define the name of the installer
Outfile "BioMetric-SYNC-installer.exe"

; Set the default installation directory
InstallDir $PROGRAMFILES\BioMetric-SYNC

; MUI settings
!include "MUI2.nsh"

; Pages
!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!define MUI_PAGE_CUSTOMFUNCTION_SHOW showUrlPage
Page custom createUrlPage showUrlPage
!insertmacro MUI_PAGE_FINISH

; Variables
Var URL

; Languages
!insertmacro MUI_LANGUAGE "English"

; Custom Page
Function createUrlPage
  nsDialogs::Create 1018
  Pop $0

  ${If} $0 == error
    Abort
  ${EndIf}

  ; URL Text
  nsDialogs::CreateControl Label 0% 0% 100% 12u "Please enter the URL for BioMetric-SYNC:"
  Pop $0

  ; URL Input
  nsDialogs::CreateControl Edit 0% 12u 100% 12u ""
  Pop $URL
  nsDialogs::Show
FunctionEnd

Function showUrlPage
  nsDialogs::Show
FunctionEnd

Section "Install"
    ; Create installation directory
    SetOutPath $INSTDIR

    ; Write the executable
    File "C:\Users\POS\tauriDemo\src-tauri\target\release\BioMetric-SYNC.exe"

    ; Save URL to a config file
    FileOpen $0 "$INSTDIR\config.ini" w
    FileWrite $0 "URL=$URL"
    FileClose $0

    ; Create a shortcut on the Desktop
    CreateShortcut "$DESKTOP\BioMetric-SYNC.lnk" "$INSTDIR\BioMetric-SYNC.exe"

    ; Create a shortcut in the Start Menu
    CreateDirectory "$SMPROGRAMS\BioMetric-SYNC"
    CreateShortcut "$SMPROGRAMS\BioMetric-SYNC\BioMetric-SYNC.lnk" "$INSTDIR\BioMetric-SYNC.exe"
SectionEnd