use crate::calculations::traits::LoanStrategy;
use crate::models::Installment;

pub struct ReducingInterest;
impl LoanStrategy for ReducingInterest {
    fn calculate(&self, amount: f64, rate: f64, periods: i32, date: String) -> Vec<Installment> {
        let mut schedule = Vec::new();
        let monthly_rate = rate / 12.0 / 100.0;
        let principal_per_month = amount / periods as f64;

        for i in 1..=periods {
            let remaining_principal = amount - (principal_per_month * (i - 1) as f64);
            let interest = remaining_principal * monthly_rate;
            schedule.push(Installment::new(i, principal_per_month, interest, &date));
        }
        schedule
    }
}
