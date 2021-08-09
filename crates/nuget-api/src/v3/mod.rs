pub use ruget_common::surf::Body;
use ruget_common::{
    semver::Version,
    serde::{Deserialize, Serialize},
    serde_json,
    surf::{self, Client, Url},
};

use crate::errors::NuGetApiError;

pub use metadata::*;
pub use search::*;

mod metadata;
mod push;
mod relist;
mod search;
mod unlist;

#[derive(Debug)]
pub struct NuGetClient {
    client: Client,
    pub key: Option<String>,
    pub endpoints: NuGetEndpoints,
}

#[derive(Debug, Serialize)]
pub struct NuGetEndpoints {
    pub package_content: Option<Url>,
    pub publish: Option<Url>,
    pub metadata: Option<Url>,
    pub search: Option<Url>,
    pub catalog: Option<Url>,
    pub signatures: Option<Url>,
    pub autocomplete: Option<Url>,
    pub symbol_publish: Option<Url>,
}

impl NuGetEndpoints {
    fn find_endpoint(resources: &[IndexResource], restype: &str) -> Option<Url> {
        resources
            .iter()
            .find(|res| res.restype == restype)
            .map(|res| res.id.clone())
    }

    fn from_resources(resources: Vec<IndexResource>) -> Self {
        NuGetEndpoints {
            package_content: Self::find_endpoint(&resources, "PackageBaseAddress/3.0.0"),
            publish: Self::find_endpoint(&resources, "PackagePublish/2.0.0"),
            metadata: Self::find_endpoint(&resources, "RegistrationsBaseUrl/3.6.0"),
            search: Self::find_endpoint(&resources, "SearchQueryService/3.5.0"),
            catalog: Self::find_endpoint(&resources, "Catalog/3.0.0"),
            signatures: Self::find_endpoint(&resources, "RepositorySignatures/5.0.0"),
            autocomplete: Self::find_endpoint(&resources, "SearchAutocompleteService/3.5.0"),
            symbol_publish: Self::find_endpoint(&resources, "SymbolPackagePublish/4.9.0"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Index {
    version: Version,
    resources: Vec<IndexResource>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IndexResource {
    #[serde(rename = "@id")]
    id: Url,
    #[serde(rename = "@type")]
    restype: String,
    comment: Option<String>,
}

impl NuGetClient {
    pub async fn from_source(source: impl AsRef<str>) -> Result<Self, NuGetApiError> {
        let client = Client::new();
        let url: Url = source
            .as_ref()
            .parse()
            .map_err(|_| NuGetApiError::InvalidSource(source.as_ref().into()))?;
        let req = surf::get(&url);
        let Index { resources, .. } = serde_json::from_slice(
            &client
                .send(req)
                .await
                .map_err(|e| NuGetApiError::SurfError(e, url.clone().into()))?
                .body_bytes()
                .await
                .map_err(|e| NuGetApiError::SurfError(e, url.clone().into()))?,
        )
        .map_err(|_| NuGetApiError::InvalidSource(source.as_ref().into()))?;
        Ok(NuGetClient {
            client,
            key: None,
            endpoints: NuGetEndpoints::from_resources(resources),
        })
    }

    pub fn get_key(&self) -> Result<String, NuGetApiError> {
        self.key.clone().ok_or(NuGetApiError::NeedsApiKey)
    }

    pub fn with_key(mut self, key: Option<impl AsRef<str>>) -> Self {
        self.key = key.map(|k| k.as_ref().into());
        self
    }
}