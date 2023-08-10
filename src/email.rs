use futures::future::join_all;

use log::warn;
use validator::validate_email;

use crate::{
    client::Client,
    error::{Error, ErrorKind, Result},
    failed,
    types::{protocol::Protocol, request::AutodiscoverRequest, response::AutodiscoverResponse},
};

const INVALID_EMAIL_MESSAGE: &str = "The given email address is invalid";

/// Parse the domain name from an email address.
///
/// Also validates that the given string is an email address.
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

/// Fetch an autodiscover config from a given email address and password.
///
/// Optionally, a different username can be used to login to the exchange server, otherwise, the specified email address will be used.
pub async fn from_email<E: AsRef<str>, P: AsRef<str>, U: AsRef<str>>(
    email: E,
    password: Option<P>,
    username: Option<U>,
) -> Result<AutodiscoverResponse> {
    let domain = domain_from_email(email.as_ref())?;

    // In this function we follow the steps to autodiscovery as specified by the following:
    // https://learn.microsoft.com/en-us/exchange/client-developer/exchange-web-services/autodiscover-for-exchange

    // We initialize our candidates with the basic links.
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
            Protocol::POX
        ),
        format!(
            "https://{}/autodiscover/autodiscover.{}",
            domain,
            Protocol::POX
        ),
    ];

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
        let request = AutodiscoverRequest::new(candidate, email.as_ref(), true);

        // We then send an authenticated request to each candidate.
        let future = client.send_request(request);

        requests.push(future);
    }

    let results = join_all(requests).await;

    let mut errors: Vec<Error> = Vec::new();

    // If any of the urls are a hit, we return it.
    for result in results {
        match result {
            Ok(config) => return Ok(config),
            Err(error) => {
                warn!("{:?}", error);

                errors.push(error);
            }
        }
    }

    // Otherwise, we try the backup candidate with an unauthenticated request.
    let candidate = format!(
        "http://autodiscover.{}/autodiscover/autodiscover.{}",
        domain,
        Protocol::POX
    );

    let request = AutodiscoverRequest::new(candidate, email.as_ref(), false);

    let response = client.send_request(request).await;

    match response {
        Ok(config) => return Ok(config),
        Err(error) => {
            warn!("{:?}", error);
            errors.push(error);
        }
    };

    // Finally, if all else failed, we try a dns query to try and resolve a domain from there.
    match client
        .dns_query(format!("_autodiscover._tcp.{}", domain))
        .await
    {
        Ok((autodiscover_domain, autodiscover_port)) => {
            let candidate = format!(
                "https://{}:{}/autodiscover/autodiscover.{}",
                autodiscover_domain,
                autodiscover_port,
                Protocol::POX
            );

            let request = AutodiscoverRequest::new(candidate, email.as_ref(), true);

            // We send an authenticated request.
            let response = client.send_request(request).await;

            match response {
                Ok(config) => return Ok(config),
                Err(error) => {
                    warn!("{:?}", error);
                    errors.push(error)
                }
            };
        }
        Err(error) => {
            warn!("{:?}", error);
            errors.push(error)
        }
    };

    // If nothing return a valid configuration, we return an error.
    failed!(
        ErrorKind::ConfigNotFound(errors),
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
