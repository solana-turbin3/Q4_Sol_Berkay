

#[error_code]
pub enum AmmError {
    #[msg("Pool is locked")]
    PoolLocked,
    #[msg("Invalid amount")]
    InvalidAmount,
}