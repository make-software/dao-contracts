use std::{panic::{AssertUnwindSafe, self}, sync::Arc, thread};

use cucumber::{
    World,
    step, event, codegen::Lazy, cli, Event, parser
};
use futures::{Stream, FutureExt, stream::{self, LocalBoxStream}, future, StreamExt, TryStreamExt, executor::block_on};
use derive_more::{Display, From};

use super::DaoWorld;

pub struct CustomRunner;

impl CustomRunner {
    fn steps_fns() -> &'static step::Collection<DaoWorld> {
        // Wire the static collection of step matching functions.
        static STEPS: Lazy<step::Collection<DaoWorld>> =
            Lazy::new(DaoWorld::collection);
        &STEPS
    }

    async fn execute_step(
        mut world: DaoWorld,
        step: gherkin::Step,
    ) -> (DaoWorld, SyncStep<DaoWorld>) {
        let ev = if let Some((step_fn, captures, loc, ctx)) =
            Self::steps_fns().find(&step).expect("Ambiguous match")
        {
            // Panic represents a failed assertion in a step matching
            // function.
            match AssertUnwindSafe(step_fn(&mut world, ctx))
                .catch_unwind()
                .await
            {
                Ok(()) => SyncStep::Passed(captures, loc),
                Err(e) => SyncStep::Failed(
                    Some(captures),
                    loc,
                    Some(Arc::new(world.clone())),
                    SyncStepError::Panic(e.downcast_ref::<String>().cloned().unwrap()),
                ),
            }
        } else {
            SyncStep::Skipped
        };
        (world, ev)
    }

    async fn execute_scenario(
        scenario: gherkin::Scenario,
    ) -> impl Stream<Item = event::Feature<DaoWorld>> {
        // Those panic hook shenanigans are done to avoid console messages like
        // "thread 'main' panicked at ..."
        //
        // 1. We obtain the current panic hook and replace it with an empty one.
        // 2. We run tests, which can panic. In that case we pass all panic info
        //    down the line to the `Writer`, which will print it at right time.
        // 3. We restore original panic hook, because suppressing all panics
        //    doesn't sound like a very good idea.
        let s = scenario.clone();
        let (tx, rx) = futures::channel::oneshot::channel();

        thread::spawn(move || {
            // let hook = panic::take_hook();
            // panic::set_hook(Box::new(|_| {}));
            
            let steps = block_on(async {
                let mut steps = Vec::with_capacity(s.steps.len());
                let mut world = DaoWorld::new().await.unwrap();
                for step in s.steps.clone() {
                    let (w, ev) = Self::execute_step(world, step.clone()).await;
                    world = w;
                    let should_stop = matches!(ev, SyncStep::Failed(..));
                    steps.push((step, ev));
                    if should_stop {
                        break;
                    }
                }
                steps
            }); 
            
            // panic::set_hook(hook);
            tx.send(steps).unwrap();
        });

        let steps = rx.await.unwrap();
        let steps: Vec<(gherkin::Step, event::Step<DaoWorld>)> = steps.into_iter().map(|(step, ev)| {
            (step, event::Step::from(ev))
        }).collect();

        let scenario = Arc::new(scenario);
        stream::once(future::ready(event::Scenario::Started))
            .chain(stream::iter(steps.into_iter().flat_map(|(step, ev)| {
                let step = Arc::new(step);
                [
                    event::Scenario::Step(step.clone(), event::Step::Started),
                    event::Scenario::Step(step, ev),
                ]
            })))
            .chain(stream::once(future::ready(event::Scenario::Finished)))
            .map(move |event| event::Feature::Scenario(
                scenario.clone(), 
                event::RetryableScenario { event, retries: None },
            ))
    }

    // fn handle_scenario_results() -> impl Stream<Item = event::Feature<DaoWorld>> {
        
    // }

    fn execute_feature(
        feature: gherkin::Feature,
    ) -> impl Stream<Item = event::Cucumber<DaoWorld>> {
        // dbg!(feature.rules.clone());
        let feature = Arc::new(feature);
        stream::once(future::ready(event::Feature::Started))
            .chain(
                stream::iter(feature.scenarios.clone())
                    .then(Self::execute_scenario)
                    .flatten(),
            )
            .chain(stream::once(future::ready(event::Feature::Finished)))
            .map(move |ev| event::Cucumber::Feature(feature.clone(), ev))
    }
}

impl cucumber::Runner<DaoWorld> for CustomRunner {
    type Cli = cli::Empty; // we provide no CLI options
    type EventStream = LocalBoxStream<
        'static,
        parser::Result<Event<event::Cucumber<DaoWorld>>>,
    >;

    fn run<S>(self, features: S, _: Self::Cli) -> Self::EventStream
    where
        S: Stream<Item = parser::Result<gherkin::Feature>> + 'static,
    {
        stream::once(future::ok(event::Cucumber::Started))
            .chain(
                features
                    .map_ok(|f| Self::execute_feature(f).map(Ok))
                    .try_flatten(),
            )
            .chain(stream::once(future::ok(event::Cucumber::Finished)))
            .map_ok(Event::new)
            .boxed_local()
    }
}

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
                cucumber::event::Step::Failed(
                    capture_location,
                    maybe_location,
                    world,
                    err.into(),
                )
            },
        }
    }
}

impl From<SyncStepError> for cucumber::event::StepError {
    fn from(value: SyncStepError) -> Self {
        match value {
            SyncStepError::NotFound => cucumber::event::StepError::NotFound,
            SyncStepError::AmbiguousMatch(err) => {
                cucumber::event::StepError::AmbiguousMatch(err)
            }
            SyncStepError::Panic(msg) => cucumber::event::StepError::Panic(Arc::new(msg)),
        }
    }
}