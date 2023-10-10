use safe_drive::{
    context::Context, error::DynError, msg::common_interfaces::geometry_msgs, topic::subscriber::Subscriber
};
use std::{sync::{Arc, Mutex}, time::Duration};

#[async_std::main]
async fn main() -> Result<(), DynError>{
    let ctx = Context::new()?;
    let node = ctx.create_node("topic_throttle", None, Default::default())?;
    let subscriber = node.create_subscriber::<geometry_msgs::msg::Twist>("cmd_vel", None)?;
    let publisher = node.create_publisher::<geometry_msgs::msg::Twist>("cmd_vel/robocon_2023", None)?;

    let pub_msg = Arc::new(Mutex::new(geometry_msgs::msg::Twist::new().unwrap()));
    let sub_msg = pub_msg.clone();



    let task_pub = async_std::task::spawn(async move {
        loop{
            publisher.send(&pub_msg.lock().unwrap()).unwrap();
            async_std::task::sleep(Duration::from_millis(100)).await;
        }
    });
    let task_sub = async_std::task::spawn(receiver(subscriber, sub_msg));

    task_pub.await;
    task_sub.await?;

    Ok(())
}

async fn receiver(mut subscriber: Subscriber<geometry_msgs::msg::Twist>, sub_msg: Arc<Mutex<geometry_msgs::msg::Twist>>) -> Result<(), DynError> {
    loop {
        let _msg = subscriber.recv().await?;
        sub_msg.lock().unwrap().linear.x = _msg.linear.x;
        sub_msg.lock().unwrap().linear.y = _msg.linear.y;
        sub_msg.lock().unwrap().angular.z = _msg.angular.z;
    }
}