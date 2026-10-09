# pdf-r2l-script-win

PDFを右綴じ(R2L)かつ見開き(右始まり)にする、インストーラー付き Windows 用 GUI アプリです。
Makes PDFs right-to-left (R2L) with a two-column-right layout — a Windows GUI app with an installer.

[INORIA/pdf-r2l-script](https://github.com/INORIA/pdf-r2l-script)(Python, MIT。元は hanepjiv/pdfr2l)を Rust で作り直したものです。
元スクリプトと同じく Direction=R2L・PageLayout=TwoColumnRight を設定します
(既存の ViewerPreferences の他項目は置き換えず保持)。「元ファイルを上書き」は `-i bak` 相当です。

## 使い方 / Usage
`pdf-r2l-script-win-setup.exe` でインストール後、PDFを選択またはドラッグ&ドロップして「変換して保存…」。
ライセンスはインストーラーの途中と、アプリの「ライセンス / License」ボタンで英語・日本語併記で表示されます。

## 開発 / Build
```
cargo test
cargo build --release
"C:\Program Files (x86)\Inno Setup 6\ISCC.exe" installer\pdf-r2l-script-win.iss
```
依存を変えたら `scripts/gen-third-party.ps1 > licenses/third-party.txt` の後に `sh scripts/build-licenses.sh`。

## LICENSE
MIT。詳細・第三者ライセンスは [LICENSES.txt](LICENSES.txt)。
