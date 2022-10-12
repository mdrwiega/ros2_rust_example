// ROS2 publisher
fn main() -> Result<(), anyhow::Error> {

    let context = rclrs::Context::new(std::env::args())?;
    let node = rclrs::create_node(&context, "rust_publisher")?;
    let publisher =
        node.create_publisher::<std_msgs::msg::String>("topic", rclrs::QOS_PROFILE_DEFAULT)?;

    let mut message = std_msgs::msg::String::default();
    let mut counter: u32 = 1;

    while context.ok() {
        message.data = format!("message {}", counter);
        publisher.publish(&message)?;
        println!("Published: {}", message.data);
        counter += 1;
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    Ok(())
}