cargo build --release --target wasm32-unknown-unknown
Remove-Item -Recurse .\\build\\*
Remove-Item -Recurse .\\out\\*
wasm-bindgen --no-typescript --target web --out-dir .\\out\\ --out-name "flora_cause" .\\target\\wasm32-unknown-unknown\\release\\flora_cause.wasm
Copy-Item .\\wasm\\* .\\out\\
[System.IO.Directory]::CreateDirectory('.\\build')
$time = Get-Date -Format "yyMMdd_HHmmss";
Compress-Archive -Force -Path .\\out\\* -DestinationPath ".\\build\\flora_cause_$time.zip"