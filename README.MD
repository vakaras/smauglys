# Prerequisites
**NSIS: Nullsoft Scriptable Install System** - allows to build installers using NSI scripting language.  
**EnVar** - plugin for NSIS for dealing with environment variables.

**NSIS**  
https://nsis.sourceforge.io/Download

**EnVar plugin**  
https://nsis.sourceforge.io/EnVar_plug-in

# Building the installer
1. Clone the source code / fetch latest changes.
2. (First time or when dependencies change) Run the "download_deps.sh" to download the required dependencies.
3. Open NSIS -> Compile NSI scripts -> Load script and locate "install_script" (from the root of source code).
4. If all went well, you should get "smauglys_installer.exe" installer.
