// use pizeon_dotfiles::store::{var::VarStore, AliasStore};
// use eyre::Result;
// 
// pub fn init_static(disable_up_arrow: bool, disable_ctrl_r: bool) {
//     let base = include_str!("../../../shell/pizeon.bash");
// 
//     let (bind_ctrl_r, bind_up_arrow) = if std::env::var("PIZEON_NOBIND").is_ok() {
//         (false, false)
//     } else {
//         (!disable_ctrl_r, !disable_up_arrow)
//     };
// 
//     println!("__pizeon_bind_ctrl_r={bind_ctrl_r}");
//     println!("__pizeon_bind_up_arrow={bind_up_arrow}");
//     println!("{base}");
// }
// 
// pub async fn init(
//     aliases: AliasStore,
//     vars: VarStore,
//     disable_up_arrow: bool,
//     disable_ctrl_r: bool,
// ) -> Result<()> {
//     init_static(disable_up_arrow, disable_ctrl_r);
// 
//     let aliases = pizeon_dotfiles::shell::bash::alias_config(&aliases).await;
//     let vars = pizeon_dotfiles::shell::bash::var_config(&vars).await;
// 
//     println!("{aliases}");
//     println!("{vars}");
// 
//     Ok(())
// }
