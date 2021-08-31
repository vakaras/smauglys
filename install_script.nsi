!define PRODUCT_NAME "Smauglys"
!define PRODUCT_VERSION "1.0"
!define PRODUCT_PUBLISHER "Vytautas Astrauskas"

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
InstallDir "$PROGRAMFILES64\Smauglys"
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

Section -SETTINGS
  SetOutPath "$INSTDIR"
  SetOverwrite ifnewer
SectionEnd

Section "Python 3.8" SEC01
  File "PythonInstaller.exe"
  File "requirements.txt"
  ExecWait '"$INSTDIR\PythonInstaller.exe" /passive InstallAllUsers=1 PrependPath=1'
  ExecWait 'python.exe -m pip install  --no-index  -r "$INSTDIR\requirements.txt"'
SectionEnd
 
Section "Visual Studio Code" SEC02
  EnVar::SetHKLM
  EnVar::AddValue "VSCODE_EXTENSIONS" "$PROGRAMFILES64\VS Code Extensions"
  
  File "VSCodeSetup.exe"
  ExecWait '"$INSTDIR\VSCodeSetup.exe" /LOG="$instdir\l1.txt" /ALLUSERS /SILENT /MERGETASKS=!runcode'

  File "vscode_extensions\ms-python.python.vsix"
  File "vscode_extensions\hediet.debug-visualizer.vsix"
  File "vscode_extensions\vakaras.vscode-language-pack-lt.vsix"

  FileOpen $0 "$instdir\install-extensions.bat" w
  FileWrite $0 '@echo off$\r$\n'
  FileWrite $0 'set VSCODE_EXTENSIONS=$programfiles64\VS Code Extensions$\r$\n'
  FileWrite $0 'call "$programfiles64\vscodium\bin\codium.cmd" --install-extension ms-python.python.vsix > e1.log$\r$\n'
  FileWrite $0 'call "$programfiles64\vscodium\bin\codium.cmd" --install-extension hediet.debug-visualizer.vsix > e2.log$\r$\n'
  FileWrite $0 'call "$programfiles64\vscodium\bin\codium.cmd" --install-extension vakaras.vscode-language-pack-lt.vsix > e3.log$\r$\n'
  FileClose $0

  ExecWait "$instdir\install-extensions.bat"
SectionEnd

Section "Log" SEC03                  
  StrCpy $0 "$instdir\install.log"
  Push $0
  Call DumpLog
SectionEnd
