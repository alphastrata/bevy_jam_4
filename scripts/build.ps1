cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --target web --out-dir .\\out\\ --out-name "flora-cause" .\\target\\wasm32-unknown-unknown\\release\\bgj.wasm
Copy-Item .\\assets\\other\\index.html .\\out\\index.html
[System.IO.Directory]::CreateDirectory('.\\build')
Compress-Archive -Force -LiteralPath .\\out -DestinationPath .\\build\\flora-cause.zip