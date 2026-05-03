use ext_php_rs::convert::IntoZval;
use ext_php_rs::types::Zval;
use std::collections::HashMap;

pub struct Installment {
    pub no: i32,
    pub principal: f64,
    pub interest: f64,
    pub date: String,
}

impl Installment {
    pub fn new(no: i32, principal: f64, interest: f64, date: &str) -> Self {
        Self { no, principal, interest, date: date.to_string() }
    }

    pub fn to_zval(&self) -> Zval {
        let mut map = HashMap::new();
        map.insert("installment_no", self.no.to_string());
        map.insert("principal", format!("{:.2}", self.principal));
        map.insert("interest", format!("{:.2}", self.interest));
        map.insert("total", format!("{:.2}", self.principal + self.interest));
        map.insert("date", self.date.clone());
        
        map.into_zval(false).unwrap()
    }
}
