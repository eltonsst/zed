use std::{any::Any, path::PathBuf};

use anyhow::Result;

use async_trait::async_trait;
use language::{LanguageServerName, LspAdapterDelegate};
use lsp::LanguageServerBinary;
use util::github::{latest_github_release, GitHubLspBinaryVersion};


#[derive(Copy, Clone)]
pub struct ScalaLspAdapter;

#[async_trait]
impl super::LspAdapter for ScalaLspAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName("metals".into())
    }

    fn short_name(&self) -> &'static str {
        "metals"
    }

    async fn fetch_latest_server_version(
        &self,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        let release = latest_github_release("scalameta/metals", false, delegate.http_client()).await?;
        Ok(Box::new(GitHubLspBinaryVersion {
            name: release.name,
            url: release.tarball_url,
        }))
    }

    async fn fetch_server_binary(
        &self,
        version: Box<dyn 'static + Send + Any>,
        container_dir: PathBuf,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        let version = version.downcast::<Option<String>>().unwrap();
        let this = *self;

        if let Some(version) = *version {
            let binary_path = container_dir.join(&format!("gopls_{version}"));

        } else if true {
            todo!()
        } else {
            todo!()
        }
        todo!()
    }

    async fn cached_server_binary(
        &self,
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        todo!()
    }

    async fn installation_test_binary(
        &self,
        container_dir: PathBuf,
    ) -> Option<LanguageServerBinary> {
        todo!()
    }
}
