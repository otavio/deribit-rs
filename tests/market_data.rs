use anyhow::Error;
use chrono::{Duration, Utc};
use deribit::{
    models::{
        market_data::GetHistoricalVolatilityRequest, Currency, GetBookSummaryByCurrencyRequest,
        GetFundingRateValueRequest, GetIndexPriceRequest, GetInstrumentsRequest,
        GetOrderBookRequest,
    },
    DeribitBuilder,
};
use fehler::{throw, throws};
use tokio::runtime::Runtime;

#[test]
#[throws(Error)]
fn get_index_price() {
    let _ = env_logger::try_init();

    let drb = DeribitBuilder::default().testnet(true).build().unwrap();
    let rt = Runtime::new().expect("cannot create tokio runtime");

    let fut = async move {
        let (mut client, _) = drb.connect().await?;
        let req = GetIndexPriceRequest::new("btc_usd".to_string());
        let _ = client.call(req).await?.await?;
        let req = GetIndexPriceRequest::new("eth_usd".to_string());
        let _ = client.call(req).await?.await?;

        Ok::<_, Error>(())
    };
    let resp = rt.block_on(fut);
    if let Err(err) = resp {
        println!("{:?}", err);
        throw!(err);
    }
}

#[test]
#[throws(Error)]
fn get_instruments() {
    let _ = env_logger::try_init();

    let drb = DeribitBuilder::default().build().unwrap();
    let rt = Runtime::new().expect("cannot create tokio runtime");

    let fut = async move {
        let (mut client, _) = drb.connect().await?;
        let req = GetInstrumentsRequest::default();
        let _ = client.call(req).await?.await?;
        let req = GetInstrumentsRequest::new(Currency::BTC);
        let _ = client.call(req).await?.await?;
        let req = GetInstrumentsRequest::expired(Currency::ETH);
        let _ = client.call(req).await?.await?;

        Ok::<_, Error>(())
    };
    let resp = rt.block_on(fut);
    if let Err(err) = resp {
        println!("{:?}", err);
        throw!(err);
    }
}

#[test]
#[throws(Error)]
fn get_book_summary_by_currency() {
    let _ = env_logger::try_init();
    let drb = DeribitBuilder::default().build().unwrap();
    let rt = Runtime::new().expect("cannot create tokio runtime");

    let fut = async move {
        let (mut client, _) = drb.connect().await?;
        let req = GetBookSummaryByCurrencyRequest::futures(Currency::BTC);
        let _ = client.call(req).await?.await?;
        let req = GetBookSummaryByCurrencyRequest::all(Currency::ETH);
        let _ = client.call(req).await?.await?;
        let req = GetBookSummaryByCurrencyRequest::options(Currency::ETH);
        let _ = client.call(req).await?.await?;

        Ok::<_, Error>(())
    };
    let resp = rt.block_on(fut);
    if let Err(err) = resp {
        println!("{:?}", err);
        throw!(err);
    }
}

#[test]
#[throws(Error)]
fn get_funding_rate_value() {
    let _ = env_logger::try_init();

    let drb = DeribitBuilder::default().build().unwrap();
    let rt = Runtime::new().expect("cannot create tokio runtime");

    let fut = async move {
        let (mut client, _) = drb.connect().await?;
        let req = GetFundingRateValueRequest::new(
            "BTC-PERPETUAL",
            Utc::now() - Duration::seconds(60),
            Utc::now(),
        );
        let _ = client.call(req).await?.await?;

        Ok::<_, Error>(())
    };
    let resp = rt.block_on(fut);
    if let Err(err) = resp {
        println!("{:?}", err);
        throw!(err);
    }
}

#[test]
#[throws(Error)]
fn get_order_book() {
    let _ = env_logger::try_init();

    let drb = DeribitBuilder::default().build().unwrap();
    let rt = Runtime::new().expect("cannot create tokio runtime");

    let fut = async move {
        let (mut client, _) = drb.connect().await?;
        let req = GetOrderBookRequest::new("BTC-PERPETUAL");
        let ret = client.call(req).await?.await?;
        println!("{:#?}", ret);

        Ok::<_, Error>(())
    };
    let resp = rt.block_on(fut);
    if let Err(err) = resp {
        println!("{:?}", err);
        throw!(err);
    }
}

#[test]
#[throws(Error)]
fn get_historical_volatility() {
    let _ = env_logger::try_init();

    let drb = DeribitBuilder::default().build().unwrap();
    let rt = Runtime::new().expect("cannot create tokio runtime");

    let fut = async move {
        let (mut client, _) = drb.connect().await?;
        let req = GetHistoricalVolatilityRequest::new(Currency::BTC);
        let ret = client.call(req).await?.await?;
        println!("{:#?}", ret);

        let first_vol = ret.first().unwrap();
        assert!(first_vol.0 > 0 && first_vol.1 > 0_f64);

        Ok::<_, Error>(())
    };

    let resp = rt.block_on(fut);
    if let Err(err) = resp {
        println!("{:?}", err);
        throw!(err);
    }
}
