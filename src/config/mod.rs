mod pox;
// mod soap;

use derive_getters::Getters;
pub use pox::Autodiscover as PoxAutodiscover;
// pub use soap::SoapConfig;

#[derive(Debug, Getters)]
pub struct Config {}

#[derive(Debug, Getters)]
pub struct User {
    display_name: String,
}
