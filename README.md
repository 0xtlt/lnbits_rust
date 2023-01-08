# lnbits_rust

[![crates.io](https://img.shields.io/crates/v/lnbits_rust.svg)](https://crates.io/crates/lnbits_rust)
[![Documentation](https://docs.rs/lnbits_rust/badge.svg)](https://docs.rs/lnbits_rust)
[![MIT/Apache-2 licensed](https://img.shields.io/crates/l/lnbits_rust.svg)](./LICENSE.txt)
[![CI](https://github.com/0xtlt/lnbits_rust/actions/workflows/ci.yml/badge.svg)](https://github.com/0xtlt/lnbits_rust/actions/workflows/ci.yml)
[![Issues](https://img.shields.io/github/issues/0xtlt/lnbits_rust)](https://img.shields.io/github/issues/0xtlt/lnbits_rust)

An ergonomic, [LNBits](https://lnbits.com/) API Client for Rust.

- [Changelog](CHANGELOG.md)

## Example

```toml
[dependencies]
tokio = { version = "1.0", features = ["full"] }
lnbits_rust = "0.1"
```

And then the code:

```rust,norun
use lnbits_rust::{api::invoice::CreateInvoiceParams, LNBitsClient};

#[tokio::main]
async fn main() {
    let client = LNBitsClient::new(
        "wallet id",
        "admin_key",
        "invoice_read_key",
        "http://lnbits_url",
        None,
    )
    .unwrap();

    // OR with tor

    let client = LNBitsClient::new(
        "wallet id",
        "admin_key",
        "invoice_read_key",
        "http://lnbits_url.onion",
        Some("socks5h://127.0.0.1:9050"),
    )
    .unwrap();

    let wallet_details = client.get_wallet_details().await.unwrap();

    println!("wallet_details: {:?}", wallet_details);

    let invoice = client
        .create_invoice(&CreateInvoiceParams {
            amount: 1,
            unit: "sat".to_string(),
            memo: None,
            webhook: None,
            internal: None,
        })
        .await
        .unwrap();

    println!("invoice: {:?}", i);

    println!(
        "decoded invoice: {:?}",
        client.decode_invoice(&i.payment_request).await.unwrap()
    );

    while !client.is_invoice_paid(&i.payment_hash).await.unwrap() {
        println!("Waiting for payment");
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }

    println!("Payment received");
}

```

## Key features

- [x] Create invoices
- [x] Decode invoices
- [x] Pay invoices
- [x] Get wallet details
- [x] Tor support

## License

Licensed under MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
