use serde::Deserialize;
use std::{collections::HashMap, vec::Vec};

#[derive(Deserialize, Debug)]
pub struct BaseResponse<DetailType> {
    pub error: Vec<String>,
    pub result: DetailType,
}

#[derive(Deserialize, Debug)]
pub struct CriticalErrorResponse {
    pub error: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct ServerTimeType {
    pub unixtime: i64,
    pub rfc1123: String,
}

#[derive(Debug, Deserialize)]
pub struct XBTUSDRequest<TradingPair> {
    pub XXBTZUSD: TradingPair,
}

#[derive(Debug, Deserialize)]
pub struct Fee(u32, f32);

#[derive(Debug, Deserialize)]
pub struct TradingPairAll {
    pub aclass_base: String,
    pub aclass_quote: String,
    pub altname: String,
    pub base: String,
    pub cost_decimals: u32,
    pub fee_volume_currency: String,
    pub fees: Vec<Fee>,
    pub fees_maker: Vec<Fee>,
    pub leverage_buy: Vec<u8>,
    pub leverage_sell: Vec<u8>,
    pub lot: String,
    pub lot_decimals: u8,
    pub lot_multiplier: u8,
    pub margin_call: u8,
    pub margin_stop: u8,
    pub ordermin: String, // Why not f32?
    pub pair_decimals: u8,
    pub quote: String,
    pub wsname: String,
}

#[derive(Debug, Deserialize)]
pub struct TradingPairFees {
    pub fee_volume_currency: String,
    pub fees: Vec<Fee>,
    pub fees_maker: Vec<Fee>,
}

#[derive(Debug, Deserialize)]
pub struct TradingPairLeverage {
    pub leverage_buy: Vec<u8>,
    pub leverage_sell: Vec<u8>,
}

#[derive(Debug, Deserialize)]
pub struct TradingPairMargin {
    pub margin_call: u8,
    pub margin_level: u8, // BUG?: Shouldn't this be named margin_stop?
                          // pub margin_stop: u8,
}

// Note: Some values are left commented to point thaat they may be placeholders for more detailed PartialEq implementations.
#[derive(Debug)]
pub struct TradingPairRuler {
    pub aclass_base: String,
    pub aclass_quote: String,
    pub altname: String,
    pub base: String,
    // pub cost_decimals: u32,
    pub fee_volume_currency: String,
    // pub fees: Vec<Fee>,
    // pub fees_maker: Vec<Fee>,
    // pub leverage_buy: Vec<u8>,
    // pub leverage_sell: Vec<u8>,
    pub lot: String,
    // pub lot_decimals: u8,
    // pub lot_multiplier: u8,
    // pub margin_call: u8,
    // pub margin_stop: u8,
    // pub ordermin: String, // Why not f32?
    // pub pair_decimals: u8,
    pub quote: String,
    pub wsname: String,
}

impl PartialEq<TradingPairRuler> for TradingPairAll {
    fn eq(&self, other: &TradingPairRuler) -> bool {
        // INFO: The comparision is not done by iterator or default implementation to have placeholder for different demandings.
        // For example it could be possible to add '(self.pair_decinals > 0 && self.pair_decimals < 10)'.
        // That comparisions should be implemented according to some output model and server state.
        // That level of comparisions should also be considered as input for component tests, where tests can have more influence on server internal state.
        self.aclass_base == other.aclass_base
            && self.aclass_quote == other.aclass_quote
            && self.altname == other.altname
            && self.base == other.base
            && self.fee_volume_currency == other.fee_volume_currency
            && self.lot == other.lot
            && self.quote == other.quote
            && self.wsname == other.wsname
    }
}

// NOTE: Example of trading pair response
// {'error': [],
//  'result': {'XXBTZUSD': {'aclass_base': 'currency',
//                          'aclass_quote': 'currency',
//                          'altname': 'XBTUSD',
//                          'base': 'XXBT',
//                          'cost_decimals': 5,
//                          'fee_volume_currency': 'ZUSD',
//                          'fees': [[0, 0.26],
//                                   [50000, 0.24],
//                                   [100000, 0.22],
//                                   [250000, 0.2],
//                                   [500000, 0.18],
//                                   [1000000, 0.16],
//                                   [2500000, 0.14],
//                                   [5000000, 0.12],
//                                   [10000000, 0.1]],
//                          'fees_maker': [[0, 0.16],
//                                         [50000, 0.14],
//                                         [100000, 0.12],
//                                         [250000, 0.1],
//                                         [500000, 0.08],
//                                         [1000000, 0.06],
//                                         [2500000, 0.04],
//                                         [5000000, 0.02],
//                                         [10000000, 0.0]],
//                          'leverage_buy': [2, 3, 4, 5],
//                          'leverage_sell': [2, 3, 4, 5],
//                          'lot': 'unit',
//                          'lot_decimals': 8,
//                          'lot_multiplier': 1,
//                          'margin_call': 80,
//                          'margin_stop': 40,
//                          'ordermin': '0.0001',
//                          'pair_decimals': 1,
//                          'quote': 'ZUSD',
//                          'wsname': 'XBT/USD'}}}

#[derive(Debug, Deserialize)]
pub struct OrderDetails {
    pub cost: String,
    pub descr: HashMap<String, String>,
    pub expiretm: u32,
    pub fee: String,
    pub limitprice: String,
    pub misc: String,
    pub oflags: String,
    pub opentm: f64,
    pub price: String,
    pub refid: Option<String>,
    pub starttm: u32,
    pub status: String,
    pub stopprice: String,
    pub userref: u32,
    pub vol: String,
    pub vol_exec: String,
}

#[derive(Debug)]
pub struct OrderDetailsRuler {
    // pub cost: String,
    pub descr: HashMap<String, String>,
    // pub expiretm: u32,
    // pub fee: String,
    // pub limitprice: String,
    // pub misc: String,
    // pub oflags: String,
    // pub opentm: f64,
    // pub price: String,
    // pub refid: Option<String>,
    // pub starttm: u32,
    pub status: String,
    // pub stopprice: String,
    // pub userref: u32,
    pub vol: String,
    // pub vol_exec: String
}

// NOTE: Example of OpenOrders response
// {'error': [],
//  'result': {'open': {'O3TYXG-PZKBP-FVQGOK': {'cost': '0.000000',
//                                              'descr': {'close': '',
//                                                        'leverage': 'none',
//                                                        'order': 'sell '
//                                                                 '0.01100000 '
//                                                                 'ETHXBT @ '
//                                                                 'limit 0.50000',
//                                                        'ordertype': 'limit',
//                                                        'pair': 'ETHXBT',
//                                                        'price': '0.50000',
//                                                        'price2': '0',
//                                                        'type': 'sell'},
//                                              'expiretm': 0,
//                                              'fee': '0.000000',
//                                              'limitprice': '0.000000',
//                                              'misc': '',
//                                              'oflags': 'fciq',
//                                              'opentm': 1665312858.4293625,
//                                              'price': '0.000000',
//                                              'refid': None,
//                                              'starttm': 0,
//                                              'status': 'open',
//                                              'stopprice': '0.000000',
//                                              'userref': 0,
//                                              'vol': '0.01100000',
//                                              'vol_exec': '0.00000000'}}}}
