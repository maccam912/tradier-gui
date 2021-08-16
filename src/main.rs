use std::{sync::Mutex, thread};

use eframe::{
    egui::{self, ComboBox},
    epi,
};
use eyre::Result;
use once_cell::sync::Lazy;
use tradier::TradierConfig;

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
struct State {
    balance: f64,
    config: TradierConfig,
    page: Page,
}

unsafe impl Send for State {}

static STATE: Lazy<Mutex<State>> = Lazy::new(|| {
    Mutex::new(State {
        balance: 0.0,
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
        let Self { state } = self;

        egui::TopBottomPanel::top("header_panel").show(ctx, |ui| {
            let config = &mut STATE.lock().unwrap().config;
            ui_login(ui, frame, config);
        });

        egui::TopBottomPanel::bottom("footer_panel").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading(format!("Balance: {}", state.lock().unwrap().balance));
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let state = &mut STATE.lock().unwrap();
            ui.vertical_centered(|ui| {
                ui.horizontal(|ui| {
                    if ui.button("Update").clicked() {
                        thread::spawn(|| {
                            let _ = update();
                        });
                    }

                    if ui.button("Balance").clicked() {
                        state.page = Page::Balance;
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
                if state.page == Page::Balance {
                    let _ = pages::ui_balance_page(ui, frame, &mut state.config);
                }

                ui.label(format!("{:?}", state.page));
            })
        });

        frame.set_window_size(ctx.used_size());
    }
}

fn update() -> Result<()> {
    let acct = tradier::account::get_user_profile(&STATE.lock().unwrap().config).unwrap();
    let balance = tradier::account::get_balances(
        &STATE.lock().unwrap().config,
        acct.profile.account[0].account_number.clone(),
    )
    .unwrap();
    let bal = balance.balances.total_equity;
    STATE.lock().unwrap().balance = bal;
    Ok(())
}

fn main() {
    let options = eframe::NativeOptions::default();

    eframe::run_native(Box::new(TradierApp::default()), options);
}
