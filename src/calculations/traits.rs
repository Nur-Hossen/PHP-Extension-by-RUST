use crate::models::Installment;
pub trait LoanStrategy {
    fn calculate(&self, amount: f64, rate: f64, periods: i32, date: String) -> Vec<Installment>;
}
