use futures::future::join_all;

use log::warn;
use validator::validate_email;

use crate::{
    client::Client,
    error::{err, Error, ErrorKind, Result},
    types::{protocol::Protocol, request::AutodiscoverRequest, response::AutodiscoverResponse},
};

const INVALID_EMAIL_MESSAGE: &str = "The given email address is invalid";

/// Parse the domain name from an email address.
///
/// Also validates that the given string is an email address.
fn domain_from_email<'a>(email: &'a str) -> Result<&'a str> {
    if !validate_email(email) {
        err!(ErrorKind::InvalidEmailAddress, "{}", INVALID_EMAIL_MESSAGE);
    };

    let domain = {
        let mut email_split = email.split('@');

        email_split.next();

        match email_split.next() {
            Some(domain) => domain,
            None => err!(ErrorKind::InvalidEmailAddress, "{}", INVALID_EMAIL_MESSAGE),
        }
    };

    Ok(domain)
}

/// Fetch an autodiscover config from a given email address and password.
///
/// Optionally, a different username can be used to login to the exchange server, otherwise, the specified email address will be used.
pub async fn from_email<E: AsRef<str>, P: AsRef<str>, U: AsRef<str>>(
    email: E,
    password: Option<P>,
    username: Option<U>,
) -> Result<AutodiscoverResponse> {
    let creds = (
        username
            .as_ref()
            .map(|username| username.as_ref())
            .unwrap_or(email.as_ref()),
        password.as_ref().map(|pass| pass.as_ref()),
    );

    let client = Client::new(creds).await?;

    let domain = domain_from_email(email.as_ref())?;

    // In this function we follow the steps to autodiscovery as specified by the following:
    // https://learn.microsoft.com/en-us/exchange/client-developer/exchange-web-services/autodiscover-for-exchange

    // We initialize our candidates with the basic links.
    let candidates: Vec<String> = vec![
        #[cfg(feature = "soap")]
        format!(
            "https://autodiscover.{}/autodiscover/autodiscover.{}",
            domain,
            Protocol::SOAP
        ),
        #[cfg(feature = "soap")]
        format!(
            "https://{}/autodiscover/autodiscover.{}",
            domain,
            Protocol::SOAP
        ),
        #[cfg(feature = "pox")]
        format!(
            "https://autodiscover.{}/autodiscover/autodiscover.{}",
            domain,
            Protocol::POX
        ),
        #[cfg(feature = "pox")]
        format!(
            "https://{}/autodiscover/autodiscover.{}",
            domain,
            Protocol::POX
        ),
    ];

    let mut futures = Vec::new();

    let mut errors: Vec<Error> = Vec::new();

    for candidate in candidates {
        let request = AutodiscoverRequest::new(candidate, email.as_ref(), true);

        // We then send an authenticated request to each candidate.
        let future = client.send_request(request);

        futures.push(future);
    }

    #[cfg(feature = "pox")]
    {
        // Otherwise, we try the backup candidate with an unauthenticated request.
        let candidate = format!(
            "http://autodiscover.{}/autodiscover/autodiscover.{}",
            domain,
            Protocol::POX
        );

        let request = AutodiscoverRequest::new(candidate, email.as_ref(), false);

        let future = client.send_request(request);

        futures.push(future);
    }

    // Finally, if all else failed, we try a dns query to try and resolve a domain from there.
    match client
        .dns_query(format!("_autodiscover._tcp.{}", domain))
        .await
    {
        Ok(records) => {
            let mut candidates: Vec<String> = Vec::new();

            for (domain, port) in records {
                #[cfg(feature = "pox")]
                {
                    let pox_candidate = format!(
                        "https://{}:{}/autodiscover/autodiscover.{}",
                        domain,
                        port,
                        Protocol::POX
                    );

                    candidates.push(pox_candidate)
                }
            }

            for candidate in candidates {
                let request = AutodiscoverRequest::new(candidate, email.as_ref(), true);

                // We send an authenticated request.
                let future = client.send_request(request);

                futures.push(future);
            }
        }
        Err(error) => {
            warn!("{:?}", error);
            errors.push(error)
        }
    };

    if futures.is_empty() {
        err!(ErrorKind::ConfigNotFound(errors), "No urls to request")
    }

    let results = join_all(futures).await;

    // If any of the urls are a hit, we return it.
    for result in results {
        match result {
            Ok(config) => return Ok(config),
            Err(error) => errors.push(error),
        }
    }

    // If nothing return a valid configuration, we return an error.
    err!(
        ErrorKind::ConfigNotFound(errors),
        "Could not find any config for that email address",
    )
}

#[cfg(test)]
mod test {
    use std::env;

    use super::*;

    #[test]
    fn test_domain() {
        let valid_email = "test@example.com";

        assert_eq!(domain_from_email(valid_email).unwrap(), "example.com");

        let invalid_emails = vec!["testexample.com", "test.example", "test@"];

        for invalid_email in invalid_emails {
            assert!(domain_from_email(invalid_email).is_err());
        }
    }

    #[cfg_attr(feature = "runtime-async-std", async_std::test)]
    #[cfg_attr(feature = "runtime-tokio", tokio::test)]
    async fn test_from_email() {
        env_logger::init();
        dotenv::dotenv().unwrap();

        let config = from_email(
            env::var("EMAIL").unwrap(),
            env::var("PASSWORD").ok(),
            env::var("USERNAME").ok(),
        )
        .await
        .unwrap();

        println!("{:?}", config)
    }
}
