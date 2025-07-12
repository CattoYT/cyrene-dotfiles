# user=$(whoami)
# echo $user
#
#
# ln -s $PWD/nvim /Users/$user/config/nvim
#
# ln -s $PWD/kitty /Users/$user/config/kitty
# ln -s $PWD/karabiner /Users/$user/config/karabiner
#
# ln -s $PWD/.zshrc /Users/$user/.zshrc
if ! command -v cargo >/dev/null 2>&1; then
    echo "Cargo is missing, do you have Rustup installed?"

    return 1
fi

cargo run --manifest-path dotfiles-installer/Cargo.toml --release



