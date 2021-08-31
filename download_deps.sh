#!/bin/bash

set -e

# Download VS Codium
curl 'https://github.com/VSCodium/vscodium/releases/download/1.59.1/VSCodiumSetup-x64-1.59.1.exe' -Lo VSCodeSetup.exe

# Download VS Code extensions
mkdir vscode_extensions

curl 'https://marketplace.visualstudio.com/_apis/public/gallery/publishers/vakaras/vsextensions/vscode-language-pack-lt/1.59.1/vspackage' --compressed -Lo vscode_extensions/vakaras.vscode-language-pack-lt.vsix
#curl 'https://marketplace.visualstudio.com/_apis/public/gallery/publishers/ms-toolsai/vsextensions/jupyter/2021.8.1236758218/vspackage' --compressed -Lo vscode_extensions/ms-toolsai.jupyter.vsix
curl 'https://open-vsx.org/api/ms-python/python/2020.10.332292344/file/ms-python.python-2020.10.332292344.vsix' --compressed -Lo vscode_extensions/ms-python.python.vsix
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
cp ../python-requirements.txt requirements.txt
"$PYTHON_EXE" -m pip download -r requirements.txt
cd ..
ls -d python_packages/*
