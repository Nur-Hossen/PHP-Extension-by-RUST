use ext_php_rs::prelude::*;
use ext_php_rs::convert::IntoZval; // এটি 'into_zval' মেথডটি ব্যবহারের জন্য প্রয়োজন
use ext_php_rs::types::Zval;       // PhpValue এর বদলে Zval ব্যবহার করা বেশি নিরাপদ
use chrono::{NaiveDate, Duration};
use std::collections::HashMap;

#[php_function]
pub fn calculate_loan_schedule(
    loan_amount: f64,
    interest_rate: f64,
    loan_type: String,
    installments: i32,
    disburse_date: String
) -> Vec<Zval> {
    let mut schedule = Vec::new();
    let mut current_date = NaiveDate::parse_from_str(&disburse_date, "%Y-%m-%d")
        .unwrap_or(NaiveDate::from_ymd_opt(2023, 1, 1).unwrap());

    let monthly_interest = interest_rate / 12.0 / 100.0;
    let principal_per_month = loan_amount / installments as f64;

    for i in 1..=installments {
        current_date = current_date + Duration::days(30);
        
        let interest = if loan_type == "flat" {
            loan_amount * monthly_interest
        } else {
            (loan_amount - (principal_per_month * (i - 1) as f64)) * monthly_interest
        };

        let mut row: HashMap<&str, String> = HashMap::new();
        row.insert("installment_no", i.to_string());
        row.insert("date", current_date.format("%Y-%m-%d").to_string());
        row.insert("principal", format!("{:.2}", principal_per_month));
        row.insert("interest", format!("{:.2}", interest));
        row.insert("total", format!("{:.2}", principal_per_month + interest));

        // ext-php-rs এ into_zval(false) আর্গুমেন্ট নেয় (false মানে মেমোরি পারসিস্টেন্ট নয়)
        if let Ok(zval) = row.into_zval(false) {
            schedule.push(zval);
        }
    }

    schedule
}

#[php_module]
pub fn module(module: ModuleBuilder) -> ModuleBuilder {
    // ফাংশনটিকে ম্যানুয়ালি মডিউলের সাথে রেজিস্টার করুন
    module.function(wrap_function!(calculate_loan_schedule))
}

