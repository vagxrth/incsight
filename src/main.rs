// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{error::Error, fmt::format};

slint::include_modules!();

const TAX: f64 = 0.30;
const INVESTMENT: f64 = 0.35;
const ALLOWANCES: f64 = 0.15;
const EXPENDITURE: f64 = 0.20;

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();

    ui.on_income_insight( move |string| {

            let ui = ui_handle.unwrap();
            let num: f64 = string.trim().parse().unwrap();
            let tax: f64 = num * TAX;
            let investment: f64 = num * INVESTMENT;
            let allowances: f64 = num * ALLOWANCES;
            let expenditure: f64 = num * EXPENDITURE;
            let insight = format!("Taxes: {:.2}\nInvestment: {:.2}\nAllowances: {:.2}\nExpenditure: {:.2}", tax, investment, allowances, expenditure);
            ui.set_insight(insight.into());
    });

    ui.run()?;

    Ok(())
}
