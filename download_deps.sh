#!/bin/bash

set -e

# Download VS Code
curl 'https://code.visualstudio.com/sha/download?build=stable&os=win32-x64' -Lo VSCodeSetup.exe

# Download VS Code extensions
mkdir vscode_extensions

curl 'https://marketplace.visualstudio.com/_apis/public/gallery/publishers/vakaras/vsextensions/vscode-language-pack-lt/1.59.1/vspackage' --compressed -Lo vscode_extensions/vakaras.vscode-language-pack-lt.vsix
curl 'https://marketplace.visualstudio.com/_apis/public/gallery/publishers/ms-python/vsextensions/vscode-pylance/2021.8.3/vspackage' --compressed -Lo vscode_extensions/ms-python.vscode-pylance.vsix
curl 'https://marketplace.visualstudio.com/_apis/public/gallery/publishers/ms-toolsai/vsextensions/jupyter/2021.8.1236758218/vspackage' --compressed -Lo vscode_extensions/ms-toolsai.jupyter.vsix
curl 'https://marketplace.visualstudio.com/_apis/public/gallery/publishers/ms-python/vsextensions/python/2021.8.1159798656/vspackage' --compressed -Lo vscode_extensions/ms-python.python.vsix
curl 'https://marketplace.visualstudio.com/_apis/public/gallery/publishers/hediet/vsextensions/debug-visualizer/2.2.4/vspackage' --compressed -Lo vscode_extensions/hediet.debug-visualizer.vsix

# Download Python
# Note: We have to use 3.8 because it is the latest version that is
# still supported on Windows 7.
curl 'https://www.python.org/ftp/python/3.8.10/python-3.8.10-amd64.exe' -Lo PythonInstaller.exe
# Since we need to use exactly the same Python and pip for downloading,
# we install the downloaded Python on the builder.
./PythonInstaller.exe /passive InstallAllUsers=1 PrependPath=1
ls "$PROGRAMFILES"
PYTHON_EXE="$PROGRAMFILES/Python38/python.exe"
ls "$PYTHON_EXE"

mkdir python_packages
cd python_packages
cp ../python-requirements.txt requirements.txt
"$PYTHON_EXE" -m pip download -r requirements.txt
