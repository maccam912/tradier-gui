use std::sync::Mutex;

use eframe::{
    egui::{self, ComboBox},
    epi,
};

use once_cell::sync::Lazy;
use pages::{ui_orders, ui_place_order, ui_portfolio};
use tradier::{
    account::{get_balances::BalancesRoot, get_orders::OrdersRoot, get_positions::PositionsRoot},
    TradierConfig,
};

mod pages;

#[derive(Debug, Clone, PartialEq)]
enum Page {
    Balance,
    Portfolio,
    Orders,
    PlaceOrder,
}

#[derive(Debug, Clone)]
pub struct OrderFilters {
    pending: bool,
    canceled: bool,
    filled: bool,
    open: bool,
    partially_filled: bool,
    expired: bool,
    rejected: bool,
    calculated: bool,
    accepted_for_bidding: bool,
    error: bool,
    held: bool,
}

impl Default for OrderFilters {
    fn default() -> Self {
        Self {
            pending: true,
            canceled: false,
            filled: false,
            open: true,
            partially_filled: true,
            expired: false,
            rejected: false,
            calculated: false,
            accepted_for_bidding: false,
            error: false,
            held: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlaceOrderState {
    security_type: tradier::Class,
    order_type: tradier::OrderType,
}

impl Default for PlaceOrderState {
    fn default() -> Self {
        Self {
            security_type: tradier::Class::equity,
            order_type: tradier::OrderType::market,
        }
    }
}

#[derive(Debug, Clone)]
pub struct State {
    balance: Option<BalancesRoot>,
    positions: Option<PositionsRoot>,
    orders: Option<OrdersRoot>,
    config: TradierConfig,
    page: Page,
    order_symbol: String,
    debug_text: String,
    order_filters: OrderFilters,
    place_order_state: PlaceOrderState,
}

unsafe impl Send for State {}

static STATE: Lazy<Mutex<State>> = Lazy::new(|| {
    Mutex::new(State {
        balance: None,
        positions: None,
        orders: None,
        config: TradierConfig {
            token: env!("TRADIER_TOKEN").into(),
            endpoint: option_env!("TRADIER_ENDPOINT")
                .unwrap_or("https://sandbox.tradier.com")
                .into(),
        },
        page: Page::Balance,
        order_symbol: "".into(),
        debug_text: "".into(),
        order_filters: OrderFilters::default(),
        place_order_state: PlaceOrderState::default(),
    })
});

struct TradierApp {
    pub state: &'static Mutex<State>,
}

impl Default for TradierApp {
    fn default() -> Self {
        Self { state: &STATE }
    }
}

fn ui_login(ui: &mut egui::Ui, _frame: &mut epi::Frame<'_>, config: &mut TradierConfig) {
    egui::Grid::new("broker_login").show(ui, |ui| {
        ui.heading("Tradier Login");
        ui.end_row();

        ui.label("Environment:");
        ComboBox::from_id_source("endpoint")
            .selected_text(config.endpoint.clone())
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut config.endpoint,
                    "https://sandbox.tradier.com".into(),
                    "https://sandbox.tradier.com",
                );
                ui.selectable_value(
                    &mut config.endpoint,
                    "https://www.tradier.com".into(),
                    "https://www.tradier.com",
                );
            });
        ui.end_row();

        ui.label("API Token: ");
        ui.text_edit_singleline(&mut config.token);
        ui.end_row();
    });
}

fn update_balance(state: &mut State) {
    let profile = tradier::account::get_user_profile::get_user_profile(&state.config).unwrap();
    let balance = tradier::account::get_balances::get_balances(
        &state.config,
        profile
            .profile
            .account
            .get(0)
            .unwrap()
            .account_number
            .clone(),
    )
    .unwrap();
    state.balance = Some(balance);
}

fn update_orders(state: &mut State) {
    let profile = tradier::account::get_user_profile::get_user_profile(&state.config).unwrap();

    let orders = tradier::account::get_orders::get_orders(
        &state.config,
        profile.profile.account[0].account_number.clone(),
        false,
    )
    .unwrap();
    state.orders = Some(orders);
}

impl epi::App for TradierApp {
    fn name(&self) -> &str {
        "Tradier Platform"
    }

    fn update(&mut self, ctx: &eframe::egui::CtxRef, frame: &mut epi::Frame<'_>) {
        // let Self { state } = self;
        let mut state = &mut STATE.lock().unwrap();

        egui::TopBottomPanel::top("header_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui_login(ui, frame, &mut state.config);
                let _ = pages::ui_balance_page(ui, frame, &state);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.horizontal(|ui| {
                    if ui.button("Update").clicked() {
                        state.page = Page::Balance;
                        update_balance(state);
                    }
                    if ui.button("Portfolio").clicked() {
                        state.page = Page::Portfolio;
                        let profile =
                            tradier::account::get_user_profile::get_user_profile(&state.config)
                                .unwrap();
                        let positions = tradier::account::get_positions::get_positions(
                            &state.config,
                            profile.profile.account[0].account_number.clone(),
                        )
                        .unwrap();
                        state.positions = Some(positions);
                    }
                    if ui.button("Orders").clicked() {
                        state.page = Page::Orders;
                        update_orders(&mut state);
                    }
                    if ui.button("Place Order").clicked() {
                        state.page = Page::PlaceOrder;
                    }
                });

                match state.page {
                    Page::Portfolio => {
                        let _ = ui_portfolio(ui, frame, &state);
                    }
                    Page::Orders => {
                        let _ = ui_orders(ui, frame, &mut state);
                    }
                    Page::PlaceOrder => {
                        let _ = ui_place_order(ui, frame, &mut state);
                    }
                    _ => {}
                };
            });
            ui.label(&state.debug_text);
        });

        if state.balance.is_none() {
            update_balance(state);
        }
        frame.set_window_size(ctx.used_size());
    }
}

fn main() {
    let options = eframe::NativeOptions::default();

    eframe::run_native(Box::new(TradierApp::default()), options);
}
