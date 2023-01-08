use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WalletDetails {
    /// Documented as "id" in the API docs, but is actually not sent
    pub id: Option<String>,
    pub name: String,
    pub balance: i64,
}

impl crate::LNBitsClient {
    pub async fn get_wallet_details(&self) -> Result<WalletDetails, crate::LNBitsError> {
        let body = self
            .make_get("api/v1/wallet", crate::api::LNBitsRequestKey::InvoiceRead)
            .await?;
        let wallet_details: WalletDetails = serde_json::from_str(&body)?;

        Ok(wallet_details)
    }
}
