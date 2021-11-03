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

def append_value(obj, path, value):
    for part in path[:-1]:
        obj = obj[part]
    obj[path[-1]].append(value)

def delete_value(obj, path):
    for part in path[:-1]:
        obj = obj[part]
    del obj[path[-1]]

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

def replace_in_file(path, source, target):
    with open(path) as fp:
        text = fp.read()
    result = text.replace(source, target)
    with open(path, 'w') as fp:
        fp.write(result)

def configure_python(extensions_dir):
    log("[enter] configure_python(extensions_dir={})", extensions_dir)
    package_json_path = find_package_json(extensions_dir, 'ms-python.python-')
    replace_in_file(package_json_path, '"icon": "$(play)"', '"icon": { "dark": "resources/dark/play.svg", "light": "resources/light/play.svg" }')
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
    set_value(
        package_info,
        (
            "contributes", "configuration", "properties",
            "python.pythonPath", "default"
        ),
        sys.executable
    )
    delete_value(
        package_info,
        (
            "contributes", "menus", "editor/title/run",
        ),
    )
    set_value(
        package_info,
        (
            "contributes", "menus", "editor/title",
        ),
        [
            {
                "command": "python.refreshTensorBoard",
                "group": "navigation@0",
                "when": "python.hasActiveTensorBoardSession && !virtualWorkspace && shellExecutionSupported"
            },
            {
                "command": "python.execInTerminal-icon",
                "group": "navigation@0",
                "title": "%python.command.python.execInTerminal.title%",
                "when": "resourceLangId == python && !isInDiffEditor && !virtualWorkspace && shellExecutionSupported"
            },
            {
                "command": "python.debugInTerminal",
                "group": "navigation@1",
                "title": "%python.command.python.debugInTerminal.title%",
                "when": "resourceLangId == python && !isInDiffEditor && !virtualWorkspace && shellExecutionSupported"
            }
        ]
    )
    with open(package_json_path, 'w') as fp:
        json.dump(package_info, fp, indent='\t')
    resources = os.path.join(os.path.dirname(package_json_path), 'resources')
    dark = os.path.join(resources, 'dark', 'play.svg')
    with open(dark, 'w') as fp:
        fp.write(r'''<svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
<path d="M 2.3771594,0.12798595 V 15.872014 L 13.622841,7.9999873 Z" fill="#5f9f00"/>
</svg>''')
    light = os.path.join(resources, 'light', 'play.svg')
    with open(light, 'w') as fp:
        fp.write(r'''<svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
<path d="M 2.3771594,0.12798595 V 15.872014 L 13.622841,7.9999873 Z" fill="#5f9f00"/>
</svg>''')
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
    delete_value(
        package_info,
        (
            "contributes", "menus", "editor/title/run",
        ),
    )
    set_value(
        package_info,
        (
            "contributes", "menus", "editor/title",
        ),
        [
            {
                "command": "code-runner.run",
                "group": "navigation"
            },
            {
                "when": "code-runner.codeRunning",
                "command": "code-runner.stop",
                "group": "navigation"
            }
        ]
    )
    with open(package_json_path, 'w') as fp:
        json.dump(package_info, fp, indent='\t')
    images = os.path.join(os.path.dirname(package_json_path), 'images')
    dark = os.path.join(images, 'run-dark.svg')
    replace_in_file(dark, 'fill:none;stroke:#C5C5C5', 'fill:#5f9f00;stroke:#5f9f00')
    light = os.path.join(images, 'run-light.svg')
    replace_in_file(light, 'fill:none;stroke:#474748', 'fill:#5f9f00;stroke:#5f9f00')
    log("[exit] configure_code_runner")

def main(extensions_dir, log_path):
    with open(log_path, 'w') as log_file:
        global LOG_FILE
        LOG_FILE = log_file
        try:
            configure_python(extensions_dir)
            # configure_code_runner(extensions_dir)
        except Exception as e:
            log_file.write("An exception occured:\n")
            log_file.write(traceback.format_exc())
            raise

if __name__ == "__main__":
    main(*sys.argv[1:])
