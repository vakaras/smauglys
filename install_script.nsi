!define PRODUCT_NAME "Smauglys"
!define PRODUCT_VERSION "1.0"
!define PRODUCT_PUBLISHER "Vytautas Astrauskas, Martynas Teleiša, Mantas Urbonas"

SetCompressor lzma

;!include "UserManagement.nsh"

; MUI 1.67 compatible ------
!include "MUI.nsh"

; MUI Settings
!define MUI_ABORTWARNING
!define MUI_ICON "${NSISDIR}\Contrib\Graphics\Icons\modern-install.ico"

; Welcome page
!insertmacro MUI_PAGE_WELCOME
; Components page
!insertmacro MUI_PAGE_COMPONENTS
; Instfiles page
!insertmacro MUI_PAGE_INSTFILES
; Finish page
!insertmacro MUI_PAGE_FINISH

; Language files
!insertmacro MUI_LANGUAGE "English"

; Reserve files
!insertmacro MUI_RESERVEFILE_INSTALLOPTIONS

Name "${PRODUCT_NAME} ${PRODUCT_VERSION}"
OutFile "smauglys_installer.exe"
InstallDir "$TEMP\Smauglys"
ShowInstDetails show
RequestExecutionLevel admin

!define /IfNDef LVM_GETITEMCOUNT 0x1004
!define /IfNDef LVM_GETITEMTEXTA 0x102D
!define /IfNDef LVM_GETITEMTEXTW 0x1073
!if "${NSIS_CHAR_SIZE}" > 1
!define /IfNDef LVM_GETITEMTEXT ${LVM_GETITEMTEXTW}
!else
!define /IfNDef LVM_GETITEMTEXT ${LVM_GETITEMTEXTA}
!endif

Var main_install_log_path
Var action_message

Section -SETTINGS
  StrCpy $main_install_log_path "$INSTDIR\install.log"
  StrCpy $action_message "$\r$\n Detalesnę informaciją apie tai kas įvyko galite rasti detalios informacijos laukelyje ir $main_install_log_path faile. Diegimo programą uždaryti galite paspausdami “Cancel” mygtuką."
  SetOutPath "$INSTDIR"
  SetOverwrite ifnewer
SectionEnd

; -----------------------------------------
;   SECTION: INSTALL PYTHON
; -----------------------------------------
Section "Python 3.8" SEC01
  DetailPrint "### Pradedamas Python 3.8 diegimas. ###"
  ClearErrors
  File "PythonInstaller.exe"

  ; Install Python.
  ExecWait '"$INSTDIR\PythonInstaller.exe" /passive InstallAllUsers=1 PrependPath=1' $0
  IfErrors handleErrorInstallPython
  DetailPrint "$INSTDIR\PythonInstaller.exe baigė be klaidų."

  ; Check if Python is successfully installed at the expected location.
  ; It might have only updated if user already had Python installed locally.
  IfFileExists "$PROGRAMFILES64\Python38\python.exe" 0 pythonInstalledCheckFailed
  DetailPrint "### Baigtas Python 3.8 diegimas. ###"
  Return

  pythonInstalledCheckFailed:
    DetailPrint "Nepavyko! Nerastas: $PROGRAMFILES64\Python38\python.exe po Python diegimo!"
    Call WriteLogToFile
    MessageBox MB_OK "Nepavyko įdiegti Python 3.8: Patikrinkite, ar neturite jau įdiegto Python, išdiekite ir bandykite dar kartą. $action_message"
    Abort

  handleErrorInstallPython:
    DetailPrint "Python diegimas baigė kodu:$0"
    Call WriteLogToFile
    MessageBox MB_OK "Nepavyko įdiegti Python 3.8: Patikrinkite, ar veikia internetas, ir bandykite dar kartą. $action_message"
    Abort
SectionEnd

; -----------------------------------------
;   SECTION: INSTALL PYTHON PACKAGES
; -----------------------------------------
Section "Python 3.8 packages"
  DetailPrint "### Pradedamas Python 3.8 paketų diegimas. ###"
  File "requirements.txt"
  File /r "python_packages\"

  ; Install Python packages. Log output.
  nsExec::ExecToLog '"$PROGRAMFILES64\Python38\python.exe" -m pip install --no-index --find-links "$INSTDIR" -r "$INSTDIR\requirements.txt"'
  IfErrors handleErrorBeforeInstallPackages
  Pop $0
  DetailPrint "Python 3.8 paketų diegimas baigė kodu:$0"
  StrCmp $0 "0" 0 handleErrorInstallPackages
  DetailPrint "### Baigtas Python 3.8 paketų diegimas. ###"
  Return

  handleErrorBeforeInstallPackages:
    Call WriteLogToFile
    MessageBox MB_OK "Nepavyko paleisti Python paketų diegimo. $action_message"
    Abort

  handleErrorInstallPackages:
    Call WriteLogToFile
    MessageBox MB_OK "Nepavyko įdiegti Python paketų. $action_message"
    Abort
