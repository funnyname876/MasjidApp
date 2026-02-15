use crate::features::donation::errors::donation_history_dto_mapping_error::DonationHistoryDTOMappingError;

pub enum GetDonationTransactionsError {
    NotFound,
    UnableToFetchDonationTransactions,
    // 1st String is field name 2nd String is value
    MappingFromDatabaseTableToDTOFailed(&'static str, String),
}

impl From<DonationHistoryDTOMappingError> for GetDonationTransactionsError {
    fn from(value: DonationHistoryDTOMappingError) -> Self {
        match value {
            DonationHistoryDTOMappingError::FailedToMapDonationFrequency(value) => {
                GetDonationTransactionsError::MappingFromDatabaseTableToDTOFailed(
                    "donation_frequency",
                    value,
                )
            }
            DonationHistoryDTOMappingError::FailedToMapTransactionStatus(value) => {
                GetDonationTransactionsError::MappingFromDatabaseTableToDTOFailed(
                    "transaction_status",
                    value,
                )
            }
            DonationHistoryDTOMappingError::FailedToMapDonationIntention(value) => {
                GetDonationTransactionsError::MappingFromDatabaseTableToDTOFailed(
                    "donation_intention",
                    value,
                )
            }
        }
    }
}
