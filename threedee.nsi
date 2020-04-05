; reference / tutorial: https://www2.seas.gwu.edu/~drum/java/lectures/appendix/installer/install.html

; The name of the installer
Name "Threedee"

; The outputfile to write, should always be setup.exe
OutFile "Threedee installer.exe"

; The text to prompt the user to enter a directory
DirText "Select Threedee install location"

; The default installation directory
InstallDir "$PROGRAMFILES64\bcow.me Threedee"

; The stuff to install
Section "Install" ;No components page, name is not important
    ; Set output path to the installation directory.
    SetOutPath $INSTDIR
    File target/x86_64-pc-windows-gnu/release/threedee.exe
    File .env
    File tmp/SDL2/x86_64-w64-mingw32/bin/SDL2.dll
    File skull.obj
    File teapot.obj
    File "LICENSE"

    ; Create start menu program group and icon
    CreateDirectory "$SMPROGRAMS\bcow.me Threedee\"
    CreateShortCut "$SMPROGRAMS\bcow.me Threedee\Threedee.lnk" "$INSTDIR\threedee.exe"
    CreateShortCut "$SMPROGRAMS\bcow.me Threedee\Uninstall Threedee.lnk" "$INSTDIR\Uninstall.exe"

    ; Tell the compiler to write an uninstaller and to look for a "Uninstall" section
    WriteUninstaller $INSTDIR\Uninstall.exe
    ; Add the uninstaller to add & remove programs (registry edit)
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\bcow.me Threedee" "DisplayName" "Threedee (remove only)"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\bcow.me Threedee" "UninstallString" "$INSTDIR\Uninstall.exe"
SectionEnd ; end the section

; The uninstall section
Section "Uninstall"
    ; remove start menu items
    Delete "$SMPROGRAMS\bcow.me Threedee\Threedee.lnk"
    Delete "$SMPROGRAMS\bcow.me Threedee\Uninstall Threedee.lnk"
    RMDIR "$SMPROGRAMS\bcow.me Threedee"

    ; remove installed files
    Delete $INSTDIR\Uninstall.exe
    Delete $INSTDIR\threedee.exe
    Delete $INSTDIR\SDL2.dll
    Delete $INSTDIR\.env
    Delete $INSTDIR\skull.obj
    Delete $INSTDIR\teapot.obj
    Delete "LICENSE"
    RMDir $INSTDIR

    ; remove registry keys
    DeleteRegKey HKEY_LOCAL_MACHINE "SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\bcow.me Threedee"
SectionEnd ; end the uninstall section

; eof
