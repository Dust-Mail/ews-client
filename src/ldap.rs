use ldap3::LdapConnAsync;

use crate::error::Result;

pub struct Ldap {
    server: String,
}

impl Ldap {
    const DEFAULT_LDAP: &str = "LDAP://RootDSE";

    pub fn new<S: Into<String>>(server: Option<S>) -> Self {
        let server = match server {
            Some(server) => {
                format!("{}/RootDSE", server.into())
            }
            None => String::from(Self::DEFAULT_LDAP),
        };

        Self { server }
    }

    pub async fn get_scp_urls<D: AsRef<str>>(&self, domain: D) -> Result<Vec<String>> {
        let (conn, mut ldap) = LdapConnAsync::new(&self.server).await?;

        ldap3::drive!(conn);

        Ok(Vec::new())
    }
}

#[cfg(test)]
mod test {
    use super::Ldap;

    async fn test_ldap() {
        let ldap = Ldap::new::<String>(None);

        ldap.get_scp_urls("").await;
    }
}
