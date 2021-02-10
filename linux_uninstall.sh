GOL_BIN_DIR="/usr/local/bin"
GOL_INSTALL_DIR="/opt/game_of_life"

echo "Removing binary"
sudo rm -f "${GOL_BIN_DIR}"/gol

echo "Emptying install dir"
sudo rm -rf "${GOL_INSTALL_DIR}"
