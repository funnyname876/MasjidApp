use crate::common::data_access_layer::mysql::setup_container;
use crate::common::data_access_layer::DatabaseCredentials;
use crate::common::logging::setup_logging;
use masjid_app_admin_manager_api::features::donation::errors::donation_history_admin_repository_errors::GetDonationTransactionsError;
use masjid_app_admin_manager_api::features::donation::models::donation_filter::DonationFilter;
use masjid_app_admin_manager_api::features::donation::repositories::new_donation_history_admin_repository;
use masjid_app_api_library::features::donation::models::donation_history::DonationHistory;
use masjid_app_api_library::shared::data_access::repository_manager::RepositoryMode;
use masjid_app_public_api::features::donation::repositories::new_donation_history_public_repository;

#[tokio::test]
async fn test_donation() {
    setup_logging();
    let container = setup_container(DatabaseCredentials {
        username: "donationadmin".to_owned(),
        password: "changeme".to_owned(),
        environment_variable: "DONATION_CONNECTION".to_owned(),
    })
    .await;

    let public_repository = new_donation_history_public_repository(RepositoryMode::Normal).await;
    let admin_repository = new_donation_history_admin_repository(RepositoryMode::Normal).await;

    let mut donation_filter = &DonationFilter::default();
    eprintln!("When retrieving donations from an empty database, I should receive an error");
    let get_donation_transaction_result = admin_repository
        .get_donation_transactions(&donation_filter)
        .await
        .unwrap_err();
    assert!(matches!(
        get_donation_transaction_result,
        GetDonationTransactionsError::NotFound
    ));

    eprintln!("When inserting a donation transaction, I should receive no error");
    public_repository
        .insert_donation_transaction(&DonationHistory {
            id: 0,
            cause: "masjid".to_string(),
            donation_intention: "Sadaqa".to_string(),
            donor_full_name: "".to_string(),
            donor_title: "".to_string(),
            phone_number: "".to_string(),
            email: None,
            address_line_1: "".to_string(),
            address_line_2: None,
            address_city: "".to_string(),
            address_region: "".to_string(),
            address_country: None,
            address_postal: "".to_string(),
            amount: 0.0,
            is_gift_aid: false,
            donation_frequency: "".to_string(),
            transaction_status: "".to_string(),
        })
        .await
        .unwrap();
    container.stop().await.unwrap();
}
