# httptest

### これはなんですか

- Rust actix web による login認証のある web application sample
- login / ユーザー登録して、体温を毎日記録しましょう
- cookieによる簡単なlogin認証があります
    - もうちょっとセキュリティに気を配ってもいいかもしれない
- oracle db との接続を行っているが ORM は使っていない

### なぜ作ったのか

- actixは最速のweb platform
- oracleはほぼ無料でapp serverを公開しているためすぐdeployするため
- Rust は強固な基盤をもち、モジュール化がgolangよりはるかに楽だから
- ORMを使うほどたくさん SQLを使っていないから

### ビルド

- `cargo build`

### 実行の前に

- `sqlplus` にて db login の後、`@httptest.sql`で sql を読ませて、 tableをあらかじめ作っておくこと

### 実行

- `./target/debug/httptest --ocistring admin/pass@//123.45.67/XEPDB1 --ssl --certkey <certkey> --domain <domain> --privkey <privkey>`
- `./target/debug/httptest --dbenv OCISTRING`

    - (環境変数`OCISTRING`に接続文字列 たとえば `admin/pass@//123.45.67/XEPDB1`が入っている場合)

### 変更点

- StructOpt 使用
- SSL化 (--ssl フラグの使用時のみ)
- `--register`フラグをつけると、webからuser追加が可能 (いたずら防止)
