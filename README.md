# library

Documentation: https://naut3.github.io/library/library/index.html

## bundler を利用して単一のソースコードにする

コンテストでは、単一のソースコードとしてプログラムを提出する必要がある。  
そのため、使いたいモジュール、それが依存しているモジュールを、指定したソースコードに展開できるスクリプトを用意してある。

* 使用例

以下は、`src/bin/main.rs` に `src/dijkstra.rs` とそれが依存しているモジュールを展開する。

```bash
python3 bundle.py main dijkstra
```

`python3 bundle.py {展開先のファイル} {モジュール1} {モジュール2} ...` の形で、モジュール1, モジュール2, ... とそれらが依存しているモジュールを展開先のファイルに展開する。
