# LR NŠA Python – a Python IDE for Lithuanian Schools

[Download page](https://github.com/vakaras/smauglys/releases/)

## Building Locally

### Prerequisites

* [**NSIS: Nullsoft Scriptable Install System**](https://nsis.sourceforge.io/Download) – allows to build installers using NSI scripting language.
* [**EnVar**](https://nsis.sourceforge.io/EnVar_plug-in) – a plugin for NSIS for dealing with environment variables.

### Steps

1. Clone the source code / fetch latest changes.
2. (First time or when dependencies change) Run the `download_deps.sh` to
   download the required dependencies.
3. Apply the changes from the [CI file](.github/workflows/windows.yml) to
   `install_script.nsi` to create either x64 or ia32 installer script.
4. Open NSIS → Compile NSI scripts → Load script and locate either
   `install_script_x64.nsi` or `install_script_ia32.nsi`.
5. If all went well, you should get `smauglys_installer-*.exe` installer.
