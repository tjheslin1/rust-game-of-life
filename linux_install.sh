echo "cleaning up /tmp/game_of_life"
rm -rf /tmp/game_of_life

echo "Shallow clone of tjheslin1/rust-game-of-life to /tmp/game_of_life"
mkdir /tmp/game_of_life
cd /tmp/game_of_life
git clone --depth 1 https://github.com/tjheslin1/rust-game-of-life.git 1>/dev/null 2>&1

echo "Copying binary to /usr/local/bin/gol"
cp /tmp/game_of_life/rust-game-of-life/release/game_of_life /usr/local/bin/gol
