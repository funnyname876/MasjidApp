use crate::features::donation::models::donation_intention::DonationIntention;
use crate::shared::types::contact_details::ContactDetails;
use crate::shared::types::recurrence::Recurrence;
use serde::{Deserialize, Serialize};
use validator::Validate;
use validator::ValidationError;

#[derive(Debug, Deserialize, Serialize, Validate, Clone, Default)]
pub struct DonationDetails {
    #[validate(length(min = 3))]
    pub cause: String,

    #[serde(rename = "donationIntention")]
    pub donation_intention: DonationIntention,

    #[serde(rename = "isGiftAid")]
    pub is_gift_aid: bool,
    #[serde(rename = "contactDetails")]
    #[validate(nested)]
    pub contact_details: ContactDetails,
    #[validate(custom(function = "validate_amount"))]
    pub amount: f64,

    #[serde(rename = "donationFrequency")]
    pub donation_frequency: Recurrence,
}

fn validate_amount(amount: f64) -> Result<(), ValidationError> {
    if amount < 0.01 {
        return Err(ValidationError::new("amount cannot be less than 0.01"));
    }
    Ok(())
}
