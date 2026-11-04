# Integrated into pdf-r2l-rs / pdf-r2l-rs へ統合しました

**English** — This repository's final, practical version is now identical to the tested release of
[aon-co-jp/pdf-r2l-rs](https://github.com/aon-co-jp/pdf-r2l-rs) (v1.1.0 line), which unifies
both `pdf-r2l-win` (re-implementation of INORIA/pdf-r2l-script idea) and `pdf-r2l-script-win` into one original,
spec-based app (MIT OR Apache-2.0) for Windows / macOS / Linux / Android, with right/left/keep binding,
OCR re-typesetting (online), searchable text, and image up-conversion. Development continues in pdf-r2l-rs;
this repository is kept as a snapshot. The upstream license is kept as `LICENSE-upstream`.

**日本語** — 本リポジトリの最終的な実用版は、試験・バグ修正・改良を重ねた
[aon-co-jp/pdf-r2l-rs](https://github.com/aon-co-jp/pdf-r2l-rs) と同一内容です。
`pdf-r2l-win` と `pdf-r2l-script-win` の両方を、PDF仕様書に基づく独自実装の1つのアプリ
(MIT OR Apache-2.0、Windows/macOS/Linux/Android、右綴じ/左綴じ/元と同じ、オンラインOCRの書体文字化、
検索可能テキスト、画像の高解像度化)に**統合**しました。今後の開発は pdf-r2l-rs で行い、
本リポジトリはスナップショットとして残します。元リポジトリのライセンスは `LICENSE-upstream` に保存しています。

---

# pdf-r2l-rs

PDFの**綴じ方向(右綴じ R2L / 左綴じ L2R)**を変更し、**見開き表示**にし、スキャンしたページを **OCR して元の書体に合わせた検索できる文字に置き換える**、
Windows / macOS / Linux(・Android)用の GUI アプリです(インストーラー付き)。Rust 製。

A GUI app for Windows / macOS / Linux (and Android) that changes the **binding direction (R2L / L2R)** of PDFs, shows them as a **two-page spread**,
and can **OCR scanned pages and replace the text with searchable text in a matching typeface**. Written in Rust, with installers.

## 機能 / Features

- **綴じ方向**: 右綴じ(R2L)/左綴じ(L2R)/**元と同じ**(変更しない。OCR だけ使いたいとき)。PDF の `/ViewerPreferences /Direction` を設定します。他の表示設定は保持します。
- **見開き表示**: `/PageLayout` を `TwoColumnRight`(右綴じ)/`TwoColumnLeft`(左綴じ)にします(オフにもできます)。
- **保存**: 別のファイルに保存(既定)/元のファイルを上書き(元は `.bak` に残します)。一時ファイル経由で保存するので、途中で失敗しても元のファイルを壊しません。
- **OCR・テキストフォント文字化**(既定でオン、チェックを外すと元のデータのまま):
  - ページを画像にして `aruaru-llm`(`POST /v1/ocr`、Tesseract)で OCR(複数ページをまとめて処理)。
  - 行ごとに書体の系統(ゴシック体/明朝体)・太字・文字色・背景色を推定し、Noto フォント(Sans / Serif、日・簡・繁・韓)のサブセットを PDF に埋め込んで置き換えます。**検索・コピーができる文字**になります。
  - 言語は **1〜12個**まで選べ(日本語・英語・中国語・韓国語・EU 言語・ロシア語・ウクライナ語・ペルシャ語・アラビア語ほか 49 言語)、**選んだ言語のデータだけ**を取得してキャッシュします。
  - 「**元の画像はそのまま、検索用の見えない文字だけ追加**」も選べます(誤認識が心配なとき)。
  - **オンライン環境が必要**です(OCR サーバーへ接続)。ページ画像は処理後すぐ破棄されます。OCR・書体の判定には誤りがあり得ます。

## インストール / Install

Releases から、お使いの OS 用のものを入手します。

| OS | 配布物 |
|---|---|
| Windows | `pdf-r2l-rs-setup.exe`(インストーラー) |
| macOS | `pdf-r2l-rs-*.dmg`(未署名・未公証。初回は右クリック→「開く」) |
| Linux | `pdf-r2l-rs_*.deb`(Debian/Ubuntu 系)または `pdf-r2l-rs-x86_64.tar.gz` |
| Android | `pdf-r2l-rs.apk`(署名済み。Android 8.0 以上。「提供元不明のアプリ」の許可が必要) |

## 画像の高解像度化 / Upscale images (v1.1.0)

**日本語**: 写真・画像(PNG/JPEG)を、**フルHD / QHD(FHDと4Kの間) / 4K / 5K(4Kと8Kの間) / 8K / 16K** の枠(縦横比を保って収める)へ拡大します。
AI超解像(Real-ESRGAN realesr-general-x4v3、CPU)+高品質な補間。AIを使わず補間のみも選べます。使うかどうかは利用者の自由で、元の画像は変更せず別名で保存します。
AI超解像は失われた細部を**推測して補う**もので、実際の細部の復元ではありません。CPU処理のため時間がかかり(Arm のスマホは特に遅い)、8K・16K は大量のメモリとファイルサイズが必要です。
デスクトップ版は複数枚をまとめて処理、Android 版は1枚ずつです。Android 版は実機(moto g53y 5G、縦・横)で画面とAI超解像の動作を確認済み。Arm向けNEONカーネルにより、480×320の画像をフルHDへ拡大するAI処理は約51秒(スカラー実装の337秒から高速化)。大きな写真は数分以上かかります。

**English**: Enlarge photos/images (PNG/JPEG) to Full HD / QHD (between FHD and 4K) / 4K / 5K (between 4K and 8K) / 8K / 16K frames, keeping the aspect ratio, using
CPU AI super-resolution (Real-ESRGAN realesr-general-x4v3) plus high-quality resampling (interpolation-only is selectable). Entirely optional; the original is never modified.
AI super-resolution guesses missing detail rather than restoring it; it is slow on the CPU (especially on Arm phones), and 8K/16K need lots of memory and disk.
The desktop app batches several images; Android handles one at a time. The Android screen and AI path were verified on a real phone (moto g53y 5G, portrait and landscape); with the Arm NEON kernel a 480x320 image to Full HD takes about 51 s (337 s with the scalar code). Large photos take minutes or more.

## AI文脈補完 / AI context completion (v1.2.0, 実験的 / experimental)

**日本語**: OCRで**かすれて読めない・自信が低い1〜2文字**を、**前後の長い文脈から推測して穴埋め**します(長文の書き換えではありません)。既定はオフ(設定でオン)。
- 対象: 信頼度の低い短い単語、文字がまるごと欠けた疑いのある隙間(CJK)、余計に挿入された1文字。
- 方式: 形の似た字の表(日↔曰、0↔O、`.`↔`,`、キリル文字の混入など)と言語モデル自身の予測から候補を作り、**「候補+後ろの文脈」が前の文脈に続く確率(対数尤度)**を言語モデル(Qwen2.5)で採点。元の読みより十分に尤もらしいときだけ、1か所・最大2文字を直します。
- サーバー: `aruaru-llm` の `POST /v1/ocr/fill`(`open-cuda` の Qwen にバッチ処理と候補採点API `score_continuations` / `top_next_tokens` を追加)。言語モデル(`/v1/qwen/install` → `/v1/qwen/select`)が必要で、無いときは何も変えずに進みます。OCRとは別のサーバー(手元のPCなど)も指定できます。
- 実測(Qwen2.5-0.5B、CPU): 「私たちは**人**毎朝」→削除、「10.000円」→「10,000円」、「署名と押印」「今日」の取り違え防止などを確認。**まるごと消えた文字は文脈からの推測にとどまり**(例: 「今□は」を「今朝は」と補った。正解は「今日」)、誤ることがあります。1か所あたり数秒〜(CPU)。結果の文章は必ず確認してください。

**English**: Fills in **one or two faded / unreadable characters** that OCR missed or read with low confidence, **guessing from the long surrounding context** (it never rewrites long text). Off by default.
- Targets: short low-confidence words, suspicious gaps where a CJK character is missing, and stray single characters.
- Method: candidates come from a table of look-alike characters (日↔曰, 0↔O, `.`↔`,`, stray Cyrillic letters, ...) and from the language model's own predictions; a language model (Qwen2.5) scores **the log-likelihood of "candidate + following text" given the preceding text**, and a change (one spot, at most two characters) is made only when clearly more likely than the original reading.
- Server: `POST /v1/ocr/fill` in `aruaru-llm` (with batched prefill and candidate scoring APIs `score_continuations` / `top_next_tokens` added to the Qwen engine of `open-cuda`). It needs a language model installed (`/v1/qwen/install` then `/v1/qwen/select`); without one nothing is changed. A different server (e.g. your own PC) can be used for this step.
- Measured (Qwen2.5-0.5B on CPU): removed a stray "人", fixed "10.000" to "10,000", etc. **A fully erased character is only a guess from context** (e.g. "今□は" was filled as "今朝は"; the truth was "今日") and can be wrong. Several seconds per spot on a CPU. Always check the result.

## OCR を使うには / Using OCR

既定の接続先は、同じ PC で動いている `aruaru-llm`(`http://127.0.0.1:4600`)です。アプリの「OCR設定」で別の URL(LAN 内の別の端末など)に変更し、
「接続テスト」で確認できます。サーバーには Tesseract が必要です(言語データは `aruaru-llm` が必要な言語だけ自動で取得します)。
OCR に接続できないときは、「OCRを使わずに、綴じ方向だけ変換」で続けられます。

## 開発 / Build

```
cargo test -p pdf-r2l-core
cargo build --release            # target/release/pdf-r2l-rs (GUI), pdf-r2l-cli (command line)
"C:\Program Files (x86)\Inno Setup 6\ISCC.exe" installer\pdf-r2l-rs.iss   # Windows installer
```

コマンドライン版: `pdf-r2l-cli 入力.pdf 出力.pdf [--ltr|--keep] [--no-spread] [--ocr URL] [--lang jpn+eng] [--fast] [--search-only]`

依存を変えたら `scripts/gen-third-party.ps1 > licenses/third-party.txt` の後に `sh scripts/build-licenses.sh` で `LICENSES.txt` を再生成してください。

## LICENSE

独自実装です。綴じ方向の書き換えは PDF の仕様書(ISO 32000-1)だけを根拠に書いています。
**MIT または Apache-2.0**(どちらかを選べます)。第三者ライセンスは [LICENSES.txt](LICENSES.txt)(英日併記)。

---

作者 / Author: 石 塚 正 浩（MASAHIRO ISHIZUKA） (https://github.com/aon-co-jp)

## API 方針: REST 全廃・gRPC / Connect へ (2026-11-04)

### 日本語
- **REST は全廃する。新しい API は REST で作らない。** ブラウザや `curl` から呼ぶ API は **Connect の単純呼び出し**(`POST /<パッケージ>.<Service>/<Method>`、`content-type: application/json`、本文は JSON)、サービス間は **gRPC / gRPC-web**(RPoem の管理API `open_runo.admin.v1.Admin` と同じ流儀)。ストリーミングは、必要になってから。
- **既存の REST は、触るときに順に移す。** 移したら旧 REST の経路は消す。移行が終わるまで残るが、**新規には追加しない**。
- **JSON は RS-JSON に統一**する(読み込みは `from_str_secure` / `parse_secure` でサイズ・深さ・重複キーの上限つき、書き出しは `rust_json::fast`)。ブラウザの WASM に `prost` は入れない(サイズが増えるため)。
- **例外(自分たちの API ではなく、相手の規格が決めているもの)**: ACME、OAuth/OIDC のコールバック、第三者(GitHub など)からの Webhook、ヘルスチェック、静的ファイル配信、OTLP など。これらは規格どおりの HTTP を受ける。
- 参考実装: `RPoem`(`open_runo.admin.v1.Admin`)、`open-easy-web`(`server/src/connect.rs`、`proto/open_easy_web/admin/v1/admin.proto`)。
- ※ このリポジトリのコードが既に方針どおりかどうかは、各リポジトリの現状による。この節は方針の記録であり、コードの変更を意味しない。

### English
- **REST is being abolished. New APIs are not built as REST.** APIs called from browsers or `curl` use **Connect unary calls** (`POST /<package>.<Service>/<Method>`, `content-type: application/json`, JSON body); service-to-service calls use **gRPC / gRPC-web** (the same style as RPoem's admin API `open_runo.admin.v1.Admin`). Streaming only when it is actually needed.
- **Existing REST endpoints are migrated one by one, when they are touched.** The old REST route is removed once migrated. Until migration is finished they remain, but **no new ones are added**.
- **JSON is unified on RS-JSON**: reading with `from_str_secure` / `parse_secure` (size, depth and duplicate-key limits), writing with `rust_json::fast`. `prost` is not put into browser WASM (it would grow the binary).
- **Exceptions (shapes dictated by someone else's standard, not our own API)**: ACME, OAuth/OIDC callbacks, webhooks from third parties (e.g. GitHub), health checks, static file serving, OTLP, and so on. These accept plain HTTP as the standard requires.
- Reference implementations: `RPoem` (`open_runo.admin.v1.Admin`) and `open-easy-web` (`server/src/connect.rs`, `proto/open_easy_web/admin/v1/admin.proto`).
- Note: whether the code in a given repository already follows this policy depends on that repository. This section records the policy; it does not mean the code has been changed.

### Deutsch (KI-Übersetzung, redaktionell überarbeitet; noch nicht von Muttersprachlern geprüft)
- **REST wird vollständig abgeschafft; neue APIs entstehen nicht mehr als REST.** APIs, die aus dem Browser oder per `curl` aufgerufen werden, verwenden **unäre Connect-Aufrufe** (`POST /<Paket>.<Service>/<Methode>`, `content-type: application/json`, JSON-Body). Zwischen Diensten kommen **gRPC bzw. gRPC-web** zum Einsatz, im selben Stil wie die Admin-API von RPoem (`open_runo.admin.v1.Admin`). Streaming wird erst ergänzt, wenn es tatsächlich gebraucht wird.
- **Bestehende REST-Endpunkte werden schrittweise umgestellt, und zwar immer dann, wenn man ohnehin an ihnen arbeitet.** Sobald ein Endpunkt umgestellt ist, wird die alte REST-Route entfernt. Bis zum Abschluss der Migration bleiben die übrigen bestehen, es werden aber **keine neuen mehr hinzugefügt**.
- **JSON wird einheitlich über RS-JSON verarbeitet**: Gelesen wird mit `from_str_secure` bzw. `parse_secure` (mit Obergrenzen für Größe, Verschachtelungstiefe und doppelte Schlüssel), geschrieben mit `rust_json::fast`. `prost` wird nicht in das Browser-WASM aufgenommen, da es die Datei vergrößern würde.
- **Ausnahmen (Schnittstellen, deren Form ein fremder Standard vorgibt und die nicht zu unserer eigenen API gehören)**: ACME, OAuth/OIDC-Callbacks, Webhooks von Drittanbietern (z. B. GitHub), Health-Checks, die Auslieferung statischer Dateien, OTLP und Ähnliches. Diese nehmen, wie vom jeweiligen Standard verlangt, gewöhnliches HTTP entgegen.
- Referenzimplementierungen: `RPoem` (`open_runo.admin.v1.Admin`) und `open-easy-web` (`server/src/connect.rs`, `proto/open_easy_web/admin/v1/admin.proto`).
- Hinweis: Ob der Code eines einzelnen Repositorys diese Richtlinie bereits erfüllt, hängt vom jeweiligen Repository ab. Dieser Abschnitt dokumentiert lediglich die Richtlinie; er besagt nicht, dass der Code schon angepasst wurde.

### Русский (перевод выполнен с помощью ИИ и отредактирован; носителями языка пока не проверен)
- **От REST мы полностью отказываемся; новые API больше не создаются в стиле REST.** API, к которым обращаются из браузера или через `curl`, используют **унарные вызовы Connect** (`POST /<пакет>.<Service>/<Method>`, `content-type: application/json`, тело запроса — JSON). Для взаимодействия между сервисами применяются **gRPC и gRPC-web** — в том же стиле, что и административный API RPoem (`open_runo.admin.v1.Admin`). Потоковая передача данных добавляется лишь тогда, когда в ней действительно возникает необходимость.
- **Существующие REST-эндпоинты переводятся поэтапно — по мере того, как с ними приходится работать.** После перевода старый REST-маршрут удаляется. Пока миграция не завершена, оставшиеся эндпоинты сохраняются, но **новые не добавляются**.
- **Работа с JSON унифицирована на основе RS-JSON**: чтение — через `from_str_secure` или `parse_secure` (с ограничениями на размер, глубину вложенности и повторяющиеся ключи), запись — через `rust_json::fast`. `prost` в браузерный WASM не включается, поскольку это увеличило бы размер файла.
- **Исключения (интерфейсы, форму которых определяет чужой стандарт, а не наш собственный API)**: ACME, обратные вызовы OAuth/OIDC, вебхуки сторонних сервисов (например, GitHub), проверки работоспособности, раздача статических файлов, OTLP и тому подобное. Такие интерфейсы принимают обычный HTTP, как того требует соответствующий стандарт.
- Эталонные реализации: `RPoem` (`open_runo.admin.v1.Admin`) и `open-easy-web` (`server/src/connect.rs`, `proto/open_easy_web/admin/v1/admin.proto`).
- Примечание: соответствует ли код конкретного репозитория этой политике, зависит от самого репозитория. Данный раздел лишь фиксирует политику и не означает, что код уже изменён.

## API 方針の補足: 用語の整理と使い分け (2026-11-04)

### 日本語
**1つに統一するものではなく、別の層の話です。** 「REST の代わり」になるのは Connect・gRPC・GraphQL だけで、WASM は画面を作る技術です。

| 用語 | 何か | REST の代わりか |
|---|---|---|
| REST | URL と HTTP メソッドで操作を表す API の作り方 | 全廃する側 |
| **Connect** | `POST /サービス名/メソッド名` に JSON を送る形式。`curl` でもブラウザの `fetch` でも呼べる | はい(**採用**) |
| **gRPC** | 同じ「サービス/メソッド」の考え方を、バイナリ(protobuf)と HTTP/2 で行う形式。速く、ストリーミングも得意 | はい(**サービス同士に採用**) |
| gRPC-web | gRPC のブラウザ向けの変形版 | はい(必要なら) |
| GraphQL | 欲しい項目を問い合わせる形式 | 一部だけ(**問い合わせ(読み取り)専用**) |
| WASM | Rust をブラウザの中で動かす仕組み | いいえ(画面を作る技術) |

- **ブラウザの画面や `curl` から呼ぶ API は Connect**(POST、JSON を RS-JSON で読み書き)。ブラウザの WASM は素の gRPC を話せず、Connect なら `fetch` のままで、`prost` を足さずに済む(画面のサイズも増えない)。
- **サービス同士の通信(サーバー間、重い処理、ストリーミング)は gRPC。** 多くは Connect で足りるので、ストリーミングが必要になった所だけ gRPC に上げる。
- **GraphQL は、まとめて問い合わせたいときの読み取り専用。** 書き込み(操作)には使わない。
- **WASM は画面を作る技術として、そのまま使う。** 画面から呼ぶ API が Connect になっただけ。
- 外部の規格が形を決めるもの(ACME、OAuth のコールバック、第三者の Webhook、ヘルスチェック、静的ファイル、OTLP)は、規格どおりの HTTP のまま。
- **迷ったら Connect。**

### English
**These are not alternatives to unify; they sit at different layers.** Only Connect, gRPC and GraphQL can replace REST; WASM is a way to build the UI.

| Term | What it is | Replaces REST? |
|---|---|---|
| REST | API style that expresses operations through URLs and HTTP methods | The thing being abolished |
| **Connect** | `POST /Service/Method` with a JSON body; callable from `curl` and from a browser `fetch` | Yes (**adopted**) |
| **gRPC** | The same service/method idea over binary (protobuf) and HTTP/2; fast, good at streaming | Yes (**for service-to-service**) |
| gRPC-web | A browser-friendly variant of gRPC | Yes (if needed) |
| GraphQL | A query language: ask for exactly the fields you want | Partly (**read-only queries**) |
| WASM | Runs Rust inside the browser | No (a UI technology) |

- **APIs called from browser screens or `curl` use Connect** (POST, JSON read and written with RS-JSON). Browser WASM cannot speak plain gRPC; with Connect it stays a `fetch`, and `prost` is not needed (the binary does not grow).
- **Service-to-service calls (server to server, heavy work, streaming) use gRPC.** Connect is enough in most places; move a call to gRPC only where streaming is actually needed.
- **GraphQL is read-only**, for queries that gather several pieces of data. It is not used for writes (operations).
- **WASM stays as the way to build the UI.** The API it calls simply became Connect.
- Interfaces whose shape an outside standard dictates (ACME, OAuth callbacks, third-party webhooks, health checks, static files, OTLP) stay plain HTTP.
- **When in doubt, use Connect.**

### Deutsch (KI-Übersetzung, sprachlich überarbeitet; noch nicht von Muttersprachlern geprüft)
**Es geht nicht darum, eine einzige Technik auszuwählen – die Begriffe liegen auf unterschiedlichen Ebenen.** Als Ersatz für REST kommen nur Connect, gRPC und GraphQL in Frage; WASM dagegen ist eine Technik, mit der die Benutzeroberfläche gebaut wird.

| Begriff | Was ist das? | Ersatz für REST? |
|---|---|---|
| REST | Ein API-Stil, bei dem Operationen über URLs und HTTP-Methoden ausgedrückt werden | wird abgeschafft |
| **Connect** | `POST /Dienst/Methode` mit JSON im Body; aufrufbar mit `curl` ebenso wie mit `fetch` im Browser | ja (**gewählt**) |
| **gRPC** | Dasselbe Prinzip aus Dienst und Methode, aber über binäres protobuf und HTTP/2; schnell und gut für Streaming geeignet | ja (**zwischen Diensten**) |
| gRPC-web | Eine für den Browser angepasste Variante von gRPC | ja (bei Bedarf) |
| GraphQL | Abfragesprache, mit der man genau die benötigten Felder anfordert | teilweise (**nur für lesende Abfragen**) |
| WASM | Führt Rust-Code direkt im Browser aus | nein (Technik für die Oberfläche) |

- **APIs, die von Browseroberflächen oder per `curl` aufgerufen werden, verwenden Connect** (POST; JSON wird mit RS-JSON gelesen und geschrieben). WASM im Browser kann kein reines gRPC sprechen. Mit Connect bleibt es bei einem gewöhnlichen `fetch`, und `prost` ist nicht nötig – die WASM-Datei wird dadurch nicht größer.
- **Für die Kommunikation zwischen Diensten (Server zu Server, rechenintensive Aufgaben, Streaming) wird gRPC eingesetzt.** In den meisten Fällen reicht Connect aus; auf gRPC wird nur dort umgestellt, wo Streaming tatsächlich gebraucht wird.
- **GraphQL dient ausschließlich lesenden Abfragen**, wenn mehrere Informationen gebündelt abgefragt werden sollen. Für Schreibzugriffe (Operationen) wird es nicht verwendet.
- **WASM bleibt die Technik für die Oberfläche.** Geändert hat sich lediglich, dass die aufgerufene API nun auf Connect beruht.
- Schnittstellen, deren Form durch einen externen Standard festgelegt ist (ACME, OAuth-Callbacks, Webhooks von Drittanbietern, Health-Checks, statische Dateien, OTLP), bleiben gewöhnliches HTTP.
- **Im Zweifel: Connect.**

### Русский (перевод выполнен с помощью ИИ и литературно отредактирован; носителями языка пока не проверен)
**Это не варианты, из которых нужно выбрать один, а технологии разных уровней.** Заменить REST могут только Connect, gRPC и GraphQL; WASM же — это способ создания пользовательского интерфейса.

| Термин | Что это | Заменяет REST? |
|---|---|---|
| REST | Стиль построения API, при котором операции выражаются через URL и HTTP-методы | отменяется |
| **Connect** | `POST /Сервис/Метод` с телом в формате JSON; вызывается как из `curl`, так и из браузерного `fetch` | да (**выбран**) |
| **gRPC** | Тот же принцип «сервис — метод», но поверх двоичного формата protobuf и HTTP/2; работает быстро и хорошо подходит для потоковой передачи | да (**для связи между сервисами**) |
| gRPC-web | Адаптированный для браузера вариант gRPC | да (при необходимости) |
| GraphQL | Язык запросов: клиент запрашивает ровно те поля, которые ему нужны | частично (**только запросы на чтение**) |
| WASM | Позволяет выполнять код на Rust прямо в браузере | нет (технология для интерфейса) |

- **API, которые вызываются из браузерных интерфейсов или через `curl`, строятся на Connect** (POST; JSON читается и записывается через RS-JSON). Браузерный WASM не умеет работать с «чистым» gRPC. С Connect достаточно обычного `fetch`, а `prost` не требуется — размер WASM-файла не увеличивается.
- **Для обмена между сервисами (сервер — сервер, ресурсоёмкие задачи, потоковая передача) применяется gRPC.** В большинстве случаев достаточно Connect; на gRPC переходят только там, где потоковая передача действительно необходима.
- **GraphQL используется только для чтения**, когда нужно получить сразу несколько связанных данных. Для записи (операций) он не применяется.
- **WASM по-прежнему остаётся технологией для интерфейса.** Изменилось лишь то, что вызываемый им API теперь построен на Connect.
- Интерфейсы, форма которых задана внешним стандартом (ACME, обратные вызовы OAuth, вебхуки сторонних сервисов, проверки работоспособности, статические файлы, OTLP), остаются обычным HTTP.
- **Если есть сомнения — выбирайте Connect.**
