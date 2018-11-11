title Packaging the game

git pull
cargo update
cargo +nightly build --release
copy /b target\release\test_game.exe test_game.exe
del merry_christmas.rar
"C:\Program Files\WinRAR\Rar.exe" a merry_christmas.rar test_game.exe resources
del test_game.exe
pause