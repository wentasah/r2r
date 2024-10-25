use r2r::{QosProfile, WrappedTypesupport};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ctx = r2r::Context::create()?;
    let mut node = r2r::Node::create(ctx, "testnode", "")?;
    let duration = std::time::Duration::from_millis(2500);

    let mut timer = node.create_wall_timer(duration)?;
    let publisher =
        node.create_publisher_untyped("/topic", "std_msgs/msg/String", QosProfile::default())?;

    let handle = tokio::task::spawn_blocking(move || loop {
        node.spin_once(std::time::Duration::from_millis(100));
    });

    for _ in 1..10 {
        timer.tick().await?;
        let msg = r2r::std_msgs::msg::String {
            data: "hello from r2r".to_string(),
        };
        publisher.publish_raw(&msg.to_serialized_bytes()?)?;
    }

    handle.await?;
    Ok(())
}
