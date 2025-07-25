use super::epoch::Epoch;
use common::duplex_channel::Endpoint;
use std::fmt;

// TODO: should I download epoch history through this coordinator?
// For example node was offline or
// if yes - add an interface
// if no - where to download it? p2p? s3?

// TODO: Don't like these names, I think it makes sense to have them.
// It provides a bit more clarity and clearnes, but should think more no naming
pub type B2AEndpoint = Endpoint<EpochUpdates, EpochRequest>;
pub type A2BEndpoint = Endpoint<EpochRequest, EpochUpdates>;

#[derive(Debug)]
pub struct EpochRequest {
  pub epoch: Epoch,
}

#[derive(Debug, PartialEq, Eq)]
pub enum EpochUpdates {
  /// when a new epoch detected
  New(Epoch),
}

#[derive(Debug)]
pub enum CommitError {
  CommitFailed(String),
}

impl fmt::Display for CommitError {
  fn fmt(
    &self,
    f: &mut fmt::Formatter<'_>,
  ) -> fmt::Result {
    match self {
      CommitError::CommitFailed(msg) => write!(f, "Failed to commit epoch: {}", msg),
    }
  }
}
