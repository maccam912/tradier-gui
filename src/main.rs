use std::{sync::Mutex, thread};

use eframe::{egui, epi};
use eyre::Result;
use once_cell::sync::Lazy;
use tradier::TradierConfig;

#[derive(Clone)]
struct State {
    balance: f64,
    config: TradierConfig,
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

impl epi::App for TradierApp {
    fn name(&self) -> &str {
        "Tradier Platform"
    }

    fn update(&mut self, ctx: &eframe::egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let Self { state } = self;

        egui::TopBottomPanel::top("header_panel").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Tradier Application");
            });
        });

        egui::TopBottomPanel::bottom("footer_panel").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading(format!("Balance: {}", state.lock().unwrap().balance));
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                if ui.button("Update").clicked() {
                    thread::spawn(|| {
                        let _ = update();
                    });
                }
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
