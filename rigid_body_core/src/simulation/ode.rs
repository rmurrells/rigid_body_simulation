use super::rigid_body::RigidBody;
use crate::math::Quarternion;

pub fn euler(delta_t: f64, rigid_bodies: &mut [RigidBody]) {
    for rigid_body in rigid_bodies {
        rigid_body
            .position
            .add_assign(&rigid_body.velocity().scale(delta_t));
        rigid_body
            .momentum
            .add_assign(&rigid_body.force.scale(delta_t));

        rigid_body.quarternion.add_assign(
            &Quarternion::new(0., rigid_body.angular_velocity())
                .mult(&rigid_body.quarternion)
                .scale(0.5 * delta_t),
        );
        rigid_body
            .angular_momentum
            .add_assign(&rigid_body.torque.scale(delta_t));
        rigid_body.update();
    }
}
