pub mod api;

use reqwest::Url;

#[derive(Clone)]
/// The LNBitsClient struct
pub struct LNBitsClient {
    // wallet_id: String, // Can be used later
    admin_key: String,
    invoice_read_key: String,
    lnbits_url: Url,
    // tor_socket: Option<String>, // Can be used later
    reqwest_client: reqwest::Client,
}

#[derive(Debug, thiserror::Error)]
pub enum LNBitsError {
    #[error("reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("url error: {0}")]
    UrlError(#[from] url::ParseError),

    #[error("serde error: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("Not found")]
    NotFound,

    #[error("Unauthorized")]
    Unauthorized,
}

impl LNBitsClient {
    /// Create a new LNBitsClient
    /// # Arguments
    /// * `wallet_id` - The wallet id of the LNBits wallet
    /// * `admin_key` - The admin key of the LNBits wallet
    /// * `invoice_read_key` - The invoice read key of the LNBits wallet
    /// * `lnbits_url` - The url of the LNBits instance
    /// * `tor_socket` - If defined, the tor socket to use
    ///
    /// # Example
    /// ```
    /// use lnbits_rust::LNBitsClient;
    /// let client = LNBitsClient::new(
    ///    "wallet_id",
    ///    "admin_key",
    ///    "invoice_read_key",
    ///    "http://localhost:5000",
    ///    None,
    /// ).unwrap();
    ///
    /// let client_using_tor = LNBitsClient::new(
    ///    "wallet_id",
    ///    "admin_key",
    ///    "invoice_read_key",
    ///    "http://oooo.onion",
    ///    Some("socks5h://127.0.0.1:9050"),
    /// ).unwrap();
    /// ```
    pub fn new(
        // Can be used later
        _wallet_id: &str,
        admin_key: &str,
        invoice_read_key: &str,
        lnbits_url: &str,
        tor_socket: Option<&str>,
    ) -> Result<LNBitsClient, LNBitsError> {
        let lnbits_url = Url::parse(lnbits_url)?;

        let client = {
            if let Some(tor_socket) = tor_socket {
                let proxy = reqwest::Proxy::all(tor_socket).expect("tor proxy should be there");
                reqwest::Client::builder().proxy(proxy).build()?
            } else {
                reqwest::Client::builder().build()?
            }
        };

        Ok(LNBitsClient {
            // wallet_id,
            admin_key: admin_key.to_string(),
            invoice_read_key: invoice_read_key.to_string(),
            lnbits_url,
            // tor_socket,
            reqwest_client: client,
        })
    }
}
