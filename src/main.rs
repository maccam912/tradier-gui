use std::sync::Mutex;

use eframe::{
    egui::{self, ComboBox},
    epi,
};

use once_cell::sync::Lazy;
use tradier::{account::BalancesRoot, TradierConfig};

mod pages;

#[derive(Debug, Clone, PartialEq)]
enum Page {
    Balance,
    Portfolio,
    Orders,
    Stocks,
    Options,
}

#[derive(Debug, Clone)]
pub struct State {
    balance: Option<BalancesRoot>,
    config: TradierConfig,
    page: Page,
}

unsafe impl Send for State {}

static STATE: Lazy<Mutex<State>> = Lazy::new(|| {
    Mutex::new(State {
        balance: None,
        config: TradierConfig {
            token: env!("TRADIER_TOKEN").into(),
            endpoint: option_env!("TRADIER_ENDPOINT")
                .unwrap_or("https://sandbox.tradier.com")
                .into(),
        },
        page: Page::Balance,
    })
});

struct TradierApp {
    state: &'static Mutex<State>,
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

impl epi::App for TradierApp {
    fn name(&self) -> &str {
        "Tradier Platform"
    }

    fn update(&mut self, ctx: &eframe::egui::CtxRef, frame: &mut epi::Frame<'_>) {
        // let Self { state } = self;
        let state = &mut STATE.lock().unwrap();

        egui::TopBottomPanel::top("header_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui_login(ui, frame, &mut state.config);
                let _ = pages::ui_balance_page(ui, frame, &state);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.horizontal(|ui| {
                    if ui.button("Balance").clicked() {
                        state.page = Page::Balance;
                        let profile = tradier::account::get_user_profile(&state.config).unwrap();
                        let balance = tradier::account::get_balances(
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
                    if ui.button("Portfolio").clicked() {
                        state.page = Page::Portfolio;
                    }
                    if ui.button("Orders").clicked() {
                        state.page = Page::Orders;
                    }
                    if ui.button("Stocks").clicked() {
                        state.page = Page::Stocks;
                    }
                    if ui.button("Options").clicked() {
                        state.page = Page::Options;
                    }
                });
            })
        });

        frame.set_window_size(ctx.used_size());
    }
}

fn main() {
    let options = eframe::NativeOptions::default();

    eframe::run_native(Box::new(TradierApp::default()), options);
}
