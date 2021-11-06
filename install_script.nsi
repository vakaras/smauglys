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


Section -SETTINGS
  SetOutPath "$INSTDIR"
  SetOverwrite ifnewer
SectionEnd

Section "Python 3.8" SEC01
  ClearErrors
  File "PythonInstaller.exe"

  ; Install Python.
  ExecWait '"$INSTDIR\PythonInstaller.exe" /passive InstallAllUsers=1 PrependPath=1' $0
  IfErrors handleErrorInstallPython
  DetailPrint "Success! $INSTDIR\PythonInstaller.exe completed without errors."

  ; Check if Python is successfully installed at the expected location.
  ; It might have only updated if user already had Python installed locally.
  IfFileExists "$PROGRAMFILES64\Python38\python.exe" 0 pythonInstalledCheckFailed
  Return

  pythonInstalledCheckFailed:
    DetailPrint "Failed! Could not find: $PROGRAMFILES64\Python38\python.exe after installing Python!"
    Call WriteLogToFile
    MessageBox MB_OK "Nepavyko instaliuoti Python 3.8: Patikrinkite, ar neturite jau instaliuoto Python, išdiekite ir bandykite dar kartą."
    Quit

  handleErrorInstallPython:
    DetailPrint "Install Python returned with error code:$0"
    Call WriteLogToFile
    MessageBox MB_OK "Nepavyko instaliuoti Python 3.8: Patikrinkite, ar veikia internetas, ir bandykite dar kartą"
    Quit
SectionEnd

Section "Python 3.8 packages"
  File "requirements.txt"
  File /r "python_packages\"

  ; Install Python packages. Log output.
  nsExec::ExecToLog '"$PROGRAMFILES64\Python38\python.exe" -m pip install --no-index --find-links "$INSTDIR" -r "$INSTDIR\requirements.txt"'
  IfErrors handleErrorBeforeInstallPackages
  Pop $0
  DetailPrint "Install Python packages returned code:$0"
  StrCmp $0 "0" 0 handleErrorInstallPackages
  Return

  handleErrorBeforeInstallPackages:
    Call WriteLogToFile
    MessageBox MB_OK "Nepavyko paleisti Python paketų instaliavimo."
    Quit

  handleErrorInstallPackages:
    Call WriteLogToFile
    MessageBox MB_OK "Nepavyko instaliuoti Python paketų."
    Quit
SectionEnd

Section "Visual Studio Code" SEC02
  EnVar::SetHKLM
  EnVar::AddValue "VSCODE_EXTENSIONS" "$PROGRAMFILES64\VS Code Extensions"

  File "VSCodeSetup.exe"
  ExecWait '"$INSTDIR\VSCodeSetup.exe" /LOG="$instdir\l1.txt" /ALLUSERS /SILENT /MERGETASKS=!runcode,desktopicon,addcontextmenufiles,addcontextmenufolders,associatewithfiles'

  IfErrors handleError

  File "vscode_extensions\ms-python.python.vsix"
  File "vscode_extensions\hediet.debug-visualizer.vsix"
  File "vscode_extensions\vakaras.vscode-language-pack-lt.vsix"
  File "vscode_extensions\formulahendry.code-runner.vsix"

  IfErrors handleError

  FileOpen $0 "$instdir\install-extensions.bat" w
  FileWrite $0 'set VSCODE_EXTENSIONS=$PROGRAMFILES64\VS Code Extensions$\r$\n'
  FileWrite $0 'call "$PROGRAMFILES64\Smauglys\bin\smauglys.cmd" --install-extension ms-python.python.vsix$\r$\n'
  FileWrite $0 'call "$PROGRAMFILES64\Smauglys\bin\smauglys.cmd" --install-extension hediet.debug-visualizer.vsix$\r$\n'
  FileWrite $0 'call "$PROGRAMFILES64\Smauglys\bin\smauglys.cmd" --install-extension formulahendry.code-runner.vsix$\r$\n'
  FileWrite $0 'call "$PROGRAMFILES64\Smauglys\bin\smauglys.cmd" --install-extension vakaras.vscode-language-pack-lt.vsix$\r$\n'
  FileClose $0

  IfErrors handleError

  nsExec::ExecToLog "'$instdir\install-extensions.bat'"

  IfErrors handleError

  File "vscode_monkey.py"

  nsExec::ExecToLog '"$PROGRAMFILES64\Python38\python.exe" vscode_monkey.py "$PROGRAMFILES64\VS Code Extensions" "$PROGRAMFILES64\VS Code Extensions\monkey.log"'
  IfErrors handleError

  Return

  handleError:
    MessageBox MB_OK "Nepavyko instaliuoti kodo redaktoriaus. Bandykite dar kartą."
    Quit

SectionEnd

Section "Python 3.8 documentation"
  File "python3810.chm"

  CopyFiles "$INSTDIR\python3810.chm" "$PROGRAMFILES64\Smauglys\python3810.chm"
  SetShellVarContext all
  CreateShortCut "$DESKTOP\Python Documentation.lnk" "$PROGRAMFILES64\Smauglys\python3810.chm"
SectionEnd


Function WriteLogToFile
  DetailPrint "Log written to: $exedir\install.log"
  StrCpy $0 "$exedir\install.log"
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

Section "Remove temp files" SEC03
  SetOutPath $TEMP
  RMDir /r /REBOOTOK $TEMP\Smauglys

  Call WriteLogToFile
SectionEnd

section "uninstall"
    delete "$PROGRAMFILES64\Smauglys\python3810.chm"
    delete "$DESKTOP\Python Documentation.lnk"
sectionEnd
