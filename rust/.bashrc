# bash
alias ..="cd .."
alias ...="cd ../.."
alias home='cd'
#ターミナルのプロンプトの色、表示の設定
PS1="\[\e[34m\]\u\[\e[33m\]@docker\[\e[32m\]:\w\$\[\e[0m\]"

# docker
alias dc="docker-compose"
alias d="docker"
alias dps="docker ps"
alias dpa="docker ps -a"
alias dcw="docker-compose exec workspace bash"
function de() {
    docker exec -it $1 bash
}
function dce() {
    docker-compose exec $1 bash
}

# git
alias gis="git status"
alias ad="git add"
alias di="git diff"

# Laravel
function art() {
    php artisan "$@"
}
