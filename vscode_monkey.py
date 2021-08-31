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

def enable_mypy(extensions_dir, log_file):
    log("[enter] enable_mypy(extensions_dir={})", extensions_dir)
    for extension_path in glob.glob(os.path.join(extensions_dir, "*")):
        extension = os.path.basename(extension_path)
        if extension.startswith('ms-python.python-'):
            log("{} is Python extension; patching", extension)
            package_json_path = os.path.join(extension_path, 'package.json')
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
            with open(package_json_path, 'w') as fp:
                json.dump(package_info, fp, indent='\t')
        else:
            log("{} is not Python extension", extension)
    log("[exit] enable_mypy")

def main(extensions_dir, log_path):
    with open(log_path, 'w') as log_file:
        global LOG_FILE
        LOG_FILE = log_file
        try:
            enable_mypy(extensions_dir, log_file)
        except Exception as e:
            log_file.write("An exception occured:\n")
            log_file.write(traceback.format_exc())

if __name__ == "__main__":
    main(*sys.argv[1:])
