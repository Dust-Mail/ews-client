# Microsoft Autodiscover

A Rust based implementation of the [Microsoft autodiscover](https://learn.microsoft.com/en-us/exchange/client-developer/exchange-web-services/autodiscover-for-exchange) protocol for Exchange.

This is usefull for automatically detecting and finding a user's mail server configuration from just their username and password.

## Usage

You can request a config by simply calling the `from_email` function:

```rust
extern crate ms_autodiscover;

#[tokio::main]
async fn main() {
    let config = ms_autodiscover::from_email("user@contoso.com", "example_password", None::<String>).await.unwrap();

    match config {
		AutodiscoverResponse::Pox(response) => {
			println!("{}", response.user().display_name())
		}
	}

    // Example output:
    // "Contoso"
}
```
