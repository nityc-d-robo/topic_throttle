use safe_drive::{
    context::Context, error::DynError, msg::common_interfaces::geometry_msgs,
};
use std::{rc::Rc, cell::RefCell, time::Duration};

fn main() -> Result<(), DynError>{
    let ctx = Context::new()?;
    let node = ctx.create_node("topic_throttle", None, Default::default())?;
    let subscriber = node.create_subscriber::<geometry_msgs::msg::Twist>("cmd_vel", None)?;
    let publisher = node.create_publisher::<geometry_msgs::msg::Twist>("cmd_vel/robocon_2023", None)?;
    let mut selector = ctx.create_selector()?;

    let msg = Rc::new(RefCell::new(geometry_msgs::msg::Twist::new().unwrap()));
    let sub_msg = msg.clone();

    selector.add_wall_timer(
        "publisher",
        Duration::from_millis(100),
        Box::new(move || {
            publisher.send(&msg.borrow()).unwrap();
        }),
    );

    selector.add_subscriber(
        subscriber,
        Box::new(move |_msg| {
            sub_msg.borrow_mut().linear.x = _msg.linear.x;
            sub_msg.borrow_mut().linear.y = _msg.linear.y;
            sub_msg.borrow_mut().angular.z = _msg.angular.z;
        }),
    );

    loop{
        selector.wait()?;
    }
}
