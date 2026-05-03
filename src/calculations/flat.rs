use crate::calculations::traits::LoanStrategy;
use crate::models::Installment;

pub struct FlatInterest;
impl LoanStrategy for FlatInterest {
    fn calculate(&self, amount: f64, rate: f64, periods: i32, date: String) -> Vec<Installment> {
        let mut schedule = Vec::new();
        let monthly_rate = rate / 12.0 / 100.0;
        let principal = amount / periods as f64;
        let interest = amount * monthly_rate;

        for i in 1..=periods {
            schedule.push(Installment::new(i, principal, interest, &date));
        }
        schedule
    }
}
