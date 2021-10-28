import sys
import traceback
import glob
import os
import json

LOG_FILE=None

def log(template, *args, **kwargs):
    LOG_FILE.write(template.format(*args, **kwargs) + '\n')

def set_value(obj, path, value):
    for part in path[:-1]:
        obj = obj[part]
    obj[path[-1]] = value

def find_package_json(extensions_dir, extension_prefix):
    log(
        "[enter] find_package_json(extensions_dir={}, extension_prefix={})",
        extensions_dir,
        extension_prefix
    )
    paths = []
    for extension_path in glob.glob(os.path.join(extensions_dir, "*")):
        extension = os.path.basename(extension_path)
        if extension.startswith(extension_prefix):
            log("selected for patching: {}", extension)
            package_json_path = os.path.join(extension_path, 'package.json')
            paths.append(package_json_path)
        else:
            log("not selected for patching: {}", extension)
    log("paths = {}", paths)
    assert len(paths) == 1
    log("[exit] find_package_json")
    return paths[0]

def configure_python(extensions_dir):
    log("[enter] configure_python(extensions_dir={})", extensions_dir)
    package_json_path = find_package_json(extensions_dir, 'ms-python.python-')
    with open(package_json_path, 'r') as fp:
        package_info = json.load(fp)
    set_value(
        package_info,
        (
            "contributes", "configuration", "properties",
            "python.linting.mypyEnabled", "default"
        ),
        True
    )
    set_value(
        package_info,
        (
            "contributes", "configuration", "properties",
            "python.defaultInterpreterPath", "default"
        ),
        sys.executable
    )
    with open(package_json_path, 'w') as fp:
        json.dump(package_info, fp, indent='\t')
    log("[exit] configure_python")

def configure_code_runner(extensions_dir):
    log("[enter] configure_code_runner(extensions_dir={})", extensions_dir)
    package_json_path = find_package_json(extensions_dir, 'formulahendry.code-runner-')
    with open(package_json_path, 'r') as fp:
        package_info = json.load(fp)
    set_value(
        package_info,
        (
            "contributes", "configuration", "properties",
            "code-runner.runInTerminal", "default"
        ),
        True
    )
    set_value(
        package_info,
        (
            "contributes", "configuration", "properties",
            "code-runner.showExecutionMessage", "default"
        ),
        False
    )
    set_value(
        package_info,
        (
            "contributes", "configuration", "properties",
            "code-runner.saveAllFilesBeforeRun", "default"
        ),
        True
    )
    set_value(
        package_info,
        (
            "contributes", "configuration", "properties",
            "code-runner.saveFileBeforeRun", "default"
        ),
        True
    )
    with open(package_json_path, 'w') as fp:
        json.dump(package_info, fp, indent='\t')
    log("[exit] configure_code_runner")

def main(extensions_dir, log_path):
    with open(log_path, 'w') as log_file:
        global LOG_FILE
        LOG_FILE = log_file
        try:
            configure_python(extensions_dir)
            configure_code_runner(extensions_dir)
        except Exception as e:
            log_file.write("An exception occured:\n")
            log_file.write(traceback.format_exc())
            raise

if __name__ == "__main__":
    main(*sys.argv[1:])
