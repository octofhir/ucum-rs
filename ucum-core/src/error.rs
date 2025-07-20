use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum UcumError {
    #[error("Invalid unit code: {0}")]
    InvalidUnit(String),
    
    #[error("Conversion not possible: {from} to {to}")]
    ConversionNotPossible { from: String, to: String },
    
    #[error("Invalid quantity value: {0}")]
    InvalidValue(String),
    
    #[error("Parser error: {0}")]
    ParserError(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    #[error("Invalid symbol: {0}")]
    InvalidSymbol(String),
    
    #[error("Empty unit symbols")]
    EmptyUnitSymbols,
    
    #[error("Invalid exponent: {0}")]
    InvalidExponent(String),
    
    #[error("Invalid exponent sign: {0}")]
    InvalidExponentSign(char),
    
    #[error("No parser available")]
    NoParserAvailable,
    
    #[error("Grammar parsing failed: {0}")]
    GrammarParsingFailed(String),
    
    #[error("IO error: {0}")]
    IoError(String),
    
    #[error("JSON error: {0}")]
    JsonError(String),
    
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Registry error: {0}")]
    RegistryError(String),
}

impl From<std::io::Error> for UcumError {
    fn from(err: std::io::Error) -> Self {
        UcumError::IoError(err.to_string())
    }
}

impl From<serde_json::Error> for UcumError {
    fn from(err: serde_json::Error) -> Self {
        UcumError::JsonError(err.to_string())
    }
}

#[cfg(feature = "network")]
impl From<reqwest::Error> for UcumError {
    fn from(err: reqwest::Error) -> Self {
        UcumError::NetworkError(err.to_string())
    }
} 