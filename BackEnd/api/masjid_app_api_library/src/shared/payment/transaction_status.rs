use crate::shared::payment::transaction_declined_reason::TransactionDeclinedReason;
use serde::Serialize;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Serialize, Eq, PartialEq)]
pub enum TransactionStatus {
    Approved,
    Declined(TransactionDeclinedReason),
}

impl Display for TransactionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status_string = match self {
            TransactionStatus::Approved => "approved".to_owned(),
            TransactionStatus::Declined(declined_reason) => declined_reason.to_string(),
        };
        write!(f, "{:?}", status_string)
    }
}

impl FromStr for TransactionStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "approved" => Ok(TransactionStatus::Approved),
            _ => TransactionDeclinedReason::from_str(s).map(TransactionStatus::Declined),
        }
    }
}
