eval "$(starship init zsh)"
alias ls="eza -a"
alias vi="nvim"
alias vim="nvim"

pfetch
# Lines configured by zsh-newuser-install
# End of lines configured by zsh-newuser-install
# The following lines were added by compinstall
zstyle :compinstall filename '/Users/hikari/.zshrc'

autoload -Uz compinit
compinit
# End of lines added by compinstall
#
eval "$(zoxide init zsh)"

alias cd="z"


# wallpaper management and kitty colours ig
wallpaper_path=$(osascript -e 'tell application "System Events" to get picture of desktop 1')

echo "$wallpaper_path"

wallust run -u $wallpaper_path
