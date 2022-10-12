// ROS2 subscriber

fn callback(msg: std_msgs::msg::String) {
  println!("Received: '{}'", msg.data);
}

fn main() -> Result<(), anyhow::Error> {
    let context = rclrs::Context::new(std::env::args())?;
    let mut node = rclrs::create_node(&context, "rust_subscriber")?;

    println!("Waiting for messages...");

    let _subscription = node.create_subscription::<std_msgs::msg::String, _>(
        "topic",
        rclrs::QOS_PROFILE_DEFAULT,
        callback
    )?;

    rclrs::spin(&node)?;
    Ok(())
}