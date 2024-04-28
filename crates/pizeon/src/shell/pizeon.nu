# Source this in your ~/.config/nushell/config.nu
$env.PIZEON_SESSION = (pizeon uuid)
hide-env -i PIZEON_HISTORY_ID

# Magic token to make sure we don't record commands run by keybindings
let PIZEON_KEYBINDING_TOKEN = $"# (random uuid)"

let _pizeon_pre_execution = {||
    if ($nu | get -i history-enabled) == false {
        return
    }
    let cmd = (commandline)
    if ($cmd | is-empty) {
        return
    }
    if not ($cmd | str starts-with $PIZEON_KEYBINDING_TOKEN) {
        $env.PIZEON_HISTORY_ID = (pizeon history start -- $cmd)
    }
}

let _pizeon_pre_prompt = {||
    let last_exit = $env.LAST_EXIT_CODE
    if 'PIZEON_HISTORY_ID' not-in $env {
        return
    }
    with-env { PIZEON_LOG: error } {
        do { pizeon history end $'--exit=($last_exit)' -- $env.PIZEON_HISTORY_ID } | complete

    }
    hide-env PIZEON_HISTORY_ID
}

def _pizeon_search_cmd [...flags: string] {
    let nu_version = do {
        let version = version
        let major = $version.major?
        if $major != null {
            # These members are only available in versions > 0.92.2
            [$major $version.minor $version.patch]
        } else {
            # So fall back to the slower parsing when they're missing
            $version.version | split row '.' | into int
        }
    }
    [
        $PIZEON_KEYBINDING_TOKEN,
        ([
            `with-env { PIZEON_LOG: error, PIZEON_QUERY: (commandline) } {`,
                (if $nu_version.0 <= 0 and $nu_version.1 <= 90 { 'commandline' } else { 'commandline edit' }),
                (if $nu_version.1 >= 92 { '(run-external pizeon search' } else { '(run-external --redirect-stderr pizeon search' }),
                    ($flags | append [--interactive] | each {|e| $'"($e)"'}),
                (if $nu_version.1 >= 92 { ' e>| str trim)' } else {' | complete | $in.stderr | str substring ..-1)'}),
            `}`,
        ] | flatten | str join ' '),
    ] | str join "\n"
}

$env.config = ($env | default {} config).config
$env.config = ($env.config | default {} hooks)
$env.config = (
    $env.config | upsert hooks (
        $env.config.hooks
        | upsert pre_execution (
            $env.config.hooks | get -i pre_execution | default [] | append $_pizeon_pre_execution)
        | upsert pre_prompt (
            $env.config.hooks | get -i pre_prompt | default [] | append $_pizeon_pre_prompt)
    )
)

$env.config = ($env.config | default [] keybindings)
