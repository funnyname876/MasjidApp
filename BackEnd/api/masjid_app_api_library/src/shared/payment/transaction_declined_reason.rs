use serde::Serialize;
use std::fmt::Display;
use std::str::FromStr;

const CARD_EXPIRED: &'static str = "declined because card expired";
const INSUFFICIENT_FUNDS: &'static str = "declined because insufficient funds";
const CARD_BLOCKED: &'static str = "declined because card blocked";
const CARD_FROZEN: &'static str = "declined because card frozen";
const SUSPECTED_FRAUD: &'static str = "declined because suspected fraud";

#[derive(Debug, Clone, Copy, Serialize, Eq, PartialEq)]
pub enum TransactionDeclinedReason {
    CardExpired,
    InsufficientFunds,
    CardBlocked,
    CardFrozen,
    SuspectedFraud,
}

impl Display for TransactionDeclinedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let decline_string = match self {
            TransactionDeclinedReason::CardExpired => CARD_EXPIRED,
            TransactionDeclinedReason::InsufficientFunds => INSUFFICIENT_FUNDS,
            TransactionDeclinedReason::CardBlocked => CARD_BLOCKED,
            TransactionDeclinedReason::CardFrozen => CARD_FROZEN,
            TransactionDeclinedReason::SuspectedFraud => SUSPECTED_FRAUD,
        };
        write!(f, "{:?}", decline_string)
    }
}

impl FromStr for TransactionDeclinedReason {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let decline_reason = match s {
            CARD_EXPIRED => TransactionDeclinedReason::CardExpired,
            INSUFFICIENT_FUNDS => TransactionDeclinedReason::InsufficientFunds,
            CARD_BLOCKED => TransactionDeclinedReason::CardBlocked,
            CARD_FROZEN => TransactionDeclinedReason::CardFrozen,
            SUSPECTED_FRAUD => TransactionDeclinedReason::SuspectedFraud,
            _ => return Err(()),
        };
        Ok(decline_reason)
    }
}
