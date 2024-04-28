set -gx PIZEON_SESSION (pizeon uuid)
set --erase PIZEON_HISTORY_ID

function _pizeon_preexec --on-event fish_preexec
    if not test -n "$fish_private_mode"
        set -g PIZEON_HISTORY_ID (pizeon history start -- "$argv[1]")
    end
end

function _pizeon_postexec --on-event fish_postexec
    set -l s $status

    if test -n "$PIZEON_HISTORY_ID"
        PIZEON_LOG=error pizeon history end --exit $s -- $PIZEON_HISTORY_ID &>/dev/null &
        disown
    end

    set --erase PIZEON_HISTORY_ID
end

function _pizeon_search
    set -l keymap_mode
    switch $fish_key_bindings
        case fish_vi_key_bindings
            switch $fish_bind_mode
                case default
                    set keymap_mode vim-normal
                case insert
                    set keymap_mode vim-insert
            end
        case '*'
            set keymap_mode emacs
    end

    # In fish 3.4 and above we can use `"$(some command)"` to keep multiple lines separate;
    # but to support fish 3.3 we need to use `(some command | string collect)`.
    # https://fishshell.com/docs/current/relnotes.html#id24 (fish 3.4 "Notable improvements and fixes")
    set -l PIZEON_H (PIZEON_SHELL_FISH=t PIZEON_LOG=error PIZEON_QUERY=(commandline -b) pizeon search --keymap-mode=$keymap_mode $argv -i 3>&1 1>&2 2>&3 | string collect)

    if test -n "$PIZEON_H"
        if string match --quiet '__pizeon_accept__:*' "$PIZEON_H"
          set -l PIZEON_HIST (string replace "__pizeon_accept__:" "" -- "$PIZEON_H" | string collect)
          commandline -r "$PIZEON_HIST"
          commandline -f repaint
          commandline -f execute
          return
        else
          commandline -r "$PIZEON_H"
        end
    end

    commandline -f repaint
end

function _pizeon_bind_up
    # Fallback to fish's builtin up-or-search if we're in search or paging mode
    if commandline --search-mode; or commandline --paging-mode
        up-or-search
        return
    end

    # Only invoke pizeon if we're on the top line of the command
    set -l lineno (commandline --line)

    switch $lineno
        case 1
            _pizeon_search --shell-up-key-binding
        case '*'
            up-or-search
    end
end