SectionEnd

; ---------------------------------------------------------------
;   SECTION: INSTALL VISUAL STUDIO CODE, EXTENSIONS, CONFIGURE
; ---------------------------------------------------------------
Section "Visual Studio Code" SEC02
  DetailPrint "### Pradedamas Visual Studio Code diegimas. ###"
  EnVar::SetHKLM
  EnVar::AddValue "VSCODE_EXTENSIONS" "$PROGRAMFILES64\VS Code Extensions"

  Var /GLOBAL vscode_setup_log_path
  StrCpy $vscode_setup_log_path "$instdir\l1.txt"

  File "VSCodeSetup.exe"
  ExecWait '"$INSTDIR\VSCodeSetup.exe" /LOG="$vscode_setup_log_path" /ALLUSERS /SILENT /MERGETASKS=!runcode,desktopicon,addcontextmenufiles,addcontextmenufolders,associatewithfiles'

  IfErrors handleErrorVSCodeSetup

  File "vscode_extensions\ms-python.python.vsix"
  File "vscode_extensions\hediet.debug-visualizer.vsix"
  File "vscode_extensions\vakaras.vscode-language-pack-lt.vsix"
  File "vscode_extensions\formulahendry.code-runner.vsix"

  IfErrors handleErrorExtensionFiles

  FileOpen $0 "$instdir\install-extensions.bat" w
  FileWrite $0 'set VSCODE_EXTENSIONS=$PROGRAMFILES64\VS Code Extensions$\r$\n'

  FileWrite $0 'call "$PROGRAMFILES64\Smauglys\bin\smauglys.cmd" --install-extension ms-python.python.vsix$\r$\n'
  FileWrite $0 'call "$PROGRAMFILES64\Smauglys\bin\smauglys.cmd" --install-extension hediet.debug-visualizer.vsix$\r$\n'
  FileWrite $0 'call "$PROGRAMFILES64\Smauglys\bin\smauglys.cmd" --install-extension formulahendry.code-runner.vsix$\r$\n'
  FileWrite $0 'call "$PROGRAMFILES64\Smauglys\bin\smauglys.cmd" --install-extension vakaras.vscode-language-pack-lt.vsix$\r$\n'
  FileClose $0

  IfErrors handleErrorBuildInstallExtensionsScript

  nsExec::ExecToLog "$instdir\install-extensions.bat"
  Pop $0
  StrCmp $0 "0" 0 handleErrorInstallExtensions

  ; Configure Visual Studio Code and extensions.
  File "vscode_monkey.py"

  nsExec::ExecToLog '"$PROGRAMFILES64\Python38\python.exe" vscode_monkey.py "$PROGRAMFILES64\VS Code Extensions" "$PROGRAMFILES64\VS Code Extensions\monkey.log"'

  Pop $0
  IfErrors handleErrorPostInstallConfigure

  DetailPrint "### Baigtas Visual Studio Code diegimas. ###"
  Return

  handleErrorVSCodeSetup:
    DetailPrint "Visual Studio Code diegimo žurnalas išsaugotas: $vscode_setup_log_path"
    Call WriteLogToFile
    MessageBox MB_OK "Nepavyko įdiegti kodo redaktoriaus. Bandykite dar kartą. $action_message"
    Abort

  handleErrorExtensionFiles:
    DetailPrint "Nepavyko paruošti plėtinių diegimo failų."
    Call WriteLogToFile
    MessageBox MB_OK "Nepavyko paruošti plėtinių diegimo failų. Bandykite dar kartą. $action_message"
    Abort
  
  handleErrorBuildInstallExtensionsScript:
    DetailPrint "Nepavyko paruošti install-extensions.bat."
    Call WriteLogToFile
    MessageBox MB_OK "Nepavyko paruošti plėtinių diegimo programos. Bandykite dar kartą. $action_message"
    Abort

  handleErrorInstallExtensions:
    DetailPrint "install-extensions.bat baigė kodu:$0"
    Call WriteLogToFile
    MessageBox MB_OK "Nepavyko įdiegti plėtinių. $action_message"
    Abort

  handleErrorPostInstallConfigure:
    DetailPrint "vscode_monkey.py baigė kodu:$0"
    Call WriteLogToFile
    MessageBox MB_OK "Nepavyko sukonfigūruoti plėtinių. $action_message"
    Abort
