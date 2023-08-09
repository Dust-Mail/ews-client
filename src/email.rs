use futures::future::join_all;

use log::warn;
use validator::validate_email;

use crate::{
    candidate::CandidateType,
    client::Client,
    config::Config,
    // ldap::Ldap,
    error::{Error, ErrorKind, Result},
    failed,
};

const INVALID_EMAIL_MESSAGE: &str = "The given email address is invalid";

fn domain_from_email<E: AsRef<str>>(email: E) -> Result<String> {
    if !validate_email(email.as_ref()) {
        failed!(ErrorKind::InvalidEmailAddress, "{}", INVALID_EMAIL_MESSAGE);
    };

    let mut email_split = email.as_ref().split('@');

    email_split.next();

    let domain = match email_split.next() {
        Some(domain) => domain,
        None => failed!(ErrorKind::InvalidEmailAddress, "{}", INVALID_EMAIL_MESSAGE),
    };

    Ok(domain.to_string())
}

pub async fn from_email<E: AsRef<str>, P: AsRef<str>, U: AsRef<str>>(
    email: E,
    password: Option<P>,
    username: Option<U>,
) -> Result<Config> {
    let domain = domain_from_email(email.as_ref())?;

    let candidates: Vec<String> = vec![
        // format!(
        //     "https://autodiscover.{}/autodiscover/autodiscover.{}",
        //     domain,
        //     CandidateType::SOAP
        // ),
        // format!(
        //     "https://{}/autodiscover/autodiscover.{}",
        //     domain,
        //     CandidateType::SOAP
        // ),
        format!(
            "https://autodiscover.{}/autodiscover/autodiscover.{}",
            domain,
            CandidateType::POX
        ),
        format!(
            "https://{}/autodiscover/autodiscover.{}",
            domain,
            CandidateType::POX
        ),
    ];

    // let ldap = Ldap::new::<String>(None);

    // let mut scp_urls = ldap.get_scp_urls(domain).await?;

    // candidates.append(&mut scp_urls);

    let creds = (
        username
            .as_ref()
            .map(|username| username.as_ref())
            .unwrap_or(email.as_ref()),
        password.as_ref().map(|pass| pass.as_ref()),
    );

    let client = Client::new(creds)?;

    let mut requests = Vec::new();

    for candidate in candidates {
        let request = client.send_authenticated((candidate, email.as_ref()));

        requests.push(request);
    }

    let results = join_all(requests).await;

    for result in results {
        match result {
            Ok(config) => return Ok(config),
            Err(error) => warn!("{:?}", error),
        }
    }

    failed!(
        ErrorKind::NotFound,
        "Could not find any config for that email address",
    )
}

#[cfg(test)]
mod test {
    use std::env;

    use crate::from_email;

    use super::domain_from_email;

    #[test]
    fn test_domain() {
        let valid_email = "test@example.com";

        assert_eq!(domain_from_email(valid_email).unwrap(), "example.com");

        let invalid_emails = vec!["testexample.com", "test.example", "test@"];

        for invalid_email in invalid_emails {
            assert!(domain_from_email(invalid_email).is_err());
        }
    }

    #[tokio::test]
    async fn test_from_email() {
        env_logger::init();
        dotenv::dotenv().unwrap();

        from_email(
            env::var("USERNAME").unwrap(),
            env::var("PASSWORD").ok(),
            env::var("USERNAME").ok(),
        )
        .await
        .unwrap();
    }
}
