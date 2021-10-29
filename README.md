# httptest

### これはなんですか

- Rust actix web による login認証のある web application sample
- login / ユーザー登録して、体温を毎日記録しましょう
- cookieによる簡単なlogin認証があります
    - もうちょっとセキュリティに気を配ってもいいかもしれない
- oracle db との接続を行っているが ORM は使っていない

### なぜ作ったのか

- actixは最速のweb platform, SSLサポートがある
- oracleはほぼ無料でapp serverを公開しているためすぐdeployできる
  - x86 や aarch64 バイナリを dockerでビルドしてバイナリのみサーバーに転送、実行
- Rust は強固な基盤をもち、バグを作りづらく、モジュール化がgolangよりはるかに楽だから
- ORMを使うほどたくさん SQLを使っていないから

### ビルド

- `cargo build`

### 実行の前に

- `sqlplus` にて db login の後、`@httptest.sql`で sql を読ませて、 tableをあらかじめ作っておくこと

### 実行

- `./target/debug/httptest --ocistring admin/pass@//123.45.67/XEPDB1 --ssl --certkey <certkey> --domain <domain> --privkey <privkey>`
- `./target/debug/httptest --dbenv OCISTRING`

- 起動時オプション
  - `--dbenv <環境変数>` あるいは `--ocistring <接続文字列>` dbへの接続方法を指定
  - `--ssl`  SSLサポート (`--certkey, --privkey, --domain` を指定すること)
  - `--register` ユーザー登録ページを有効にする つけないと登録もできません
  - `--port` 待ち受けポート番号 ただし1024番ポート以前のポート番号は、開放時にroot権限が必要です

### 変更点

- bug fix
- README をわかりやすく
