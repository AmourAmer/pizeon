# shellcheck disable=SC2034,SC2153,SC2086,SC2155

# Above line is because shellcheck doesn't support zsh, per
# https://github.com/koalaman/shellcheck/wiki/SC1071, and the ignore: param in
# ludeeus/action-shellcheck only supports _directories_, not _files_. So
# instead, we manually add any error the shellcheck step finds in the file to
# the above line ...

# Source this in your ~/.zshrc
autoload -U add-zsh-hook

zmodload zsh/datetime 2>/dev/null

# If zsh-autosuggestions is installed, configure it to use Atuin's search. If
# you'd like to override this, then add your config after the $(pizeon init zsh)
# in your .zshrc
_zsh_autosuggest_strategy_pizeon() {
    suggestion=$(ATUIN_QUERY="$1" pizeon search --cmd-only --limit 1 --search-mode prefix)
}

if [ -n "${ZSH_AUTOSUGGEST_STRATEGY:-}" ]; then
    ZSH_AUTOSUGGEST_STRATEGY=("pizeon" "${ZSH_AUTOSUGGEST_STRATEGY[@]}")
else
    ZSH_AUTOSUGGEST_STRATEGY=("pizeon")
fi

export ATUIN_SESSION=$(pizeon uuid)
ATUIN_HISTORY_ID=""

_pizeon_preexec() {
    local id
    id=$(pizeon history start -- "$1")
    export ATUIN_HISTORY_ID="$id"
    __pizeon_preexec_time=${EPOCHREALTIME-}
}

_pizeon_precmd() {
    local EXIT="$?" __pizeon_precmd_time=${EPOCHREALTIME-}

    [[ -z "${ATUIN_HISTORY_ID:-}" ]] && return

    local duration=""
    if [[ -n $__pizeon_preexec_time && -n $__pizeon_precmd_time ]]; then
        printf -v duration %.0f $(((__pizeon_precmd_time - __pizeon_preexec_time) * 1000000000))
    fi

    (ATUIN_LOG=error pizeon history end --exit $EXIT ${duration:+--duration=$duration} -- $ATUIN_HISTORY_ID &) >/dev/null 2>&1
    export ATUIN_HISTORY_ID=""
}

_pizeon_search() {
    emulate -L zsh
    zle -I

    # swap stderr and stdout, so that the tui stuff works
    # TODO: not this
    local output
    # shellcheck disable=SC2048
    output=$(ATUIN_SHELL_ZSH=t ATUIN_LOG=error ATUIN_QUERY=$BUFFER pizeon search $* -i 3>&1 1>&2 2>&3)

    zle reset-prompt

    if [[ -n $output ]]; then
        RBUFFER=""
        LBUFFER=$output

        if [[ $LBUFFER == __pizeon_accept__:* ]]
        then
            LBUFFER=${LBUFFER#__pizeon_accept__:}
            zle accept-line
        fi
    fi
}
_pizeon_search_vicmd() {
    _pizeon_search --keymap-mode=vim-normal
}
_pizeon_search_viins() {
    _pizeon_search --keymap-mode=vim-insert
}

_pizeon_up_search() {
    # Only trigger if the buffer is a single line
    if [[ ! $BUFFER == *$'\n'* ]]; then
        _pizeon_search --shell-up-key-binding "$@"
    else
        zle up-line
    fi
}
_pizeon_up_search_vicmd() {
    _pizeon_up_search --keymap-mode=vim-normal
}
_pizeon_up_search_viins() {
    _pizeon_up_search --keymap-mode=vim-insert
}

add-zsh-hook preexec _pizeon_preexec
add-zsh-hook precmd _pizeon_precmd

zle -N pizeon-search _pizeon_search
zle -N pizeon-search-vicmd _pizeon_search_vicmd
zle -N pizeon-search-viins _pizeon_search_viins
zle -N pizeon-up-search _pizeon_up_search
zle -N pizeon-up-search-vicmd _pizeon_up_search_vicmd
zle -N pizeon-up-search-viins _pizeon_up_search_viins

# These are compatibility widget names for "pizeon <= 17.2.1" users.
zle -N _pizeon_search_widget _pizeon_search
zle -N _pizeon_up_search_widget _pizeon_up_search
