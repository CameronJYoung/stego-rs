/// Members are different strategy specific errors
pub enum StegoStrategyError {

}

/// Members are different cover media specific errors
pub enum StegoCoverMediaError {

}

/// Members are different generic errors
pub enum StegoGenericError {

}

/// Members contain all the different types of errors
pub enum StegoError {
    Strategy(StegoStrategyError),
    CoverMedia(StegoCoverMediaError),
    Generic(StegoGenericError)
}