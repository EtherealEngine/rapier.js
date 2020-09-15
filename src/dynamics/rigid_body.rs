use crate::dynamics::{RawBodyStatus, RawRigidBodySet};
use crate::math::{RawRotation, RawVector};
use rapier::dynamics::{
    BodyStatus, RigidBody as RRigidBody, RigidBodyBuilder as RRigidBodyBuilder, RigidBodyHandle,
    RigidBodyMut as RRigidBodyMut, RigidBodySet,
};
use rapier::geometry::{ColliderBuilder, ColliderSet};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl RawRigidBodySet {
    /// The world-space translation of this rigid-body.
    pub fn rbTranslation(&self, handle: usize) -> RawVector {
        self.map(handle, |rb| RawVector(rb.position.translation.vector))
    }

    /// The world-space orientation of this rigid-body.
    pub fn rbRotation(&self, handle: usize) -> RawRotation {
        self.map(handle, |rb| RawRotation(rb.position.rotation))
    }

    /// The world-space predicted translation of this rigid-body.
    ///
    /// If this rigid-body is kinematic this value is set by the `setNextKinematicTranslation`
    /// method and is used for estimating the kinematic body velocity at the next timestep.
    /// For non-kinematic bodies, this value is currently unspecified.
    pub fn rbPredictedTranslation(&self, handle: usize) -> RawVector {
        self.map(handle, |rb| {
            RawVector(rb.predicted_position().translation.vector)
        })
    }

    /// The world-space predicted orientation of this rigid-body.
    ///
    /// If this rigid-body is kinematic this value is set by the `setNextKinematicRotation`
    /// method and is used for estimating the kinematic body velocity at the next timestep.
    /// For non-kinematic bodies, this value is currently unspecified.
    pub fn rbPredictedRotation(&self, handle: usize) -> RawRotation {
        self.map(handle, |rb| RawRotation(rb.predicted_position().rotation))
    }

    /// Sets the translation of this rigid-body.
    ///
    /// # Parameters
    /// - `x`: the world-space position of the rigid-body along the `x` axis.
    /// - `y`: the world-space position of the rigid-body along the `y` axis.
    /// - `z`: the world-space position of the rigid-body along the `z` axis.
    /// - `wakeUp`: forces the rigid-body to wake-up so it is properly affected by forces if it
    /// wasn't moving before modifying its position.
    #[cfg(feature = "dim3")]
    pub fn rbSetTranslation(&mut self, handle: usize, x: f32, y: f32, z: f32, wakeUp: bool) {
        self.map_mut_wake(handle, wakeUp, |mut rb| {
            let mut pos = rb.position;
            pos.translation.vector = na::Vector3::new(x, y, z);
            rb.set_position(pos);
        })
    }

    /// Sets the translation of this rigid-body.
    ///
    /// # Parameters
    /// - `x`: the world-space position of the rigid-body along the `x` axis.
    /// - `y`: the world-space position of the rigid-body along the `y` axis.
    /// - `wakeUp`: forces the rigid-body to wake-up so it is properly affected by forces if it
    /// wasn't moving before modifying its position.
    #[cfg(feature = "dim2")]
    pub fn rbSetTranslation(&mut self, handle: usize, x: f32, y: f32, wakeUp: bool) {
        self.map_mut_wake(handle, wakeUp, |mut rb| {
            let mut pos = rb.position;
            pos.translation.vector = na::Vector2::new(x, y);
            rb.set_position(pos);
        })
    }

    /// Sets the rotation quaternion of this rigid-body.
    ///
    /// This does nothing if a zero quaternion is provided.
    ///
    /// # Parameters
    /// - `x`: the first vector component of the quaternion.
    /// - `y`: the second vector component of the quaternion.
    /// - `z`: the third vector component of the quaternion.
    /// - `w`: the scalar component of the quaternion.
    /// - `wakeUp`: forces the rigid-body to wake-up so it is properly affected by forces if it
    /// wasn't moving before modifying its position.
    #[cfg(feature = "dim3")]
    pub fn rbSetRotation(&mut self, handle: usize, x: f32, y: f32, z: f32, w: f32, wakeUp: bool) {
        if let Some(q) = na::Unit::try_new(na::Quaternion::new(w, x, y, z), 0.0) {
            self.map_mut_wake(handle, wakeUp, |mut rb| {
                let mut pos = rb.position;
                pos.rotation = q;
                rb.set_position(pos);
            })
        }
    }

    /// Sets the rotation angle of this rigid-body.
    ///
    /// # Parameters
    /// - `angle`: the rotation angle, in radians.
    /// - `wakeUp`: forces the rigid-body to wake-up so it is properly affected by forces if it
    /// wasn't moving before modifying its position.
    #[cfg(feature = "dim2")]
    pub fn rbSetRotation(&mut self, handle: usize, angle: f32, wakeUp: bool) {
        self.map_mut_wake(handle, wakeUp, |mut rb| {
            let mut pos = rb.position;
            pos.rotation = na::UnitComplex::new(angle);
            rb.set_position(pos);
        })
    }

    /// If this rigid body is kinematic, sets its future translation after the next timestep integration.
    ///
    /// This should be used instead of `rigidBody.setTranslation` to make the dynamic object
    /// interacting with this kinematic body behave as expected. Internally, Rapier will compute
    /// an artificial velocity for this rigid-body from its current position and its next kinematic
    /// position. This velocity will be used to compute forces on dynamic bodies interacting with
    /// this body.
    ///
    /// # Parameters
    /// - `x`: the world-space position of the rigid-body along the `x` axis.
    /// - `y`: the world-space position of the rigid-body along the `y` axis.
    /// - `z`: the world-space position of the rigid-body along the `z` axis.
    #[cfg(feature = "dim3")]
    pub fn rbSetNextKinematicTranslation(&mut self, handle: usize, x: f32, y: f32, z: f32) {
        self.map_mut(handle, |mut rb| {
            let mut pos = *rb.predicted_position();
            pos.translation.vector = na::Vector3::new(x, y, z);
            rb.set_next_kinematic_position(pos);
        })
    }

    /// If this rigid body is kinematic, sets its future translation after the next timestep integration.
    ///
    /// This should be used instead of `rigidBody.setTranslation` to make the dynamic object
    /// interacting with this kinematic body behave as expected. Internally, Rapier will compute
    /// an artificial velocity for this rigid-body from its current position and its next kinematic
    /// position. This velocity will be used to compute forces on dynamic bodies interacting with
    /// this body.
    ///
    /// # Parameters
    /// - `x`: the world-space position of the rigid-body along the `x` axis.
    /// - `y`: the world-space position of the rigid-body along the `y` axis.
    #[cfg(feature = "dim2")]
    pub fn rbSetNextKinematicTranslation(&mut self, handle: usize, x: f32, y: f32) {
        self.map_mut(handle, |mut rb| {
            let mut pos = *rb.predicted_position();
            pos.translation.vector = na::Vector2::new(x, y);
            rb.set_next_kinematic_position(pos);
        })
    }

    /// If this rigid body is kinematic, sets its future rotation after the next timestep integration.
    ///
    /// This should be used instead of `rigidBody.setRotation` to make the dynamic object
    /// interacting with this kinematic body behave as expected. Internally, Rapier will compute
    /// an artificial velocity for this rigid-body from its current position and its next kinematic
    /// position. This velocity will be used to compute forces on dynamic bodies interacting with
    /// this body.
    ///
    /// # Parameters
    /// - `x`: the first vector component of the quaternion.
    /// - `y`: the second vector component of the quaternion.
    /// - `z`: the third vector component of the quaternion.
    /// - `w`: the scalar component of the quaternion.
    #[cfg(feature = "dim3")]
    pub fn rbSetNextKinematicRotation(&mut self, handle: usize, x: f32, y: f32, z: f32, w: f32) {
        if let Some(q) = na::Unit::try_new(na::Quaternion::new(w, x, y, z), 0.0) {
            self.map_mut(handle, |mut rb| {
                let mut pos = *rb.predicted_position();
                pos.rotation = q;
                rb.set_next_kinematic_position(pos);
            })
        }
    }

    /// If this rigid body is kinematic, sets its future rotation after the next timestep integration.
    ///
    /// This should be used instead of `rigidBody.setRotation` to make the dynamic object
    /// interacting with this kinematic body behave as expected. Internally, Rapier will compute
    /// an artificial velocity for this rigid-body from its current position and its next kinematic
    /// position. This velocity will be used to compute forces on dynamic bodies interacting with
    /// this body.
    ///
    /// # Parameters
    /// - `angle`: the rotation angle, in radians.
    #[cfg(feature = "dim2")]
    pub fn rbSetNextKinematicRotation(&mut self, handle: usize, angle: f32) {
        self.map_mut(handle, |mut rb| {
            let mut pos = *rb.predicted_position();
            pos.rotation = na::UnitComplex::new(angle);
            rb.set_next_kinematic_position(pos);
        })
    }

    /// The linear velocity of this rigid-body.
    pub fn rbLinvel(&self, handle: usize) -> RawVector {
        self.map(handle, |rb| RawVector(rb.linvel))
    }

    /// The mass of this rigid-body.
    pub fn rbMass(&self, handle: usize) -> f32 {
        self.map(handle, |rb| rb.mass())
    }

    /// Wakes this rigid-body up.
    ///
    /// A dynamic rigid-body that does not move during several consecutive frames will
    /// be put to sleep by the physics engine, i.e., it will stop being simulated in order
    /// to avoid useless computations.
    /// This methods forces a sleeping rigid-body to wake-up. This is useful, e.g., before modifying
    /// the position of a dynamic body so that it is properly simulated afterwards.
    pub fn rbWakeUp(&mut self, handle: usize) {
        self.map_mut(handle, |mut rb| rb.wake_up())
    }

    /*
    /// Creates a new collider attached to his rigid-body from the given collider descriptor.
    ///
    /// # Parameters
    /// - `collider`: The collider description used to create the collider.
    pub fn createCollider(&mut self, collider: &ColliderDesc) -> Collider {
        let builder: ColliderBuilder = collider.clone().into();
        let collider = builder.build();
        let colliders = self.colliders.clone();
        let bodies = self.bodies.clone();
        let handle =
            colliders
                .borrow_mut()
                .insert(collider, self.handle, &mut *bodies.borrow_mut());
        Collider {
            colliders,
            bodies,
            handle,
        }
    }
    */

    /// The number of colliders attached to this rigid-body.
    pub fn rbNumColliders(&self, handle: usize) -> usize {
        self.map(handle, |rb| rb.colliders().len())
    }

    /*
    /// Retrieves the `i-th` collider attached to this rigid-body.
    ///
    /// # Parameters
    /// - `at`: The index of the collider to retrieve. Must be a number in `[0, this.numColliders()[`.
    ///         This index is **not** the same as the unique identifier of the collider.
    pub fn collider(&self, at: usize) -> Collider {
        self.map(|rb| {
            let handle = rb.colliders()[at];
            Collider {
                colliders: self.colliders.clone(),
                bodies: self.bodies.clone(),
                handle,
            }
        })
    }
    */

    /// The type of this rigid-body: static, dynamic, or kinematic.
    pub fn rbBodyType(&self, handle: usize) -> RawBodyStatus {
        self.map(handle, |rb| rb.body_status.into())
    }

    /// Is this rigid-body static?
    pub fn rbIsStatic(&self, handle: usize) -> bool {
        self.map(handle, |rb| rb.is_static())
    }

    /// Is this rigid-body kinematic?
    pub fn rbIsKinematic(&self, handle: usize) -> bool {
        self.map(handle, |rb| rb.is_kinematic())
    }

    /// Is this rigid-body dynamic?
    pub fn rbIsDynamic(&self, handle: usize) -> bool {
        self.map(handle, |rb| rb.is_dynamic())
    }

    /// Applies a force at the center-of-mass of this rigid-body.
    ///
    /// # Parameters
    /// - `force`: the world-space force to apply on the rigid-body.
    /// - `wakeUp`: should the rigid-body be automatically woken-up?
    pub fn rbApplyForce(&mut self, handle: usize, force: &RawVector, wakeUp: bool) {
        self.map_mut_wake(handle, wakeUp, |mut rb| {
            rb.apply_force(force.0);
        })
    }

    /// Applies an impulse at the center-of-mass of this rigid-body.
    ///
    /// # Parameters
    /// - `impulse`: the world-space impulse to apply on the rigid-body.
    /// - `wakeUp`: should the rigid-body be automatically woken-up?
    pub fn rbApplyImpulse(&mut self, handle: usize, impulse: &RawVector, wakeUp: bool) {
        self.map_mut_wake(handle, wakeUp, |mut rb| {
            rb.apply_impulse(impulse.0);
        })
    }

    /// Applies a torque at the center-of-mass of this rigid-body.
    ///
    /// # Parameters
    /// - `torque`: the torque to apply on the rigid-body.
    /// - `wakeUp`: should the rigid-body be automatically woken-up?
    #[cfg(feature = "dim2")]
    pub fn rbApplyTorque(&mut self, handle: usize, torque: f32, wakeUp: bool) {
        self.map_mut_wake(handle, wakeUp, |mut rb| {
            rb.apply_torque(torque);
        })
    }

    /// Applies a torque at the center-of-mass of this rigid-body.
    ///
    /// # Parameters
    /// - `torque`: the world-space torque to apply on the rigid-body.
    /// - `wakeUp`: should the rigid-body be automatically woken-up?
    #[cfg(feature = "dim3")]
    pub fn rbApplyTorque(&mut self, handle: usize, torque: &RawVector, wakeUp: bool) {
        self.map_mut_wake(handle, wakeUp, |mut rb| {
            rb.apply_torque(torque.0);
        })
    }

    /// Applies an impulsive torque at the center-of-mass of this rigid-body.
    ///
    /// # Parameters
    /// - `torque impulse`: the torque impulse to apply on the rigid-body.
    /// - `wakeUp`: should the rigid-body be automatically woken-up?
    #[cfg(feature = "dim2")]
    pub fn rbApplyTorqueImpulse(&mut self, handle: usize, torque_impulse: f32, wakeUp: bool) {
        self.map_mut_wake(handle, wakeUp, |mut rb| {
            rb.apply_torque_impulse(torque_impulse);
        })
    }

    /// Applies an impulsive torque at the center-of-mass of this rigid-body.
    ///
    /// # Parameters
    /// - `torque impulse`: the world-space torque impulse to apply on the rigid-body.
    /// - `wakeUp`: should the rigid-body be automatically woken-up?
    #[cfg(feature = "dim3")]
    pub fn rbApplyTorqueImpulse(
        &mut self,
        handle: usize,
        torque_impulse: &RawVector,
        wakeUp: bool,
    ) {
        self.map_mut_wake(handle, wakeUp, |mut rb| {
            rb.apply_torque_impulse(torque_impulse.0);
        })
    }

    /// Applies a force at the given world-space point of this rigid-body.
    ///
    /// # Parameters
    /// - `force`: the world-space force to apply on the rigid-body.
    /// - `point`: the world-space point where the impulse is to be applied on the rigid-body.
    /// - `wakeUp`: should the rigid-body be automatically woken-up?
    pub fn rbApplyForceAtPoint(
        &mut self,
        handle: usize,
        force: &RawVector,
        point: &RawVector,
        wakeUp: bool,
    ) {
        self.map_mut_wake(handle, wakeUp, |mut rb| {
            rb.apply_force_at_point(force.0, point.0.into());
        })
    }

    /// Applies an impulse at the given world-space point of this rigid-body.
    ///
    /// # Parameters
    /// - `impulse`: the world-space impulse to apply on the rigid-body.
    /// - `point`: the world-space point where the impulse is to be applied on the rigid-body.
    /// - `wakeUp`: should the rigid-body be automatically woken-up?
    pub fn rbApplyImpulseAtPoint(
        &mut self,
        handle: usize,
        impulse: &RawVector,
        point: &RawVector,
        wakeUp: bool,
    ) {
        self.map_mut_wake(handle, wakeUp, |mut rb| {
            rb.apply_impulse_at_point(impulse.0, point.0.into());
        })
    }
}
