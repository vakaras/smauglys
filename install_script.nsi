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
InstallDir "$PROGRAMFILES\Smauglys"
ShowInstDetails show

Section -SETTINGS
  SetOutPath "$INSTDIR"
  SetOverwrite ifnewer
SectionEnd

Section "Python 3.8" SEC01
  File "PythonInstaller.exe"
  ExecWait "$INSTDIR\PythonInstaller.exe"
SectionEnd
 
Section "Visual Studio Code" SEC02
  File "VSCodeSetup.exe"
  ExecWait "$INSTDIR\VSCodeSetup.exe"
SectionEnd
