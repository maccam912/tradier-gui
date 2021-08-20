use eframe::{egui, epi};
use eyre::{ContextCompat, Result};

use crate::{Page, State};

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
    egui::Grid::new("portfolio_page").show(ui, |ui| {
        ui.heading("Symbol");
        ui.heading("Quantity");
        ui.heading("Basis");
        ui.heading("Current Price");
        ui.heading("P/L");
        ui.heading("Date Acquired");
        ui.end_row();

        let positions_list = state.positions.as_ref().unwrap().positions.position.clone();
        for position in positions_list {
            ui.label(position.symbol);
            ui.label(format!("{:?}", position.quantity));
            ui.label(format!("{:?}", position.cost_basis));
            ui.label("not implemented");
            ui.label("not implemented");
            ui.label(format!("{:?}", position.date_acquired));
            ui.end_row();
        }
    });
    Ok(())
}

pub fn ui_orders(ui: &mut egui::Ui, _frame: &mut epi::Frame<'_>, state: &State) -> Result<()> {
    egui::Grid::new("orders_page").show(ui, |ui| {
        ui.heading("Symbol");
        ui.heading("Side");
        ui.heading("Quantity");
        ui.heading("Type");
        ui.heading("Duration");
        ui.heading("Filled");
        ui.heading("Status");
        ui.end_row();

        let orders_list = state.orders.as_ref().unwrap().orders.order.clone();
        for order in orders_list {
            ui.label(order.symbol);
            ui.label(format!("{:?}", order.side));
            ui.label(format!("{:?}", order.quantity));
            ui.label(format!("{:?}", order.order_type));
            ui.label(format!("{:?}", order.duration));
            ui.label(format!(
                "{:?}/{:?}",
                order.quantity - order.remaining_quantity,
                order.quantity
            ));
            ui.label(format!("{:?}", order.status));
            ui.end_row();
        }
    });
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
            state.page = Page::Orders;
        }
    });

    Ok(())
}
