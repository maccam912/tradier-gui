use eframe::{egui, epi};
use eyre::Result;
use tradier::TradierConfig;

pub fn ui_balance_page(
    ui: &mut egui::Ui,
    _frame: &mut epi::Frame<'_>,
    config: &mut TradierConfig,
) -> Result<()> {
    let profile = tradier::account::get_user_profile(config)?;
    let balance = tradier::account::get_balances(
        config,
        profile
            .profile
            .account
            .get(0)
            .unwrap()
            .account_number
            .clone(),
    )?;

    ui.label(format!("{:?}", balance));

    Ok(())
}
