mod binance;
mod kraken;
mod huobi;

pub use binance::get_binance_price;
pub use kraken::get_kraken_price;
pub use huobi::get_huobi_price;
