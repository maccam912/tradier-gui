use eframe::{egui, epi};
use eyre::{eyre, ContextCompat, Result};

use crate::State;

pub fn ui_balance_page(
    ui: &mut egui::Ui,
    _frame: &mut epi::Frame<'_>,
    state: &State,
) -> Result<()> {
    if let Some(balance) = &state.balance {
        let balances = &balance.balances;
        let margin = balances.margin.as_ref().wrap_err("margin was None")?;

        egui::Grid::new("balance_page").show(ui, |ui| {
            ui.label("Account Number: ");
            ui.label(format!("{}", balances.account_number));

            ui.label("Market Value: ");
            ui.label(format!("${:.2}", balances.market_value));

            ui.label("Total Cash: ");
            ui.label(format!("${:.2}", balances.total_cash));

            ui.label("Stock Buying Power: ");
            ui.label(format!("${:.2}", margin.stock_buying_power));

            ui.end_row();

            ui.label("");
            ui.label("");

            ui.label("Open P&L: ");
            ui.label(format!("${:.2}", balances.open_pl));

            ui.label("Total Equity: ");
            ui.label(format!("${:.2}", balances.total_equity));

            ui.label("Option Buying Power: ");
            ui.label(format!("${:.2}", margin.option_buying_power));

            ui.end_row();
        });
    }

    Ok(())
}
