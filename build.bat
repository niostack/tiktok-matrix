@echo off
set RUSTFLAGS=-A warnings
$env:TAURI_PRIVATE_KEY="dW50cnVzdGVkIGNvbW1lbnQ6IHJzaWduIGVuY3J5cHRlZCBzZWNyZXQga2V5ClJXUlRZMEl5QjJyVXB3OE1DSFBmMDJ1a2NMNGxVZDZ2U1pHaWplRldvRms1NXNPeTlYUUFBQkFBQUFBQUFBQUFBQUlBQUFBQXFRbnAxVGNmK2lheVl1ZTFYTmVtSTNiU1lPMVdrZUxEQlErUFdWc0ZOdS9ZUkJaQ2RTajhXWEFUaWlTMDU2WHU0WnVPQ2xPck5GbXZlcnpIYXRWWUNSZkhmVFNEaXpiZ1NQSVJFOW5xdVlHeURVS1JSOXBjQTU3WVBXRXlrS1VOOUNoMjY0MlY4aE09Cg=="
$env:TAURI_KEY_PASSWORD="123456"
@REM call npm config set proxy http://localhost:7890
@REM call npm config set https-proxy http://localhost:7890
npm run tauri build
@REM call npm config delete proxy
@REM call npm config delete https-proxy