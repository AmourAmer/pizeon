import subprocess

from prompt_toolkit.application.current import get_app
from prompt_toolkit.filters import Condition
from prompt_toolkit.keys import Keys


$PIZEON_SESSION=$(pizeon uuid).rstrip('\n')

@events.on_precommand
def _pizeon_precommand(cmd: str):
    cmd = cmd.rstrip("\n")
    $PIZEON_HISTORY_ID = $(pizeon history start -- @(cmd)).rstrip("\n")


@events.on_postcommand
def _pizeon_postcommand(cmd: str, rtn: int, out, ts):
    if "PIZEON_HISTORY_ID" not in ${...}:
        return

    duration = ts[1] - ts[0]
    # Duration is float representing seconds, but pizeon expects integer of nanoseconds
    nanos = round(duration * 10 ** 9)
    with ${...}.swap(PIZEON_LOG="error"):
        # This causes the entire .xonshrc to be re-executed, which is incredibly slow
        # This happens when using a subshell and using output redirection at the same time
        # For more details, see https://github.com/xonsh/xonsh/issues/5224
        # (pizeon history end --exit @(rtn) -- $PIZEON_HISTORY_ID &) > /dev/null 2>&1
        pizeon history end --exit @(rtn) --duration @(nanos) -- $PIZEON_HISTORY_ID > /dev/null 2>&1
    del $PIZEON_HISTORY_ID


def _search(event, extra_args: list[str]):
    buffer = event.current_buffer
    cmd = ["pizeon", "search", "--interactive", *extra_args]
    # We need to explicitly pass in xonsh env, in case user has set XDG_HOME or something else that matters
    env = ${...}.detype()
    env["PIZEON_SHELL_XONSH"] = "t"
    env["PIZEON_QUERY"] = buffer.text

    p = subprocess.run(cmd, stderr=subprocess.PIPE, encoding="utf-8", env=env)
    result = p.stderr.rstrip("\n")
    # redraw prompt - necessary if pizeon is configured to run inline, rather than fullscreen
    event.cli.renderer.erase()

    if not result:
        return

    buffer.reset()
    if result.startswith("__pizeon_accept__:"):
        buffer.insert_text(result[17:])
        buffer.validate_and_handle()
    else:
        buffer.insert_text(result)


@events.on_ptk_create
def _custom_keybindings(bindings, **kw):
    if _PIZEON_BIND_CTRL_R:
        @bindings.add(Keys.ControlR)
        def r_search(event):
            _search(event, extra_args=[])

    if _PIZEON_BIND_UP_ARROW:
        @Condition
        def should_search():
            buffer = get_app().current_buffer
            # disable keybind when there is an active completion, so
            # that up arrow can be used to navigate completion menu
            if buffer.complete_state is not None:
                return False
            # similarly, disable when buffer text contains multiple lines
            if '\n' in buffer.text:
                return False

            return True

        @bindings.add(Keys.Up, filter=should_search)
        def up_search(event):
            _search(event, extra_args=["--shell-up-key-binding"])
