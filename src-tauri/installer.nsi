; Define the name of the installer
Outfile "MyTauriAppInstaller.exe"

; Set the default installation directory
InstallDir $PROGRAMFILES\MyTauriApp

; MUI settings
!include "MUI2.nsh"

; Pages
!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!define MUI_PAGE_CUSTOMFUNCTION_SHOW showUrlPage
Page custom urlPage createUrlPage
!insertmacro MUI_PAGE_FINISH

; Languages
!insertmacro MUI_LANGUAGE "English"

; Variables
Var URL

; Custom Page
Function createUrlPage
  nsDialogs::Create 1018
  Pop $0

  ${If} $0 == error
    Abort
  ${EndIf}

  ; URL Text
  nsDialogs::CreateControl Label 0% 0% 100% 12u "Please enter the URL:"
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
    File /r "$DESKTOP\Bio-SYNC.exe"

    ; Save URL to a config file
    FileOpen $0 "$INSTDIR\config.ini" w
    FileWrite $0 "URL=$URL"
    FileClose $0

    ; Create a shortcut on the Desktop
    CreateShortcut "$DESKTOP\MyTauriApp.lnk" "$INSTDIR\Bio-SYNC.exe"

    ; Create a shortcut in the Start Menu
    CreateShortcut "$SMPROGRAMS\MyTauriApp\MyTauriApp.lnk" "$INSTDIR\Bio-SYNC.exe"
SectionEnd

Section "Uninstall"
    ; Remove the installation directory and its contents
    RMDir /r $INSTDIR

    ; Remove the shortcut from the Desktop
    Delete "$DESKTOP\MyTauriApp.lnk"

    ; Remove the shortcut from the Start Menu
    Delete "$SMPROGRAMS\MyTauriApp\MyTauriApp.lnk"
SectionEnd
