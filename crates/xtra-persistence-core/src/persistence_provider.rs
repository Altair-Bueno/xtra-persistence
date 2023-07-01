use xtra::Actor;
use xtra::Handler;

use crate::FetchSnapshot;
use crate::PersistSnapshot;

pub trait PersistenceActor<ID, SNAPSHOT>
where
    Self: Actor
        + Handler<PersistSnapshot<ID, SNAPSHOT>, Return = ()>
        + Handler<FetchSnapshot<ID, SNAPSHOT>, Return = Option<SNAPSHOT>>
        + 'static,
    ID: Send,
    SNAPSHOT: Send,
{
}

impl<ID, SNAPSHOT, ANY> PersistenceActor<ID, SNAPSHOT> for ANY
where
    Self: Actor
        + Handler<PersistSnapshot<ID, SNAPSHOT>, Return = ()>
        + Handler<FetchSnapshot<ID, SNAPSHOT>, Return = Option<SNAPSHOT>>,
    ID: Send,
    SNAPSHOT: Send,
{
}
