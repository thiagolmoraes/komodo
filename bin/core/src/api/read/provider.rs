use anyhow::{Context, anyhow};
use database::mongo_indexed::{Document, doc};
use database::mungos::{
  by_id::find_one_by_id, find::find_collect,
  mongodb::options::FindOptions,
};
use komodo_client::api::read::*;
use mogh_resolver::Resolve;

use crate::state::db_client;

use super::ReadArgs;

fn mask_token(token: String) -> String {
  if token.is_empty() { token } else { "***".to_string() }
}

impl Resolve<ReadArgs> for GetGitProviderAccount {
  async fn resolve(
    self,
    ReadArgs { user }: &ReadArgs,
  ) -> mogh_error::Result<GetGitProviderAccountResponse> {
    if !user.admin {
      return Err(
        anyhow!("Only admins can read git provider accounts").into(),
      );
    }
    let mut res = find_one_by_id(&db_client().git_accounts, &self.id)
      .await
      .context("failed to query db for git provider accounts")?
      .context(
        "did not find git provider account with the given id",
      )?;
    res.token = mask_token(res.token);
    Ok(res)
  }
}

impl Resolve<ReadArgs> for ListGitProviderAccounts {
  async fn resolve(
    self,
    ReadArgs { user }: &ReadArgs,
  ) -> mogh_error::Result<ListGitProviderAccountsResponse> {
    if !user.admin {
      return Err(
        anyhow!("Only admins can read git provider accounts").into(),
      );
    }
    let mut filter = Document::new();
    if let Some(domain) = self.domain {
      filter.insert("domain", domain);
    }
    if let Some(username) = self.username {
      filter.insert("username", username);
    }
    let mut res = find_collect(
      &db_client().git_accounts,
      filter,
      FindOptions::builder()
        .sort(doc! { "domain": 1, "username": 1 })
        .build(),
    )
    .await
    .context("failed to query db for git provider accounts")?;
    for account in &mut res {
      account.token = mask_token(account.token.clone());
    }
    Ok(res)
  }
}

impl Resolve<ReadArgs> for GetDockerRegistryAccount {
  async fn resolve(
    self,
    ReadArgs { user }: &ReadArgs,
  ) -> mogh_error::Result<GetDockerRegistryAccountResponse> {
    if !user.admin {
      return Err(
        anyhow!("Only admins can read docker registry accounts")
          .into(),
      );
    }
    let mut res =
      find_one_by_id(&db_client().registry_accounts, &self.id)
        .await
        .context("failed to query db for docker registry accounts")?
        .context(
          "did not find docker registry account with the given id",
        )?;
    res.token = mask_token(res.token);
    Ok(res)
  }
}

impl Resolve<ReadArgs> for ListDockerRegistryAccounts {
  async fn resolve(
    self,
    ReadArgs { user }: &ReadArgs,
  ) -> mogh_error::Result<ListDockerRegistryAccountsResponse> {
    if !user.admin {
      return Err(
        anyhow!("Only admins can read docker registry accounts")
          .into(),
      );
    }
    let mut filter = Document::new();
    if let Some(domain) = self.domain {
      filter.insert("domain", domain);
    }
    if let Some(username) = self.username {
      filter.insert("username", username);
    }
    let mut res = find_collect(
      &db_client().registry_accounts,
      filter,
      FindOptions::builder()
        .sort(doc! { "domain": 1, "username": 1 })
        .build(),
    )
    .await
    .context("failed to query db for docker registry accounts")?;
    for account in &mut res {
      account.token = mask_token(account.token.clone());
    }
    Ok(res)
  }
}
