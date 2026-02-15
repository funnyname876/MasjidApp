use crate::features::donation::models::donation_filter::DonationFilter;
use masjid_app_api_library::features::donation::models::donation_intention::DonationIntention;
use serde::Deserialize;
use validator::Validate;

#[derive(Default, Validate, Deserialize)]
pub struct GetDonationHistoryRequest {
    #[validate(email)]
    pub email: Option<String>,
    pub donation_intention: Option<DonationIntention>,
    #[validate(length(min = 10, max = 10))]
    pub phone_number: Option<String>,
    #[validate(length(min = 3, max = 25))]
    pub donation_cause: Option<String>,
}
impl TryInto<DonationFilter> for GetDonationHistoryRequest {
    type Error = String;

    fn try_into(self) -> Result<DonationFilter, Self::Error> {
        self.validate().map_err(|err| err.to_string())?;
        Ok(DonationFilter {
            email: self.email,
            donation_intention: self.donation_intention,
            phone_number: self.phone_number,
            donation_cause: self.donation_cause,
        })
    }
}
