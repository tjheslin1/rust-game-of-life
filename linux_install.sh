GOL_HOLE_INSTALL_DIR="/opt/game_of_life"
GOL_HOLE_BIN_DIR="/usr/local/bin"

if [ -z "${USER}" ]; then
  USER="$(id -un)"
fi

echo "cleaning up /opt/game_of_life"
rm -rf "${GOL_HOLE_INSTALL_DIR}"

echo "Shallow clone of tjheslin1/rust-game-of-life to ${GOL_HOLE_INSTALL_DIR}"
mkdir -p "${GOL_HOLE_INSTALL_DIR}"
cd "${GOL_HOLE_INSTALL_DIR}"
git clone --depth 1 https://github.com/tjheslin1/rust-game-of-life.git 1>/dev/null 2>&1

echo "Copying binary to /usr/local/bin/gol"
mv "${GOL_HOLE_INSTALL_DIR}"/rust-game-of-life/release/game_of_life "${GOL_HOLE_INSTALL_DIR}"/rust-game-of-life/release/gol
chmod +x "${GOL_HOLE_INSTALL_DIR}"/rust-game-of-life/release/gol
# cp /opt/game_of_life/rust-game-of-life/release/game_of_life /usr/local/bin/gol
install -o "${USER}" -Dm755 -t "${GOL_HOLE_BIN_DIR}" "${GOL_HOLE_INSTALL_DIR}"/rust-game-of-life/release/gol