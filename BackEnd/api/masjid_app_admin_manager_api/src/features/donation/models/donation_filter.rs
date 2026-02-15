use masjid_app_api_library::features::donation::models::donation_intention::DonationIntention;

#[derive(Default)]
pub struct DonationFilter {
    pub donation_cause: Option<String>,
    pub donation_intention: Option<DonationIntention>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
}
