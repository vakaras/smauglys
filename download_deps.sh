#!/bin/bash

set -e

# Download VS Codium
curl 'https://github.com/vakaras/smauglys-ide/releases/download/1.59.1/SmauglysSetup-x64-1.59.1.exe' -Lo VSCodeSetup.exe

# Download VS Code extensions
mkdir vscode_extensions

curl 'https://github.com/vakaras/vscode-language-pack-lt/releases/download/v-2021-09-04-1742/vscode-language-pack-lt.vsix' --compressed -Lo vscode_extensions/vakaras.vscode-language-pack-lt.vsix
#curl 'https://marketplace.visualstudio.com/_apis/public/gallery/publishers/ms-toolsai/vsextensions/jupyter/2021.8.1236758218/vspackage' --compressed -Lo vscode_extensions/ms-toolsai.jupyter.vsix
# Download our translated version of the Python extension.
curl 'https://github.com/vakaras/vscode-python/releases/download/v-2021-09-04-1253/ms-python-v-2021-09-04-1253.vsix' --compressed -Lo vscode_extensions/ms-python.python.vsix
curl 'https://marketplace.visualstudio.com/_apis/public/gallery/publishers/hediet/vsextensions/debug-visualizer/2.2.4/vspackage' --compressed -Lo vscode_extensions/hediet.debug-visualizer.vsix

# Download Python
# Note: We have to use 3.8 because it is the latest version that is
# still supported on Windows 7.
curl 'https://www.python.org/ftp/python/3.8.10/python-3.8.10-amd64.exe' -Lo PythonInstaller.exe
# For downloading dependencies we have to use exactly the same version
# of Python.
PYTHON_EXE="$RUNNER_TOOL_CACHE/Python/3.8.10/x64/python.exe"

mkdir python_packages
cd python_packages
cp ../requirements.txt requirements.txt
"$PYTHON_EXE" -m pip download -r requirements.txt
cd ..
ls -d python_packages/*
