# firebase-api_gateway-rs

mac上でクロスコンパイルできるようにする
```
rustup target add x86_64-unknown-linux-musl
brew install filosottile/musl-cross/musl-cross
ln -s /usr/local/bin/x86_64-linux-musl-gcc /usr/local/bin/musl-gcc
```
ビルドの後、ローカル実行
```
sam build
sam local start-api
```