SectionEnd

; -----------------------------------------------
;   SECTION: INSTALL PYTHON DOCUMENTATION
; -----------------------------------------------
Section "Python 3.8 documentation"
  DetailPrint "### Pradedamas Python 3.8 dokumentacijos diegimas. ###"

  SetShellVarContext all
  CreateShortCut "$DESKTOP\Python Documentation.lnk" "$PROGRAMFILES64\Python38\Doc\python3810.chm"
  DetailPrint "### Baigtas Python 3.8 dokumentacijos diegimas. ###"
SectionEnd

; -----------------------------------------------
;   SECTION: Create uninstaller
; -----------------------------------------------
Section "Uninstaller creation"
  DetailPrint "### Kuriama išdiegimo programa. ###"
  Rename "$PROGRAMFILES64\Smauglys\unins000.exe" "$PROGRAMFILES64\Smauglys\unins000-orig.exe"
  WriteUninstaller "$PROGRAMFILES64\Smauglys\unins000.exe"
  DetailPrint "### Išdiegimo programa sukurta. ###"
SectionEnd

; -----------------------------------------------
;   SECTION: REMOVE TEMPORARY FILES
; -----------------------------------------------
Section "Remove temp files" SEC03
  DetailPrint "### Pradedamas laikinų diegimo failų trynimas. ###"
  SetOutPath $TEMP
  RMDir /r /REBOOTOK $TEMP\Smauglys

  DetailPrint "### Baigtas laikinų diegimo failų trynimas. ###"
  StrCpy $0 "$PROGRAMFILES64\Smauglys\install.log"
  Push $0
  Call DumpLog
  DetailPrint "### Baigtas diegimas. ###"
SectionEnd

; -----------------------------------------------
;   SECTION: UNINSTALLER
; -----------------------------------------------
Section "Uninstall"
  delete /REBOOTOK "$PROGRAMFILES64\Smauglys\install.log"
  delete /REBOOTOK "$DESKTOP\Python Documentation.lnk"
  delete /REBOOTOK "$PROGRAMFILES64\Smauglys\unins000.exe"
  ExecWait '"$PROGRAMFILES64\Smauglys\unins000-orig.exe" /SILENT'
  RMDir /REBOOTOK "$PROGRAMFILES64\Smauglys"
  EnVar::SetHKLM
  EnVar::Delete "VSCODE_EXTENSIONS"
  RMDir /r /REBOOTOK "$PROGRAMFILES64\VS Code Extensions"
SectionEnd

; -----------------------------------------------
;   HELPER FUNCTIONS
; -----------------------------------------------

Function WriteLogToFile
  DetailPrint "Diegimo žurnalas išsaugotas: $main_install_log_path"
  StrCpy $0 "$main_install_log_path"
  Push $0
  Call DumpLog
FunctionEnd

; Dumps the log of the installer to a specified file.
; https://nsis.sourceforge.io/Dump_log_to_file
Function DumpLog
  Exch $5
  Push $0
  Push $1
  Push $2
  Push $3
  Push $4
  Push $6
  FindWindow $0 "#32770" "" $HWNDPARENT
  GetDlgItem $0 $0 1016
  StrCmp $0 0 exit
  FileOpen $5 $5 "w"
  StrCmp $5 "" exit
    SendMessage $0 ${LVM_GETITEMCOUNT} 0 0 $6
    System::Call '*(&t${NSIS_MAX_STRLEN})p.r3'
    StrCpy $2 0
    System::Call "*(i, i, i, i, i, p, i, i, i) p  (0, 0, 0, 0, 0, r3, ${NSIS_MAX_STRLEN}) .r1"
    loop: StrCmp $2 $6 done
      System::Call "User32::SendMessage(p, i, p, p) p ($0, ${LVM_GETITEMTEXT}, $2, r1)"
      System::Call "*$3(&t${NSIS_MAX_STRLEN} .r4)"
      !ifdef DumpLog_As_UTF16LE
      FileWriteUTF16LE ${DumpLog_As_UTF16LE} $5 "$4$\r$\n"
      !else
      FileWrite $5 "$4$\r$\n" ; Unicode will be translated to ANSI!
      !endif
      IntOp $2 $2 + 1
      Goto loop
    done:
      FileClose $5
      System::Free $1
      System::Free $3
  exit:
    Pop $6
    Pop $4
    Pop $3
    Pop $2
    Pop $1
    Pop $0
    Pop $5
FunctionEnd
