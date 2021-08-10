use std::{sync::Mutex, thread};

use eframe::{egui, epi};
use once_cell::sync::Lazy;
use eyre::Result;

#[derive(Clone, Copy)]
struct State {
    balance: f64,
}

unsafe impl Send for State {}

static STATE: Lazy<Mutex<State>> = Lazy::new(|| {
    Mutex::new(State{balance: 0.0})
});

struct TradierApp {
    state: &'static Mutex<State>,
}

impl Default for TradierApp {
    fn default() -> Self {
        Self {
            state: &STATE,
        }
    }
}

impl epi::App for TradierApp {
    fn name(&self) -> &str {
        "Tradier Platform"
    }

    fn update(&mut self, ctx: &eframe::egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let Self { state } = self;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Tradier Application");
            ui.horizontal(|ui| {
                ui.label("Balance: ");
                ui.label(format!("{:?}", state.lock().unwrap().balance));
            });

        });

        frame.set_window_size(ctx.used_size());
    }
}

fn update() -> Result<()> {
    let acct = tradier::account::get_user_profile::get_user_profile().unwrap();
    let balance = tradier::account::get_balances::get_balances(acct.profile.account[0].account_number.clone()).unwrap();
    let bal = balance.balances.total_equity;
    STATE.lock().unwrap().balance = bal;
    Ok(())
}

fn main() {
    let options = eframe::NativeOptions::default();

    thread::spawn(
        || {
            let _ = update();
        }
    );

    eframe::run_native(Box::new(TradierApp::default()), options);
}
