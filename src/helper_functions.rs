use bevy::prelude::*;

// ───── Body ─────────────────────────────────────────────────────────────── //

pub trait VectorUtilities<T> {
    type Item;
    fn reflect(&mut self, _rhs: T) -> Self;
    fn rotated(&self, angle: Self::Item) -> Self;
    fn to_unit_rad(&self) -> Self::Item;
}

impl VectorUtilities<Vec2> for Vec2 {
    type Item = f32;

    // fn reflect(&mut self, rhs: Vec2) -> Self {
    //     let scalar = 2. * self.dot(rhs) / rhs.dot(rhs);

    //     self.x = self.x - (rhs.x * scalar);
    //     self.y = self.y - (rhs.y * scalar);
    //     *self
    // }

    #[must_use]
    #[inline]
    fn reflect(&mut self, rhs: Vec2) -> Self {
        let scalar = 2. * self.dot(rhs) / rhs.dot(rhs);

        self.x = self.x - (rhs.x * scalar);
        self.y = self.y - (rhs.y * scalar);
        *self
    }

    fn rotated(&self, angle: f32) -> Self {
        let mut temp = Vec2::default();
        temp.x = self.x * angle.cos() - self.y * angle.sin();
        temp.y = self.x * angle.sin() + self.y * angle.cos();
        temp
    }

    fn to_unit_rad(&self) -> Self::Item {
        let temp = self.clone().normalize();
        temp.x.acos()
    }
}

pub fn clamp_translation(
    mut translation: Vec3,
    x_min: f32,
    x_max: f32,
    y_min: f32,
    y_max: f32,
) -> Vec3 {
    // Bound the player x position
    if translation.x < x_min {
        translation.x = x_min;
    } else if translation.x > x_max {
        translation.x = x_max;
    }

    // Bound the players y position
    if translation.y < y_min {
        translation.y = y_min;
    } else if translation.y > y_max {
        translation.y = y_max;
    }

    translation
}

pub fn get_camera_borders(
    cam_transform: &Transform,
    projection_area: Rect,
) -> (Vec3, Vec3, Vec3, Vec3) {
    // Left
    let x_left = cam_transform.translation.x - projection_area.max.x;
    let y_left = projection_area.max.y;

    // Right
    let x_right = cam_transform.translation.x + projection_area.max.x;
    let y_right = projection_area.max.y;

    // Top
    let x_top = cam_transform.translation.x;
    let y_top = cam_transform.translation.y + projection_area.max.y;

    // Bottom
    let x_bottom = cam_transform.translation.x;
    let y_bottom = cam_transform.translation.y - projection_area.max.y;

    (
        Vec3::new(x_left, y_left, 0.),
        Vec3::new(x_right, y_right, 0.),
        Vec3::new(x_top, y_top, 0.),
        Vec3::new(x_bottom, y_bottom, 0.),
    )
}

/// Function for calculating a unit vector (on unit circle) from `angle` in radians, with
/// `axis` unit vector matches with \[x: 1, y: 0\], so you can rotate axis.
pub fn angle_to_vec(angle: f32, axis: Vec2) -> Vec2 {
    // Unit vector on unit circle
    let pure = Vec2::from_angle(angle).normalize();
    // Angle between unit vector on unit circle and base vector
    let signed_pure_base_angle =
        axis.rotated(angle).y.atan2(axis.rotated(angle).x)
            - pure.y.atan2(pure.x);

    let sum = signed_pure_base_angle + angle;
    Vec2::new(round_float(sum.cos()), round_float(sum.sin())).normalize()
}

// -1.0..1.0 == -PI..PI
pub fn units_to_radians(value: f32) -> f32 {
    value * std::f32::consts::PI
}

// TODO: create documentation.
pub fn rotate_transform_with_parent_calibration(
    parent_rotation: &Quat,
    self_transform: &mut Transform,
    rotate_to: Vec2,
    on_axis: Vec2,
    time: Option<&Res<Time>>,
) {
    let multiplier: f32;
    match time {
        Some(value) => multiplier = value.delta_seconds().sin() * 10.,
        None => multiplier = 1.0,
    }

    // Make consideration about parent's axis, sprite's axis, and
    // transform into vector.
    let (_, _, balanced_angle) = self_transform
        .rotation
        .mul_quat(*parent_rotation)
        .to_euler(EulerRot::XYZ);
    let self_vec = angle_to_vec(balanced_angle, on_axis);

    if self_vec != rotate_to {
        let quat = Quat::from_rotation_z(
            self_vec.angle_between(rotate_to) * multiplier,
        );
        self_transform.rotate(quat);
    }
}

// ───── Small local helpers ──────────────────────────────────────────────── //

pub fn round_float(num: f32) -> f32 {
    f32::trunc(num * 100.0) / 100.0
}

// ───── Unit tests ───────────────────────────────────────────────────────── //

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn angle_to_vec_correct_output() {
        assert_eq!(angle_to_vec(0., Vec2::new(0., -1.)), Vec2::new(0., -1.));
        assert_eq!(angle_to_vec(PI, Vec2::new(0., -1.)), Vec2::new(0., 1.));
        assert_eq!(
            angle_to_vec(PI / 2., Vec2::new(0., -1.)),
            Vec2::new(1., 0.)
        );
        assert_eq!(
            angle_to_vec(PI + PI / 2., Vec2::new(0., -1.)),
            Vec2::new(-1., 0.,)
        );
        assert_eq!(
            angle_to_vec(PI * 2., Vec2::new(0., -1.)),
            Vec2::new(0., -1.,)
        );
    }

    use bevy::render::camera::{ComputedCameraValues, ScalingMode};

    const WINDOW_WIDTH: f32 = 1280.;
    const WINDOW_HEIGHT: f32 = 720.;

    #[test]
    fn check_camera_border_calculation() {
        let mut app = App::new();
        let camera = Camera2dBundle {
            camera: Camera {
                // +1 percent load on cpu when true
                hdr: true,
                ..default()
            },
            transform: Transform::from_xyz(
                WINDOW_WIDTH / 2.,
                WINDOW_HEIGHT / 2.,
                // IMPORTANT: Camera can see only objects down by z axis!
                1000.,
            ),
            projection: OrthographicProjection {
                scale: 1.,
                scaling_mode: ScalingMode::AutoMax {
                    max_width: WINDOW_WIDTH,
                    max_height: WINDOW_HEIGHT,
                },
                ..default()
            },
            ..default()
        };
        let cam_entity = app.world.spawn(camera).id();
        app.update();
        let cam_transform =
            app.world.entity(cam_entity).get::<Transform>().unwrap();
        // let cam_projection = app
        //     .world
        //     .entity(cam_entity)
        //     .get::<OrthographicProjection>()
        //     .unwrap()
        //     .area;
        let cam_projection = Rect {
            min: Vec2::new(-640., -360.),
            max: Vec2::new(640., 360.),
        };

        assert_eq!(
            get_camera_borders(cam_transform, cam_projection),
            (
                Vec3::new(0., 360., 0.),
                Vec3::new(1280., 360., 0.),
                Vec3::new(640., 720., 0.),
                Vec3::new(640., 0., 0.),
            )
        )
    }
}
