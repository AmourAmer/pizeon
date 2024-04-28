use pizeon_dotfiles::store::{var::VarStore, AliasStore};
use eyre::Result;

pub fn init_static(disable_up_arrow: bool, disable_ctrl_r: bool) {
    let base = include_str!("../../../shell/pizeon.zsh");

    println!("{base}");

    if std::env::var("ATUIN_NOBIND").is_err() {
        const BIND_CTRL_R: &str = r"bindkey -M emacs '^r' pizeon-search
bindkey -M viins '^r' pizeon-search-viins
bindkey -M vicmd '/' pizeon-search";

        const BIND_UP_ARROW: &str = r"bindkey -M emacs '^[[A' pizeon-up-search
bindkey -M vicmd '^[[A' pizeon-up-search-vicmd
bindkey -M viins '^[[A' pizeon-up-search-viins
bindkey -M emacs '^[OA' pizeon-up-search
bindkey -M vicmd '^[OA' pizeon-up-search-vicmd
bindkey -M viins '^[OA' pizeon-up-search-viins
bindkey -M vicmd 'k' pizeon-up-search-vicmd";

        if !disable_ctrl_r {
            println!("{BIND_CTRL_R}");
        }
        if !disable_up_arrow {
            println!("{BIND_UP_ARROW}");
        }
    }
}

pub async fn init(
    aliases: AliasStore,
    vars: VarStore,
    disable_up_arrow: bool,
    disable_ctrl_r: bool,
) -> Result<()> {
    init_static(disable_up_arrow, disable_ctrl_r);

    let aliases = pizeon_dotfiles::shell::zsh::alias_config(&aliases).await;
    let vars = pizeon_dotfiles::shell::zsh::var_config(&vars).await;

    println!("{aliases}");
    println!("{vars}");

    Ok(())
}
