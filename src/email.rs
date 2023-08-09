use std::io;

use log::warn;
use reqwest::Method;
use validator::validate_email;

use crate::{
    candidate::CandidateType,
    config::Config,
    // ldap::Ldap,
    error::{Error, ErrorKind, Result},
    http::Http,
};

const INVALID_EMAIL_MESSAGE: &str = "The given email address is invalid";

fn domain_from_email<E: AsRef<str>>(email: E) -> Result<String> {
    if !validate_email(email.as_ref()) {
        return Err(Error::new(
            ErrorKind::InvalidEmailAddress,
            INVALID_EMAIL_MESSAGE,
        ));
    };

    let mut email_split = email.as_ref().split('@');

    email_split.next();

    let domain = email_split.next().ok_or(Error::new(
        ErrorKind::InvalidEmailAddress,
        INVALID_EMAIL_MESSAGE,
    ))?;

    Ok(domain.to_string())
}

pub async fn from_email<E: AsRef<str>, P: AsRef<str>>(
    email: E,
    password: Option<P>,
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

    let http = Http::new()?;

    for candidate in candidates {
        if let Some(candidate_type) = CandidateType::from_url(&candidate) {
            let creds: (String, Option<String>) = (
                email.as_ref().into(),
                password.as_ref().map(|password| password.as_ref().into()),
            );

            let body = candidate_type.create_request_body(email.as_ref())?;

            match http
                .fetch_xml(&candidate, Method::POST, body, Some(creds))
                .await
            {
                Ok(bytes) => {
                    let reader = io::Cursor::new(bytes);

                    let config = candidate_type.parse_config(reader)?;

                    return Ok(config);
                }
                Err(err) => {
                    warn!("Error fetching {}: {:?}", candidate, err)
                }
            }
        } else {
            warn!("Url does not have a recognizable extension")
        }
    }

    Err(Error::new(
        ErrorKind::NotFound,
        "Could not find any config for that email address",
    ))
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

        from_email(env::var("USERNAME").unwrap(), env::var("PASSWORD").ok())
            .await
            .unwrap();
    }
}
