use crate::features::donation::errors::donation_history_admin_repository_errors::GetDonationTransactionsError;
use crate::features::donation::errors::donation_history_dto_mapping_error::DonationHistoryDTOMappingError;
use crate::features::donation::models::donation_dto::DonationHistoryDTO;
use crate::features::donation::models::donation_filter::DonationFilter;
use crate::features::donation::services::DonationHistoryService;
use async_trait::async_trait;
use masjid_app_api_library::features::donation::models::donation_history::DonationHistory;
use masjid_app_api_library::new_repository;
use masjid_app_api_library::shared::data_access::repository_manager::{
    InMemoryRepository, MySqlRepository, RepositoryMode, RepositoryType,
};
use mockall::automock;
use sqlx::mysql::MySqlRow;
use sqlx::Row;
use std::sync::Arc;

#[automock]
#[async_trait]
pub trait DonationHistoryAdminRepository: Send + Sync {
    async fn get_donation_transactions(
        &self,
        filters: &DonationFilter,
    ) -> Result<Vec<DonationHistoryDTO>, GetDonationTransactionsError>;
}
#[async_trait]
impl DonationHistoryAdminRepository for InMemoryRepository {
    async fn get_donation_transactions(
        &self,
        filters: &DonationFilter,
    ) -> Result<Vec<DonationHistoryDTO>, GetDonationTransactionsError> {
        todo!()
    }
}

#[async_trait]
impl DonationHistoryAdminRepository for MySqlRepository {
    async fn get_donation_transactions(
        &self,
        filters: &DonationFilter,
    ) -> Result<Vec<DonationHistoryDTO>, GetDonationTransactionsError> {
        let donation_intention = if let Some(donation_intention) = &filters.donation_intention {
            Some(donation_intention.to_string())
        } else {
            None
        };
        const STORED_PROCEDURE: &'static str = "CALL get_donation_transactions(?, ?, ?)";
        let db_connection = self.db_connection.clone();
        let donation_transactions = sqlx::query(STORED_PROCEDURE)
            .bind(&filters.email)
            .bind(&filters.phone_number)
            .bind(&filters.donation_cause)
            .bind(&donation_intention)
            .map(donation_history_from_my_sql_row)
            .fetch_all(&*db_connection)
            .await
            .map_err(|err| {
                if let sqlx::Error::RowNotFound = err {
                    return GetDonationTransactionsError::NotFound;
                }
                tracing::error!(
                    email = &filters.email,
                    phone_number = &filters.phone_number,
                    donation_cause = &filters.donation_cause,
                    stored_procedure = STORED_PROCEDURE,
                    error = err.to_string(),
                    "unable to fetch donation transaction history from database"
                );
                GetDonationTransactionsError::UnableToFetchDonationTransactions
            })?;

        let mut donation_transactions_dto = Vec::with_capacity(donation_transactions.len());
        for transaction in donation_transactions {
            donation_transactions_dto.push(DonationHistoryDTO::try_from(transaction)?)
        }
        Ok(donation_transactions_dto)
    }
}

pub async fn new_donation_history_admin_repository(
    repository_mode: RepositoryMode,
) -> Arc<dyn DonationHistoryAdminRepository> {
    new_repository!(repository_mode, RepositoryType::Donation)
}
fn donation_history_from_my_sql_row(row: MySqlRow) -> DonationHistory {
    DonationHistory {
        id: row.get(0),
        cause: row.get(1),
        donation_intention: row.get(2),
        donor_full_name: row.get(3),
        donor_title: row.get(4),
        phone_number: row.get(5),
        email: row.get(6),
        address_line_1: row.get(7),
        address_line_2: row.get(8),
        address_city: row.get(9),
        address_region: row.get(10),
        address_country: row.get(11),
        address_postal: row.get(12),
        amount: row.get(13),
        is_gift_aid: row.get(14),
        donation_frequency: row.get(15),
        transaction_status: row.get(16),
    }
}
