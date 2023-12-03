# hoshiyomi
天体の出没時刻計算を行う。現在は、月の出没・月齢の計算のみ対応。  
実行するとgRPCサーバーが起動するので、`proto/moon.proto`の定義に従ってリクエストを送信することで結果が得られる。

## Get started
1. Build
```shell
$ cargo build --release
```

2. Run
```shell
$ cargo run -- serve
```

3. Example request
```shell
$  grpcurl -plaintext -import-path ./proto -proto moon.proto \
-d '{"date": "2022-07-17T00:00:00.000Z", "longitude": "133.833990", "latitude": "34.861972"}' \
[::]:50051 moon.MoonApi/MoonInfo
```

## Command Line Mode
Calc moon age at 12:00:00 on specified date.
```shell
$ cargo run -- calc --date "2023-12-31"
```
