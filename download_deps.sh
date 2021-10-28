#!/bin/bash

set -e

# Download VS Codium
curl 'https://github.com/vakaras/smauglys-ide/releases/download/1.59.1/SmauglysSetup-x64-1.59.1.exe' -Lo VSCodeSetup-x64.exe
curl 'https://github.com/vakaras/smauglys-ide/releases/download/1.59.1/SmauglysSetup-ia32-1.59.1.exe' -Lo VSCodeSetup-ia32.exe

# Download VS Code extensions
mkdir -p vscode_extensions

curl 'https://github.com/vakaras/vscode-language-pack-lt/releases/download/v-2021-09-04-1742/vscode-language-pack-lt.vsix' --compressed -Lo vscode_extensions/vakaras.vscode-language-pack-lt.vsix
#curl 'https://marketplace.visualstudio.com/_apis/public/gallery/publishers/ms-toolsai/vsextensions/jupyter/2021.8.1236758218/vspackage' --compressed -Lo vscode_extensions/ms-toolsai.jupyter.vsix
# Download our translated version of the Python extension.
curl 'https://github.com/vakaras/vscode-python/releases/download/v-2021-09-04-1253/ms-python-v-2021-09-04-1253.vsix' --compressed -Lo vscode_extensions/ms-python.python.vsix
curl 'https://marketplace.visualstudio.com/_apis/public/gallery/publishers/hediet/vsextensions/debug-visualizer/2.2.4/vspackage' --compressed -Lo vscode_extensions/hediet.debug-visualizer.vsix
curl 'https://marketplace.visualstudio.com/_apis/public/gallery/publishers/formulahendry/vsextensions/code-runner/0.11.6/vspackage' --compressed -Lo vscode_extensions/formulahendry.code-runner.vsix

OLD_PWD=$(pwd)

mkdir -p "$TEMP/extension_test"
cd "$TEMP/extension_test"
unzip "$OLD_PWD/vscode_extensions/vakaras.vscode-language-pack-lt.vsix"
unzip "$OLD_PWD/vscode_extensions/ms-python.python.vsix"
unzip "$OLD_PWD/vscode_extensions/hediet.debug-visualizer.vsix"
unzip "$OLD_PWD/vscode_extensions/formulahendry.code-runner.vsix"

cd "$OLD_PWD"

# Download Python
# Note: We have to use 3.8 because it is the latest version that is
# still supported on Windows 7.
curl 'https://www.python.org/ftp/python/3.8.10/python-3.8.10-amd64.exe' -Lo PythonInstaller-x64.exe
# For downloading dependencies we have to use exactly the same version
# of Python.
PYTHON_EXE="$RUNNER_TOOL_CACHE/Python/3.8.10/x64/python.exe"

mkdir -p python_packages-x64
cd python_packages-x64
cp ../requirements.txt requirements.txt
"$PYTHON_EXE" -m pip download -r requirements.txt
cd ..

curl 'https://www.python.org/ftp/python/3.8.10/python-3.8.10.exe' -Lo PythonInstaller-ia32.exe
# For downloading dependencies we have to use exactly the same version
# of Python.
PYTHON_EXE="$RUNNER_TOOL_CACHE/Python/3.8.10/x86/python.exe"

mkdir -p python_packages-ia32
cd python_packages-ia32
cp ../requirements.txt requirements.txt
"$PYTHON_EXE" -m pip download -r requirements.txt
cd ..
