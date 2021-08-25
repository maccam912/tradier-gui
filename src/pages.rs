use eframe::{
    egui::{self, ComboBox},
    epi,
};
use eyre::{ContextCompat, Result};
use tradier::{account::get_orders::Order, OrderStatus};

use crate::{update_orders, Page, State};

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

pub fn ui_orders(ui: &mut egui::Ui, _frame: &mut epi::Frame<'_>, state: &mut State) -> Result<()> {
    ui.horizontal(|ui| {
        ui.checkbox(&mut state.order_filters.open, "open");
        ui.checkbox(&mut state.order_filters.pending, "pending");
        ui.checkbox(
            &mut state.order_filters.partially_filled,
            "partially filled",
        );
        ui.checkbox(
            &mut state.order_filters.accepted_for_bidding,
            "accepted for bidding",
        );
        ui.checkbox(&mut state.order_filters.filled, "filled");
        ui.checkbox(&mut state.order_filters.expired, "expired");
        ui.checkbox(&mut state.order_filters.canceled, "canceled");
        ui.checkbox(&mut state.order_filters.rejected, "rejected");
        ui.checkbox(&mut state.order_filters.calculated, "calculated");
        ui.checkbox(&mut state.order_filters.error, "error");
        ui.checkbox(&mut state.order_filters.held, "held");
    });

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
        let filtered_orders: Vec<&Order> = orders_list
            .iter()
            .filter(|order| match order.status {
                OrderStatus::pending => state.order_filters.pending,
                OrderStatus::open => state.order_filters.open,
                OrderStatus::partially_filled => state.order_filters.partially_filled,
                OrderStatus::filled => state.order_filters.filled,
                OrderStatus::expired => state.order_filters.expired,
                OrderStatus::canceled => state.order_filters.canceled,
                OrderStatus::rejected => state.order_filters.rejected,
                OrderStatus::calculated => state.order_filters.calculated,
                OrderStatus::accepted_for_bidding => state.order_filters.accepted_for_bidding,
                OrderStatus::error => state.order_filters.error,
                OrderStatus::held => state.order_filters.held,
            })
            .collect();
        for order in filtered_orders {
            ui.label(order.symbol.clone());
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
            if ui.button("Cancel").clicked() {
                let profile =
                    tradier::account::get_user_profile::get_user_profile(&state.config).unwrap();
                let _ = tradier::trading::orders::cancel_order(
                    &state.config,
                    profile.profile.account[0].account_number.clone(),
                    order.id as i64,
                );
                update_orders(state);
            }
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
        ui.label("Security Type");
        ComboBox::from_id_source("security_type")
            .selected_text(state.place_order_state.security_type.clone())
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut state.place_order_state.security_type,
                    tradier::Class::equity,
                    "equity",
                );
                ui.selectable_value(
                    &mut state.place_order_state.security_type,
                    tradier::Class::option,
                    "option",
                );
            });
        ui.end_row();

        ui.label("Order Type");
        ComboBox::from_id_source("order_type")
            .selected_text(state.place_order_state.order_type.clone())
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut state.place_order_state.order_type,
                    tradier::OrderType::market,
                    "market",
                );
                ui.selectable_value(
                    &mut state.place_order_state.order_type,
                    tradier::OrderType::limit,
                    "limit",
                );
                ui.selectable_value(
                    &mut state.place_order_state.order_type,
                    tradier::OrderType::stop,
                    "stop",
                );
                ui.selectable_value(
                    &mut state.place_order_state.order_type,
                    tradier::OrderType::stop_limit,
                    "stop limit",
                );
            });
        ui.end_row();

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
            let _resp = tradier::trading::orders::post_order(
                &state.config,
                account_id.into(),
                state.place_order_state.security_type,
                state.order_symbol.clone(),
                tradier::Side::buy,
                1,
                state.place_order_state.order_type,
                tradier::Duration::gtc,
                None,
                None,
                None,
            );
            state.page = Page::Orders;
        }
        ui.end_row();
    });

    Ok(())
}
