use eframe::{egui, epi};
use eyre::{ContextCompat, Result};

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

pub fn ui_portfolio(ui: &mut egui::Ui, _frame: &mut epi::Frame<'_>, state: &State) -> Result<()> {
    ui.label(format!("{:?}", state.positions));
    Ok(())
}

pub fn ui_orders(ui: &mut egui::Ui, _frame: &mut epi::Frame<'_>, state: &State) -> Result<()> {
    ui.label(format!("{:?}", state.orders));
    Ok(())
}

pub fn ui_place_order(
    ui: &mut egui::Ui,
    _frame: &mut epi::Frame<'_>,
    state: &mut State,
) -> Result<()> {
    egui::Grid::new("place_order_page").show(ui, |ui| {
        ui.label("Symbol");
        ui.text_edit_singleline(&mut state.order_symbol);
        ui.end_row();
        if ui.button("Submit").clicked() {
            let account_id: &str =
                &tradier::account::get_user_profile::get_user_profile(&state.config)
                    .unwrap()
                    .profile
                    .account[0]
                    .account_number;
            let resp = tradier::trading::orders::post_order(
                &state.config,
                account_id.into(),
                tradier::Class::equity,
                state.order_symbol.clone(),
                tradier::Side::buy,
                1,
                tradier::OrderType::market,
                tradier::Duration::gtc,
                None,
                None,
                None,
            );
            state.debug_text = format!("{:?}", resp);
        }
    });

    Ok(())
}
