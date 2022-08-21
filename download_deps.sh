#!/bin/bash

# Tries to download a packages, retries when file seems too small.
try_download_package () {
	for I in {1..20}
	do
		curl $1 --compressed -Lo $2
		hash=$(md5sum $2 | awk '{print $1}')
		if [ $hash != $3 ]; then
			echo $hash
			echo $3
			echo "File hash does not match, retrying in 5 seconds"
			sleep 60
		else
			break # File size seems ok, break out of retry loop.
		fi
	done
}

set -e
set -v

# Download VS Codium
curl 'https://github.com/vakaras/smauglys-ide/releases/download/1.70.2/SmauglysSetup-x64-1.70.2.exe' -Lo VSCodeSetup-x64.exe
curl 'https://github.com/vakaras/smauglys-ide/releases/download/1.70.2/SmauglysSetup-ia32-1.70.2.exe' -Lo VSCodeSetup-ia32.exe

# Download VS Code extensions
mkdir -p vscode_extensions

try_download_package 'https://github.com/vakaras/vscode-language-pack-lt/releases/download/v-2021-09-04-1742/vscode-language-pack-lt.vsix' 'vscode_extensions/vakaras.vscode-language-pack-lt.vsix' '58a9ee12c5336658a38ccb40f1ec3e9a'
#curl 'https://marketplace.visualstudio.com/_apis/public/gallery/publishers/ms-toolsai/vsextensions/jupyter/2021.8.1236758218/vspackage' --compressed -Lo vscode_extensions/ms-toolsai.jupyter.vsix
# Download our translated version of the Python extension.
try_download_package 'https://github.com/vakaras/vscode-python/releases/download/v-2022-08-21-0945/ms-python-v-2022-08-21-0945.vsix' 'vscode_extensions/ms-python.python.vsix' 'bcc9d2629ea124b2128b7e558caa9b49'
#try_download_package 'https://marketplace.visualstudio.com/_apis/public/gallery/publishers/hediet/vsextensions/debug-visualizer/2.3.1/vspackage' 'vscode_extensions/hediet.debug-visualizer.vsix' 'd08cdc2b35326f04337ffe64a59a4ce4'
try_download_package 'https://github.com/gedas-luksas/quick-run-code/releases/download/0.0.3/quick-run-code-0.0.3.vsix' 'vscode_extensions/quick-run-code.vsix' '85ed1708da7913022756899e04bab986'
#try_download_package 'https://github.com/gedas-luksas/vscode-code-runner/releases/download/0.11.6/code-runner-0.11.6.vsix' 'vscode_extensions/formulahendry.code-runner.vsix' 'f070e588b5fafaaffe0c088161c19b7b'

OLD_PWD=$(pwd)

mkdir -p "$TEMP/extension_test"
cd "$TEMP/extension_test"
mkdir -p vscode-language-pack-lt
cd vscode-language-pack-lt
unzip -o "$OLD_PWD/vscode_extensions/vakaras.vscode-language-pack-lt.vsix"
cd ..
mkdir -p ms-python.python
cd ms-python.python
unzip -o "$OLD_PWD/vscode_extensions/ms-python.python.vsix"
cd ..
#mkdir -p hediet.debug-visualizer
#cd hediet.debug-visualizer
#unzip -o "$OLD_PWD/vscode_extensions/hediet.debug-visualizer.vsix"
#cd ..
#mkdir -p code-runner
#cd code-runner
#unzip -o "$OLD_PWD/vscode_extensions/formulahendry.code-runner.vsix"
#cd ..
mkdir -p quick-run-code
cd quick-run-code
unzip -o "$OLD_PWD/vscode_extensions/quick-run-code.vsix"
cd ..

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
