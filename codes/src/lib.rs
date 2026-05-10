mod calculations;
mod models;

use ext_php_rs::prelude::*;
use calculations::traits::LoanStrategy;
use calculations::flat::FlatInterest;
use calculations::reducing::ReducingInterest;

#[php_function]
pub fn calculate_loan_schedule(
    loan_amount: f64,
    interest_rate: f64,
    loan_type: String,
    installments: i32,
    disburse_date: String
) -> Vec<ext_php_rs::types::Zval> {
    
    // Strategy Selection (Open/Closed Principle: নতুন টাইপ আসলে এখানে অ্যাড করা সহজ)
    let strategy: Box<dyn LoanStrategy> = match loan_type.as_str() {
        "flat" => Box::new(FlatInterest),
        _ => Box::new(ReducingInterest),
    };

    let schedule_data = strategy.calculate(loan_amount, interest_rate, installments, disburse_date);
    
    // PHP Array তে রূপান্তর
    schedule_data.into_iter()
        .map(|item| item.to_zval())
        .collect()
}

#[php_module]
pub fn module(module: ModuleBuilder) -> ModuleBuilder {
    // ফাংশনটিকে ম্যানুয়ালি মডিউলের সাথে রেজিস্টার করুন
    module.function(wrap_function!(calculate_loan_schedule))
}

