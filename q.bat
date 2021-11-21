rem	cargo build --release
cargo build
cls
target\debug\existe_en_path.exe %1
echo %ERRORLEVEL%
