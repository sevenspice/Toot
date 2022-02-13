# 概要

マストドンへトゥートするスクリプト。

# スタートガイド
| アプリケーション | バージョン               |
| :--------------- | :----------------------- |
| rustup           | `1.24.3`                 |
| rustc            | `1.57.0`                 |
| cargo            | `1.57.0`                 |

```
git clone https://github.com/sevenspice/Toot.git
cd Toot
cargo install --path .
```

# 使い方
設定ファイルを用意する。
```
cp mastodon.toml.origin mastodon.toml
vim mastodon.toml
```
* 対象のマストドンホスト名とアプリのトークンを記載して保存。
* 設定ファイルは適当なパスに配置する。

トゥート例
```
toot -s /path/to/mastodon.toml -t "禍福は糾える縄の如し"
```
