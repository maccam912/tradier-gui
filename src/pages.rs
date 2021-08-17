use eframe::{egui, epi};
use eyre::{eyre, ContextCompat, Result};
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

    egui::Grid::new("balance_page").show(ui, |ui| {
        ui.label("Account Number: ");
        ui.label(format!("{}", balance.balances.account_number));
        ui.end_row();

        ui.label("Market Value: ");
        ui.label(format!("${:.2}", balance.balances.market_value));
        ui.end_row();

        ui.label("Open P&L: ");
        ui.label(format!("${:.2}", balance.balances.open_pl));
        ui.end_row();

        ui.label("Total Cash: ");
        ui.label(format!("${:.2}", balance.balances.total_cash));
        ui.end_row();

        ui.label("Total Equity: ");
        ui.label(format!("${:.2}", balance.balances.total_equity));
        ui.end_row();

        let margin = balance.balances.margin.unwrap();

        ui.label("Option Buying Power: ");
        ui.label(format!("${:.2}", margin.option_buying_power));
        ui.end_row();

        ui.label("Stock Buying Power: ");
        ui.label(format!("${:.2}", margin.stock_buying_power));
        ui.end_row();
    });

    Ok(())
}
