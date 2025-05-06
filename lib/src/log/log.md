**You need to manually kick of the tracing initialization by flowing steps**
```rust
use lib::log::init_subscriber;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // call this function as the first step.
    init_subscriber();
    Ok(())
}

```

**If you want to add some MDCs, you could using the span, here is an example**
```rust
    let span = tracing::span!(
        tracing::Level::INFO,
        "request",
        request_id = "abc-123",
        user_id = "user-42"
    );
    let _enter = span.enter();

    tracing::info!("User request started");
```
