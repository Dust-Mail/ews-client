use std::cmp::Ordering;

use crate::{
    error::{ErrorKind, Result},
    failed,
};

use trust_dns_resolver::{config::ResolverConfig, proto::rr::rdata::SRV, TokioAsyncResolver};

struct WeightedSrvRecord {
    record: SRV,
    weight: u16,
    priority: u16,
}

impl WeightedSrvRecord {
    fn new(record: SRV) -> Self {
        Self {
            weight: record.weight(),
            priority: record.priority(),
            record,
        }
    }
}

// Compare WeightedSrvRecord by priority and weight
impl Ord for WeightedSrvRecord {
    fn cmp(&self, other: &Self) -> Ordering {
        // Sort by priority in ascending order
        let priority_cmp = self.priority.cmp(&other.priority);

        if priority_cmp == Ordering::Equal {
            // Sort by weight in descending order
            other.weight.cmp(&self.weight)
        } else {
            priority_cmp
        }
    }
}

// Implement PartialOrd for WeightedSrvRecord
impl PartialOrd for WeightedSrvRecord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Implement Eq for WeightedSrvRecord
impl Eq for WeightedSrvRecord {}

// Implement PartialEq for WeightedSrvRecord
impl PartialEq for WeightedSrvRecord {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority && self.weight == other.weight
    }
}

pub struct Dns {
    resolver: TokioAsyncResolver,
}

impl Dns {
    pub fn new() -> Result<Self> {
        let resolver = TokioAsyncResolver::tokio(ResolverConfig::default(), Default::default())?;

        let dns = Self { resolver };

        Ok(dns)
    }

    pub async fn srv_lookup<D: AsRef<str>>(&self, domain: D) -> Result<String> {
        let records = self.resolver.srv_lookup(domain.as_ref()).await?;

        let mut weighted_records: Vec<_> =
            records.into_iter().map(WeightedSrvRecord::new).collect();

        weighted_records.sort();

        if let Some(record) = weighted_records.first() {
            return Ok(record.record.target().to_string());
        }

        failed!(
            ErrorKind::NotFound,
            "Could not find any domains from the SRV query"
        )
    }
}
