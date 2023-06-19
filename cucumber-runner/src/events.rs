use cucumber::step;
use derive_more::{Display, From};
use std::sync::Arc;

#[derive(Debug)]
pub enum SyncStep<World> {
    Started,
    Skipped,
    Passed(regex::CaptureLocations, Option<step::Location>),
    Failed(
        Option<regex::CaptureLocations>,
        Option<step::Location>,
        Option<Arc<World>>,
        SyncStepError,
    ),
}

#[derive(Clone, Debug, Display, From)]
pub enum SyncStepError {
    #[display(fmt = "Step doesn't match any function")]
    NotFound,
    #[display(fmt = "Step match is ambiguous: {}", _0)]
    AmbiguousMatch(step::AmbiguousMatchError),
    #[display(fmt = "Step panicked. Captured output: {}", _0)]
    Panic(String),
}

impl<W> From<SyncStep<W>> for cucumber::event::Step<W> {
    fn from(value: SyncStep<W>) -> Self {
        match value {
            SyncStep::Started => cucumber::event::Step::Started,
            SyncStep::Skipped => cucumber::event::Step::Skipped,
            SyncStep::Passed(capture_location, maybe_location) => {
                cucumber::event::Step::Passed(capture_location, maybe_location)
            }
            SyncStep::Failed(capture_location, maybe_location, world, err) => {
                cucumber::event::Step::Failed(capture_location, maybe_location, world, err.into())
            }
        }
    }
}

impl From<SyncStepError> for cucumber::event::StepError {
    fn from(value: SyncStepError) -> Self {
        match value {
            SyncStepError::NotFound => cucumber::event::StepError::NotFound,
            SyncStepError::AmbiguousMatch(err) => cucumber::event::StepError::AmbiguousMatch(err),
            SyncStepError::Panic(msg) => cucumber::event::StepError::Panic(Arc::new(msg)),
        }
    }
}
