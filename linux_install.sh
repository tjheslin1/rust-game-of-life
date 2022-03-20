GOL_INSTALL_DIR="/opt/game_of_life"
GOL_BIN_DIR="/usr/local/bin"

if [ -z "${USER}" ]; then
  USER="$(id -un)"
fi

sudo install -o "${USER}" -Dm755 -d "${GOL_INSTALL_DIR}"

echo "Shallow clone of tjheslin1/rust-game-of-life to ${GOL_INSTALL_DIR}"

cd "${GOL_INSTALL_DIR}"

git clone -q --depth 1 https://github.com/tjheslin1/rust-game-of-life.git 1>/dev/null 2>&1
# chown -R "${USER}"

echo "Copying binary to /usr/local/bin/gol"

sudo install -o "${USER}" -Dm755 -t "${GOL_BIN_DIR}" "${GOL_INSTALL_DIR}"/rust-game-of-life/release/gol
