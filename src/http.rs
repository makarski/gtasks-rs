use std::pin::Pin;
use std::result::Result as StdResult;

use anyhow::{anyhow, Error as AnyError};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Request;
use reqwest_middleware::{ClientBuilder, Middleware, Next, Result as MidWareResult};
use task_local_extensions::Extensions;

use crate::errors::Result;

pub trait TokenProvider: Fn() -> StdResult<String, AnyError> + Send + Sync + 'static {}

impl<ClosureFunc> TokenProvider for ClosureFunc where
    ClosureFunc: Fn() -> StdResult<String, AnyError> + Send + Sync + 'static
{
}

pub(crate) use reqwest_middleware::ClientWithMiddleware as HttpClient;

pub(crate) struct AuthMiddleware<TP>(Pin<Box<TP>>)
where
    TP: Fn() -> StdResult<String, AnyError> + Send + Sync + 'static;

impl<TP> AuthMiddleware<TP>
where
    TP: TokenProvider,
{
    pub(crate) fn new(token_provider: TP) -> Self {
        AuthMiddleware(Box::pin(token_provider))
    }

    pub(crate) fn init_http_client(self) -> Result<HttpClient> {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        let http_client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(ClientBuilder::new(http_client).with(self).build())
    }
}

#[async_trait::async_trait]
impl<TP> Middleware for AuthMiddleware<TP>
where
    TP: TokenProvider,
{
    async fn handle(
        &self,
        mut req: Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> MidWareResult<reqwest::Response> {
        let token = (self.0)()?;
        req.headers_mut().insert(
            AUTHORIZATION,
            HeaderValue::from_bytes(token.as_bytes()).map_err(|err| anyhow!(err))?,
        );
        next.run(req, extensions).await
    }
}
