pub mod invoice;
pub mod wallet;

pub enum LNBitsRequestKey {
    Admin,
    InvoiceRead,
}

impl crate::LNBitsClient {
    pub async fn make_get(
        &self,
        endpoint: &str,
        key: LNBitsRequestKey,
    ) -> Result<String, crate::LNBitsError> {
        let url = self.lnbits_url.join(endpoint)?;
        let response = self
            .reqwest_client
            .get(url)
            .header("X-Api-Key", {
                match key {
                    LNBitsRequestKey::Admin => self.admin_key.clone(),
                    LNBitsRequestKey::InvoiceRead => self.invoice_read_key.clone(),
                }
            })
            .send()
            .await?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(crate::LNBitsError::NotFound);
        }

        let body = response.text().await?;

        Ok(body)
    }

    pub async fn make_post(
        &self,
        endpoint: &str,
        key: LNBitsRequestKey,
        body: &str,
    ) -> Result<String, crate::LNBitsError> {
        let url = self.lnbits_url.join(endpoint)?;
        let response = self
            .reqwest_client
            .post(url)
            .header("X-Api-Key", {
                match key {
                    LNBitsRequestKey::Admin => self.admin_key.clone(),
                    LNBitsRequestKey::InvoiceRead => self.invoice_read_key.clone(),
                }
            })
            .body(body.to_string())
            .send()
            .await?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(crate::LNBitsError::NotFound);
        }

        if response.status() == reqwest::StatusCode::UNAUTHORIZED {
            return Err(crate::LNBitsError::Unauthorized);
        }

        let body = response.text().await?;

        Ok(body)
    }
}
