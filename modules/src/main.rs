
use chrono::{NaiveDate, Utc};
use crate::exchange::wallet::Balance;

fn main() {
    let currency = exchange::Currency::USD;
    let balance: Balance = exchange::wallet::get_balance(&currency).unwrap();
    let current_price = exchange::market::get_current_btc_price();

    const START_DATE: Option<NaiveDate> = NaiveDate::from_ymd_opt(2009, 1, 9);
    let today = Utc::now().naive_utc();
    let duration = today.signed_duration_since(START_DATE.expect("Not what we expect").into());
    let days_from_genisis = duration.num_days();
    let model_price: f64 = power_law::model::calculate_value_today(&days_from_genisis)
        .try_into().unwrap();

    println!("the model price is {}, current price {}, balance: {:?}{:?}", model_price, current_price, balance.amount, balance.currency);

}

pub mod power_law {
    pub mod dca {
        pub fn calculate_multiplier() -> f64 {
            println!("calculate the multiplier value");
            0.85
        }
    }

    pub mod model {
        pub fn calculate_value_today(days_from_genisis: &i64) -> f64 {
            println!("Use days from genesis in power law formula for predicted price {}", days_from_genisis);
            48_000.0000
        }

    }
}

pub mod exchange {
    #[derive(Debug)]
    pub enum Currency {
        USD,
    }

    pub mod market {
        pub fn get_current_btc_price() -> f64 {
            println!("Get price quote from exchange");
            61_000.0000
        }
    }

    pub mod wallet {

        use crate::exchange;

        #[derive(Debug)]
        pub struct Balance<'a> {
            pub currency: &'a exchange::Currency,
            pub amount: f64,
        }

        impl Balance<'_> {
            fn new(currency: &exchange::Currency, amount: f64) -> Balance {
                Balance { currency, amount}
            }
        }

        pub fn get_balance(currency: &exchange::Currency) -> Option<Balance> {
            println!("Query the balance from the exchange via API");
            let amount: f64 = 10_321.567;
            Some(Balance::new(currency, amount))
        }
    }

}