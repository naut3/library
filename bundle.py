import argparse
import os
import re
from pathlib import Path

SRC_FOLDER = Path("./src")
BIN_FOLDER = SRC_FOLDER / Path("./bin")

def main():
    parser = argparse.ArgumentParser(description="bin_path に module_names とそれが依存したファイルを展開する")

    # 引数の設定を記述する
    parser.add_argument("bin", type=str, help="bundleする対象のファイル名を書く")
    parser.add_argument("module_names", type=str, help="bundleしたいモジュール名を書く(複数でもOK)", nargs='*')
    args = parser.parse_args()

    # 受け取ってきたものの処理をする
    bin_name = args.bin

    # 拡張子はなくても適当に補完する
    if len(bin_name) < 3 or bin_name[-3:] != '.rs':
        bin_name = bin_name + '.rs'

    module_names = args.module_names

    module_names_new = []

    for name in module_names:
        if len(name) < 3 or name[-3:] != '.rs':
            name = name + '.rs'

        path = SRC_FOLDER / Path(name)
        module_names_new.append(path)

    module_names = module_names_new

    # ファイルの存在確認をする
    bin_path = BIN_FOLDER / Path(bin_name)

    if not os.path.isfile(bin_path):
        print(f"存在しないファイルが指定されています: {bin_path}")
        return

    for path in module_names:
        if not os.path.isfile(path):
            print(f"存在しないファイルが指定されています: {path}")
            return

    expand_modules(bin_path, module_names)
    return

def expand_modules(bin_path: Path, module_paths):
    # 1. bin_path で指定したファイルにすでにある module がなにかを解析する
    existed_module_paths = set()
    
    with open(bin_path) as bin_file:
        for line in bin_file:
            match = re.match(r"pub mod .+ {", line)
            
            if match:
                module_name = line[match.start() + 8:match.end() - 2]
                module_path = SRC_FOLDER / Path(module_name + ".rs")
                existed_module_paths.add(module_path)

    # 2. modules が依存している module を特定する
    seen_modules = set()

    for path in module_paths:
        seen_modules.add(path)
    
    q = module_paths[:]

    while q:
        path = q.pop()

        # ファイルの中身を見て、依存しているファイルを検索する
        with open(path) as module_file:
            for line in module_file:
                match = re.match(r"use crate::(\w*)::", line)

                if match:
                    module_name = line[match.start() + 11:match.end() - 2]
                    module_path = SRC_FOLDER / Path(module_name + ".rs")

                    if module_path not in seen_modules:
                        seen_modules.add(module_path)
                        q.append(module_path)

    # 3. 展開する
    with open(bin_path, mode='a') as bin_file:
        bin_file.write('\n')

        for path in seen_modules:
            if path in existed_module_paths:
                continue

            name = path.name[:-3]
            
            bin_file.write(f'#[rustfmt::skip]\npub mod {name} {{')
            
            with open(path) as module_file:
                for line in module_file:
                    if "//" in line:
                        continue

                    bin_file.write(re.sub('\s+', ' ', line.rstrip('\n')))
            
            bin_file.write('}\n')

    return

if __name__ == '__main__':
    main()
