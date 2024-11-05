use anchor_lang::error_code;

#[error_code]
pub enum CustomErrorCode {
    #[msg("Invalid price")]
    InvalidPrice,
    #[msg("Health Factor Too Low")]
    HealthFactorTooLow,
}
