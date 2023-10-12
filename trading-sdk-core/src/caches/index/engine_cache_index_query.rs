#[derive(Debug)]
pub struct EngineCacheQueryBuilder {
    pub base: Option<String>,
    pub quote: Option<String>,
    pub collateral: Option<String>,
    pub client: Option<String>,
    pub account: Option<String>,
}

impl EngineCacheQueryBuilder {
    pub fn new() -> Self {
        Self {
            base: None,
            quote: None,
            collateral: None,
            client: None,
            account: None,
        }
    }

    pub fn with_base(&mut self, base: &str) {
        self.base = Some(base.to_string());
    }

    pub fn with_quote(&mut self, quote: &str) {
        self.quote = Some(quote.to_string());
    }

    pub fn with_collateral(&mut self, collateral: &str) {
        self.collateral = Some(collateral.to_string());
    }
    pub fn with_client(&mut self, client_ident: &str) {
        self.client = Some(client_ident.to_string());
    }
    pub fn with_account(&mut self, account_ident: &str) {
        self.account = Some(account_ident.to_string());
    }

    pub fn filters_count(&self) -> usize {
        let mut count = 0;
        if self.base.is_some() {
            count += 1;
        }
        if self.quote.is_some() {
            count += 1;
        }
        if self.collateral.is_some() {
            count += 1;
        }
        if self.client.is_some() {
            count += 1;
        }
        if self.account.is_some() {
            count += 1;
        }
        count
    }
}
