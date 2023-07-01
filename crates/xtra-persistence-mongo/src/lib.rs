use mongodb::bson::oid::ObjectId;
use mongodb::bson::Bson;
use mongodb::Collection;
use serde::Deserialize;
use serde::Serialize;
use xtra::prelude::*;
use xtra_persistence_core::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    #[serde(rename = "_id")]
    id: ObjectId,
    content: Bson,
}

#[derive(Debug)]
pub struct MongoPersistenceActor {
    collection: Collection<Record>,
}

impl MongoPersistenceActor {
    pub fn new(collection: Collection<Record>) -> Self {
        Self { collection }
    }
}

#[async_trait]
impl Actor for MongoPersistenceActor {
    type Stop = ();
    async fn started(&mut self, _: &mut Mailbox<Self>) -> Result<(), Self::Stop> {
        Ok(())
    }

    async fn stopped(self) -> Self::Stop {}
}

#[async_trait]
impl<K, V> Handler<PersistSnapshot<K, V>> for MongoPersistenceActor
where
    K: Serialize + Send + 'static,
    V: Serialize + Send + 'static,
{
    type Return = ();

    async fn handle(
        &mut self,
        PersistSnapshot(k, v): PersistSnapshot<K, V>,
        _: &mut Context<Self>,
    ) -> Self::Return {
        todo!()
    }
}
#[async_trait]
impl<K, V> Handler<FetchSnapshot<K, V>> for MongoPersistenceActor
where
    K: Deserialize<'static> + Send + 'static,
    V: Deserialize<'static> + Send + 'static,
{
    type Return = Option<V>;

    async fn handle(
        &mut self,
        FetchSnapshot(k, _): FetchSnapshot<K, V>,
        _: &mut Context<Self>,
    ) -> Self::Return {
        todo!()
    }
}
