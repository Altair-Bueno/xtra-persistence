use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct PersistSnapshot<ID, SNAPSHOT>(pub ID, pub SNAPSHOT);

#[derive(Debug, Clone)]
pub struct FetchSnapshot<ID, SNAPSHOT>(pub ID, pub PhantomData<SNAPSHOT>);
