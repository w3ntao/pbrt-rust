use crate::pbrt::*;

// White Balance Definitions
// These are the Bradford transformation matrices.

pub const LMS_FROM_XYZ: SquareMatrix<3> = SquareMatrix::<3>::new([
    [0.8951, 0.2664, -0.1614],
    [-0.7502, 1.7135, 0.0367],
    [0.0389, -0.0685, 1.0296],
]);

pub const XYZ_FROM_LMS: SquareMatrix<3> = SquareMatrix::<3>::new([
    [0.986993, -0.147054, 0.159963],
    [0.432305, 0.51836, 0.0492912],
    [-0.00852866, 0.0400428, 0.968487],
]);

pub fn white_balance(src_white: Point2f, target_white: Point2f) -> SquareMatrix<3> {
    // Find LMS coefficients for source and target white
    let src_xyz = CIEXYZ::from_xy_y(src_white, 1.0);
    let dst_xyz = CIEXYZ::from_xy_y(target_white, 1.0);

    let src_lms = LMS_FROM_XYZ * src_xyz;
    let dst_lms = LMS_FROM_XYZ * dst_xyz;

    // Return white balancing matrix for source and target white

    let lms_correct = SquareMatrix::<3>::from_diag([
        dst_lms[0] / src_lms[0],
        dst_lms[1] / src_lms[1],
        dst_lms[2] / src_lms[2],
    ]);

    return XYZ_FROM_LMS * lms_correct * LMS_FROM_XYZ;
}
