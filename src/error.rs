use std::error;
use std::fmt;

#[derive(Debug)]
pub enum WeightedRandError {
    SumWeights,
}

impl fmt::Display for WeightedRandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WeightedRandError::SumWeights => write!(f, "Sum of weights is 0."),
        }
    }
}

impl error::Error for WeightedRandError {
    fn description(&self) -> &str {
        match *self {
            WeightedRandError::SumWeights => "Sum of weights is 0.",
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            WeightedRandError::SumWeights => None,
        }
    }
}
