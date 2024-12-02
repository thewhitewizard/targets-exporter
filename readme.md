
Prometheus exporter in Rust

## note to build for RPI 4

``` sh
sudo apt-get install -y gcc-aarch64-linux-gnu
rustup target add aarch64-unknown-linux-gnu
cargo build --target aarch64-unknown-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu
``` 

``` sh
curl localhost:8080/metrics

cryptocurrency_price_target{token="bitcoin"} 150000
cryptocurrency_price_target_tp1{token="bitcoin"} 112500
cryptocurrency_price_target_tp2{token="bitcoin"} 120000
cryptocurrency_price_target_tp3{token="bitcoin"} 135000
cryptocurrency_price_target{token="ethereum"} 10000
cryptocurrency_price_target_tp1{token="ethereum"} 7500
cryptocurrency_price_target_tp2{token="ethereum"} 8000
cryptocurrency_price_target_tp3{token="ethereum"} 9000
``` 

