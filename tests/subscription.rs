#![feature(async_await, await_macro)]

use deribit::models::subscription::{PrivateSubscribeRequest, PublicSubscribeRequest};
use deribit::models::{AuthRequest, BuyRequest, CancelRequest};
use deribit::{Deribit, DeribitBuilder};
use failure::{Error, Fallible};
use fluid::prelude::*;
use futures::{FutureExt, StreamExt, TryFutureExt};
use std::env::var;
use tokio::runtime::Runtime;


#[derive(Default)]
struct SubscriptionTest;


// The tests:
#[session]
impl SubscriptionTest {
    #[fact]
    fn ticker(self) -> Fallible<()> {
        dotenv::dotenv()?;

        let drb = Deribit::new();
        let mut rt = Runtime::new()?;

        let fut = async {
            let (mut client, subscription) = await!(drb.connect()).unwrap();

            let req = PublicSubscribeRequest {
                channels: vec!["ticker.BTC-28JUN19.100ms".into()],
            };

            let _ = await!(client.call(req)).unwrap();

            let v = await!(subscription.take(5).collect::<Vec<_>>());
            Ok::<_, Error>(v)
        };

        let fut = fut.boxed().compat();
        let v = rt.block_on(fut)?;
        v.len().should().be_equal_to(5);
        Ok(())
    }

    #[fact]
    fn orderbook(self) -> Fallible<()> {
        dotenv::dotenv()?;

        let drb = Deribit::new();
        let mut rt = Runtime::new()?;

        let fut = async {
            let (mut client, subscription) = await!(drb.connect()).unwrap();

            let req = PublicSubscribeRequest {
                channels: vec!["book.BTC-PERPETUAL.raw".into()],
            };

            let _ = await!(client.call(req)).unwrap();

            let v = await!(subscription.take(5).collect::<Vec<_>>());
            Ok::<_, Error>(v)
        };

        let fut = fut.boxed().compat();
        let v = rt.block_on(fut)?;
        v.len().should().be_equal_to(5);
        Ok(())
    }

    #[fact]
    fn trades(self) -> Fallible<()> {
        dotenv::dotenv()?;

        let drb = Deribit::new();
        let mut rt = Runtime::new()?;

        let fut = async {
            let (mut client, subscription) = await!(drb.connect()).unwrap();

            let req = PublicSubscribeRequest {
                channels: vec![
                    "trades.BTC-PERPETUAL.raw".into(),
                    "trades.ETH-PERPETUAL.raw".into(),
                ],
            };

            let _ = await!(client.call(req)).unwrap();

            let v = await!(subscription.take(5).collect::<Vec<_>>());
            Ok::<_, Error>(v)
        };

        let fut = fut.boxed().compat();
        let v = rt.block_on(fut)?;
        v.len().should().be_equal_to(5);
        Ok(())
    }

    #[fact]
    fn user_orders(self) -> Fallible<()> {
        dotenv::dotenv()?;

        let key = var("DERIBIT_KEY").unwrap();
        let secret = var("DERIBIT_SECRET").unwrap();

        let drb = DeribitBuilder::default().testnet(true).build().unwrap();
        let mut rt = Runtime::new()?;

        let fut = async move {
            let (mut client, subscription) = await!(drb.connect()).unwrap();

            let _ = await!(client.call(AuthRequest::credential_auth(&key, &secret)))?;

            let req = PrivateSubscribeRequest {
                channels: vec!["user.orders.BTC-PERPETUAL.raw".into()],
            };
            let _ = await!(client.call(req)).unwrap();

            let req = BuyRequest::limit("BTC-PERPETUAL", 10f64, 10f64);

            let resp = await!(await!(client.call(req))?)?;
            let id = resp.0.order.order_id;

            let v = await!(subscription.take(1).collect::<Vec<_>>());
            let req = CancelRequest::new(&id);
            let resp = await!(await!(client.call(req))?)?;
            id.should().be_equal_to(resp.order_id);
            Ok::<_, Error>(v)
        };

        let fut = fut.boxed().compat();
        let _ = rt.block_on(fut)?;
        Ok(())
    }
}