use std::time::Duration;

use xtra::prelude::*;
use xtra_persistence::mem::InMemoryPersistenceActor;
use xtra_persistence::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Counter {
    persistence: Address<InMemoryPersistenceActor<&'static str>>,
    count: u64,
}

struct Inc;
struct TakeSnapshot;

#[async_trait]
impl Actor for Counter {
    type Stop = ();

    async fn started(&mut self, _: &mut Mailbox<Self>) -> Result<(), Self::Stop> {
        let count = self
            .persistence
            .send(FetchSnapshot("counter", Default::default()))
            .await
            .map_err(|_| ())?;
        self.count = count.unwrap_or(self.count);
        Ok(())
    }

    async fn stopped(self) -> Self::Stop {
        println!("Stopping counter...")
    }
}

#[async_trait]
impl Handler<Inc> for Counter {
    type Return = ();

    async fn handle(&mut self, _: Inc, ctx: &mut Context<Self>) -> Self::Return {
        println!("Adding one. Current count: {}", self.count);
        self.count += 1;
    }
}

// Handle persist and restore messages

#[async_trait]
impl Handler<TakeSnapshot> for Counter {
    type Return = ();

    async fn handle(&mut self, _: TakeSnapshot, ctx: &mut Context<Self>) -> Self::Return {
        println!("Taking snapshot!");
        let msg = PersistSnapshot("counter", self.count);
        xtra::scoped(&ctx.mailbox().address(), self.persistence.send(msg)).await;
    }
}

#[tokio::main]
async fn main() -> Result<(), xtra::Error> {
    let persistence_provider =
        xtra::spawn_tokio(InMemoryPersistenceActor::new(), Mailbox::unbounded());
    let counter = xtra::spawn_tokio(
        Counter {
            persistence: persistence_provider.clone(),
            count: 0,
        },
        Mailbox::unbounded(),
    );

    counter.send(Inc).await?;
    counter.send(Inc).await?;
    counter.send(TakeSnapshot).priority(u32::MAX).await?;
    // This increment will be ignored, as the snapshot was taken before
    counter.send(Inc).await?;
    tokio::time::sleep(Duration::from_millis(200)).await;
    // Stop the counter
    drop(counter);
    tokio::time::sleep(Duration::from_millis(200)).await;

    println!("Restart the counter from the snapshot");
    let counter = xtra::spawn_tokio(
        Counter {
            persistence: persistence_provider.clone(),
            count: 0,
        },
        Mailbox::unbounded(),
    );
    counter.send(Inc).await?;
    counter.send(Inc).await?;
    counter.send(Inc).await?;

    tokio::time::sleep(Duration::from_millis(200)).await;
    Ok(())
}
