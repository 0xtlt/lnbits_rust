use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateInvoiceResult {
    pub payment_hash: String,
    pub payment_request: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayInvoiceResult {
    pub payment_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateInvoiceParams {
    pub amount: i64,
    pub unit: String,
    pub memo: Option<String>,
    pub expiry: Option<i64>,
    pub webhook: Option<String>,
    pub internal: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecodedInvoice {
    pub payment_hash: String,
    pub amount_msat: i64,
    pub description: String,
    pub description_hash: Option<String>,
    pub payee: String,
    pub date: i64,
    pub expiry: i64,
    pub secret: String,
    pub route_hints: Vec<String>,
    pub min_final_cltv_expiry: i64,
}

impl crate::LNBitsClient {
    /// Create an invoice
    ///
    /// # Example
    /// ```rust
    /// use lnbits_rust::*;
    /// use lnbits_rust::api::invoice::CreateInvoiceParams;
    ///
    /// async fn test_create_invoice() {
    ///     let client = LNBitsClient::new(
    ///        env!("WALLET_1_ID"),
    ///        env!("WALLET_1_ADMIN_KEY"),
    ///        env!("WALLET_1_INVOICE_READ_KEY"),
    ///        env!("LNBITS_URL"),
    ///        Some(env!("LNBITS_TOR_SOCKET")),
    ///     ).unwrap();
    ///
    ///     let params = CreateInvoiceParams {
    ///        amount: 1,
    ///        unit: "sat".to_string(),
    ///        expiry: None,
    ///        memo: None,
    ///        webhook: None,
    ///        internal: None,
    ///     };
    ///
    ///     let invoice_result = client.create_invoice(&params).await.unwrap();
    /// }
    ///
    /// tokio_test::block_on(async {
    ///     test_create_invoice().await;
    /// })
    /// ```
    pub async fn create_invoice(
        &self,
        params: &CreateInvoiceParams,
    ) -> Result<CreateInvoiceResult, crate::LNBitsError> {
        // Add out: true to the params
        let params = serde_json::json!({
            "out": false,
            "amount": params.amount,
            "unit": params.unit,
            "memo": params.memo,
            "webhook": params.webhook,
            "internal": params.internal,
            "expiry": params.expiry,
        });

        let body = self
            .make_post(
                "api/v1/payments",
                crate::api::LNBitsRequestKey::InvoiceRead,
                &serde_json::to_string(&params)?,
            )
            .await?;

        let invoice_result: CreateInvoiceResult = serde_json::from_str(&body)?;
        Ok(invoice_result)
    }

    /// Pay an invoice
    ///
    /// # Example
    /// ```rust
    /// use lnbits_rust::*;
    /// use lnbits_rust::api::invoice::CreateInvoiceParams;
    ///
    /// async fn test_pay_invoice() {
    ///     let client_alice = LNBitsClient::new(
    ///        env!("WALLET_1_ID"),
    ///        env!("WALLET_1_ADMIN_KEY"),
    ///        env!("WALLET_1_INVOICE_READ_KEY"),
    ///        env!("LNBITS_URL"),
    ///        Some(env!("LNBITS_TOR_SOCKET")),
    ///     ).unwrap();
    ///
    ///     let client_bob = LNBitsClient::new(
    ///        env!("WALLET_2_ID"),
    ///        env!("WALLET_2_ADMIN_KEY"),
    ///        env!("WALLET_2_INVOICE_READ_KEY"),
    ///        env!("LNBITS_URL"),
    ///        Some(env!("LNBITS_TOR_SOCKET")),
    ///     ).unwrap();
    ///
    ///    let params = CreateInvoiceParams {
    ///       amount: 1,
    ///       unit: "sat".to_string(),
    ///       memo: None,
    ///       expiry: None,
    ///       webhook: None,
    ///       internal: None,
    ///    };
    ///
    ///    let invoice_result = client_alice.create_invoice(&params).await.unwrap();
    ///
    ///    let pay_result = client_bob.pay_invoice(&invoice_result.payment_request).await.unwrap();
    ///
    ///    assert_eq!(pay_result.payment_hash, invoice_result.payment_hash);
    ///
    ///    // Wait for the payment to be processed
    ///    while client_alice.is_invoice_paid(&invoice_result.payment_hash).await.unwrap() == false {
    ///         tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    ///     }
    ///
    ///    // Now send back the payment
    ///    let pay_back = client_bob.create_invoice(&params).await.unwrap();
    ///
    ///   let pay_result = client_alice.pay_invoice(&pay_back.payment_request).await.unwrap();
    ///
    ///  assert_eq!(pay_result.payment_hash, pay_back.payment_hash);
    ///
    /// // Wait for the payment to be processed
    /// while client_bob.is_invoice_paid(&pay_back.payment_hash).await.unwrap() == false {
    ///    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    /// }
    ///}
    ///
    /// tokio_test::block_on(async {
    ///    test_pay_invoice().await;
    /// });
    /// ```
    pub async fn pay_invoice(&self, bolt11: &str) -> Result<PayInvoiceResult, crate::LNBitsError> {
        let body = self
            .make_post(
                "api/v1/payments",
                crate::api::LNBitsRequestKey::Admin,
                &serde_json::to_string(&serde_json::json!({ "out": true, "bolt11": bolt11 }))?,
            )
            .await?;

        let invoice_result: PayInvoiceResult = serde_json::from_str(&body)?;
        Ok(invoice_result)
    }

    pub async fn decode_invoice(
        &self,
        invoice: &str,
    ) -> Result<DecodedInvoice, crate::LNBitsError> {
        let body = self
            .make_post(
                "api/v1/payments/decode",
                crate::api::LNBitsRequestKey::Admin,
                &serde_json::to_string(&serde_json::json!({ "data": invoice }))?,
            )
            .await?;

        let invoice_result: DecodedInvoice = serde_json::from_str(&body)?;
        Ok(invoice_result)
    }

    pub async fn is_invoice_paid(&self, payment_hash: &str) -> Result<bool, crate::LNBitsError> {
        let body = self
            .make_get(
                &format!("api/v1/payments/{payment_hash}"),
                crate::api::LNBitsRequestKey::Admin,
            )
            .await?;

        let invoice_result: serde_json::Value = serde_json::from_str(&body)?;
        Ok(invoice_result["paid"].as_bool().unwrap_or(false))
    }
}
