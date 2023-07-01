use std::any::Any;
use std::collections::HashMap;
use std::hash::Hash;
use xtra::prelude::*;
use xtra_persistence_core::*;

#[derive(Debug)]
pub struct InMemoryPersistenceActor<K> {
    actor_store: HashMap<K, Box<dyn Any + Send>>,
}

pub struct GetSnapshots;

impl<K> InMemoryPersistenceActor<K> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<K> Default for InMemoryPersistenceActor<K> {
    fn default() -> Self {
        Self {
            actor_store: Default::default(),
        }
    }
}

#[async_trait]
impl<K> Actor for InMemoryPersistenceActor<K>
where
    K: Send + 'static,
{
    type Stop = ();
    async fn started(&mut self, _: &mut Mailbox<Self>) -> Result<(), Self::Stop> {
        Ok(())
    }

    async fn stopped(self) -> Self::Stop {}
}

#[async_trait]
impl<K, V> Handler<PersistSnapshot<K, V>> for InMemoryPersistenceActor<K>
where
    K: Hash + Send + 'static + Eq + PartialEq,
    V: Send + 'static,
{
    type Return = ();

    async fn handle(
        &mut self,
        PersistSnapshot(k, v): PersistSnapshot<K, V>,
        _: &mut Context<Self>,
    ) -> Self::Return {
        let v = Box::new(v) as _;
        self.actor_store.insert(k, v);
    }
}
#[async_trait]
impl<K, V> Handler<FetchSnapshot<K, V>> for InMemoryPersistenceActor<K>
where
    K: Hash + Send + 'static + Eq + PartialEq,
    V: Send + 'static,
{
    type Return = Option<V>;

    async fn handle(
        &mut self,
        FetchSnapshot(k, _): FetchSnapshot<K, V>,
        _: &mut Context<Self>,
    ) -> Self::Return {
        let v = self.actor_store.remove(&k)?;
        let v = v.downcast().ok()?;
        Some(*v)
    }
}
