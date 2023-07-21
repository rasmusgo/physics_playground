struct Scalar {
    // 1
    float g0;
};

struct MultiVector {
    // 1, e23, -e13, e12
    vec4 g0;
    // e0, -e023, e013, -e012
    vec4 g1;
    // e123, e1, e2, e3
    vec4 g2;
    // e0123, e01, e02, e03
    vec4 g3;
};

struct Rotor {
    // 1, e23, -e13, e12
    vec4 g0;
};

struct Point {
    // e123, -e023, e013, -e012
    vec4 g0;
};

struct IdealPoint {
    // e01, e02, e03
    vec3 g0;
};

struct Plane {
    // e0, e1, e2, e3
    vec4 g0;
};

struct Line {
    // e01, e02, e03
    vec3 g0;
    // e23, -e13, e12
    vec3 g1;
};

struct Translator {
    // 1, e01, e02, e03
    vec4 g0;
};

struct Motor {
    // 1, e23, -e13, e12
    vec4 g0;
    // e0123, e01, e02, e03
    vec4 g1;
};

struct PointAndPlane {
    // e123, -e023, e013, -e012
    vec4 g0;
    // e0, e1, e2, e3
    vec4 g1;
};

Scalar scalar_zero() {
    return Scalar(0.0);
}

Scalar scalar_one() {
    return Scalar(1.0);
}

Scalar scalar_neg(Scalar self) {
    return Scalar(self.g0 * -1.0);
}

Scalar scalar_automorphism(Scalar self) {
    return Scalar(self.g0);
}

Scalar scalar_reversal(Scalar self) {
    return Scalar(self.g0);
}

Scalar scalar_conjugation(Scalar self) {
    return Scalar(self.g0);
}

Scalar scalar_scalar_add(Scalar self, Scalar other) {
    return Scalar(self.g0 + other.g0);
}

Scalar scalar_scalar_sub(Scalar self, Scalar other) {
    return Scalar(self.g0 - other.g0);
}

Scalar scalar_scalar_mul(Scalar self, Scalar other) {
    return Scalar(self.g0 * other.g0);
}

Scalar scalar_scalar_div(Scalar self, Scalar other) {
    return Scalar(self.g0 * 1.0 / other.g0 * 1.0);
}

Scalar scalar_scalar_geometric_product(Scalar self, Scalar other) {
    return Scalar(self.g0 * other.g0);
}

Scalar scalar_scalar_outer_product(Scalar self, Scalar other) {
    return Scalar(self.g0 * other.g0);
}

Scalar scalar_scalar_inner_product(Scalar self, Scalar other) {
    return Scalar(self.g0 * other.g0);
}

Scalar scalar_scalar_left_contraction(Scalar self, Scalar other) {
    return Scalar(self.g0 * other.g0);
}

Scalar scalar_scalar_right_contraction(Scalar self, Scalar other) {
    return Scalar(self.g0 * other.g0);
}

Scalar scalar_scalar_scalar_product(Scalar self, Scalar other) {
    return Scalar(self.g0 * other.g0);
}

MultiVector scalar_multi_vector_add(Scalar self, MultiVector other) {
    return MultiVector(vec4(self.g0) * vec4(1.0, 0.0, 0.0, 0.0) + other.g0, other.g1, other.g2, other.g3);
}

MultiVector scalar_multi_vector_sub(Scalar self, MultiVector other) {
    return MultiVector(vec4(self.g0) * vec4(1.0, 0.0, 0.0, 0.0) - other.g0, vec4(0.0) - other.g1, vec4(0.0) - other.g2, vec4(0.0) - other.g3);
}

MultiVector scalar_multi_vector_geometric_product(Scalar self, MultiVector other) {
    return MultiVector(vec4(self.g0) * other.g0, vec4(self.g0) * other.g1, vec4(self.g0) * other.g2, vec4(self.g0) * other.g3);
}

Scalar scalar_multi_vector_regressive_product(Scalar self, MultiVector other) {
    return Scalar(self.g0 * other.g3.x);
}

MultiVector scalar_multi_vector_outer_product(Scalar self, MultiVector other) {
    return MultiVector(vec4(self.g0) * other.g0, vec4(self.g0) * other.g1, vec4(self.g0) * other.g2, vec4(self.g0) * other.g3);
}

MultiVector scalar_multi_vector_inner_product(Scalar self, MultiVector other) {
    return MultiVector(vec4(self.g0) * other.g0, vec4(self.g0) * other.g1, vec4(self.g0) * other.g2, vec4(self.g0) * other.g3);
}

MultiVector scalar_multi_vector_left_contraction(Scalar self, MultiVector other) {
    return MultiVector(vec4(self.g0) * other.g0, vec4(self.g0) * other.g1, vec4(self.g0) * other.g2, vec4(self.g0) * other.g3);
}

Scalar scalar_multi_vector_right_contraction(Scalar self, MultiVector other) {
    return Scalar(self.g0 * other.g0.x);
}

Scalar scalar_multi_vector_scalar_product(Scalar self, MultiVector other) {
    return Scalar(self.g0 * other.g0.x);
}

Rotor scalar_rotor_add(Scalar self, Rotor other) {
    return Rotor(vec4(self.g0) * vec4(1.0, 0.0, 0.0, 0.0) + other.g0);
}

Rotor scalar_rotor_sub(Scalar self, Rotor other) {
    return Rotor(vec4(self.g0) * vec4(1.0, 0.0, 0.0, 0.0) - other.g0);
}

Rotor scalar_rotor_geometric_product(Scalar self, Rotor other) {
    return Rotor(vec4(self.g0) * other.g0);
}

Rotor scalar_rotor_outer_product(Scalar self, Rotor other) {
    return Rotor(vec4(self.g0) * other.g0);
}

Rotor scalar_rotor_inner_product(Scalar self, Rotor other) {
    return Rotor(vec4(self.g0) * other.g0);
}

Rotor scalar_rotor_left_contraction(Scalar self, Rotor other) {
    return Rotor(vec4(self.g0) * other.g0);
}

Scalar scalar_rotor_right_contraction(Scalar self, Rotor other) {
    return Scalar(self.g0 * other.g0.x);
}

Scalar scalar_rotor_scalar_product(Scalar self, Rotor other) {
    return Scalar(self.g0 * other.g0.x);
}

Point scalar_point_geometric_product(Scalar self, Point other) {
    return Point(vec4(self.g0) * other.g0);
}

Point scalar_point_outer_product(Scalar self, Point other) {
    return Point(vec4(self.g0) * other.g0);
}

Point scalar_point_inner_product(Scalar self, Point other) {
    return Point(vec4(self.g0) * other.g0);
}

Point scalar_point_left_contraction(Scalar self, Point other) {
    return Point(vec4(self.g0) * other.g0);
}

Translator scalar_ideal_point_add(Scalar self, IdealPoint other) {
    return Translator(vec4(self.g0) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0));
}

Translator scalar_ideal_point_sub(Scalar self, IdealPoint other) {
    return Translator(vec4(self.g0) * vec4(1.0, 0.0, 0.0, 0.0) - vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0));
}

IdealPoint scalar_ideal_point_geometric_product(Scalar self, IdealPoint other) {
    return IdealPoint(vec3(self.g0) * other.g0);
}

IdealPoint scalar_ideal_point_outer_product(Scalar self, IdealPoint other) {
    return IdealPoint(vec3(self.g0) * other.g0);
}

IdealPoint scalar_ideal_point_inner_product(Scalar self, IdealPoint other) {
    return IdealPoint(vec3(self.g0) * other.g0);
}

IdealPoint scalar_ideal_point_left_contraction(Scalar self, IdealPoint other) {
    return IdealPoint(vec3(self.g0) * other.g0);
}

Plane scalar_plane_geometric_product(Scalar self, Plane other) {
    return Plane(vec4(self.g0) * other.g0);
}

Plane scalar_plane_outer_product(Scalar self, Plane other) {
    return Plane(vec4(self.g0) * other.g0);
}

Plane scalar_plane_inner_product(Scalar self, Plane other) {
    return Plane(vec4(self.g0) * other.g0);
}

Plane scalar_plane_left_contraction(Scalar self, Plane other) {
    return Plane(vec4(self.g0) * other.g0);
}

Line scalar_line_geometric_product(Scalar self, Line other) {
    return Line(vec3(self.g0) * other.g0, vec3(self.g0) * other.g1);
}

Line scalar_line_outer_product(Scalar self, Line other) {
    return Line(vec3(self.g0) * other.g0, vec3(self.g0) * other.g1);
}

Line scalar_line_inner_product(Scalar self, Line other) {
    return Line(vec3(self.g0) * other.g0, vec3(self.g0) * other.g1);
}

Line scalar_line_left_contraction(Scalar self, Line other) {
    return Line(vec3(self.g0) * other.g0, vec3(self.g0) * other.g1);
}

Translator scalar_translator_add(Scalar self, Translator other) {
    return Translator(vec4(self.g0) * vec4(1.0, 0.0, 0.0, 0.0) + other.g0);
}

Translator scalar_translator_sub(Scalar self, Translator other) {
    return Translator(vec4(self.g0) * vec4(1.0, 0.0, 0.0, 0.0) - other.g0);
}

Translator scalar_translator_geometric_product(Scalar self, Translator other) {
    return Translator(vec4(self.g0) * other.g0);
}

Translator scalar_translator_outer_product(Scalar self, Translator other) {
    return Translator(vec4(self.g0) * other.g0);
}

Translator scalar_translator_inner_product(Scalar self, Translator other) {
    return Translator(vec4(self.g0) * other.g0);
}

Translator scalar_translator_left_contraction(Scalar self, Translator other) {
    return Translator(vec4(self.g0) * other.g0);
}

Scalar scalar_translator_right_contraction(Scalar self, Translator other) {
    return Scalar(self.g0 * other.g0.x);
}

Scalar scalar_translator_scalar_product(Scalar self, Translator other) {
    return Scalar(self.g0 * other.g0.x);
}

Motor scalar_motor_add(Scalar self, Motor other) {
    return Motor(vec4(self.g0) * vec4(1.0, 0.0, 0.0, 0.0) + other.g0, other.g1);
}

Motor scalar_motor_sub(Scalar self, Motor other) {
    return Motor(vec4(self.g0) * vec4(1.0, 0.0, 0.0, 0.0) - other.g0, vec4(0.0) - other.g1);
}

Motor scalar_motor_geometric_product(Scalar self, Motor other) {
    return Motor(vec4(self.g0) * other.g0, vec4(self.g0) * other.g1);
}

Scalar scalar_motor_regressive_product(Scalar self, Motor other) {
    return Scalar(self.g0 * other.g1.x);
}

Motor scalar_motor_outer_product(Scalar self, Motor other) {
    return Motor(vec4(self.g0) * other.g0, vec4(self.g0) * other.g1);
}

Motor scalar_motor_inner_product(Scalar self, Motor other) {
    return Motor(vec4(self.g0) * other.g0, vec4(self.g0) * other.g1);
}

Motor scalar_motor_left_contraction(Scalar self, Motor other) {
    return Motor(vec4(self.g0) * other.g0, vec4(self.g0) * other.g1);
}

Scalar scalar_motor_right_contraction(Scalar self, Motor other) {
    return Scalar(self.g0 * other.g0.x);
}

Scalar scalar_motor_scalar_product(Scalar self, Motor other) {
    return Scalar(self.g0 * other.g0.x);
}

PointAndPlane scalar_point_and_plane_geometric_product(Scalar self, PointAndPlane other) {
    return PointAndPlane(vec4(self.g0) * other.g0, vec4(self.g0) * other.g1);
}

PointAndPlane scalar_point_and_plane_outer_product(Scalar self, PointAndPlane other) {
    return PointAndPlane(vec4(self.g0) * other.g0, vec4(self.g0) * other.g1);
}

PointAndPlane scalar_point_and_plane_inner_product(Scalar self, PointAndPlane other) {
    return PointAndPlane(vec4(self.g0) * other.g0, vec4(self.g0) * other.g1);
}

PointAndPlane scalar_point_and_plane_left_contraction(Scalar self, PointAndPlane other) {
    return PointAndPlane(vec4(self.g0) * other.g0, vec4(self.g0) * other.g1);
}

Scalar scalar_squared_magnitude(Scalar self) {
    return scalar_scalar_scalar_product(self, scalar_reversal(self));
}

Scalar scalar_magnitude(Scalar self) {
    return Scalar(sqrt(scalar_squared_magnitude(self).g0));
}

Scalar scalar_scale(Scalar self, float other) {
    return scalar_scalar_geometric_product(self, Scalar(other));
}

Scalar scalar_signum(Scalar self) {
    return scalar_scalar_geometric_product(self, Scalar(1.0 / scalar_magnitude(self).g0));
}

Scalar scalar_inverse(Scalar self) {
    return scalar_scalar_geometric_product(scalar_reversal(self), Scalar(1.0 / scalar_squared_magnitude(self).g0));
}

MultiVector multi_vector_zero() {
    return MultiVector(vec4(0.0), vec4(0.0), vec4(0.0), vec4(0.0));
}

MultiVector multi_vector_one() {
    return MultiVector(vec4(1.0, 0.0, 0.0, 0.0), vec4(0.0), vec4(0.0), vec4(0.0));
}

MultiVector multi_vector_neg(MultiVector self) {
    return MultiVector(self.g0 * vec4(-1.0), self.g1 * vec4(-1.0), self.g2 * vec4(-1.0), self.g3 * vec4(-1.0));
}

MultiVector multi_vector_automorphism(MultiVector self) {
    return MultiVector(self.g0, self.g1 * vec4(-1.0), self.g2 * vec4(-1.0), self.g3);
}

MultiVector multi_vector_reversal(MultiVector self) {
    return MultiVector(self.g0 * vec4(1.0, -1.0, -1.0, -1.0), self.g1 * vec4(1.0, -1.0, -1.0, -1.0), self.g2 * vec4(-1.0, 1.0, 1.0, 1.0), self.g3 * vec4(1.0, -1.0, -1.0, -1.0));
}

MultiVector multi_vector_conjugation(MultiVector self) {
    return MultiVector(self.g0 * vec4(1.0, -1.0, -1.0, -1.0), self.g1 * vec4(-1.0, 1.0, 1.0, 1.0), self.g2 * vec4(1.0, -1.0, -1.0, -1.0), self.g3 * vec4(1.0, -1.0, -1.0, -1.0));
}

MultiVector multi_vector_dual(MultiVector self) {
    return MultiVector(self.g3, self.g2 * vec4(-1.0, 1.0, 1.0, 1.0), self.g1 * vec4(1.0, -1.0, -1.0, -1.0), self.g0);
}

Scalar multi_vector_scalar_into(MultiVector self) {
    return Scalar(self.g0.x);
}

MultiVector multi_vector_scalar_add(MultiVector self, Scalar other) {
    return MultiVector(self.g0 + vec4(other.g0) * vec4(1.0, 0.0, 0.0, 0.0), self.g1, self.g2, self.g3);
}

MultiVector multi_vector_scalar_sub(MultiVector self, Scalar other) {
    return MultiVector(self.g0 - vec4(other.g0) * vec4(1.0, 0.0, 0.0, 0.0), self.g1, self.g2, self.g3);
}

MultiVector multi_vector_scalar_geometric_product(MultiVector self, Scalar other) {
    return MultiVector(self.g0 * vec4(other.g0), self.g1 * vec4(other.g0), self.g2 * vec4(other.g0), self.g3 * vec4(other.g0));
}

Scalar multi_vector_scalar_regressive_product(MultiVector self, Scalar other) {
    return Scalar(self.g3.x * other.g0);
}

MultiVector multi_vector_scalar_outer_product(MultiVector self, Scalar other) {
    return MultiVector(self.g0 * vec4(other.g0), self.g1 * vec4(other.g0), self.g2 * vec4(other.g0), self.g3 * vec4(other.g0));
}

MultiVector multi_vector_scalar_inner_product(MultiVector self, Scalar other) {
    return MultiVector(self.g0 * vec4(other.g0), self.g1 * vec4(other.g0), self.g2 * vec4(other.g0), self.g3 * vec4(other.g0));
}

Scalar multi_vector_scalar_left_contraction(MultiVector self, Scalar other) {
    return Scalar(self.g0.x * other.g0);
}

MultiVector multi_vector_scalar_right_contraction(MultiVector self, Scalar other) {
    return MultiVector(self.g0 * vec4(other.g0), self.g1 * vec4(other.g0), self.g2 * vec4(other.g0), self.g3 * vec4(other.g0));
}

Scalar multi_vector_scalar_scalar_product(MultiVector self, Scalar other) {
    return Scalar(self.g0.x * other.g0);
}

MultiVector multi_vector_multi_vector_add(MultiVector self, MultiVector other) {
    return MultiVector(self.g0 + other.g0, self.g1 + other.g1, self.g2 + other.g2, self.g3 + other.g3);
}

MultiVector multi_vector_multi_vector_sub(MultiVector self, MultiVector other) {
    return MultiVector(self.g0 - other.g0, self.g1 - other.g1, self.g2 - other.g2, self.g3 - other.g3);
}

MultiVector multi_vector_multi_vector_mul(MultiVector self, MultiVector other) {
    return MultiVector(self.g0 * other.g0, self.g1 * other.g1, self.g2 * other.g2, self.g3 * other.g3);
}

MultiVector multi_vector_multi_vector_div(MultiVector self, MultiVector other) {
    return MultiVector(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.w) * vec4(1.0, 1.0, 1.0, 1.0) / vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.w) * vec4(1.0, 1.0, 1.0, 1.0), vec4(self.g1.x, self.g1.y, self.g1.z, self.g1.w) * vec4(1.0, 1.0, 1.0, 1.0) / vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.w) * vec4(1.0, 1.0, 1.0, 1.0), vec4(self.g2.x, self.g2.y, self.g2.z, self.g2.w) * vec4(1.0, 1.0, 1.0, 1.0) / vec4(other.g2.x, other.g2.y, other.g2.z, other.g2.w) * vec4(1.0, 1.0, 1.0, 1.0), vec4(self.g3.x, self.g3.y, self.g3.z, self.g3.w) * vec4(1.0, 1.0, 1.0, 1.0) / vec4(other.g3.x, other.g3.y, other.g3.z, other.g3.w) * vec4(1.0, 1.0, 1.0, 1.0));
}

MultiVector multi_vector_multi_vector_geometric_product(MultiVector self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g0 + vec4(self.g0.y) * other.g0.yxwz * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g0.zwxy * vec4(-1.0, -1.0, 1.0, 1.0) + vec4(self.g0.w) * other.g0.wzyx * vec4(-1.0, 1.0, -1.0, 1.0) + vec4(self.g1.x) * other.g1 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g1.y) * other.g1.yxwz * vec4(-1.0, -1.0, 1.0, -1.0) + vec4(self.g1.z) * other.g1.zwxy * vec4(-1.0, -1.0, -1.0, 1.0) + vec4(self.g1.w) * other.g1.wzyx * vec4(-1.0, 1.0, -1.0, -1.0) + vec4(self.g2.x) * other.g2 * vec4(-1.0, 1.0, 1.0, 1.0) + vec4(self.g2.y) * other.g2.yxwz * vec4(1.0, 1.0, -1.0, 1.0) + vec4(self.g2.z) * other.g2.zwxy * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g2.w) * other.g2.wzyx * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g3.x) * other.g3 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g3.y) * other.g3.yxwz * vec4(-1.0, -1.0, 1.0, -1.0) + vec4(self.g3.z) * other.g3.zwxy * vec4(-1.0, -1.0, -1.0, 1.0) + vec4(self.g3.w) * other.g3.wzyx * vec4(-1.0, 1.0, -1.0, -1.0), vec4(self.g0.x) * other.g1 + vec4(self.g0.y) * other.g1.yxwz * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g1.zwxy * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.w) * other.g1.wzyx * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g1.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g1.y) * other.g0.yxwz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g1.z) * other.g0.zwxy * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g1.w) * other.g0.wzyx * vec4(1.0, 1.0, -1.0, 1.0) + vec4(self.g2.x) * other.g3 + vec4(self.g2.y) * other.g3.yxwz * vec4(-1.0, 1.0, -1.0, 1.0) + vec4(self.g2.z) * other.g3.zwxy * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g2.w) * other.g3.wzyx * vec4(-1.0, -1.0, 1.0, 1.0) - vec4(self.g3.x) * other.g2 + vec4(self.g3.y) * other.g2.yxwz * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g3.z) * other.g2.zwxy * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g3.w) * other.g2.wzyx * vec4(1.0, 1.0, -1.0, -1.0), vec4(self.g0.x) * other.g2 + vec4(self.g0.y) * other.g2.yxwz * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g2.zwxy * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.w) * other.g2.wzyx * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g1.x) * other.g3 + vec4(self.g1.y) * other.g3.yxwz * vec4(-1.0, 1.0, -1.0, 1.0) + vec4(self.g1.z) * other.g3.zwxy * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.w) * other.g3.wzyx * vec4(-1.0, -1.0, 1.0, 1.0) + vec4(self.g2.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g2.y) * other.g0.yxwz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g2.z) * other.g0.zwxy * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g2.w) * other.g0.wzyx * vec4(1.0, 1.0, -1.0, 1.0) - vec4(self.g3.x) * other.g1 + vec4(self.g3.y) * other.g1.yxwz * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g3.z) * other.g1.zwxy * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g3.w) * other.g1.wzyx * vec4(1.0, 1.0, -1.0, -1.0), vec4(self.g0.x) * other.g3 + vec4(self.g0.y) * other.g3.yxwz * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g3.zwxy * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.w) * other.g3.wzyx * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g1.x) * other.g2 + vec4(self.g1.y) * other.g2.yxwz * vec4(-1.0, 1.0, -1.0, 1.0) + vec4(self.g1.z) * other.g2.zwxy * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.w) * other.g2.wzyx * vec4(-1.0, -1.0, 1.0, 1.0) - vec4(self.g2.x) * other.g1 + vec4(self.g2.y) * other.g1.yxwz * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g2.z) * other.g1.zwxy * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g2.w) * other.g1.wzyx * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g3.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g3.y) * other.g0.yxwz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g3.z) * other.g0.zwxy * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g3.w) * other.g0.wzyx * vec4(1.0, 1.0, -1.0, 1.0));
}

MultiVector multi_vector_multi_vector_regressive_product(MultiVector self, MultiVector other) {
    return MultiVector(vec4(self.g0.y) * other.g3.yxyy * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g0.z) * other.g3.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g3.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * vec4(other.g2.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y) * other.g2.yxyy * vec4(-1.0, -1.0, 0.0, 0.0) + vec4(self.g1.z) * other.g2.zzxz * vec4(-1.0, 0.0, -1.0, 0.0) + vec4(self.g1.w) * other.g2.wwwx * vec4(-1.0, 0.0, 0.0, -1.0) + vec4(self.g2.x) * other.g1 * vec4(-1.0, 1.0, 1.0, 1.0) + vec4(self.g2.y) * vec4(other.g1.y) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g2.z) * vec4(other.g1.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g2.w) * vec4(other.g1.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g3.x) * other.g0 + vec4(self.g3.y) * vec4(other.g0.y) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g3.z) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g3.w) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.x) * vec4(other.g3.x) * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g1.y) * other.g3.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * other.g3.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * other.g3.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + vec4(self.g3.x) * other.g1 + vec4(self.g3.y) * vec4(other.g1.y) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g3.z) * vec4(other.g1.z) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g3.w) * vec4(other.g1.w) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g1.x) * vec4(other.g3.x) * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g0.z) * other.g1.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.w) * other.g1.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g1.y) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * other.g0.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g1.w) * other.g0.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g2.x) * other.g3 + vec4(self.g2.z) * vec4(other.g3.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g2.w) * vec4(other.g3.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g3.x) * other.g2 + vec4(self.g3.y) * vec4(other.g2.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g3.z) * vec4(other.g2.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g3.w) * vec4(other.g2.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.x, self.g2.y, self.g0.y, self.g0.y) * vec4(other.g1.x, other.g3.x, other.g1.w, other.g1.z) * vec4(0.0, 1.0, -1.0, 1.0), vec4(self.g1.z) * other.g1.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.w) * other.g1.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g3.x) * other.g3 + vec4(self.g3.z) * vec4(other.g3.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g3.w) * vec4(other.g3.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x, self.g3.y, self.g1.y, self.g1.y) * vec4(other.g1.x, other.g3.x, other.g1.w, other.g1.z) * vec4(0.0, 1.0, -1.0, 1.0));
}

MultiVector multi_vector_multi_vector_outer_product(MultiVector self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g0 + vec4(self.g2.y) * other.g2.wwwz * vec4(0.0, 0.0, -1.0, 1.0) + vec4(self.g2.z) * other.g2.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g2.w) * other.g2.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + self.g0 * vec4(other.g0.x) * vec4(0.0, 1.0, 1.0, 1.0), vec4(self.g0.x) * other.g1 + vec4(self.g1.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g2.y) * other.g3.wwwz * vec4(0.0, 0.0, -1.0, 1.0) + vec4(self.g2.z) * other.g3.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g2.w) * other.g3.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g3.y) * other.g2.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g3.z) * other.g2.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g3.w) * other.g2.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + self.g0 * vec4(other.g1.x) * vec4(0.0, -1.0, -1.0, -1.0), vec4(self.g0.x) * other.g2 + vec4(self.g0.z) * vec4(other.g2.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g2.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g2.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g2.y) * other.g0.yxyy * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g2.z) * other.g0.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g2.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + self.g0.yxxx * other.g2.yxxx * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g0.x) * other.g3 + vec4(self.g0.z) * vec4(other.g3.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g3.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.x) * other.g2 + vec4(self.g1.y) * vec4(other.g2.y) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g2.z) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g1.w) * vec4(other.g2.w) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g2.x) * vec4(other.g1.x) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g2.y) * other.g1.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g2.z) * other.g1.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g2.w) * other.g1.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g3.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g3.y) * other.g0.yxyy * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g3.z) * other.g0.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g3.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + self.g0.yxxx * other.g3.yxxx * vec4(1.0, 0.0, 0.0, 0.0));
}

MultiVector multi_vector_multi_vector_inner_product(MultiVector self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * other.g1 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g1.y) * other.g1.yxyy * vec4(-1.0, -1.0, 0.0, 0.0) + vec4(self.g1.z) * other.g1.zzxz * vec4(-1.0, 0.0, -1.0, 0.0) + vec4(self.g1.w) * other.g1.wwwx * vec4(-1.0, 0.0, 0.0, -1.0) + vec4(self.g2.x) * other.g2 * vec4(-1.0, 1.0, 1.0, 1.0) + vec4(self.g2.y) * other.g2.yxyy * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g2.z) * other.g2.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g2.w) * other.g2.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + vec4(self.g3.x) * other.g3 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g3.y) * other.g3.yxyy * vec4(-1.0, -1.0, 0.0, 0.0) + vec4(self.g3.z) * other.g3.zzxz * vec4(-1.0, 0.0, -1.0, 0.0) + vec4(self.g3.w) * other.g3.wwwx * vec4(-1.0, 0.0, 0.0, -1.0) + self.g0.yyxx * other.g0.yxxx * vec4(-1.0, 1.0, 0.0, 0.0), vec4(self.g0.x) * other.g1 + vec4(self.g0.z) * vec4(other.g1.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g1.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y) * other.g0.yxyy * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * other.g0.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + vec4(self.g2.x) * vec4(other.g3.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g2.y) * other.g3.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g2.z) * other.g3.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g2.w) * other.g3.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) - vec4(self.g3.x) * other.g2 + vec4(self.g3.y) * vec4(other.g2.y) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g3.z) * vec4(other.g2.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g3.w) * vec4(other.g2.w) * vec4(1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * other.g1.yxxx * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g0.x) * other.g2 + vec4(self.g0.z) * other.g2.wwxy * vec4(0.0, -1.0, -1.0, 1.0) + vec4(self.g0.w) * other.g2.zzyx * vec4(0.0, 1.0, -1.0, -1.0) + vec4(self.g1.x) * other.g3 + vec4(self.g1.y) * other.g3.xxwz * vec4(0.0, 1.0, -1.0, 1.0) + vec4(self.g1.z) * other.g3.wwxy * vec4(0.0, 1.0, 1.0, -1.0) + vec4(self.g1.w) * other.g3.zzyx * vec4(0.0, -1.0, 1.0, 1.0) + vec4(self.g2.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g2.y) * other.g0.xxwz * vec4(0.0, 1.0, 1.0, -1.0) + vec4(self.g2.z) * other.g0.wwxy * vec4(0.0, -1.0, 1.0, 1.0) + vec4(self.g2.w) * other.g0.zzyx * vec4(0.0, 1.0, -1.0, 1.0) - vec4(self.g3.x) * other.g1 + vec4(self.g3.y) * other.g1.xxwz * vec4(0.0, -1.0, 1.0, -1.0) + vec4(self.g3.z) * other.g1.wwxy * vec4(0.0, -1.0, -1.0, 1.0) + vec4(self.g3.w) * other.g1.zzyx * vec4(0.0, 1.0, -1.0, -1.0) + self.g0.xyyy * other.g2.xxwz * vec4(0.0, -1.0, 1.0, -1.0), vec4(self.g0.x) * other.g3 + vec4(self.g1.y) * other.g2.wwwz * vec4(0.0, 0.0, -1.0, 1.0) + vec4(self.g1.z) * other.g2.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.w) * other.g2.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g2.y) * other.g1.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g2.z) * other.g1.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g2.w) * other.g1.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g3.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g3.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g3.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g3.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + self.g0 * vec4(other.g3.x) * vec4(0.0, -1.0, -1.0, -1.0));
}

MultiVector multi_vector_multi_vector_left_contraction(MultiVector self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * vec4(other.g0.z) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.w) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g1.x) * other.g1 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g1.y) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g1.z) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g1.w) * vec4(other.g1.w) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g2.x) * vec4(other.g2.x) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g2.y) * other.g2.yxyy * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g2.z) * other.g2.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g2.w) * other.g2.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + vec4(self.g3.x) * vec4(other.g3.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g3.y) * other.g3.yxyy * vec4(-1.0, -1.0, 0.0, 0.0) + vec4(self.g3.z) * other.g3.zzxz * vec4(-1.0, 0.0, -1.0, 0.0) + vec4(self.g3.w) * other.g3.wwwx * vec4(-1.0, 0.0, 0.0, -1.0) + self.g0.yxxx * other.g0.yxxx * vec4(-1.0, 0.0, 0.0, 0.0), vec4(self.g0.x) * other.g1 + vec4(self.g0.z) * vec4(other.g1.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g1.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g2.x) * vec4(other.g3.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g2.y) * other.g3.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g2.z) * other.g3.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g2.w) * other.g3.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + self.g0.yxxx * other.g1.yxxx * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g0.x) * other.g2 + vec4(self.g1.x) * other.g3 + vec4(self.g1.y) * vec4(other.g3.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g3.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * vec4(other.g3.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g2.y) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g2.z) * other.g0.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g2.w) * other.g0.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g3.y) * other.g1.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g3.z) * other.g1.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g3.w) * other.g1.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + self.g0 * vec4(other.g2.x) * vec4(0.0, -1.0, -1.0, -1.0), vec4(self.g0.x) * other.g3 + vec4(self.g2.y) * other.g1.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g2.z) * other.g1.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g2.w) * other.g1.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + self.g0 * vec4(other.g3.x) * vec4(0.0, -1.0, -1.0, -1.0));
}

MultiVector multi_vector_multi_vector_right_contraction(MultiVector self, MultiVector other) {
    return MultiVector(vec4(self.g0.y) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.z) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * vec4(other.g1.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y) * other.g1.yxyy * vec4(-1.0, -1.0, 0.0, 0.0) + vec4(self.g1.z) * other.g1.zzxz * vec4(-1.0, 0.0, -1.0, 0.0) + vec4(self.g1.w) * other.g1.wwwx * vec4(-1.0, 0.0, 0.0, -1.0) + vec4(self.g2.x) * other.g2 * vec4(-1.0, 1.0, 1.0, 1.0) + vec4(self.g2.y) * vec4(other.g2.y) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g2.z) * vec4(other.g2.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g2.w) * vec4(other.g2.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g3.x) * other.g3 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g3.y) * vec4(other.g3.y) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g3.z) * vec4(other.g3.z) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g3.w) * vec4(other.g3.w) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g1.y) * other.g0.yxyy * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * other.g0.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, 1.0) - vec4(self.g3.x) * other.g2 + vec4(self.g3.y) * vec4(other.g2.y) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g3.z) * vec4(other.g2.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g3.w) * vec4(other.g2.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g0.z) * other.g2.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * other.g2.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g1.y) * other.g3.wwwz * vec4(0.0, 0.0, -1.0, 1.0) + vec4(self.g1.z) * other.g3.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.w) * other.g3.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g2.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g2.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g2.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) - vec4(self.g3.x) * other.g1 + vec4(self.g3.y) * vec4(other.g1.x) * vec4(0.0, -1.0, 0.0, 0.0) + vec4(self.g3.z) * vec4(other.g1.x) * vec4(0.0, 0.0, -1.0, 0.0) + vec4(self.g3.w) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.x, self.g2.y, self.g0.y, self.g0.y) * vec4(other.g2.x, other.g0.x, other.g2.w, other.g2.z) * vec4(0.0, 1.0, 1.0, -1.0), vec4(self.g1.z) * other.g2.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.w) * other.g2.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g3.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g3.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g3.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x, self.g3.y, self.g1.y, self.g1.y) * vec4(other.g2.x, other.g0.x, other.g2.w, other.g2.z) * vec4(0.0, 1.0, -1.0, 1.0));
}

Scalar multi_vector_multi_vector_scalar_product(MultiVector self, MultiVector other) {
    return Scalar(self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g0.w * other.g0.w + self.g1.x * other.g1.x - self.g1.y * other.g1.y - self.g1.z * other.g1.z - self.g1.w * other.g1.w - self.g2.x * other.g2.x + self.g2.y * other.g2.y + self.g2.z * other.g2.z + self.g2.w * other.g2.w + self.g3.x * other.g3.x - self.g3.y * other.g3.y - self.g3.z * other.g3.z - self.g3.w * other.g3.w);
}

Rotor multi_vector_rotor_into(MultiVector self) {
    return Rotor(self.g0);
}

MultiVector multi_vector_rotor_add(MultiVector self, Rotor other) {
    return MultiVector(self.g0 + other.g0, self.g1, self.g2, self.g3);
}

MultiVector multi_vector_rotor_sub(MultiVector self, Rotor other) {
    return MultiVector(self.g0 - other.g0, self.g1, self.g2, self.g3);
}

MultiVector multi_vector_rotor_geometric_product(MultiVector self, Rotor other) {
    return MultiVector(vec4(self.g0.x) * other.g0 + vec4(self.g0.y) * other.g0.yxwz * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g0.zwxy * vec4(-1.0, -1.0, 1.0, 1.0) + vec4(self.g0.w) * other.g0.wzyx * vec4(-1.0, 1.0, -1.0, 1.0), vec4(self.g1.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g1.y) * other.g0.yxwz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g1.z) * other.g0.zwxy * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g1.w) * other.g0.wzyx * vec4(1.0, 1.0, -1.0, 1.0), vec4(self.g2.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g2.y) * other.g0.yxwz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g2.z) * other.g0.zwxy * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g2.w) * other.g0.wzyx * vec4(1.0, 1.0, -1.0, 1.0), vec4(self.g3.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g3.y) * other.g0.yxwz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g3.z) * other.g0.zwxy * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g3.w) * other.g0.wzyx * vec4(1.0, 1.0, -1.0, 1.0));
}

MultiVector multi_vector_rotor_outer_product(MultiVector self, Rotor other) {
    return MultiVector(vec4(self.g0.x) * other.g0 + self.g0 * vec4(other.g0.x) * vec4(0.0, 1.0, 1.0, 1.0), vec4(self.g1.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + self.g1 * vec4(other.g0.x) * vec4(0.0, 1.0, 1.0, 1.0), vec4(self.g2.y) * other.g0.yxyy * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g2.z) * other.g0.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g2.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + vec4(self.g2.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g3.y) * other.g0.yxyy * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g3.z) * other.g0.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g3.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + vec4(self.g3.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0));
}

MultiVector multi_vector_rotor_inner_product(MultiVector self, Rotor other) {
    return MultiVector(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + self.g0.yyxx * other.g0.yxxx * vec4(-1.0, 1.0, 0.0, 0.0), vec4(self.g1.y) * other.g0.yxyy * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * other.g0.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g2.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g2.z) * other.g0.wwxy * vec4(0.0, -1.0, 1.0, 1.0) + vec4(self.g2.w) * other.g0.zzyx * vec4(0.0, 1.0, -1.0, 1.0) + self.g2.xyyy * other.g0.xxwz * vec4(0.0, 1.0, 1.0, -1.0), vec4(self.g3.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + self.g3 * vec4(other.g0.x) * vec4(0.0, 1.0, 1.0, 1.0));
}

MultiVector multi_vector_rotor_right_contraction(MultiVector self, Rotor other) {
    return MultiVector(vec4(self.g0.y) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.z) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g1.y) * other.g0.yxyy * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * other.g0.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g2.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + self.g2 * vec4(other.g0.x) * vec4(0.0, 1.0, 1.0, 1.0), vec4(self.g3.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + self.g3 * vec4(other.g0.x) * vec4(0.0, 1.0, 1.0, 1.0));
}

Scalar multi_vector_rotor_scalar_product(MultiVector self, Rotor other) {
    return Scalar(self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g0.w * other.g0.w);
}

Point multi_vector_point_into(MultiVector self) {
    return Point(vec4(self.g2.x, self.g1.y, self.g1.z, self.g1.w));
}

MultiVector multi_vector_point_add(MultiVector self, Point other) {
    return MultiVector(self.g0, self.g1 + other.g0 * vec4(0.0, 1.0, 1.0, 1.0), self.g2 + vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0), self.g3);
}

MultiVector multi_vector_point_sub(MultiVector self, Point other) {
    return MultiVector(self.g0, self.g1 - other.g0 * vec4(0.0, 1.0, 1.0, 1.0), self.g2 - vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0), self.g3);
}

MultiVector multi_vector_point_geometric_product(MultiVector self, Point other) {
    return MultiVector(vec4(self.g1.y) * other.g0.yywz * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * other.g0.zwzy * vec4(-1.0, -1.0, 0.0, 1.0) + vec4(self.g1.w) * other.g0.wzyw * vec4(-1.0, 1.0, -1.0, 0.0) + vec4(self.g2.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g2.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g2.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g2.x, self.g1.x, self.g1.x, self.g1.x) * other.g0 * vec4(-1.0), vec4(self.g0.y) * other.g0.yywz * vec4(1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * other.g0.zwzy * vec4(1.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * other.g0.wzyw * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g3.y) * vec4(other.g0.x) * vec4(0.0, -1.0, 0.0, 0.0) + vec4(self.g3.z) * vec4(other.g0.x) * vec4(0.0, 0.0, -1.0, 0.0) + vec4(self.g3.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g3.x, self.g0.x, self.g0.x, self.g0.x) * other.g0 * vec4(-1.0, 1.0, 1.0, 1.0), vec4(self.g3.x) * other.g0.yyzw * vec4(0.0, -1.0, -1.0, -1.0) + vec4(self.g3.y) * other.g0.yywz * vec4(1.0, 0.0, 1.0, -1.0) + vec4(self.g3.z) * other.g0.zwzy * vec4(1.0, -1.0, 0.0, 1.0) + vec4(self.g3.w) * other.g0.wzyw * vec4(1.0, 1.0, -1.0, 0.0) + self.g0 * vec4(other.g0.x) * vec4(1.0, -1.0, -1.0, -1.0), vec4(self.g2.x) * other.g0.yyzw * vec4(0.0, -1.0, -1.0, -1.0) + vec4(self.g2.y) * other.g0.yywz * vec4(1.0, 0.0, 1.0, -1.0) + vec4(self.g2.z) * other.g0.zwzy * vec4(1.0, -1.0, 0.0, 1.0) + vec4(self.g2.w) * other.g0.wzyw * vec4(1.0, 1.0, -1.0, 0.0) + self.g1 * vec4(other.g0.x));
}

Scalar multi_vector_point_scalar_product(MultiVector self, Point other) {
    return Scalar(0.0 - self.g1.y * other.g0.y - self.g1.z * other.g0.z - self.g1.w * other.g0.w - self.g2.x * other.g0.x);
}

IdealPoint multi_vector_ideal_point_into(MultiVector self) {
    return IdealPoint(vec3(self.g3.y, self.g3.z, self.g3.w));
}

MultiVector multi_vector_ideal_point_add(MultiVector self, IdealPoint other) {
    return MultiVector(self.g0, self.g1, self.g2, self.g3 + vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0));
}

MultiVector multi_vector_ideal_point_sub(MultiVector self, IdealPoint other) {
    return MultiVector(self.g0, self.g1, self.g2, self.g3 - vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0));
}

MultiVector multi_vector_ideal_point_geometric_product(MultiVector self, IdealPoint other) {
    return MultiVector(vec4(self.g3.y) * vec4(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g3.z) * vec4(other.g0.y, other.g0.z, other.g0.y, other.g0.x) * vec4(-1.0, -1.0, 0.0, 1.0) + vec4(self.g3.w) * vec4(other.g0.z, other.g0.y, other.g0.x, other.g0.z) * vec4(-1.0, 1.0, -1.0, 0.0) + vec4(self.g3.x) * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, -1.0, -1.0, -1.0), vec4(self.g2.y) * vec4(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4(-1.0, 0.0, -1.0, 1.0) + vec4(self.g2.z) * vec4(other.g0.y, other.g0.z, other.g0.y, other.g0.x) * vec4(-1.0, 1.0, 0.0, -1.0) + vec4(self.g2.w) * vec4(other.g0.z, other.g0.y, other.g0.x, other.g0.z) * vec4(-1.0, -1.0, 1.0, 0.0) + vec4(self.g2.x) * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0), vec4(self.g1.y) * vec4(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4(-1.0, 0.0, -1.0, 1.0) + vec4(self.g1.z) * vec4(other.g0.y, other.g0.z, other.g0.y, other.g0.x) * vec4(-1.0, 1.0, 0.0, -1.0) + vec4(self.g1.w) * vec4(other.g0.z, other.g0.y, other.g0.x, other.g0.z) * vec4(-1.0, -1.0, 1.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0), vec4(self.g0.y) * vec4(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4(1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.z, other.g0.y, other.g0.x) * vec4(1.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * vec4(other.g0.z, other.g0.y, other.g0.x, other.g0.z) * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0));
}

Scalar multi_vector_ideal_point_scalar_product(MultiVector self, IdealPoint other) {
    return Scalar(0.0 - self.g3.y * other.g0.x - self.g3.z * other.g0.y - self.g3.w * other.g0.z);
}

Plane multi_vector_plane_into(MultiVector self) {
    return Plane(vec4(self.g1.x, self.g2.y, self.g2.z, self.g2.w));
}

MultiVector multi_vector_plane_add(MultiVector self, Plane other) {
    return MultiVector(self.g0, self.g1 + vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0), self.g2 + other.g0 * vec4(0.0, 1.0, 1.0, 1.0), self.g3);
}

MultiVector multi_vector_plane_sub(MultiVector self, Plane other) {
    return MultiVector(self.g0, self.g1 - vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0), self.g2 - other.g0 * vec4(0.0, 1.0, 1.0, 1.0), self.g3);
}

MultiVector multi_vector_plane_geometric_product(MultiVector self, Plane other) {
    return MultiVector(vec4(self.g2.x) * other.g0.yyzw * vec4(0.0, 1.0, 1.0, 1.0) + vec4(self.g2.y) * other.g0.yywz * vec4(1.0, 0.0, -1.0, 1.0) + vec4(self.g2.z) * other.g0.zwzy * vec4(1.0, 1.0, 0.0, -1.0) + vec4(self.g2.w) * other.g0.wzyw * vec4(1.0, -1.0, 1.0, 0.0) + self.g1 * vec4(other.g0.x) * vec4(1.0, -1.0, -1.0, -1.0), vec4(self.g3.x) * other.g0.yyzw * vec4(0.0, -1.0, -1.0, -1.0) + vec4(self.g3.y) * other.g0.yywz * vec4(1.0, 0.0, 1.0, -1.0) + vec4(self.g3.z) * other.g0.zwzy * vec4(1.0, -1.0, 0.0, 1.0) + vec4(self.g3.w) * other.g0.wzyw * vec4(1.0, 1.0, -1.0, 0.0) + self.g0 * vec4(other.g0.x) * vec4(1.0, -1.0, -1.0, -1.0), vec4(self.g0.y) * other.g0.yywz * vec4(1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * other.g0.zwzy * vec4(1.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * other.g0.wzyw * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g3.y) * vec4(other.g0.x) * vec4(0.0, -1.0, 0.0, 0.0) + vec4(self.g3.z) * vec4(other.g0.x) * vec4(0.0, 0.0, -1.0, 0.0) + vec4(self.g3.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g3.x, self.g0.x, self.g0.x, self.g0.x) * other.g0 * vec4(-1.0, 1.0, 1.0, 1.0), vec4(self.g1.y) * other.g0.yywz * vec4(-1.0, 0.0, -1.0, 1.0) + vec4(self.g1.z) * other.g0.zwzy * vec4(-1.0, 1.0, 0.0, -1.0) + vec4(self.g1.w) * other.g0.wzyw * vec4(-1.0, -1.0, 1.0, 0.0) + vec4(self.g2.y) * vec4(other.g0.x) * vec4(0.0, -1.0, 0.0, 0.0) + vec4(self.g2.z) * vec4(other.g0.x) * vec4(0.0, 0.0, -1.0, 0.0) + vec4(self.g2.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g2.x, self.g1.x, self.g1.x, self.g1.x) * other.g0 * vec4(-1.0, 1.0, 1.0, 1.0));
}

Scalar multi_vector_plane_scalar_product(MultiVector self, Plane other) {
    return Scalar(self.g1.x * other.g0.x + self.g2.y * other.g0.y + self.g2.z * other.g0.z + self.g2.w * other.g0.w);
}

Line multi_vector_line_into(MultiVector self) {
    return Line(vec3(self.g3.y, self.g3.z, self.g3.w), vec3(self.g0.y, self.g0.z, self.g0.w));
}

MultiVector multi_vector_line_add(MultiVector self, Line other) {
    return MultiVector(self.g0 + vec4(other.g0.x, other.g1.x, other.g1.y, other.g1.z) * vec4(0.0, 1.0, 1.0, 1.0), self.g1, self.g2, self.g3 + vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0));
}

MultiVector multi_vector_line_sub(MultiVector self, Line other) {
    return MultiVector(self.g0 - vec4(other.g0.x, other.g1.x, other.g1.y, other.g1.z) * vec4(0.0, 1.0, 1.0, 1.0), self.g1, self.g2, self.g3 - vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0));
}

MultiVector multi_vector_line_geometric_product(MultiVector self, Line other) {
    return MultiVector(vec4(self.g0.y) * vec4(other.g1.x, other.g1.x, other.g1.z, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.z, other.g1.y, other.g1.x) * vec4(-1.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * vec4(other.g1.z, other.g1.y, other.g1.x, other.g1.z) * vec4(-1.0, 1.0, -1.0, 0.0) + vec4(self.g3.x) * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, -1.0, -1.0, -1.0) + vec4(self.g3.y) * vec4(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g3.z) * vec4(other.g0.y, other.g0.z, other.g0.y, other.g0.x) * vec4(-1.0, -1.0, 0.0, 1.0) + vec4(self.g3.w) * vec4(other.g0.z, other.g0.y, other.g0.x, other.g0.z) * vec4(-1.0, 1.0, -1.0, 0.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.x, other.g1.y, other.g1.z) * vec4(0.0, 1.0, 1.0, 1.0), vec4(self.g1.y) * vec4(other.g1.x, other.g1.x, other.g1.z, other.g1.y) * vec4(1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.y, other.g1.z, other.g1.y, other.g1.x) * vec4(1.0, -1.0, 0.0, 1.0) + vec4(self.g1.w) * vec4(other.g1.z, other.g1.y, other.g1.x, other.g1.z) * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g2.x) * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0) + vec4(self.g2.y) * vec4(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4(-1.0, 0.0, -1.0, 1.0) + vec4(self.g2.z) * vec4(other.g0.y, other.g0.z, other.g0.y, other.g0.x) * vec4(-1.0, 1.0, 0.0, -1.0) + vec4(self.g2.w) * vec4(other.g0.z, other.g0.y, other.g0.x, other.g0.z) * vec4(-1.0, -1.0, 1.0, 0.0) + vec4(self.g1.x) * vec4(other.g1.x, other.g1.x, other.g1.y, other.g1.z) * vec4(0.0, -1.0, -1.0, -1.0), vec4(self.g1.y) * vec4(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4(-1.0, 0.0, -1.0, 1.0) + vec4(self.g1.z) * vec4(other.g0.y, other.g0.z, other.g0.y, other.g0.x) * vec4(-1.0, 1.0, 0.0, -1.0) + vec4(self.g1.w) * vec4(other.g0.z, other.g0.y, other.g0.x, other.g0.z) * vec4(-1.0, -1.0, 1.0, 0.0) + vec4(self.g2.x) * vec4(other.g1.x, other.g1.x, other.g1.y, other.g1.z) * vec4(0.0, -1.0, -1.0, -1.0) + vec4(self.g2.y) * vec4(other.g1.x, other.g1.x, other.g1.z, other.g1.y) * vec4(1.0, 0.0, 1.0, -1.0) + vec4(self.g2.z) * vec4(other.g1.y, other.g1.z, other.g1.y, other.g1.x) * vec4(1.0, -1.0, 0.0, 1.0) + vec4(self.g2.w) * vec4(other.g1.z, other.g1.y, other.g1.x, other.g1.z) * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0), vec4(self.g0.y) * vec4(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4(1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.z, other.g0.y, other.g0.x) * vec4(1.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * vec4(other.g0.z, other.g0.y, other.g0.x, other.g0.z) * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g3.x) * vec4(other.g1.x, other.g1.x, other.g1.y, other.g1.z) * vec4(0.0, -1.0, -1.0, -1.0) + vec4(self.g3.y) * vec4(other.g1.x, other.g1.x, other.g1.z, other.g1.y) * vec4(1.0, 0.0, 1.0, -1.0) + vec4(self.g3.z) * vec4(other.g1.y, other.g1.z, other.g1.y, other.g1.x) * vec4(1.0, -1.0, 0.0, 1.0) + vec4(self.g3.w) * vec4(other.g1.z, other.g1.y, other.g1.x, other.g1.z) * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0));
}

Scalar multi_vector_line_scalar_product(MultiVector self, Line other) {
    return Scalar(0.0 - self.g0.y * other.g1.x - self.g0.z * other.g1.y - self.g0.w * other.g1.z - self.g3.y * other.g0.x - self.g3.z * other.g0.y - self.g3.w * other.g0.z);
}

Translator multi_vector_translator_into(MultiVector self) {
    return Translator(vec4(self.g0.x, self.g3.y, self.g3.z, self.g3.w));
}

MultiVector multi_vector_translator_add(MultiVector self, Translator other) {
    return MultiVector(self.g0 + vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0), self.g1, self.g2, self.g3 + other.g0 * vec4(0.0, 1.0, 1.0, 1.0));
}

MultiVector multi_vector_translator_sub(MultiVector self, Translator other) {
    return MultiVector(self.g0 - vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0), self.g1, self.g2, self.g3 - other.g0 * vec4(0.0, 1.0, 1.0, 1.0));
}

MultiVector multi_vector_translator_geometric_product(MultiVector self, Translator other) {
    return MultiVector(vec4(self.g3.x) * other.g0.yyzw * vec4(0.0, -1.0, -1.0, -1.0) + vec4(self.g3.y) * other.g0.yywz * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g3.z) * other.g0.zwzy * vec4(-1.0, -1.0, 0.0, 1.0) + vec4(self.g3.w) * other.g0.wzyw * vec4(-1.0, 1.0, -1.0, 0.0) + self.g0 * vec4(other.g0.x), vec4(self.g2.x) * other.g0.yyzw * vec4(0.0, 1.0, 1.0, 1.0) + vec4(self.g2.y) * other.g0.yywz * vec4(-1.0, 0.0, -1.0, 1.0) + vec4(self.g2.z) * other.g0.zwzy * vec4(-1.0, 1.0, 0.0, -1.0) + vec4(self.g2.w) * other.g0.wzyw * vec4(-1.0, -1.0, 1.0, 0.0) + self.g1 * vec4(other.g0.x), vec4(self.g1.y) * other.g0.yywz * vec4(-1.0, 0.0, -1.0, 1.0) + vec4(self.g1.z) * other.g0.zwzy * vec4(-1.0, 1.0, 0.0, -1.0) + vec4(self.g1.w) * other.g0.wzyw * vec4(-1.0, -1.0, 1.0, 0.0) + vec4(self.g2.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g2.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g2.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g2.x, self.g1.x, self.g1.x, self.g1.x) * other.g0, vec4(self.g0.y) * other.g0.yywz * vec4(1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * other.g0.zwzy * vec4(1.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * other.g0.wzyw * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g3.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g3.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g3.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g3.x, self.g0.x, self.g0.x, self.g0.x) * other.g0);
}

MultiVector multi_vector_translator_outer_product(MultiVector self, Translator other) {
    return MultiVector(self.g0 * vec4(other.g0.x), vec4(self.g2.y) * other.g0.wwwz * vec4(0.0, 0.0, -1.0, 1.0) + vec4(self.g2.z) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g2.w) * other.g0.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + self.g1 * vec4(other.g0.x), self.g2 * vec4(other.g0.x), vec4(self.g0.z) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g3.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g3.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g3.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g3.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + self.g0.yxxx * other.g0.yyzw);
}

MultiVector multi_vector_translator_inner_product(MultiVector self, Translator other) {
    return MultiVector(vec4(self.g3.x) * other.g0.yyzw * vec4(0.0, -1.0, -1.0, -1.0) + vec4(self.g3.y) * vec4(other.g0.y) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g3.z) * vec4(other.g0.z) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g3.w) * vec4(other.g0.w) * vec4(-1.0, 0.0, 0.0, 0.0) + self.g0 * vec4(other.g0.x), vec4(self.g2.y) * vec4(other.g0.y) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g2.z) * vec4(other.g0.z) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g2.w) * vec4(other.g0.w) * vec4(-1.0, 0.0, 0.0, 0.0) + self.g1 * vec4(other.g0.x), vec4(self.g1.y) * other.g0.wwwz * vec4(0.0, 0.0, -1.0, 1.0) + vec4(self.g1.z) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.w) * other.g0.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g2.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g2.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g2.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g2.x, self.g1.x, self.g1.x, self.g1.x) * other.g0, vec4(self.g3.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g3.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g3.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g3.x, self.g0.x, self.g0.x, self.g0.x) * other.g0);
}

MultiVector multi_vector_translator_right_contraction(MultiVector self, Translator other) {
    return MultiVector(vec4(self.g3.x) * other.g0.yyzw * vec4(0.0, -1.0, -1.0, -1.0) + vec4(self.g3.y) * vec4(other.g0.y) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g3.z) * vec4(other.g0.z) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g3.w) * vec4(other.g0.w) * vec4(-1.0, 0.0, 0.0, 0.0) + self.g0 * vec4(other.g0.x), self.g1 * vec4(other.g0.x), vec4(self.g1.z) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.w) * other.g0.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g2.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g2.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g2.x, self.g2.y, self.g1.y, self.g1.y) * other.g0.xxwz * vec4(1.0, 1.0, -1.0, 1.0), self.g3 * vec4(other.g0.x));
}

Scalar multi_vector_translator_scalar_product(MultiVector self, Translator other) {
    return Scalar(self.g0.x * other.g0.x - self.g3.y * other.g0.y - self.g3.z * other.g0.z - self.g3.w * other.g0.w);
}

Motor multi_vector_motor_into(MultiVector self) {
    return Motor(self.g0, self.g3);
}

MultiVector multi_vector_motor_add(MultiVector self, Motor other) {
    return MultiVector(self.g0 + other.g0, self.g1, self.g2, self.g3 + other.g1);
}

MultiVector multi_vector_motor_sub(MultiVector self, Motor other) {
    return MultiVector(self.g0 - other.g0, self.g1, self.g2, self.g3 - other.g1);
}

MultiVector multi_vector_motor_geometric_product(MultiVector self, Motor other) {
    return MultiVector(vec4(self.g0.x) * other.g0 + vec4(self.g0.y) * other.g0.yxwz * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g0.zwxy * vec4(-1.0, -1.0, 1.0, 1.0) + vec4(self.g0.w) * other.g0.wzyx * vec4(-1.0, 1.0, -1.0, 1.0) + vec4(self.g3.x) * other.g1 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g3.y) * other.g1.yxwz * vec4(-1.0, -1.0, 1.0, -1.0) + vec4(self.g3.z) * other.g1.zwxy * vec4(-1.0, -1.0, -1.0, 1.0) + vec4(self.g3.w) * other.g1.wzyx * vec4(-1.0, 1.0, -1.0, -1.0), vec4(self.g1.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g1.y) * other.g0.yxwz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g1.z) * other.g0.zwxy * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g1.w) * other.g0.wzyx * vec4(1.0, 1.0, -1.0, 1.0) + vec4(self.g2.x) * other.g1 + vec4(self.g2.y) * other.g1.yxwz * vec4(-1.0, 1.0, -1.0, 1.0) + vec4(self.g2.z) * other.g1.zwxy * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g2.w) * other.g1.wzyx * vec4(-1.0, -1.0, 1.0, 1.0), vec4(self.g1.x) * other.g1 + vec4(self.g1.y) * other.g1.yxwz * vec4(-1.0, 1.0, -1.0, 1.0) + vec4(self.g1.z) * other.g1.zwxy * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.w) * other.g1.wzyx * vec4(-1.0, -1.0, 1.0, 1.0) + vec4(self.g2.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g2.y) * other.g0.yxwz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g2.z) * other.g0.zwxy * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g2.w) * other.g0.wzyx * vec4(1.0, 1.0, -1.0, 1.0), vec4(self.g0.x) * other.g1 + vec4(self.g0.y) * other.g1.yxwz * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g1.zwxy * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.w) * other.g1.wzyx * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g3.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g3.y) * other.g0.yxwz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g3.z) * other.g0.zwxy * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g3.w) * other.g0.wzyx * vec4(1.0, 1.0, -1.0, 1.0));
}

MultiVector multi_vector_motor_regressive_product(MultiVector self, Motor other) {
    return MultiVector(vec4(self.g0.y) * other.g1.yxyy * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g0.z) * other.g1.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g1.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + vec4(self.g3.x) * other.g0 + vec4(self.g3.y) * vec4(other.g0.y) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g3.z) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g3.w) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.x) * vec4(other.g1.x) * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g1.y) * other.g1.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * other.g1.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * other.g1.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * vec4(other.g1.x) * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g1.z) * other.g0.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g1.w) * other.g0.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g2.x) * other.g1 + vec4(self.g2.z) * vec4(other.g1.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g2.w) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x, self.g2.y, self.g1.y, self.g1.y) * vec4(other.g0.x, other.g1.x, other.g0.w, other.g0.z) * vec4(0.0, 1.0, 1.0, -1.0), vec4(self.g3.x) * other.g1 + self.g3 * vec4(other.g1.x) * vec4(0.0, 1.0, 1.0, 1.0));
}

MultiVector multi_vector_motor_outer_product(MultiVector self, Motor other) {
    return MultiVector(vec4(self.g0.x) * other.g0 + self.g0 * vec4(other.g0.x) * vec4(0.0, 1.0, 1.0, 1.0), vec4(self.g1.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g2.y) * other.g1.wwwz * vec4(0.0, 0.0, -1.0, 1.0) + vec4(self.g2.z) * other.g1.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g2.w) * other.g1.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + self.g1 * vec4(other.g0.x) * vec4(0.0, 1.0, 1.0, 1.0), vec4(self.g2.y) * other.g0.yxyy * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g2.z) * other.g0.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g2.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + vec4(self.g2.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g0.x) * other.g1 + vec4(self.g0.z) * vec4(other.g1.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g1.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g3.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g3.y) * other.g0.yxyy * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g3.z) * other.g0.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g3.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + self.g0.yxxx * other.g1.yxxx * vec4(1.0, 0.0, 0.0, 0.0));
}

MultiVector multi_vector_motor_inner_product(MultiVector self, Motor other) {
    return MultiVector(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + vec4(self.g3.x) * other.g1 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g3.y) * other.g1.yxyy * vec4(-1.0, -1.0, 0.0, 0.0) + vec4(self.g3.z) * other.g1.zzxz * vec4(-1.0, 0.0, -1.0, 0.0) + vec4(self.g3.w) * other.g1.wwwx * vec4(-1.0, 0.0, 0.0, -1.0) + self.g0.yyxx * other.g0.yxxx * vec4(-1.0, 1.0, 0.0, 0.0), vec4(self.g1.y) * other.g0.yxyy * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * other.g0.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + vec4(self.g2.x) * vec4(other.g1.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g2.y) * other.g1.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g2.z) * other.g1.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g2.w) * other.g1.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g1.x) * other.g1 + vec4(self.g1.z) * other.g1.wwxy * vec4(0.0, 1.0, 1.0, -1.0) + vec4(self.g1.w) * other.g1.zzyx * vec4(0.0, -1.0, 1.0, 1.0) + vec4(self.g2.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g2.y) * other.g0.xxwz * vec4(0.0, 1.0, 1.0, -1.0) + vec4(self.g2.z) * other.g0.wwxy * vec4(0.0, -1.0, 1.0, 1.0) + vec4(self.g2.w) * other.g0.zzyx * vec4(0.0, 1.0, -1.0, 1.0) + self.g1.xyyy * other.g1.xxwz * vec4(0.0, 1.0, -1.0, 1.0), vec4(self.g0.x) * other.g1 + vec4(self.g3.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g3.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g3.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g3.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + self.g0 * vec4(other.g1.x) * vec4(0.0, -1.0, -1.0, -1.0));
}

MultiVector multi_vector_motor_left_contraction(MultiVector self, Motor other) {
    return MultiVector(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * vec4(other.g0.z) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.w) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g3.x) * vec4(other.g1.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g3.y) * other.g1.yxyy * vec4(-1.0, -1.0, 0.0, 0.0) + vec4(self.g3.z) * other.g1.zzxz * vec4(-1.0, 0.0, -1.0, 0.0) + vec4(self.g3.w) * other.g1.wwwx * vec4(-1.0, 0.0, 0.0, -1.0) + self.g0.yxxx * other.g0.yxxx * vec4(-1.0, 0.0, 0.0, 0.0), vec4(self.g2.y) * other.g1.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g2.z) * other.g1.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g2.w) * other.g1.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + vec4(self.g2.x) * vec4(other.g1.x) * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g1.x) * other.g1 + vec4(self.g2.y) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g2.z) * other.g0.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g2.w) * other.g0.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + self.g1 * vec4(other.g1.x) * vec4(0.0, 1.0, 1.0, 1.0), vec4(self.g0.x) * other.g1 + self.g0 * vec4(other.g1.x) * vec4(0.0, -1.0, -1.0, -1.0));
}

MultiVector multi_vector_motor_right_contraction(MultiVector self, Motor other) {
    return MultiVector(vec4(self.g0.y) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.z) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + vec4(self.g3.x) * other.g1 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g3.y) * vec4(other.g1.y) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g3.z) * vec4(other.g1.z) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g3.w) * vec4(other.g1.w) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g1.y) * other.g0.yxyy * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * other.g0.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g1.z) * other.g1.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.w) * other.g1.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g2.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g2.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g2.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x, self.g2.y, self.g1.y, self.g1.y) * vec4(other.g1.x, other.g0.x, other.g1.w, other.g1.z) * vec4(0.0, 1.0, -1.0, 1.0), vec4(self.g3.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + self.g3 * vec4(other.g0.x) * vec4(0.0, 1.0, 1.0, 1.0));
}

Scalar multi_vector_motor_scalar_product(MultiVector self, Motor other) {
    return Scalar(self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g0.w * other.g0.w + self.g3.x * other.g1.x - self.g3.y * other.g1.y - self.g3.z * other.g1.z - self.g3.w * other.g1.w);
}

PointAndPlane multi_vector_point_and_plane_into(MultiVector self) {
    return PointAndPlane(vec4(self.g2.x, self.g1.y, self.g1.z, self.g1.w), vec4(self.g1.x, self.g2.y, self.g2.z, self.g2.w));
}

MultiVector multi_vector_point_and_plane_add(MultiVector self, PointAndPlane other) {
    return MultiVector(self.g0, self.g1 + vec4(other.g1.x, other.g0.y, other.g0.z, other.g0.w), self.g2 + vec4(other.g0.x, other.g1.y, other.g1.z, other.g1.w), self.g3);
}

MultiVector multi_vector_point_and_plane_sub(MultiVector self, PointAndPlane other) {
    return MultiVector(self.g0, self.g1 - vec4(other.g1.x, other.g0.y, other.g0.z, other.g0.w), self.g2 - vec4(other.g0.x, other.g1.y, other.g1.z, other.g1.w), self.g3);
}

MultiVector multi_vector_point_and_plane_geometric_product(MultiVector self, PointAndPlane other) {
    return MultiVector(vec4(self.g1.x) * vec4(other.g1.x, other.g0.y, other.g0.z, other.g0.w) * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g0.y, other.g1.x, other.g0.w, other.g0.z) * vec4(-1.0, -1.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.z, other.g0.w, other.g1.x, other.g0.y) * vec4(-1.0, -1.0, -1.0, 1.0) + vec4(self.g1.w) * vec4(other.g0.w, other.g0.z, other.g0.y, other.g1.x) * vec4(-1.0, 1.0, -1.0, -1.0) + vec4(self.g2.x) * vec4(other.g0.x, other.g1.y, other.g1.z, other.g1.w) * vec4(-1.0, 1.0, 1.0, 1.0) + vec4(self.g2.y) * vec4(other.g1.y, other.g0.x, other.g1.w, other.g1.z) * vec4(1.0, 1.0, -1.0, 1.0) + vec4(self.g2.z) * vec4(other.g1.z, other.g1.w, other.g0.x, other.g1.y) * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g2.w) * vec4(other.g1.w, other.g1.z, other.g1.y, other.g0.x) * vec4(1.0, -1.0, 1.0, 1.0), vec4(self.g0.x) * vec4(other.g1.x, other.g0.y, other.g0.z, other.g0.w) + vec4(self.g0.y) * vec4(other.g0.y, other.g1.x, other.g0.w, other.g0.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z, other.g0.w, other.g1.x, other.g0.y) * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.w) * vec4(other.g0.w, other.g0.z, other.g0.y, other.g1.x) * vec4(1.0, 1.0, -1.0, -1.0) - vec4(self.g3.x) * vec4(other.g0.x, other.g1.y, other.g1.z, other.g1.w) + vec4(self.g3.y) * vec4(other.g1.y, other.g0.x, other.g1.w, other.g1.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g3.z) * vec4(other.g1.z, other.g1.w, other.g0.x, other.g1.y) * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g3.w) * vec4(other.g1.w, other.g1.z, other.g1.y, other.g0.x) * vec4(1.0, 1.0, -1.0, -1.0), vec4(self.g0.x) * vec4(other.g0.x, other.g1.y, other.g1.z, other.g1.w) + vec4(self.g0.y) * vec4(other.g1.y, other.g0.x, other.g1.w, other.g1.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.z, other.g1.w, other.g0.x, other.g1.y) * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.w) * vec4(other.g1.w, other.g1.z, other.g1.y, other.g0.x) * vec4(1.0, 1.0, -1.0, -1.0) - vec4(self.g3.x) * vec4(other.g1.x, other.g0.y, other.g0.z, other.g0.w) + vec4(self.g3.y) * vec4(other.g0.y, other.g1.x, other.g0.w, other.g0.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g3.z) * vec4(other.g0.z, other.g0.w, other.g1.x, other.g0.y) * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g3.w) * vec4(other.g0.w, other.g0.z, other.g0.y, other.g1.x) * vec4(1.0, 1.0, -1.0, -1.0), vec4(self.g1.x) * vec4(other.g0.x, other.g1.y, other.g1.z, other.g1.w) + vec4(self.g1.y) * vec4(other.g1.y, other.g0.x, other.g1.w, other.g1.z) * vec4(-1.0, 1.0, -1.0, 1.0) + vec4(self.g1.z) * vec4(other.g1.z, other.g1.w, other.g0.x, other.g1.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.w) * vec4(other.g1.w, other.g1.z, other.g1.y, other.g0.x) * vec4(-1.0, -1.0, 1.0, 1.0) - vec4(self.g2.x) * vec4(other.g1.x, other.g0.y, other.g0.z, other.g0.w) + vec4(self.g2.y) * vec4(other.g0.y, other.g1.x, other.g0.w, other.g0.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g2.z) * vec4(other.g0.z, other.g0.w, other.g1.x, other.g0.y) * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g2.w) * vec4(other.g0.w, other.g0.z, other.g0.y, other.g1.x) * vec4(1.0, 1.0, -1.0, -1.0));
}

Scalar multi_vector_point_and_plane_scalar_product(MultiVector self, PointAndPlane other) {
    return Scalar(self.g1.x * other.g1.x - self.g1.y * other.g0.y - self.g1.z * other.g0.z - self.g1.w * other.g0.w - self.g2.x * other.g0.x + self.g2.y * other.g1.y + self.g2.z * other.g1.z + self.g2.w * other.g1.w);
}

Scalar multi_vector_squared_magnitude(MultiVector self) {
    return multi_vector_multi_vector_scalar_product(self, multi_vector_reversal(self));
}

Scalar multi_vector_magnitude(MultiVector self) {
    return Scalar(sqrt(multi_vector_squared_magnitude(self).g0));
}

MultiVector multi_vector_scale(MultiVector self, float other) {
    return multi_vector_scalar_geometric_product(self, Scalar(other));
}

MultiVector multi_vector_signum(MultiVector self) {
    return multi_vector_scalar_geometric_product(self, Scalar(1.0 / multi_vector_magnitude(self).g0));
}

MultiVector multi_vector_inverse(MultiVector self) {
    return multi_vector_scalar_geometric_product(multi_vector_reversal(self), Scalar(1.0 / multi_vector_squared_magnitude(self).g0));
}

Rotor rotor_zero() {
    return Rotor(vec4(0.0));
}

Rotor rotor_one() {
    return Rotor(vec4(1.0, 0.0, 0.0, 0.0));
}

Rotor rotor_neg(Rotor self) {
    return Rotor(self.g0 * vec4(-1.0));
}

Rotor rotor_automorphism(Rotor self) {
    return Rotor(self.g0);
}

Rotor rotor_reversal(Rotor self) {
    return Rotor(self.g0 * vec4(1.0, -1.0, -1.0, -1.0));
}

Rotor rotor_conjugation(Rotor self) {
    return Rotor(self.g0 * vec4(1.0, -1.0, -1.0, -1.0));
}

Scalar rotor_scalar_into(Rotor self) {
    return Scalar(self.g0.x);
}

Rotor rotor_scalar_add(Rotor self, Scalar other) {
    return Rotor(self.g0 + vec4(other.g0) * vec4(1.0, 0.0, 0.0, 0.0));
}

Rotor rotor_scalar_sub(Rotor self, Scalar other) {
    return Rotor(self.g0 - vec4(other.g0) * vec4(1.0, 0.0, 0.0, 0.0));
}

Rotor rotor_scalar_geometric_product(Rotor self, Scalar other) {
    return Rotor(self.g0 * vec4(other.g0));
}

Rotor rotor_scalar_outer_product(Rotor self, Scalar other) {
    return Rotor(self.g0 * vec4(other.g0));
}

Rotor rotor_scalar_inner_product(Rotor self, Scalar other) {
    return Rotor(self.g0 * vec4(other.g0));
}

Scalar rotor_scalar_left_contraction(Rotor self, Scalar other) {
    return Scalar(self.g0.x * other.g0);
}

Rotor rotor_scalar_right_contraction(Rotor self, Scalar other) {
    return Rotor(self.g0 * vec4(other.g0));
}

Scalar rotor_scalar_scalar_product(Rotor self, Scalar other) {
    return Scalar(self.g0.x * other.g0);
}

MultiVector rotor_multi_vector_add(Rotor self, MultiVector other) {
    return MultiVector(self.g0 + other.g0, other.g1, other.g2, other.g3);
}

MultiVector rotor_multi_vector_sub(Rotor self, MultiVector other) {
    return MultiVector(self.g0 - other.g0, vec4(0.0) - other.g1, vec4(0.0) - other.g2, vec4(0.0) - other.g3);
}

MultiVector rotor_multi_vector_geometric_product(Rotor self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g0 + vec4(self.g0.y) * other.g0.yxwz * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g0.zwxy * vec4(-1.0, -1.0, 1.0, 1.0) + vec4(self.g0.w) * other.g0.wzyx * vec4(-1.0, 1.0, -1.0, 1.0), vec4(self.g0.x) * other.g1 + vec4(self.g0.y) * other.g1.yxwz * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g1.zwxy * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.w) * other.g1.wzyx * vec4(1.0, 1.0, -1.0, -1.0), vec4(self.g0.x) * other.g2 + vec4(self.g0.y) * other.g2.yxwz * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g2.zwxy * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.w) * other.g2.wzyx * vec4(1.0, 1.0, -1.0, -1.0), vec4(self.g0.x) * other.g3 + vec4(self.g0.y) * other.g3.yxwz * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g3.zwxy * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.w) * other.g3.wzyx * vec4(1.0, 1.0, -1.0, -1.0));
}

MultiVector rotor_multi_vector_outer_product(Rotor self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g0 + self.g0 * vec4(other.g0.x) * vec4(0.0, 1.0, 1.0, 1.0), vec4(self.g0.x) * other.g1 + self.g0 * vec4(other.g1.x) * vec4(0.0, -1.0, -1.0, -1.0), vec4(self.g0.x) * other.g2 + vec4(self.g0.z) * vec4(other.g2.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g2.w) * vec4(1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * other.g2.yxxx * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g0.x) * other.g3 + vec4(self.g0.z) * vec4(other.g3.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g3.w) * vec4(1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * other.g3.yxxx * vec4(1.0, 0.0, 0.0, 0.0));
}

MultiVector rotor_multi_vector_inner_product(Rotor self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + self.g0.yyxx * other.g0.yxxx * vec4(-1.0, 1.0, 0.0, 0.0), vec4(self.g0.x) * other.g1 + vec4(self.g0.z) * vec4(other.g1.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g1.w) * vec4(1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * other.g1.yxxx * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g0.x) * other.g2 + vec4(self.g0.z) * other.g2.wwxy * vec4(0.0, -1.0, -1.0, 1.0) + vec4(self.g0.w) * other.g2.zzyx * vec4(0.0, 1.0, -1.0, -1.0) + self.g0.xyyy * other.g2.xxwz * vec4(0.0, -1.0, 1.0, -1.0), vec4(self.g0.x) * other.g3 + self.g0 * vec4(other.g3.x) * vec4(0.0, -1.0, -1.0, -1.0));
}

MultiVector rotor_multi_vector_left_contraction(Rotor self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * vec4(other.g0.z) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.w) * vec4(-1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * other.g0.yxxx * vec4(-1.0, 0.0, 0.0, 0.0), vec4(self.g0.x) * other.g1 + vec4(self.g0.z) * vec4(other.g1.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g1.w) * vec4(1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * other.g1.yxxx * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g0.x) * other.g2 + self.g0 * vec4(other.g2.x) * vec4(0.0, -1.0, -1.0, -1.0), vec4(self.g0.x) * other.g3 + self.g0 * vec4(other.g3.x) * vec4(0.0, -1.0, -1.0, -1.0));
}

Scalar rotor_multi_vector_scalar_product(Rotor self, MultiVector other) {
    return Scalar(self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g0.w * other.g0.w);
}

Rotor rotor_rotor_add(Rotor self, Rotor other) {
    return Rotor(self.g0 + other.g0);
}

Rotor rotor_rotor_sub(Rotor self, Rotor other) {
    return Rotor(self.g0 - other.g0);
}

Rotor rotor_rotor_mul(Rotor self, Rotor other) {
    return Rotor(self.g0 * other.g0);
}

Rotor rotor_rotor_div(Rotor self, Rotor other) {
    return Rotor(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.w) * vec4(1.0, 1.0, 1.0, 1.0) / vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.w) * vec4(1.0, 1.0, 1.0, 1.0));
}

Rotor rotor_rotor_geometric_product(Rotor self, Rotor other) {
    return Rotor(vec4(self.g0.x) * other.g0 + vec4(self.g0.y) * other.g0.yxwz * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g0.zwxy * vec4(-1.0, -1.0, 1.0, 1.0) + vec4(self.g0.w) * other.g0.wzyx * vec4(-1.0, 1.0, -1.0, 1.0));
}

Rotor rotor_rotor_outer_product(Rotor self, Rotor other) {
    return Rotor(vec4(self.g0.x) * other.g0 + self.g0 * vec4(other.g0.x) * vec4(0.0, 1.0, 1.0, 1.0));
}

Rotor rotor_rotor_inner_product(Rotor self, Rotor other) {
    return Rotor(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + self.g0.yyxx * other.g0.yxxx * vec4(-1.0, 1.0, 0.0, 0.0));
}

Rotor rotor_rotor_left_contraction(Rotor self, Rotor other) {
    return Rotor(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * vec4(other.g0.z) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.w) * vec4(-1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * other.g0.yxxx * vec4(-1.0, 0.0, 0.0, 0.0));
}

Rotor rotor_rotor_right_contraction(Rotor self, Rotor other) {
    return Rotor(vec4(self.g0.y) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.z) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0));
}

Scalar rotor_rotor_scalar_product(Rotor self, Rotor other) {
    return Scalar(self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g0.w * other.g0.w);
}

PointAndPlane rotor_point_geometric_product(Rotor self, Point other) {
    return PointAndPlane(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * other.g0.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * other.g0.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + self.g0.xxyy * other.g0.xxwz * vec4(0.0, 0.0, 1.0, -1.0), vec4(self.g0.z) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + self.g0.yyxx * other.g0.yxxx * vec4(1.0, -1.0, 0.0, 0.0));
}

Point rotor_point_outer_product(Rotor self, Point other) {
    return Point(vec4(self.g0.x) * other.g0);
}

PointAndPlane rotor_point_inner_product(Rotor self, Point other) {
    return PointAndPlane(vec4(self.g0.x) * other.g0, vec4(self.g0.z) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + self.g0.yyxx * other.g0.yxxx * vec4(1.0, -1.0, 0.0, 0.0));
}

PointAndPlane rotor_point_left_contraction(Rotor self, Point other) {
    return PointAndPlane(vec4(self.g0.x) * other.g0, vec4(self.g0.z) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + self.g0.yyxx * other.g0.yxxx * vec4(1.0, -1.0, 0.0, 0.0));
}

Scalar rotor_ideal_point_regressive_product(Rotor self, IdealPoint other) {
    return Scalar(self.g0.y * other.g0.x + self.g0.z * other.g0.y + self.g0.w * other.g0.z);
}

IdealPoint rotor_ideal_point_inner_product(Rotor self, IdealPoint other) {
    return IdealPoint(vec3(self.g0.x) * other.g0);
}

IdealPoint rotor_ideal_point_left_contraction(Rotor self, IdealPoint other) {
    return IdealPoint(vec3(self.g0.x) * other.g0);
}

PointAndPlane rotor_plane_geometric_product(Rotor self, Plane other) {
    return PointAndPlane(vec4(self.g0.z) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + self.g0.yyxx * other.g0.yxxx * vec4(1.0, -1.0, 0.0, 0.0), vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * other.g0.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * other.g0.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + self.g0.xxyy * other.g0.xxwz * vec4(0.0, 0.0, 1.0, -1.0));
}

PointAndPlane rotor_plane_outer_product(Rotor self, Plane other) {
    return PointAndPlane(vec4(self.g0.z) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + self.g0.yyxx * other.g0.yxxx * vec4(1.0, -1.0, 0.0, 0.0), vec4(self.g0.x) * other.g0);
}

Plane rotor_plane_inner_product(Rotor self, Plane other) {
    return Plane(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * other.g0.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * other.g0.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + self.g0.xxyy * other.g0.xxwz * vec4(0.0, 0.0, 1.0, -1.0));
}

Plane rotor_plane_left_contraction(Rotor self, Plane other) {
    return Plane(vec4(self.g0.x) * other.g0);
}

Motor rotor_line_geometric_product(Rotor self, Line other) {
    return Motor(vec4(self.g0.y) * vec4(other.g1.x, other.g1.x, other.g1.z, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.z, other.g1.y, other.g1.x) * vec4(-1.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * vec4(other.g1.z, other.g1.y, other.g1.x, other.g1.z) * vec4(-1.0, 1.0, -1.0, 0.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.x, other.g1.y, other.g1.z) * vec4(0.0, 1.0, 1.0, 1.0), vec4(self.g0.y) * vec4(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4(1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.z, other.g0.y, other.g0.x) * vec4(1.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * vec4(other.g0.z, other.g0.y, other.g0.x, other.g0.z) * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0));
}

Scalar rotor_line_regressive_product(Rotor self, Line other) {
    return Scalar(self.g0.y * other.g0.x + self.g0.z * other.g0.y + self.g0.w * other.g0.z);
}

Scalar rotor_line_right_contraction(Rotor self, Line other) {
    return Scalar(0.0 - self.g0.y * other.g1.x - self.g0.z * other.g1.y - self.g0.w * other.g1.z);
}

Scalar rotor_line_scalar_product(Rotor self, Line other) {
    return Scalar(0.0 - self.g0.y * other.g1.x - self.g0.z * other.g1.y - self.g0.w * other.g1.z);
}

Motor rotor_translator_geometric_product(Rotor self, Translator other) {
    return Motor(self.g0 * vec4(other.g0.x), vec4(self.g0.y) * other.g0.yywz * vec4(1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * other.g0.zwzy * vec4(1.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * other.g0.wzyw * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g0.x) * other.g0 * vec4(0.0, 1.0, 1.0, 1.0));
}

Scalar rotor_translator_regressive_product(Rotor self, Translator other) {
    return Scalar(self.g0.y * other.g0.y + self.g0.z * other.g0.z + self.g0.w * other.g0.w);
}

Motor rotor_translator_outer_product(Rotor self, Translator other) {
    return Motor(self.g0 * vec4(other.g0.x), vec4(self.g0.z) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * other.g0.yyzw);
}

Translator rotor_translator_left_contraction(Rotor self, Translator other) {
    return Translator(vec4(self.g0.x) * other.g0);
}

Rotor rotor_translator_right_contraction(Rotor self, Translator other) {
    return Rotor(self.g0 * vec4(other.g0.x));
}

Scalar rotor_translator_scalar_product(Rotor self, Translator other) {
    return Scalar(self.g0.x * other.g0.x);
}

Motor rotor_motor_add(Rotor self, Motor other) {
    return Motor(self.g0 + other.g0, other.g1);
}

Motor rotor_motor_sub(Rotor self, Motor other) {
    return Motor(self.g0 - other.g0, vec4(0.0) - other.g1);
}

Motor rotor_motor_geometric_product(Rotor self, Motor other) {
    return Motor(vec4(self.g0.x) * other.g0 + vec4(self.g0.y) * other.g0.yxwz * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g0.zwxy * vec4(-1.0, -1.0, 1.0, 1.0) + vec4(self.g0.w) * other.g0.wzyx * vec4(-1.0, 1.0, -1.0, 1.0), vec4(self.g0.x) * other.g1 + vec4(self.g0.y) * other.g1.yxwz * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g1.zwxy * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.w) * other.g1.wzyx * vec4(1.0, 1.0, -1.0, -1.0));
}

Rotor rotor_motor_regressive_product(Rotor self, Motor other) {
    return Rotor(vec4(self.g0.y) * other.g1.yxyy * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g0.z) * other.g1.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g1.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g1.x) * vec4(1.0, 0.0, 0.0, 0.0));
}

Motor rotor_motor_outer_product(Rotor self, Motor other) {
    return Motor(vec4(self.g0.x) * other.g0 + self.g0 * vec4(other.g0.x) * vec4(0.0, 1.0, 1.0, 1.0), vec4(self.g0.x) * other.g1 + vec4(self.g0.z) * vec4(other.g1.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g1.w) * vec4(1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * other.g1.yxxx * vec4(1.0, 0.0, 0.0, 0.0));
}

Motor rotor_motor_inner_product(Rotor self, Motor other) {
    return Motor(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + self.g0.yyxx * other.g0.yxxx * vec4(-1.0, 1.0, 0.0, 0.0), vec4(self.g0.x) * other.g1 + self.g0 * vec4(other.g1.x) * vec4(0.0, -1.0, -1.0, -1.0));
}

Motor rotor_motor_left_contraction(Rotor self, Motor other) {
    return Motor(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * vec4(other.g0.z) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.w) * vec4(-1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * other.g0.yxxx * vec4(-1.0, 0.0, 0.0, 0.0), vec4(self.g0.x) * other.g1 + self.g0 * vec4(other.g1.x) * vec4(0.0, -1.0, -1.0, -1.0));
}

Rotor rotor_motor_right_contraction(Rotor self, Motor other) {
    return Rotor(vec4(self.g0.y) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.z) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0));
}

Scalar rotor_motor_scalar_product(Rotor self, Motor other) {
    return Scalar(self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g0.w * other.g0.w);
}

PointAndPlane rotor_point_and_plane_geometric_product(Rotor self, PointAndPlane other) {
    return PointAndPlane(vec4(self.g0.x) * other.g0 + vec4(self.g0.y) * vec4(other.g1.y, other.g1.x, other.g0.w, other.g0.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.z, other.g0.w, other.g1.x, other.g0.y) * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.w) * vec4(other.g1.w, other.g0.z, other.g0.y, other.g1.x) * vec4(1.0, 1.0, -1.0, -1.0), vec4(self.g0.x) * other.g1 + vec4(self.g0.y) * vec4(other.g0.y, other.g0.x, other.g1.w, other.g1.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z, other.g1.w, other.g0.x, other.g1.y) * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.w) * vec4(other.g0.w, other.g1.z, other.g1.y, other.g0.x) * vec4(1.0, 1.0, -1.0, -1.0));
}

PointAndPlane rotor_point_and_plane_outer_product(Rotor self, PointAndPlane other) {
    return PointAndPlane(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * other.g1.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.w) * other.g1.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + self.g0.yyxx * other.g1.yxxx * vec4(1.0, -1.0, 0.0, 0.0), vec4(self.g0.x) * other.g1);
}

PointAndPlane rotor_point_and_plane_inner_product(Rotor self, PointAndPlane other) {
    return PointAndPlane(vec4(self.g0.x) * other.g0, vec4(self.g0.x) * other.g1 + vec4(self.g0.y) * vec4(other.g0.y, other.g0.x, other.g1.w, other.g1.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z, other.g1.w, other.g0.x, other.g1.y) * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.w) * vec4(other.g0.w, other.g1.z, other.g1.y, other.g0.x) * vec4(1.0, 1.0, -1.0, -1.0));
}

PointAndPlane rotor_point_and_plane_left_contraction(Rotor self, PointAndPlane other) {
    return PointAndPlane(vec4(self.g0.x) * other.g0, vec4(self.g0.x) * other.g1 + vec4(self.g0.z) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + self.g0.yyxx * other.g0.yxxx * vec4(1.0, -1.0, 0.0, 0.0));
}

Scalar rotor_squared_magnitude(Rotor self) {
    return rotor_rotor_scalar_product(self, rotor_reversal(self));
}

Scalar rotor_magnitude(Rotor self) {
    return Scalar(sqrt(rotor_squared_magnitude(self).g0));
}

Rotor rotor_scale(Rotor self, float other) {
    return rotor_scalar_geometric_product(self, Scalar(other));
}

Rotor rotor_signum(Rotor self) {
    return rotor_scalar_geometric_product(self, Scalar(1.0 / rotor_magnitude(self).g0));
}

Rotor rotor_inverse(Rotor self) {
    return rotor_scalar_geometric_product(rotor_reversal(self), Scalar(1.0 / rotor_squared_magnitude(self).g0));
}

Point point_zero() {
    return Point(vec4(0.0));
}

Point point_one() {
    return Point(vec4(0.0));
}

Point point_neg(Point self) {
    return Point(self.g0 * vec4(-1.0));
}

Point point_automorphism(Point self) {
    return Point(self.g0 * vec4(-1.0));
}

Point point_reversal(Point self) {
    return Point(self.g0 * vec4(-1.0));
}

Point point_conjugation(Point self) {
    return Point(self.g0);
}

Plane point_dual(Point self) {
    return Plane(self.g0 * vec4(-1.0));
}

Point point_scalar_geometric_product(Point self, Scalar other) {
    return Point(self.g0 * vec4(other.g0));
}

Point point_scalar_outer_product(Point self, Scalar other) {
    return Point(self.g0 * vec4(other.g0));
}

Point point_scalar_inner_product(Point self, Scalar other) {
    return Point(self.g0 * vec4(other.g0));
}

Point point_scalar_right_contraction(Point self, Scalar other) {
    return Point(self.g0 * vec4(other.g0));
}

MultiVector point_multi_vector_add(Point self, MultiVector other) {
    return MultiVector(other.g0, self.g0 * vec4(0.0, 1.0, 1.0, 1.0) + other.g1, vec4(self.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + other.g2, other.g3);
}

MultiVector point_multi_vector_sub(Point self, MultiVector other) {
    return MultiVector(vec4(0.0) - other.g0, self.g0 * vec4(0.0, 1.0, 1.0, 1.0) - other.g1, vec4(self.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) - other.g2, vec4(0.0) - other.g3);
}

MultiVector point_multi_vector_geometric_product(Point self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g2 * vec4(-1.0, 1.0, 1.0, 1.0) + vec4(self.g0.y) * other.g1.yxwz * vec4(-1.0, -1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g1.zwxy * vec4(-1.0, -1.0, -1.0, 1.0) + vec4(self.g0.w) * other.g1.wzyx * vec4(-1.0, 1.0, -1.0, -1.0), vec4(self.g0.x) * other.g3 + vec4(self.g0.y) * other.g0.yxwz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g0.zwxy * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g0.w) * other.g0.wzyx * vec4(1.0, 1.0, -1.0, 1.0), vec4(self.g0.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g0.y) * other.g3.yxwz * vec4(-1.0, 1.0, -1.0, 1.0) + vec4(self.g0.z) * other.g3.zwxy * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.w) * other.g3.wzyx * vec4(-1.0, -1.0, 1.0, 1.0), vec4(0.0) - vec4(self.g0.x) * other.g1 + vec4(self.g0.y) * other.g2.yxwz * vec4(-1.0, 1.0, -1.0, 1.0) + vec4(self.g0.z) * other.g2.zwxy * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.w) * other.g2.wzyx * vec4(-1.0, -1.0, 1.0, 1.0));
}

Scalar point_multi_vector_scalar_product(Point self, MultiVector other) {
    return Scalar(0.0 - self.g0.x * other.g2.x - self.g0.y * other.g1.y - self.g0.z * other.g1.z - self.g0.w * other.g1.w);
}

PointAndPlane point_rotor_geometric_product(Point self, Rotor other) {
    return PointAndPlane(vec4(self.g0.z) * other.g0.wwxy * vec4(0.0, -1.0, 1.0, 1.0) + vec4(self.g0.w) * other.g0.zzyx * vec4(0.0, 1.0, -1.0, 1.0) + self.g0.xyyy * other.g0.xxwz * vec4(1.0, 1.0, 1.0, -1.0), vec4(self.g0.z) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * other.g0.yyzw * vec4(1.0, -1.0, -1.0, -1.0));
}

Point point_rotor_outer_product(Point self, Rotor other) {
    return Point(self.g0 * vec4(other.g0.x));
}

PointAndPlane point_rotor_inner_product(Point self, Rotor other) {
    return PointAndPlane(self.g0 * vec4(other.g0.x), vec4(self.g0.z) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * other.g0.yyzw * vec4(1.0, -1.0, -1.0, -1.0));
}

PointAndPlane point_rotor_right_contraction(Point self, Rotor other) {
    return PointAndPlane(self.g0 * vec4(other.g0.x), vec4(self.g0.z) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * other.g0.yyzw * vec4(1.0, -1.0, -1.0, -1.0));
}

Point point_point_add(Point self, Point other) {
    return Point(self.g0 + other.g0);
}

Point point_point_sub(Point self, Point other) {
    return Point(self.g0 - other.g0);
}

Point point_point_mul(Point self, Point other) {
    return Point(self.g0 * other.g0);
}

Point point_point_div(Point self, Point other) {
    return Point(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.w) * vec4(1.0, 1.0, 1.0, 1.0) / vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.w) * vec4(1.0, 1.0, 1.0, 1.0));
}

Line point_point_regressive_product(Point self, Point other) {
    return Line(vec3(self.g0.z) * vec3(other.g0.w, other.g0.w, other.g0.y) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.w) * vec3(other.g0.z, other.g0.y, other.g0.z) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x, self.g0.y, self.g0.y) * vec3(other.g0.x, other.g0.w, other.g0.z) * vec3(0.0, -1.0, 1.0), vec3(self.g0.x) * vec3(other.g0.y, other.g0.z, other.g0.w) + vec3(self.g0.y, self.g0.z, self.g0.w) * vec3(other.g0.x) * vec3(-1.0));
}

Scalar point_point_inner_product(Point self, Point other) {
    return Scalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g0.w * other.g0.w);
}

Scalar point_point_left_contraction(Point self, Point other) {
    return Scalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g0.w * other.g0.w);
}

Scalar point_point_right_contraction(Point self, Point other) {
    return Scalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g0.w * other.g0.w);
}

Scalar point_point_scalar_product(Point self, Point other) {
    return Scalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g0.w * other.g0.w);
}

Plane point_ideal_point_regressive_product(Point self, IdealPoint other) {
    return Plane(vec4(self.g0.z) * vec4(other.g0.y) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.z) * vec4(-1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(-1.0, 1.0, 1.0, 1.0));
}

PointAndPlane point_plane_add(Point self, Plane other) {
    return PointAndPlane(self.g0, other.g0);
}

PointAndPlane point_plane_sub(Point self, Plane other) {
    return PointAndPlane(self.g0, vec4(0.0) - other.g0);
}

Scalar point_plane_regressive_product(Point self, Plane other) {
    return Scalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g0.w * other.g0.w);
}

Line point_plane_inner_product(Point self, Plane other) {
    return Line(vec3(self.g0.z) * vec3(other.g0.w, other.g0.w, other.g0.y) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.w) * vec3(other.g0.z, other.g0.y, other.g0.z) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x, self.g0.y, self.g0.y) * vec3(other.g0.x, other.g0.w, other.g0.z) * vec3(0.0, -1.0, 1.0), vec3(self.g0.x) * vec3(other.g0.y, other.g0.z, other.g0.w) + vec3(self.g0.y, self.g0.z, self.g0.w) * vec3(other.g0.x) * vec3(-1.0));
}

Line point_plane_right_contraction(Point self, Plane other) {
    return Line(vec3(self.g0.z) * vec3(other.g0.w, other.g0.w, other.g0.y) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.w) * vec3(other.g0.z, other.g0.y, other.g0.z) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x, self.g0.y, self.g0.y) * vec3(other.g0.x, other.g0.w, other.g0.z) * vec3(0.0, -1.0, 1.0), vec3(self.g0.x) * vec3(other.g0.y, other.g0.z, other.g0.w) + vec3(self.g0.y, self.g0.z, self.g0.w) * vec3(other.g0.x) * vec3(-1.0));
}

PointAndPlane point_line_geometric_product(Point self, Line other) {
    return PointAndPlane(vec4(self.g0.y) * vec4(other.g0.x, other.g0.x, other.g1.z, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g1.z, other.g0.y, other.g1.x) * vec4(-1.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * vec4(other.g0.z, other.g1.y, other.g1.x, other.g0.z) * vec4(-1.0, 1.0, -1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0), vec4(self.g0.y) * vec4(other.g1.x, other.g1.x, other.g0.z, other.g0.y) * vec4(1.0, 0.0, -1.0, 1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g0.z, other.g1.y, other.g0.x) * vec4(1.0, 1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g1.z, other.g0.y, other.g0.x, other.g1.z) * vec4(1.0, -1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.x, other.g1.y, other.g1.z) * vec4(0.0, -1.0, -1.0, -1.0));
}

Plane point_line_regressive_product(Point self, Line other) {
    return Plane(vec4(self.g0.y) * vec4(other.g0.x, other.g0.x, other.g1.z, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g1.z, other.g0.y, other.g1.x) * vec4(-1.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * vec4(other.g0.z, other.g1.y, other.g1.x, other.g0.z) * vec4(-1.0, 1.0, -1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0));
}

Plane point_line_inner_product(Point self, Line other) {
    return Plane(vec4(self.g0.y) * vec4(other.g1.x, other.g1.x, other.g0.z, other.g0.y) * vec4(1.0, 0.0, -1.0, 1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g0.z, other.g1.y, other.g0.x) * vec4(1.0, 1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g1.z, other.g0.y, other.g0.x, other.g1.z) * vec4(1.0, -1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.x, other.g1.y, other.g1.z) * vec4(0.0, -1.0, -1.0, -1.0));
}

Plane point_line_right_contraction(Point self, Line other) {
    return Plane(vec4(self.g0.y) * vec4(other.g1.x, other.g1.x, other.g0.z, other.g0.y) * vec4(1.0, 0.0, -1.0, 1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g0.z, other.g1.y, other.g0.x) * vec4(1.0, 1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g1.z, other.g0.y, other.g0.x, other.g1.z) * vec4(1.0, -1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.x, other.g1.y, other.g1.z) * vec4(0.0, -1.0, -1.0, -1.0));
}

Plane point_translator_regressive_product(Point self, Translator other) {
    return Plane(vec4(self.g0.z) * vec4(other.g0.z) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.w) * vec4(-1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * other.g0.yyzw * vec4(-1.0, 1.0, 1.0, 1.0));
}

Point point_translator_outer_product(Point self, Translator other) {
    return Point(self.g0 * vec4(other.g0.x));
}

PointAndPlane point_motor_geometric_product(Point self, Motor other) {
    return PointAndPlane(vec4(self.g0.x) * vec4(other.g0.x, other.g1.y, other.g1.z, other.g1.w) + vec4(self.g0.y) * vec4(other.g1.y, other.g0.x, other.g0.w, other.g0.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.z, other.g0.w, other.g0.x, other.g0.y) * vec4(-1.0, -1.0, 1.0, 1.0) + vec4(self.g0.w) * vec4(other.g1.w, other.g0.z, other.g0.y, other.g0.x) * vec4(-1.0, 1.0, -1.0, 1.0), vec4(self.g0.x) * vec4(other.g1.x, other.g0.y, other.g0.z, other.g0.w) * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g0.y) * vec4(other.g0.y, other.g1.x, other.g1.w, other.g1.z) * vec4(1.0, 1.0, -1.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.z, other.g1.w, other.g1.x, other.g1.y) * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g0.w) * vec4(other.g0.w, other.g1.z, other.g1.y, other.g1.x) * vec4(1.0, -1.0, 1.0, 1.0));
}

PointAndPlane point_motor_regressive_product(Point self, Motor other) {
    return PointAndPlane(self.g0 * vec4(other.g1.x), vec4(self.g0.y) * vec4(other.g1.y, other.g1.y, other.g0.w, other.g0.z) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.z, other.g0.w, other.g1.z, other.g0.y) * vec4(-1.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * vec4(other.g1.w, other.g0.z, other.g0.y, other.g1.w) * vec4(-1.0, 1.0, -1.0, 0.0) + vec4(self.g0.x) * other.g1 * vec4(0.0, 1.0, 1.0, 1.0));
}

Point point_motor_outer_product(Point self, Motor other) {
    return Point(self.g0 * vec4(other.g0.x));
}

PointAndPlane point_motor_inner_product(Point self, Motor other) {
    return PointAndPlane(self.g0 * vec4(other.g0.x), vec4(self.g0.x) * vec4(other.g1.x, other.g0.y, other.g0.z, other.g0.w) * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g0.y) * vec4(other.g0.y, other.g1.x, other.g1.w, other.g1.z) * vec4(1.0, 1.0, -1.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.z, other.g1.w, other.g1.x, other.g1.y) * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g0.w) * vec4(other.g0.w, other.g1.z, other.g1.y, other.g1.x) * vec4(1.0, -1.0, 1.0, 1.0));
}

Plane point_motor_left_contraction(Point self, Motor other) {
    return Plane(self.g0 * vec4(other.g1.x));
}

PointAndPlane point_motor_right_contraction(Point self, Motor other) {
    return PointAndPlane(self.g0 * vec4(other.g0.x), vec4(self.g0.y) * vec4(other.g0.y, other.g0.y, other.g1.w, other.g1.z) * vec4(1.0, 0.0, -1.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.z, other.g1.w, other.g0.z, other.g1.y) * vec4(1.0, 1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g0.w, other.g1.z, other.g1.y, other.g0.w) * vec4(1.0, -1.0, 1.0, 0.0) + vec4(self.g0.x) * other.g0 * vec4(0.0, -1.0, -1.0, -1.0));
}

PointAndPlane point_point_and_plane_add(Point self, PointAndPlane other) {
    return PointAndPlane(self.g0 + other.g0, other.g1);
}

PointAndPlane point_point_and_plane_sub(Point self, PointAndPlane other) {
    return PointAndPlane(self.g0 - other.g0, vec4(0.0) - other.g1);
}

Motor point_point_and_plane_geometric_product(Point self, PointAndPlane other) {
    return Motor(vec4(self.g0.x) * vec4(other.g0.x, other.g1.y, other.g1.z, other.g1.w) * vec4(-1.0, 1.0, 1.0, 1.0) + vec4(self.g0.y) * vec4(other.g0.y, other.g1.x, other.g0.w, other.g0.z) * vec4(-1.0, -1.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z, other.g0.w, other.g1.x, other.g0.y) * vec4(-1.0, -1.0, -1.0, 1.0) + vec4(self.g0.w) * vec4(other.g0.w, other.g0.z, other.g0.y, other.g1.x) * vec4(-1.0, 1.0, -1.0, -1.0), vec4(0.0) - vec4(self.g0.x) * vec4(other.g1.x, other.g0.y, other.g0.z, other.g0.w) + vec4(self.g0.y) * vec4(other.g1.y, other.g0.x, other.g1.w, other.g1.z) * vec4(-1.0, 1.0, -1.0, 1.0) + vec4(self.g0.z) * vec4(other.g1.z, other.g1.w, other.g0.x, other.g1.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.w) * vec4(other.g1.w, other.g1.z, other.g1.y, other.g0.x) * vec4(-1.0, -1.0, 1.0, 1.0));
}

Scalar point_point_and_plane_left_contraction(Point self, PointAndPlane other) {
    return Scalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g0.w * other.g0.w);
}

Scalar point_point_and_plane_scalar_product(Point self, PointAndPlane other) {
    return Scalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g0.w * other.g0.w);
}

Scalar point_squared_magnitude(Point self) {
    return point_point_scalar_product(self, point_reversal(self));
}

Scalar point_magnitude(Point self) {
    return Scalar(sqrt(point_squared_magnitude(self).g0));
}

Point point_scale(Point self, float other) {
    return point_scalar_geometric_product(self, Scalar(other));
}

Point point_signum(Point self) {
    return point_scalar_geometric_product(self, Scalar(1.0 / point_magnitude(self).g0));
}

Point point_inverse(Point self) {
    return point_scalar_geometric_product(point_reversal(self), Scalar(1.0 / point_squared_magnitude(self).g0));
}

IdealPoint ideal_point_zero() {
    return IdealPoint(vec3(0.0));
}

IdealPoint ideal_point_one() {
    return IdealPoint(vec3(0.0));
}

IdealPoint ideal_point_neg(IdealPoint self) {
    return IdealPoint(self.g0 * vec3(-1.0));
}

IdealPoint ideal_point_automorphism(IdealPoint self) {
    return IdealPoint(self.g0);
}

IdealPoint ideal_point_reversal(IdealPoint self) {
    return IdealPoint(self.g0 * vec3(-1.0));
}

IdealPoint ideal_point_conjugation(IdealPoint self) {
    return IdealPoint(self.g0 * vec3(-1.0));
}

Translator ideal_point_scalar_add(IdealPoint self, Scalar other) {
    return Translator(vec4(self.g0.x, self.g0.x, self.g0.y, self.g0.z) * vec4(0.0, 1.0, 1.0, 1.0) + vec4(other.g0) * vec4(1.0, 0.0, 0.0, 0.0));
}

Translator ideal_point_scalar_sub(IdealPoint self, Scalar other) {
    return Translator(vec4(self.g0.x, self.g0.x, self.g0.y, self.g0.z) * vec4(0.0, 1.0, 1.0, 1.0) - vec4(other.g0) * vec4(1.0, 0.0, 0.0, 0.0));
}

IdealPoint ideal_point_scalar_geometric_product(IdealPoint self, Scalar other) {
    return IdealPoint(self.g0 * vec3(other.g0));
}

IdealPoint ideal_point_scalar_outer_product(IdealPoint self, Scalar other) {
    return IdealPoint(self.g0 * vec3(other.g0));
}

IdealPoint ideal_point_scalar_inner_product(IdealPoint self, Scalar other) {
    return IdealPoint(self.g0 * vec3(other.g0));
}

IdealPoint ideal_point_scalar_right_contraction(IdealPoint self, Scalar other) {
    return IdealPoint(self.g0 * vec3(other.g0));
}

MultiVector ideal_point_multi_vector_add(IdealPoint self, MultiVector other) {
    return MultiVector(other.g0, other.g1, other.g2, vec4(self.g0.x, self.g0.x, self.g0.y, self.g0.z) * vec4(0.0, 1.0, 1.0, 1.0) + other.g3);
}

MultiVector ideal_point_multi_vector_sub(IdealPoint self, MultiVector other) {
    return MultiVector(vec4(0.0) - other.g0, vec4(0.0) - other.g1, vec4(0.0) - other.g2, vec4(self.g0.x, self.g0.x, self.g0.y, self.g0.z) * vec4(0.0, 1.0, 1.0, 1.0) - other.g3);
}

MultiVector ideal_point_multi_vector_geometric_product(IdealPoint self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g3.yxwz * vec4(-1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * other.g3.zwxy * vec4(-1.0, -1.0, -1.0, 1.0) + vec4(self.g0.z) * other.g3.wzyx * vec4(-1.0, 1.0, -1.0, -1.0), vec4(self.g0.x) * other.g2.yxwz * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * other.g2.zwxy * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.z) * other.g2.wzyx * vec4(1.0, 1.0, -1.0, -1.0), vec4(self.g0.x) * other.g1.yxwz * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * other.g1.zwxy * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.z) * other.g1.wzyx * vec4(1.0, 1.0, -1.0, -1.0), vec4(self.g0.x) * other.g0.yxwz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g0.y) * other.g0.zwxy * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g0.z) * other.g0.wzyx * vec4(1.0, 1.0, -1.0, 1.0));
}

Scalar ideal_point_multi_vector_scalar_product(IdealPoint self, MultiVector other) {
    return Scalar(0.0 - self.g0.x * other.g3.y - self.g0.y * other.g3.z - self.g0.z * other.g3.w);
}

Scalar ideal_point_rotor_regressive_product(IdealPoint self, Rotor other) {
    return Scalar(self.g0.x * other.g0.y + self.g0.y * other.g0.z + self.g0.z * other.g0.w);
}

IdealPoint ideal_point_rotor_inner_product(IdealPoint self, Rotor other) {
    return IdealPoint(self.g0 * vec3(other.g0.x));
}

IdealPoint ideal_point_rotor_right_contraction(IdealPoint self, Rotor other) {
    return IdealPoint(self.g0 * vec3(other.g0.x));
}

Plane ideal_point_point_regressive_product(IdealPoint self, Point other) {
    return Plane(vec4(self.g0.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.z) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * other.g0.yxxx * vec4(-1.0, 1.0, 0.0, 0.0));
}

IdealPoint ideal_point_ideal_point_add(IdealPoint self, IdealPoint other) {
    return IdealPoint(self.g0 + other.g0);
}

IdealPoint ideal_point_ideal_point_sub(IdealPoint self, IdealPoint other) {
    return IdealPoint(self.g0 - other.g0);
}

IdealPoint ideal_point_ideal_point_mul(IdealPoint self, IdealPoint other) {
    return IdealPoint(self.g0 * other.g0);
}

IdealPoint ideal_point_ideal_point_div(IdealPoint self, IdealPoint other) {
    return IdealPoint(vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(1.0, 1.0, 1.0) / vec3(other.g0.x, other.g0.y, other.g0.z) * vec3(1.0, 1.0, 1.0));
}

Rotor ideal_point_ideal_point_geometric_product(IdealPoint self, IdealPoint other) {
    return Rotor(vec4(self.g0.y) * vec4(other.g0.y, other.g0.z, other.g0.y, other.g0.x) * vec4(-1.0, -1.0, 0.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.z, other.g0.y, other.g0.x, other.g0.z) * vec4(-1.0, 1.0, -1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0));
}

Scalar ideal_point_ideal_point_inner_product(IdealPoint self, IdealPoint other) {
    return Scalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

Scalar ideal_point_ideal_point_left_contraction(IdealPoint self, IdealPoint other) {
    return Scalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

Scalar ideal_point_ideal_point_right_contraction(IdealPoint self, IdealPoint other) {
    return Scalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

Scalar ideal_point_ideal_point_scalar_product(IdealPoint self, IdealPoint other) {
    return Scalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

Plane ideal_point_plane_inner_product(IdealPoint self, Plane other) {
    return Plane(vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g0.x) * other.g0.yxxx * vec4(1.0, -1.0, 0.0, 0.0));
}

Plane ideal_point_plane_right_contraction(IdealPoint self, Plane other) {
    return Plane(vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g0.x) * other.g0.yxxx * vec4(1.0, -1.0, 0.0, 0.0));
}

Line ideal_point_line_add(IdealPoint self, Line other) {
    return Line(self.g0 + other.g0, other.g1);
}

Line ideal_point_line_sub(IdealPoint self, Line other) {
    return Line(self.g0 - other.g0, vec3(0.0) - other.g1);
}

Motor ideal_point_line_geometric_product(IdealPoint self, Line other) {
    return Motor(vec4(self.g0.y) * vec4(other.g0.y, other.g0.z, other.g0.y, other.g0.x) * vec4(-1.0, -1.0, 0.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.z, other.g0.y, other.g0.x, other.g0.z) * vec4(-1.0, 1.0, -1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0), vec4(self.g0.y) * vec4(other.g1.y, other.g1.z, other.g1.y, other.g1.x) * vec4(1.0, -1.0, 0.0, 1.0) + vec4(self.g0.z) * vec4(other.g1.z, other.g1.y, other.g1.x, other.g1.z) * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.x, other.g1.z, other.g1.y) * vec4(1.0, 0.0, 1.0, -1.0));
}

Scalar ideal_point_line_regressive_product(IdealPoint self, Line other) {
    return Scalar(self.g0.x * other.g1.x + self.g0.y * other.g1.y + self.g0.z * other.g1.z);
}

Scalar ideal_point_line_inner_product(IdealPoint self, Line other) {
    return Scalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

Scalar ideal_point_line_left_contraction(IdealPoint self, Line other) {
    return Scalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

Scalar ideal_point_line_right_contraction(IdealPoint self, Line other) {
    return Scalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

Scalar ideal_point_line_scalar_product(IdealPoint self, Line other) {
    return Scalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

Translator ideal_point_translator_add(IdealPoint self, Translator other) {
    return Translator(vec4(self.g0.x, self.g0.x, self.g0.y, self.g0.z) * vec4(0.0, 1.0, 1.0, 1.0) + other.g0);
}

Translator ideal_point_translator_sub(IdealPoint self, Translator other) {
    return Translator(vec4(self.g0.x, self.g0.x, self.g0.y, self.g0.z) * vec4(0.0, 1.0, 1.0, 1.0) - other.g0);
}

IdealPoint ideal_point_translator_outer_product(IdealPoint self, Translator other) {
    return IdealPoint(self.g0 * vec3(other.g0.x));
}

Translator ideal_point_translator_inner_product(IdealPoint self, Translator other) {
    return Translator(vec4(self.g0.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.z) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * other.g0.yxxx * vec4(-1.0, 1.0, 0.0, 0.0));
}

Scalar ideal_point_translator_left_contraction(IdealPoint self, Translator other) {
    return Scalar(0.0 - self.g0.x * other.g0.y - self.g0.y * other.g0.z - self.g0.z * other.g0.w);
}

Translator ideal_point_translator_right_contraction(IdealPoint self, Translator other) {
    return Translator(vec4(self.g0.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.z) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * other.g0.yxxx * vec4(-1.0, 1.0, 0.0, 0.0));
}

Scalar ideal_point_translator_scalar_product(IdealPoint self, Translator other) {
    return Scalar(0.0 - self.g0.x * other.g0.y - self.g0.y * other.g0.z - self.g0.z * other.g0.w);
}

Motor ideal_point_motor_add(IdealPoint self, Motor other) {
    return Motor(other.g0, vec4(self.g0.x, self.g0.x, self.g0.y, self.g0.z) * vec4(0.0, 1.0, 1.0, 1.0) + other.g1);
}

Motor ideal_point_motor_sub(IdealPoint self, Motor other) {
    return Motor(vec4(0.0) - other.g0, vec4(self.g0.x, self.g0.x, self.g0.y, self.g0.z) * vec4(0.0, 1.0, 1.0, 1.0) - other.g1);
}

Motor ideal_point_motor_geometric_product(IdealPoint self, Motor other) {
    return Motor(vec4(self.g0.x) * other.g1.yxwz * vec4(-1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * other.g1.zwxy * vec4(-1.0, -1.0, -1.0, 1.0) + vec4(self.g0.z) * other.g1.wzyx * vec4(-1.0, 1.0, -1.0, -1.0), vec4(self.g0.x) * other.g0.yxwz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g0.y) * other.g0.zwxy * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g0.z) * other.g0.wzyx * vec4(1.0, 1.0, -1.0, 1.0));
}

Translator ideal_point_motor_regressive_product(IdealPoint self, Motor other) {
    return Translator(vec4(self.g0.y) * vec4(other.g0.z, other.g0.z, other.g1.x, other.g0.z) * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g0.z) * vec4(other.g0.w, other.g0.w, other.g0.w, other.g1.x) * vec4(1.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g0.y, other.g1.x, other.g0.x, other.g0.x) * vec4(1.0, 1.0, 0.0, 0.0));
}

Rotor ideal_point_motor_left_contraction(IdealPoint self, Motor other) {
    return Rotor(vec4(self.g0.y) * other.g1.zzxz * vec4(-1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g1.wwwx * vec4(-1.0, 0.0, 0.0, -1.0) + vec4(self.g0.x) * other.g1.yxxx * vec4(-1.0, -1.0, 0.0, 0.0));
}

Translator ideal_point_motor_right_contraction(IdealPoint self, Motor other) {
    return Translator(vec4(self.g0.y) * vec4(other.g1.z, other.g1.z, other.g0.x, other.g1.z) * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.z) * vec4(other.g1.w, other.g1.w, other.g1.w, other.g0.x) * vec4(-1.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g1.y, other.g0.x, other.g1.x, other.g1.x) * vec4(-1.0, 1.0, 0.0, 0.0));
}

Scalar ideal_point_motor_scalar_product(IdealPoint self, Motor other) {
    return Scalar(0.0 - self.g0.x * other.g1.y - self.g0.y * other.g1.z - self.g0.z * other.g1.w);
}

PointAndPlane ideal_point_point_and_plane_geometric_product(IdealPoint self, PointAndPlane other) {
    return PointAndPlane(vec4(self.g0.x) * vec4(other.g0.y, other.g0.x, other.g1.w, other.g1.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * vec4(other.g0.z, other.g1.w, other.g0.x, other.g1.y) * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.w, other.g1.z, other.g1.y, other.g0.x) * vec4(1.0, 1.0, -1.0, -1.0), vec4(self.g0.x) * vec4(other.g1.y, other.g1.x, other.g0.w, other.g0.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * vec4(other.g1.z, other.g0.w, other.g1.x, other.g0.y) * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.z) * vec4(other.g1.w, other.g0.z, other.g0.y, other.g1.x) * vec4(1.0, 1.0, -1.0, -1.0));
}

Plane ideal_point_point_and_plane_regressive_product(IdealPoint self, PointAndPlane other) {
    return Plane(vec4(self.g0.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.z) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * other.g0.yxxx * vec4(-1.0, 1.0, 0.0, 0.0));
}

Plane ideal_point_point_and_plane_inner_product(IdealPoint self, PointAndPlane other) {
    return Plane(vec4(self.g0.x) * vec4(other.g1.y, other.g1.x, other.g0.w, other.g0.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * vec4(other.g1.z, other.g0.w, other.g1.x, other.g0.y) * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.z) * vec4(other.g1.w, other.g0.z, other.g0.y, other.g1.x) * vec4(1.0, 1.0, -1.0, -1.0));
}

Plane ideal_point_point_and_plane_right_contraction(IdealPoint self, PointAndPlane other) {
    return Plane(vec4(self.g0.y) * other.g1.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g1.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g0.x) * other.g1.yxxx * vec4(1.0, -1.0, 0.0, 0.0));
}

Scalar ideal_point_squared_magnitude(IdealPoint self) {
    return ideal_point_ideal_point_scalar_product(self, ideal_point_reversal(self));
}

Scalar ideal_point_magnitude(IdealPoint self) {
    return Scalar(sqrt(ideal_point_squared_magnitude(self).g0));
}

IdealPoint ideal_point_scale(IdealPoint self, float other) {
    return ideal_point_scalar_geometric_product(self, Scalar(other));
}

IdealPoint ideal_point_signum(IdealPoint self) {
    return ideal_point_scalar_geometric_product(self, Scalar(1.0 / ideal_point_magnitude(self).g0));
}

IdealPoint ideal_point_inverse(IdealPoint self) {
    return ideal_point_scalar_geometric_product(ideal_point_reversal(self), Scalar(1.0 / ideal_point_squared_magnitude(self).g0));
}

Plane plane_zero() {
    return Plane(vec4(0.0));
}

Plane plane_one() {
    return Plane(vec4(0.0));
}

Plane plane_neg(Plane self) {
    return Plane(self.g0 * vec4(-1.0));
}

Plane plane_automorphism(Plane self) {
    return Plane(self.g0 * vec4(-1.0));
}

Plane plane_reversal(Plane self) {
    return Plane(self.g0);
}

Plane plane_conjugation(Plane self) {
    return Plane(self.g0 * vec4(-1.0));
}

Point plane_dual(Plane self) {
    return Point(self.g0);
}

Plane plane_scalar_geometric_product(Plane self, Scalar other) {
    return Plane(self.g0 * vec4(other.g0));
}

Plane plane_scalar_outer_product(Plane self, Scalar other) {
    return Plane(self.g0 * vec4(other.g0));
}

Plane plane_scalar_inner_product(Plane self, Scalar other) {
    return Plane(self.g0 * vec4(other.g0));
}

Plane plane_scalar_right_contraction(Plane self, Scalar other) {
    return Plane(self.g0 * vec4(other.g0));
}

MultiVector plane_multi_vector_add(Plane self, MultiVector other) {
    return MultiVector(other.g0, vec4(self.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + other.g1, self.g0 * vec4(0.0, 1.0, 1.0, 1.0) + other.g2, other.g3);
}

MultiVector plane_multi_vector_sub(Plane self, MultiVector other) {
    return MultiVector(vec4(0.0) - other.g0, vec4(self.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) - other.g1, self.g0 * vec4(0.0, 1.0, 1.0, 1.0) - other.g2, vec4(0.0) - other.g3);
}

MultiVector plane_multi_vector_geometric_product(Plane self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g1 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g0.y) * other.g2.yxwz * vec4(1.0, 1.0, -1.0, 1.0) + vec4(self.g0.z) * other.g2.zwxy * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g0.w) * other.g2.wzyx * vec4(1.0, -1.0, 1.0, 1.0), vec4(self.g0.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g0.y) * other.g3.yxwz * vec4(-1.0, 1.0, -1.0, 1.0) + vec4(self.g0.z) * other.g3.zwxy * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.w) * other.g3.wzyx * vec4(-1.0, -1.0, 1.0, 1.0), vec4(self.g0.x) * other.g3 + vec4(self.g0.y) * other.g0.yxwz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g0.zwxy * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g0.w) * other.g0.wzyx * vec4(1.0, 1.0, -1.0, 1.0), vec4(self.g0.x) * other.g2 + vec4(self.g0.y) * other.g1.yxwz * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g1.zwxy * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.w) * other.g1.wzyx * vec4(1.0, 1.0, -1.0, -1.0));
}

Scalar plane_multi_vector_scalar_product(Plane self, MultiVector other) {
    return Scalar(self.g0.x * other.g1.x + self.g0.y * other.g2.y + self.g0.z * other.g2.z + self.g0.w * other.g2.w);
}

PointAndPlane plane_rotor_geometric_product(Plane self, Rotor other) {
    return PointAndPlane(vec4(self.g0.z) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * other.g0.yyzw * vec4(1.0, -1.0, -1.0, -1.0), vec4(self.g0.z) * other.g0.wwxy * vec4(0.0, -1.0, 1.0, 1.0) + vec4(self.g0.w) * other.g0.zzyx * vec4(0.0, 1.0, -1.0, 1.0) + self.g0.xyyy * other.g0.xxwz * vec4(1.0, 1.0, 1.0, -1.0));
}

PointAndPlane plane_rotor_outer_product(Plane self, Rotor other) {
    return PointAndPlane(vec4(self.g0.z) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * other.g0.yyzw * vec4(1.0, -1.0, -1.0, -1.0), self.g0 * vec4(other.g0.x));
}

Plane plane_rotor_inner_product(Plane self, Rotor other) {
    return Plane(vec4(self.g0.z) * other.g0.wwxy * vec4(0.0, -1.0, 1.0, 1.0) + vec4(self.g0.w) * other.g0.zzyx * vec4(0.0, 1.0, -1.0, 1.0) + self.g0.xyyy * other.g0.xxwz * vec4(1.0, 1.0, 1.0, -1.0));
}

Plane plane_rotor_right_contraction(Plane self, Rotor other) {
    return Plane(self.g0 * vec4(other.g0.x));
}

PointAndPlane plane_point_add(Plane self, Point other) {
    return PointAndPlane(other.g0, self.g0);
}

PointAndPlane plane_point_sub(Plane self, Point other) {
    return PointAndPlane(vec4(0.0) - other.g0, self.g0);
}

Scalar plane_point_regressive_product(Plane self, Point other) {
    return Scalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z + self.g0.w * other.g0.w);
}

Line plane_point_inner_product(Plane self, Point other) {
    return Line(vec3(self.g0.z) * vec3(other.g0.w, other.g0.w, other.g0.y) * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.w) * vec3(other.g0.z, other.g0.y, other.g0.z) * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x, self.g0.y, self.g0.y) * vec3(other.g0.x, other.g0.w, other.g0.z) * vec3(0.0, 1.0, -1.0), vec3(0.0) - vec3(self.g0.x) * vec3(other.g0.y, other.g0.z, other.g0.w) + vec3(self.g0.y, self.g0.z, self.g0.w) * vec3(other.g0.x));
}

Line plane_point_left_contraction(Plane self, Point other) {
    return Line(vec3(self.g0.z) * vec3(other.g0.w, other.g0.w, other.g0.y) * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.w) * vec3(other.g0.z, other.g0.y, other.g0.z) * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x, self.g0.y, self.g0.y) * vec3(other.g0.x, other.g0.w, other.g0.z) * vec3(0.0, 1.0, -1.0), vec3(0.0) - vec3(self.g0.x) * vec3(other.g0.y, other.g0.z, other.g0.w) + vec3(self.g0.y, self.g0.z, self.g0.w) * vec3(other.g0.x));
}

Plane plane_ideal_point_inner_product(Plane self, IdealPoint other) {
    return Plane(vec4(self.g0.z) * vec4(other.g0.y) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.z) * vec4(-1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(-1.0, 1.0, 1.0, 1.0));
}

Plane plane_ideal_point_left_contraction(Plane self, IdealPoint other) {
    return Plane(vec4(self.g0.z) * vec4(other.g0.y) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.z) * vec4(-1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(-1.0, 1.0, 1.0, 1.0));
}

Plane plane_plane_add(Plane self, Plane other) {
    return Plane(self.g0 + other.g0);
}

Plane plane_plane_sub(Plane self, Plane other) {
    return Plane(self.g0 - other.g0);
}

Plane plane_plane_mul(Plane self, Plane other) {
    return Plane(self.g0 * other.g0);
}

Plane plane_plane_div(Plane self, Plane other) {
    return Plane(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.w) * vec4(1.0, 1.0, 1.0, 1.0) / vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.w) * vec4(1.0, 1.0, 1.0, 1.0));
}

Line plane_plane_outer_product(Plane self, Plane other) {
    return Line(vec3(self.g0.x) * vec3(other.g0.y, other.g0.z, other.g0.w) + vec3(self.g0.y, self.g0.z, self.g0.w) * vec3(other.g0.x) * vec3(-1.0), vec3(self.g0.z) * vec3(other.g0.w, other.g0.w, other.g0.y) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.w) * vec3(other.g0.z, other.g0.y, other.g0.z) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x, self.g0.y, self.g0.y) * vec3(other.g0.x, other.g0.w, other.g0.z) * vec3(0.0, -1.0, 1.0));
}

Scalar plane_plane_inner_product(Plane self, Plane other) {
    return Scalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z + self.g0.w * other.g0.w);
}

Scalar plane_plane_left_contraction(Plane self, Plane other) {
    return Scalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z + self.g0.w * other.g0.w);
}

Scalar plane_plane_right_contraction(Plane self, Plane other) {
    return Scalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z + self.g0.w * other.g0.w);
}

Scalar plane_plane_scalar_product(Plane self, Plane other) {
    return Scalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z + self.g0.w * other.g0.w);
}

PointAndPlane plane_line_geometric_product(Plane self, Line other) {
    return PointAndPlane(vec4(self.g0.y) * vec4(other.g1.x, other.g1.x, other.g0.z, other.g0.y) * vec4(1.0, 0.0, -1.0, 1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g0.z, other.g1.y, other.g0.x) * vec4(1.0, 1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g1.z, other.g0.y, other.g0.x, other.g1.z) * vec4(1.0, -1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.x, other.g1.y, other.g1.z) * vec4(0.0, -1.0, -1.0, -1.0), vec4(self.g0.y) * vec4(other.g0.x, other.g0.x, other.g1.z, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g1.z, other.g0.y, other.g1.x) * vec4(-1.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * vec4(other.g0.z, other.g1.y, other.g1.x, other.g0.z) * vec4(-1.0, 1.0, -1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0));
}

Point plane_line_outer_product(Plane self, Line other) {
    return Point(vec4(self.g0.y) * vec4(other.g1.x, other.g1.x, other.g0.z, other.g0.y) * vec4(1.0, 0.0, -1.0, 1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g0.z, other.g1.y, other.g0.x) * vec4(1.0, 1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g1.z, other.g0.y, other.g0.x, other.g1.z) * vec4(1.0, -1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.x, other.g1.y, other.g1.z) * vec4(0.0, -1.0, -1.0, -1.0));
}

Plane plane_line_inner_product(Plane self, Line other) {
    return Plane(vec4(self.g0.y) * vec4(other.g0.x, other.g0.x, other.g1.z, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g1.z, other.g0.y, other.g1.x) * vec4(-1.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * vec4(other.g0.z, other.g1.y, other.g1.x, other.g0.z) * vec4(-1.0, 1.0, -1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0));
}

Plane plane_line_left_contraction(Plane self, Line other) {
    return Plane(vec4(self.g0.y) * vec4(other.g0.x, other.g0.x, other.g1.z, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g1.z, other.g0.y, other.g1.x) * vec4(-1.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * vec4(other.g0.z, other.g1.y, other.g1.x, other.g0.z) * vec4(-1.0, 1.0, -1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0));
}

Plane plane_translator_inner_product(Plane self, Translator other) {
    return Plane(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + self.g0.yyxx * other.g0.yxxx * vec4(-1.0, 1.0, 0.0, 0.0));
}

Plane plane_translator_left_contraction(Plane self, Translator other) {
    return Plane(vec4(self.g0.z) * vec4(other.g0.z) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.w) * vec4(-1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * other.g0.yyzw * vec4(-1.0, 1.0, 1.0, 1.0));
}

Plane plane_translator_right_contraction(Plane self, Translator other) {
    return Plane(self.g0 * vec4(other.g0.x));
}

PointAndPlane plane_motor_geometric_product(Plane self, Motor other) {
    return PointAndPlane(vec4(self.g0.x) * vec4(other.g1.x, other.g0.y, other.g0.z, other.g0.w) * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g0.y) * vec4(other.g0.y, other.g1.x, other.g1.w, other.g1.z) * vec4(1.0, 1.0, -1.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.z, other.g1.w, other.g1.x, other.g1.y) * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g0.w) * vec4(other.g0.w, other.g1.z, other.g1.y, other.g1.x) * vec4(1.0, -1.0, 1.0, 1.0), vec4(self.g0.x) * vec4(other.g0.x, other.g1.y, other.g1.z, other.g1.w) + vec4(self.g0.y) * vec4(other.g1.y, other.g0.x, other.g0.w, other.g0.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.z, other.g0.w, other.g0.x, other.g0.y) * vec4(-1.0, -1.0, 1.0, 1.0) + vec4(self.g0.w) * vec4(other.g1.w, other.g0.z, other.g0.y, other.g0.x) * vec4(-1.0, 1.0, -1.0, 1.0));
}

Plane plane_motor_regressive_product(Plane self, Motor other) {
    return Plane(self.g0 * vec4(other.g1.x));
}

PointAndPlane plane_motor_outer_product(Plane self, Motor other) {
    return PointAndPlane(vec4(self.g0.y) * vec4(other.g0.y, other.g0.y, other.g1.w, other.g1.z) * vec4(1.0, 0.0, -1.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.z, other.g1.w, other.g0.z, other.g1.y) * vec4(1.0, 1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g0.w, other.g1.z, other.g1.y, other.g0.w) * vec4(1.0, -1.0, 1.0, 0.0) + vec4(self.g0.x) * other.g0 * vec4(0.0, -1.0, -1.0, -1.0), self.g0 * vec4(other.g0.x));
}

PointAndPlane plane_motor_inner_product(Plane self, Motor other) {
    return PointAndPlane(self.g0 * vec4(other.g1.x), vec4(self.g0.x) * vec4(other.g0.x, other.g1.y, other.g1.z, other.g1.w) + vec4(self.g0.y) * vec4(other.g1.y, other.g0.x, other.g0.w, other.g0.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.z, other.g0.w, other.g0.x, other.g0.y) * vec4(-1.0, -1.0, 1.0, 1.0) + vec4(self.g0.w) * vec4(other.g1.w, other.g0.z, other.g0.y, other.g0.x) * vec4(-1.0, 1.0, -1.0, 1.0));
}

PointAndPlane plane_motor_left_contraction(Plane self, Motor other) {
    return PointAndPlane(self.g0 * vec4(other.g1.x), vec4(self.g0.y) * vec4(other.g1.y, other.g1.y, other.g0.w, other.g0.z) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.z, other.g0.w, other.g1.z, other.g0.y) * vec4(-1.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * vec4(other.g1.w, other.g0.z, other.g0.y, other.g1.w) * vec4(-1.0, 1.0, -1.0, 0.0) + vec4(self.g0.x) * other.g1 * vec4(0.0, 1.0, 1.0, 1.0));
}

Plane plane_motor_right_contraction(Plane self, Motor other) {
    return Plane(self.g0 * vec4(other.g0.x));
}

PointAndPlane plane_point_and_plane_add(Plane self, PointAndPlane other) {
    return PointAndPlane(other.g0, self.g0 + other.g1);
}

PointAndPlane plane_point_and_plane_sub(Plane self, PointAndPlane other) {
    return PointAndPlane(vec4(0.0) - other.g0, self.g0 - other.g1);
}

Motor plane_point_and_plane_geometric_product(Plane self, PointAndPlane other) {
    return Motor(vec4(self.g0.x) * vec4(other.g1.x, other.g0.y, other.g0.z, other.g0.w) * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g0.y) * vec4(other.g1.y, other.g0.x, other.g1.w, other.g1.z) * vec4(1.0, 1.0, -1.0, 1.0) + vec4(self.g0.z) * vec4(other.g1.z, other.g1.w, other.g0.x, other.g1.y) * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g0.w) * vec4(other.g1.w, other.g1.z, other.g1.y, other.g0.x) * vec4(1.0, -1.0, 1.0, 1.0), vec4(self.g0.x) * vec4(other.g0.x, other.g1.y, other.g1.z, other.g1.w) + vec4(self.g0.y) * vec4(other.g0.y, other.g1.x, other.g0.w, other.g0.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z, other.g0.w, other.g1.x, other.g0.y) * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.w) * vec4(other.g0.w, other.g0.z, other.g0.y, other.g1.x) * vec4(1.0, 1.0, -1.0, -1.0));
}

Scalar plane_point_and_plane_regressive_product(Plane self, PointAndPlane other) {
    return Scalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z + self.g0.w * other.g0.w);
}

Scalar plane_point_and_plane_right_contraction(Plane self, PointAndPlane other) {
    return Scalar(self.g0.x * other.g1.x + self.g0.y * other.g1.y + self.g0.z * other.g1.z + self.g0.w * other.g1.w);
}

Scalar plane_point_and_plane_scalar_product(Plane self, PointAndPlane other) {
    return Scalar(self.g0.x * other.g1.x + self.g0.y * other.g1.y + self.g0.z * other.g1.z + self.g0.w * other.g1.w);
}

Scalar plane_squared_magnitude(Plane self) {
    return plane_plane_scalar_product(self, plane_reversal(self));
}

Scalar plane_magnitude(Plane self) {
    return Scalar(sqrt(plane_squared_magnitude(self).g0));
}

Plane plane_scale(Plane self, float other) {
    return plane_scalar_geometric_product(self, Scalar(other));
}

Plane plane_signum(Plane self) {
    return plane_scalar_geometric_product(self, Scalar(1.0 / plane_magnitude(self).g0));
}

Plane plane_inverse(Plane self) {
    return plane_scalar_geometric_product(plane_reversal(self), Scalar(1.0 / plane_squared_magnitude(self).g0));
}

Line line_zero() {
    return Line(vec3(0.0), vec3(0.0));
}

Line line_one() {
    return Line(vec3(0.0), vec3(0.0));
}

Line line_neg(Line self) {
    return Line(self.g0 * vec3(-1.0), self.g1 * vec3(-1.0));
}

Line line_automorphism(Line self) {
    return Line(self.g0, self.g1);
}

Line line_reversal(Line self) {
    return Line(self.g0 * vec3(-1.0), self.g1 * vec3(-1.0));
}

Line line_conjugation(Line self) {
    return Line(self.g0 * vec3(-1.0), self.g1 * vec3(-1.0));
}

Line line_dual(Line self) {
    return Line(self.g1, self.g0);
}

Line line_scalar_geometric_product(Line self, Scalar other) {
    return Line(self.g0 * vec3(other.g0), self.g1 * vec3(other.g0));
}

Line line_scalar_outer_product(Line self, Scalar other) {
    return Line(self.g0 * vec3(other.g0), self.g1 * vec3(other.g0));
}

Line line_scalar_inner_product(Line self, Scalar other) {
    return Line(self.g0 * vec3(other.g0), self.g1 * vec3(other.g0));
}

Line line_scalar_right_contraction(Line self, Scalar other) {
    return Line(self.g0 * vec3(other.g0), self.g1 * vec3(other.g0));
}

MultiVector line_multi_vector_add(Line self, MultiVector other) {
    return MultiVector(vec4(self.g0.x, self.g1.x, self.g1.y, self.g1.z) * vec4(0.0, 1.0, 1.0, 1.0) + other.g0, other.g1, other.g2, vec4(self.g0.x, self.g0.x, self.g0.y, self.g0.z) * vec4(0.0, 1.0, 1.0, 1.0) + other.g3);
}

MultiVector line_multi_vector_sub(Line self, MultiVector other) {
    return MultiVector(vec4(self.g0.x, self.g1.x, self.g1.y, self.g1.z) * vec4(0.0, 1.0, 1.0, 1.0) - other.g0, vec4(0.0) - other.g1, vec4(0.0) - other.g2, vec4(self.g0.x, self.g0.x, self.g0.y, self.g0.z) * vec4(0.0, 1.0, 1.0, 1.0) - other.g3);
}

MultiVector line_multi_vector_geometric_product(Line self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g3.yxwz * vec4(-1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * other.g3.zwxy * vec4(-1.0, -1.0, -1.0, 1.0) + vec4(self.g0.z) * other.g3.wzyx * vec4(-1.0, 1.0, -1.0, -1.0) + vec4(self.g1.x) * other.g0.yxwz * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.y) * other.g0.zwxy * vec4(-1.0, -1.0, 1.0, 1.0) + vec4(self.g1.z) * other.g0.wzyx * vec4(-1.0, 1.0, -1.0, 1.0), vec4(self.g0.x) * other.g2.yxwz * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * other.g2.zwxy * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.z) * other.g2.wzyx * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g1.x) * other.g1.yxwz * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g1.y) * other.g1.zwxy * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g1.z) * other.g1.wzyx * vec4(1.0, 1.0, -1.0, -1.0), vec4(self.g0.x) * other.g1.yxwz * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * other.g1.zwxy * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.z) * other.g1.wzyx * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g1.x) * other.g2.yxwz * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g1.y) * other.g2.zwxy * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g1.z) * other.g2.wzyx * vec4(1.0, 1.0, -1.0, -1.0), vec4(self.g0.x) * other.g0.yxwz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g0.y) * other.g0.zwxy * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g0.z) * other.g0.wzyx * vec4(1.0, 1.0, -1.0, 1.0) + vec4(self.g1.x) * other.g3.yxwz * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g1.y) * other.g3.zwxy * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g1.z) * other.g3.wzyx * vec4(1.0, 1.0, -1.0, -1.0));
}

Scalar line_multi_vector_scalar_product(Line self, MultiVector other) {
    return Scalar(0.0 - self.g0.x * other.g3.y - self.g0.y * other.g3.z - self.g0.z * other.g3.w - self.g1.x * other.g0.y - self.g1.y * other.g0.z - self.g1.z * other.g0.w);
}

Motor line_rotor_geometric_product(Line self, Rotor other) {
    return Motor(vec4(self.g1.x) * other.g0.yxwz * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.y) * other.g0.zwxy * vec4(-1.0, -1.0, 1.0, 1.0) + vec4(self.g1.z) * other.g0.wzyx * vec4(-1.0, 1.0, -1.0, 1.0), vec4(self.g0.x) * other.g0.yxwz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g0.y) * other.g0.zwxy * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g0.z) * other.g0.wzyx * vec4(1.0, 1.0, -1.0, 1.0));
}

Scalar line_rotor_regressive_product(Line self, Rotor other) {
    return Scalar(self.g0.x * other.g0.y + self.g0.y * other.g0.z + self.g0.z * other.g0.w);
}

Scalar line_rotor_left_contraction(Line self, Rotor other) {
    return Scalar(0.0 - self.g1.x * other.g0.y - self.g1.y * other.g0.z - self.g1.z * other.g0.w);
}

Scalar line_rotor_scalar_product(Line self, Rotor other) {
    return Scalar(0.0 - self.g1.x * other.g0.y - self.g1.y * other.g0.z - self.g1.z * other.g0.w);
}

PointAndPlane line_point_geometric_product(Line self, Point other) {
    return PointAndPlane(vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g1.y) * other.g0.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g1.z) * other.g0.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g0.x, self.g0.x, self.g1.x, self.g1.x) * other.g0.yxwz * vec4(1.0, -1.0, 1.0, -1.0), vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g0.z) * other.g0.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g1.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g1.z) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g1.x, self.g1.x, self.g0.x, self.g0.x) * other.g0.yxwz * vec4(1.0, -1.0, 1.0, -1.0));
}

Plane line_point_regressive_product(Line self, Point other) {
    return Plane(vec4(self.g0.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.z) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + vec4(self.g1.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.z) * other.g0.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g0.x, self.g0.x, self.g1.x, self.g1.x) * other.g0.yxwz * vec4(-1.0, 1.0, -1.0, 1.0));
}

Plane line_point_inner_product(Line self, Point other) {
    return Plane(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g0.z) * other.g0.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g1.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g1.z) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g1.x, self.g1.x, self.g0.x, self.g0.x) * other.g0.yxwz * vec4(1.0, -1.0, 1.0, -1.0));
}

Plane line_point_left_contraction(Line self, Point other) {
    return Plane(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g0.z) * other.g0.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g1.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g1.z) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g1.x, self.g1.x, self.g0.x, self.g0.x) * other.g0.yxwz * vec4(1.0, -1.0, 1.0, -1.0));
}

IdealPoint line_ideal_point_into(Line self) {
    return IdealPoint(self.g0);
}

Line line_ideal_point_add(Line self, IdealPoint other) {
    return Line(self.g0 + other.g0, self.g1);
}

Line line_ideal_point_sub(Line self, IdealPoint other) {
    return Line(self.g0 - other.g0, self.g1);
}

Motor line_ideal_point_geometric_product(Line self, IdealPoint other) {
    return Motor(vec4(self.g0.y) * vec4(other.g0.y, other.g0.z, other.g0.y, other.g0.x) * vec4(-1.0, -1.0, 0.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.z, other.g0.y, other.g0.x, other.g0.z) * vec4(-1.0, 1.0, -1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0), vec4(self.g1.y) * vec4(other.g0.y, other.g0.z, other.g0.y, other.g0.x) * vec4(1.0, -1.0, 0.0, 1.0) + vec4(self.g1.z) * vec4(other.g0.z, other.g0.y, other.g0.x, other.g0.z) * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4(1.0, 0.0, 1.0, -1.0));
}

Scalar line_ideal_point_regressive_product(Line self, IdealPoint other) {
    return Scalar(self.g1.x * other.g0.x + self.g1.y * other.g0.y + self.g1.z * other.g0.z);
}

Scalar line_ideal_point_inner_product(Line self, IdealPoint other) {
    return Scalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

Scalar line_ideal_point_left_contraction(Line self, IdealPoint other) {
    return Scalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

Scalar line_ideal_point_right_contraction(Line self, IdealPoint other) {
    return Scalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

Scalar line_ideal_point_scalar_product(Line self, IdealPoint other) {
    return Scalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

PointAndPlane line_plane_geometric_product(Line self, Plane other) {
    return PointAndPlane(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g0.z) * other.g0.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g1.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g1.z) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g1.x, self.g1.x, self.g0.x, self.g0.x) * other.g0.yxwz * vec4(1.0, -1.0, 1.0, -1.0), vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g1.y) * other.g0.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g1.z) * other.g0.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g0.x, self.g0.x, self.g1.x, self.g1.x) * other.g0.yxwz * vec4(1.0, -1.0, 1.0, -1.0));
}

Point line_plane_outer_product(Line self, Plane other) {
    return Point(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g0.z) * other.g0.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g1.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g1.z) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g1.x, self.g1.x, self.g0.x, self.g0.x) * other.g0.yxwz * vec4(1.0, -1.0, 1.0, -1.0));
}

Plane line_plane_inner_product(Line self, Plane other) {
    return Plane(vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g1.y) * other.g0.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g1.z) * other.g0.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g0.x, self.g0.x, self.g1.x, self.g1.x) * other.g0.yxwz * vec4(1.0, -1.0, 1.0, -1.0));
}

Plane line_plane_right_contraction(Line self, Plane other) {
    return Plane(vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g1.y) * other.g0.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g1.z) * other.g0.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g0.x, self.g0.x, self.g1.x, self.g1.x) * other.g0.yxwz * vec4(1.0, -1.0, 1.0, -1.0));
}

Line line_line_add(Line self, Line other) {
    return Line(self.g0 + other.g0, self.g1 + other.g1);
}

Line line_line_sub(Line self, Line other) {
    return Line(self.g0 - other.g0, self.g1 - other.g1);
}

Line line_line_mul(Line self, Line other) {
    return Line(self.g0 * other.g0, self.g1 * other.g1);
}

Line line_line_div(Line self, Line other) {
    return Line(vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(1.0, 1.0, 1.0) / vec3(other.g0.x, other.g0.y, other.g0.z) * vec3(1.0, 1.0, 1.0), vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(1.0, 1.0, 1.0) / vec3(other.g1.x, other.g1.y, other.g1.z) * vec3(1.0, 1.0, 1.0));
}

Motor line_line_geometric_product(Line self, Line other) {
    return Motor(vec4(self.g0.y) * vec4(other.g0.y, other.g0.z, other.g0.y, other.g0.x) * vec4(-1.0, -1.0, 0.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.z, other.g0.y, other.g0.x, other.g0.z) * vec4(-1.0, 1.0, -1.0, 0.0) + vec4(self.g1.x) * vec4(other.g1.x, other.g1.x, other.g1.z, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.y) * vec4(other.g1.y, other.g1.z, other.g1.y, other.g1.x) * vec4(-1.0, -1.0, 0.0, 1.0) + vec4(self.g1.z) * vec4(other.g1.z, other.g1.y, other.g1.x, other.g1.z) * vec4(-1.0, 1.0, -1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0), vec4(self.g0.y) * vec4(other.g1.y, other.g1.z, other.g1.y, other.g1.x) * vec4(1.0, -1.0, 0.0, 1.0) + vec4(self.g0.z) * vec4(other.g1.z, other.g1.y, other.g1.x, other.g1.z) * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4(1.0, 0.0, 1.0, -1.0) + vec4(self.g1.y) * vec4(other.g0.y, other.g0.z, other.g0.y, other.g0.x) * vec4(1.0, -1.0, 0.0, 1.0) + vec4(self.g1.z) * vec4(other.g0.z, other.g0.y, other.g0.x, other.g0.z) * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.x, other.g1.z, other.g1.y) * vec4(1.0, 0.0, 1.0, -1.0));
}

Scalar line_line_regressive_product(Line self, Line other) {
    return Scalar(self.g0.x * other.g1.x + self.g0.y * other.g1.y + self.g0.z * other.g1.z + self.g1.x * other.g0.x + self.g1.y * other.g0.y + self.g1.z * other.g0.z);
}

Scalar line_line_inner_product(Line self, Line other) {
    return Scalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g1.x * other.g1.x - self.g1.y * other.g1.y - self.g1.z * other.g1.z);
}

Scalar line_line_left_contraction(Line self, Line other) {
    return Scalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g1.x * other.g1.x - self.g1.y * other.g1.y - self.g1.z * other.g1.z);
}

Scalar line_line_right_contraction(Line self, Line other) {
    return Scalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g1.x * other.g1.x - self.g1.y * other.g1.y - self.g1.z * other.g1.z);
}

Scalar line_line_scalar_product(Line self, Line other) {
    return Scalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g1.x * other.g1.x - self.g1.y * other.g1.y - self.g1.z * other.g1.z);
}

Motor line_translator_geometric_product(Line self, Translator other) {
    return Motor(vec4(self.g0.y) * other.g0.zwzy * vec4(-1.0, -1.0, 0.0, 1.0) + vec4(self.g0.z) * other.g0.wzyw * vec4(-1.0, 1.0, -1.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.x, self.g1.x, self.g0.x, self.g0.x) * other.g0.yxwz * vec4(-1.0, 1.0, 1.0, -1.0), vec4(self.g1.x) * other.g0.yywz * vec4(1.0, 0.0, 1.0, -1.0) + vec4(self.g1.y) * other.g0.zwzy * vec4(1.0, -1.0, 0.0, 1.0) + vec4(self.g1.z) * other.g0.wzyw * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g0.x, self.g0.x, self.g0.y, self.g0.z) * vec4(other.g0.x) * vec4(0.0, 1.0, 1.0, 1.0));
}

Scalar line_translator_regressive_product(Line self, Translator other) {
    return Scalar(self.g1.x * other.g0.y + self.g1.y * other.g0.z + self.g1.z * other.g0.w);
}

Scalar line_translator_left_contraction(Line self, Translator other) {
    return Scalar(0.0 - self.g0.x * other.g0.y - self.g0.y * other.g0.z - self.g0.z * other.g0.w);
}

Scalar line_translator_scalar_product(Line self, Translator other) {
    return Scalar(0.0 - self.g0.x * other.g0.y - self.g0.y * other.g0.z - self.g0.z * other.g0.w);
}

Motor line_motor_add(Line self, Motor other) {
    return Motor(vec4(self.g0.x, self.g1.x, self.g1.y, self.g1.z) * vec4(0.0, 1.0, 1.0, 1.0) + other.g0, vec4(self.g0.x, self.g0.x, self.g0.y, self.g0.z) * vec4(0.0, 1.0, 1.0, 1.0) + other.g1);
}

Motor line_motor_sub(Line self, Motor other) {
    return Motor(vec4(self.g0.x, self.g1.x, self.g1.y, self.g1.z) * vec4(0.0, 1.0, 1.0, 1.0) - other.g0, vec4(self.g0.x, self.g0.x, self.g0.y, self.g0.z) * vec4(0.0, 1.0, 1.0, 1.0) - other.g1);
}

Motor line_motor_geometric_product(Line self, Motor other) {
    return Motor(vec4(self.g0.x) * other.g1.yxwz * vec4(-1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * other.g1.zwxy * vec4(-1.0, -1.0, -1.0, 1.0) + vec4(self.g0.z) * other.g1.wzyx * vec4(-1.0, 1.0, -1.0, -1.0) + vec4(self.g1.x) * other.g0.yxwz * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.y) * other.g0.zwxy * vec4(-1.0, -1.0, 1.0, 1.0) + vec4(self.g1.z) * other.g0.wzyx * vec4(-1.0, 1.0, -1.0, 1.0), vec4(self.g0.x) * other.g0.yxwz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g0.y) * other.g0.zwxy * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g0.z) * other.g0.wzyx * vec4(1.0, 1.0, -1.0, 1.0) + vec4(self.g1.x) * other.g1.yxwz * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g1.y) * other.g1.zwxy * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g1.z) * other.g1.wzyx * vec4(1.0, 1.0, -1.0, -1.0));
}

Scalar line_motor_scalar_product(Line self, Motor other) {
    return Scalar(0.0 - self.g0.x * other.g1.y - self.g0.y * other.g1.z - self.g0.z * other.g1.w - self.g1.x * other.g0.y - self.g1.y * other.g0.z - self.g1.z * other.g0.w);
}

PointAndPlane line_point_and_plane_geometric_product(Line self, PointAndPlane other) {
    return PointAndPlane(vec4(self.g0.x) * vec4(other.g0.y, other.g0.x, other.g1.w, other.g1.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * vec4(other.g0.z, other.g1.w, other.g0.x, other.g1.y) * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.w, other.g1.z, other.g1.y, other.g0.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g1.x) * vec4(other.g1.y, other.g1.x, other.g0.w, other.g0.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g1.y) * vec4(other.g1.z, other.g0.w, other.g1.x, other.g0.y) * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g1.z) * vec4(other.g1.w, other.g0.z, other.g0.y, other.g1.x) * vec4(1.0, 1.0, -1.0, -1.0), vec4(self.g0.x) * vec4(other.g1.y, other.g1.x, other.g0.w, other.g0.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * vec4(other.g1.z, other.g0.w, other.g1.x, other.g0.y) * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.z) * vec4(other.g1.w, other.g0.z, other.g0.y, other.g1.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g1.x) * vec4(other.g0.y, other.g0.x, other.g1.w, other.g1.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g1.y) * vec4(other.g0.z, other.g1.w, other.g0.x, other.g1.y) * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g1.z) * vec4(other.g0.w, other.g1.z, other.g1.y, other.g0.x) * vec4(1.0, 1.0, -1.0, -1.0));
}

Plane line_point_and_plane_regressive_product(Line self, PointAndPlane other) {
    return Plane(vec4(self.g0.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.z) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + vec4(self.g1.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.z) * other.g0.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g0.x, self.g0.x, self.g1.x, self.g1.x) * other.g0.yxwz * vec4(-1.0, 1.0, -1.0, 1.0));
}

Point line_point_and_plane_outer_product(Line self, PointAndPlane other) {
    return Point(vec4(self.g0.y) * other.g1.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g0.z) * other.g1.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g1.y) * other.g1.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g1.z) * other.g1.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g1.x, self.g1.x, self.g0.x, self.g0.x) * other.g1.yxwz * vec4(1.0, -1.0, 1.0, -1.0));
}

Plane line_point_and_plane_inner_product(Line self, PointAndPlane other) {
    return Plane(vec4(self.g0.x) * vec4(other.g1.y, other.g1.x, other.g0.w, other.g0.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * vec4(other.g1.z, other.g0.w, other.g1.x, other.g0.y) * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.z) * vec4(other.g1.w, other.g0.z, other.g0.y, other.g1.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g1.x) * vec4(other.g0.y, other.g0.x, other.g1.w, other.g1.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g1.y) * vec4(other.g0.z, other.g1.w, other.g0.x, other.g1.y) * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g1.z) * vec4(other.g0.w, other.g1.z, other.g1.y, other.g0.x) * vec4(1.0, 1.0, -1.0, -1.0));
}

Plane line_point_and_plane_left_contraction(Line self, PointAndPlane other) {
    return Plane(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g0.z) * other.g0.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g1.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g1.z) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g1.x, self.g1.x, self.g0.x, self.g0.x) * other.g0.yxwz * vec4(1.0, -1.0, 1.0, -1.0));
}

Plane line_point_and_plane_right_contraction(Line self, PointAndPlane other) {
    return Plane(vec4(self.g0.y) * other.g1.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g1.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g1.y) * other.g1.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g1.z) * other.g1.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g0.x, self.g0.x, self.g1.x, self.g1.x) * other.g1.yxwz * vec4(1.0, -1.0, 1.0, -1.0));
}

Scalar line_squared_magnitude(Line self) {
    return line_line_scalar_product(self, line_reversal(self));
}

Scalar line_magnitude(Line self) {
    return Scalar(sqrt(line_squared_magnitude(self).g0));
}

Line line_scale(Line self, float other) {
    return line_scalar_geometric_product(self, Scalar(other));
}

Line line_signum(Line self) {
    return line_scalar_geometric_product(self, Scalar(1.0 / line_magnitude(self).g0));
}

Line line_inverse(Line self) {
    return line_scalar_geometric_product(line_reversal(self), Scalar(1.0 / line_squared_magnitude(self).g0));
}

Translator translator_zero() {
    return Translator(vec4(0.0));
}

Translator translator_one() {
    return Translator(vec4(1.0, 0.0, 0.0, 0.0));
}

Translator translator_neg(Translator self) {
    return Translator(self.g0 * vec4(-1.0));
}

Translator translator_automorphism(Translator self) {
    return Translator(self.g0);
}

Translator translator_reversal(Translator self) {
    return Translator(self.g0 * vec4(1.0, -1.0, -1.0, -1.0));
}

Translator translator_conjugation(Translator self) {
    return Translator(self.g0 * vec4(1.0, -1.0, -1.0, -1.0));
}

Scalar translator_scalar_into(Translator self) {
    return Scalar(self.g0.x);
}

Translator translator_scalar_add(Translator self, Scalar other) {
    return Translator(self.g0 + vec4(other.g0) * vec4(1.0, 0.0, 0.0, 0.0));
}

Translator translator_scalar_sub(Translator self, Scalar other) {
    return Translator(self.g0 - vec4(other.g0) * vec4(1.0, 0.0, 0.0, 0.0));
}

Translator translator_scalar_geometric_product(Translator self, Scalar other) {
    return Translator(self.g0 * vec4(other.g0));
}

Translator translator_scalar_outer_product(Translator self, Scalar other) {
    return Translator(self.g0 * vec4(other.g0));
}

Translator translator_scalar_inner_product(Translator self, Scalar other) {
    return Translator(self.g0 * vec4(other.g0));
}

Scalar translator_scalar_left_contraction(Translator self, Scalar other) {
    return Scalar(self.g0.x * other.g0);
}

Translator translator_scalar_right_contraction(Translator self, Scalar other) {
    return Translator(self.g0 * vec4(other.g0));
}

Scalar translator_scalar_scalar_product(Translator self, Scalar other) {
    return Scalar(self.g0.x * other.g0);
}

MultiVector translator_multi_vector_add(Translator self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + other.g0, other.g1, other.g2, self.g0 * vec4(0.0, 1.0, 1.0, 1.0) + other.g3);
}

MultiVector translator_multi_vector_sub(Translator self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) - other.g0, vec4(0.0) - other.g1, vec4(0.0) - other.g2, self.g0 * vec4(0.0, 1.0, 1.0, 1.0) - other.g3);
}

MultiVector translator_multi_vector_geometric_product(Translator self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g0 + vec4(self.g0.y) * other.g3.yxwz * vec4(-1.0, -1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g3.zwxy * vec4(-1.0, -1.0, -1.0, 1.0) + vec4(self.g0.w) * other.g3.wzyx * vec4(-1.0, 1.0, -1.0, -1.0), vec4(self.g0.x) * other.g1 + vec4(self.g0.y) * other.g2.yxwz * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g2.zwxy * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.w) * other.g2.wzyx * vec4(1.0, 1.0, -1.0, -1.0), vec4(self.g0.x) * other.g2 + vec4(self.g0.y) * other.g1.yxwz * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g1.zwxy * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.w) * other.g1.wzyx * vec4(1.0, 1.0, -1.0, -1.0), vec4(self.g0.x) * other.g3 + vec4(self.g0.y) * other.g0.yxwz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g0.zwxy * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g0.w) * other.g0.wzyx * vec4(1.0, 1.0, -1.0, 1.0));
}

MultiVector translator_multi_vector_outer_product(Translator self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g0, vec4(self.g0.x) * other.g1 + vec4(self.g0.z) * other.g2.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * other.g2.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + self.g0.xxyy * other.g2.xxwz * vec4(0.0, 0.0, 1.0, -1.0), vec4(self.g0.x) * other.g2, vec4(self.g0.x) * other.g3 + vec4(self.g0.z) * other.g0.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + self.g0.yyxx * other.g0.yxxx * vec4(1.0, 1.0, 0.0, 0.0));
}

MultiVector translator_multi_vector_inner_product(Translator self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * other.g3.zzxz * vec4(-1.0, 0.0, -1.0, 0.0) + vec4(self.g0.w) * other.g3.wwwx * vec4(-1.0, 0.0, 0.0, -1.0) + self.g0.yyxx * other.g3.yxxx * vec4(-1.0, -1.0, 0.0, 0.0), vec4(self.g0.x) * other.g1 + vec4(self.g0.z) * vec4(other.g2.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g2.w) * vec4(1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * other.g2.yxxx * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g0.x) * other.g2 + vec4(self.g0.z) * other.g1.wwxy * vec4(0.0, -1.0, -1.0, 1.0) + vec4(self.g0.w) * other.g1.zzyx * vec4(0.0, 1.0, -1.0, -1.0) + self.g0.xyyy * other.g1.xxwz * vec4(0.0, -1.0, 1.0, -1.0), vec4(self.g0.x) * other.g3 + self.g0 * vec4(other.g0.x) * vec4(0.0, 1.0, 1.0, 1.0));
}

MultiVector translator_multi_vector_left_contraction(Translator self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * other.g3.zzxz * vec4(-1.0, 0.0, -1.0, 0.0) + vec4(self.g0.w) * other.g3.wwwx * vec4(-1.0, 0.0, 0.0, -1.0) + self.g0.yyxx * other.g3.yxxx * vec4(-1.0, -1.0, 0.0, 0.0), vec4(self.g0.x) * other.g1, vec4(self.g0.x) * other.g2 + vec4(self.g0.z) * other.g1.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * other.g1.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + self.g0.xxyy * other.g1.xxwz * vec4(0.0, 0.0, 1.0, -1.0), vec4(self.g0.x) * other.g3);
}

Scalar translator_multi_vector_scalar_product(Translator self, MultiVector other) {
    return Scalar(self.g0.x * other.g0.x - self.g0.y * other.g3.y - self.g0.z * other.g3.z - self.g0.w * other.g3.w);
}

Motor translator_rotor_geometric_product(Translator self, Rotor other) {
    return Motor(vec4(self.g0.x) * other.g0, vec4(self.g0.y) * other.g0.yxwz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g0.zwxy * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g0.w) * other.g0.wzyx * vec4(1.0, 1.0, -1.0, 1.0));
}

Scalar translator_rotor_regressive_product(Translator self, Rotor other) {
    return Scalar(self.g0.y * other.g0.y + self.g0.z * other.g0.z + self.g0.w * other.g0.w);
}

Motor translator_rotor_outer_product(Translator self, Rotor other) {
    return Motor(vec4(self.g0.x) * other.g0, vec4(self.g0.z) * other.g0.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + self.g0.yyxx * other.g0.yxxx * vec4(1.0, 1.0, 0.0, 0.0));
}

Rotor translator_rotor_left_contraction(Translator self, Rotor other) {
    return Rotor(vec4(self.g0.x) * other.g0);
}

Translator translator_rotor_right_contraction(Translator self, Rotor other) {
    return Translator(self.g0 * vec4(other.g0.x));
}

Scalar translator_rotor_scalar_product(Translator self, Rotor other) {
    return Scalar(self.g0.x * other.g0.x);
}

Plane translator_point_regressive_product(Translator self, Point other) {
    return Plane(vec4(self.g0.z) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + self.g0.yyxx * other.g0.yxxx * vec4(-1.0, 1.0, 0.0, 0.0));
}

Point translator_point_outer_product(Translator self, Point other) {
    return Point(vec4(self.g0.x) * other.g0);
}

IdealPoint translator_ideal_point_into(Translator self) {
    return IdealPoint(vec3(self.g0.y, self.g0.z, self.g0.w));
}

Translator translator_ideal_point_add(Translator self, IdealPoint other) {
    return Translator(self.g0 + vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0));
}

Translator translator_ideal_point_sub(Translator self, IdealPoint other) {
    return Translator(self.g0 - vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0));
}

IdealPoint translator_ideal_point_outer_product(Translator self, IdealPoint other) {
    return IdealPoint(vec3(self.g0.x) * other.g0);
}

Translator translator_ideal_point_inner_product(Translator self, IdealPoint other) {
    return Translator(vec4(self.g0.z) * vec4(other.g0.y) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.z) * vec4(-1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(-1.0, 1.0, 1.0, 1.0));
}

Translator translator_ideal_point_left_contraction(Translator self, IdealPoint other) {
    return Translator(vec4(self.g0.z) * vec4(other.g0.y) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.z) * vec4(-1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(-1.0, 1.0, 1.0, 1.0));
}

Scalar translator_ideal_point_right_contraction(Translator self, IdealPoint other) {
    return Scalar(0.0 - self.g0.y * other.g0.x - self.g0.z * other.g0.y - self.g0.w * other.g0.z);
}

Scalar translator_ideal_point_scalar_product(Translator self, IdealPoint other) {
    return Scalar(0.0 - self.g0.y * other.g0.x - self.g0.z * other.g0.y - self.g0.w * other.g0.z);
}

Plane translator_plane_inner_product(Translator self, Plane other) {
    return Plane(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + self.g0.yyxx * other.g0.yxxx * vec4(1.0, -1.0, 0.0, 0.0));
}

Plane translator_plane_left_contraction(Translator self, Plane other) {
    return Plane(vec4(self.g0.x) * other.g0);
}

Plane translator_plane_right_contraction(Translator self, Plane other) {
    return Plane(vec4(self.g0.z) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + self.g0.yyxx * other.g0.yxxx * vec4(1.0, -1.0, 0.0, 0.0));
}

Motor translator_line_geometric_product(Translator self, Line other) {
    return Motor(vec4(self.g0.y) * vec4(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.z, other.g0.y, other.g0.x) * vec4(-1.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * vec4(other.g0.z, other.g0.y, other.g0.x, other.g0.z) * vec4(-1.0, 1.0, -1.0, 0.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.x, other.g1.y, other.g1.z) * vec4(0.0, 1.0, 1.0, 1.0), vec4(self.g0.y) * vec4(other.g1.x, other.g1.x, other.g1.z, other.g1.y) * vec4(1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.z, other.g1.y, other.g1.x) * vec4(1.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * vec4(other.g1.z, other.g1.y, other.g1.x, other.g1.z) * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0));
}

Scalar translator_line_regressive_product(Translator self, Line other) {
    return Scalar(self.g0.y * other.g1.x + self.g0.z * other.g1.y + self.g0.w * other.g1.z);
}

Scalar translator_line_right_contraction(Translator self, Line other) {
    return Scalar(0.0 - self.g0.y * other.g0.x - self.g0.z * other.g0.y - self.g0.w * other.g0.z);
}

Scalar translator_line_scalar_product(Translator self, Line other) {
    return Scalar(0.0 - self.g0.y * other.g0.x - self.g0.z * other.g0.y - self.g0.w * other.g0.z);
}

Translator translator_translator_add(Translator self, Translator other) {
    return Translator(self.g0 + other.g0);
}

Translator translator_translator_sub(Translator self, Translator other) {
    return Translator(self.g0 - other.g0);
}

Translator translator_translator_mul(Translator self, Translator other) {
    return Translator(self.g0 * other.g0);
}

Translator translator_translator_div(Translator self, Translator other) {
    return Translator(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.w) * vec4(1.0, 1.0, 1.0, 1.0) / vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.w) * vec4(1.0, 1.0, 1.0, 1.0));
}

Translator translator_translator_outer_product(Translator self, Translator other) {
    return Translator(vec4(self.g0.x) * other.g0 + self.g0 * vec4(other.g0.x) * vec4(0.0, 1.0, 1.0, 1.0));
}

Translator translator_translator_inner_product(Translator self, Translator other) {
    return Translator(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + self.g0.yyxx * other.g0.yxxx * vec4(-1.0, 1.0, 0.0, 0.0));
}

Translator translator_translator_left_contraction(Translator self, Translator other) {
    return Translator(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * vec4(other.g0.z) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.w) * vec4(-1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * other.g0.yxxx * vec4(-1.0, 0.0, 0.0, 0.0));
}

Translator translator_translator_right_contraction(Translator self, Translator other) {
    return Translator(vec4(self.g0.y) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.z) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0));
}

Scalar translator_translator_scalar_product(Translator self, Translator other) {
    return Scalar(self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g0.w * other.g0.w);
}

Motor translator_motor_add(Translator self, Motor other) {
    return Motor(vec4(self.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + other.g0, self.g0 * vec4(0.0, 1.0, 1.0, 1.0) + other.g1);
}

Motor translator_motor_sub(Translator self, Motor other) {
    return Motor(vec4(self.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) - other.g0, self.g0 * vec4(0.0, 1.0, 1.0, 1.0) - other.g1);
}

Motor translator_motor_geometric_product(Translator self, Motor other) {
    return Motor(vec4(self.g0.x) * other.g0 + vec4(self.g0.y) * other.g1.yxwz * vec4(-1.0, -1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g1.zwxy * vec4(-1.0, -1.0, -1.0, 1.0) + vec4(self.g0.w) * other.g1.wzyx * vec4(-1.0, 1.0, -1.0, -1.0), vec4(self.g0.x) * other.g1 + vec4(self.g0.y) * other.g0.yxwz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g0.zwxy * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g0.w) * other.g0.wzyx * vec4(1.0, 1.0, -1.0, 1.0));
}

Translator translator_motor_regressive_product(Translator self, Motor other) {
    return Translator(vec4(self.g0.y) * vec4(other.g0.y, other.g1.x, other.g0.y, other.g0.y) * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g0.z) * vec4(other.g0.z, other.g0.z, other.g1.x, other.g0.z) * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.w, other.g0.w, other.g0.w, other.g1.x) * vec4(1.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g1.x) * vec4(1.0, 0.0, 0.0, 0.0));
}

Motor translator_motor_outer_product(Translator self, Motor other) {
    return Motor(vec4(self.g0.x) * other.g0, vec4(self.g0.x) * other.g1 + vec4(self.g0.z) * other.g0.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + self.g0.yyxx * other.g0.yxxx * vec4(1.0, 1.0, 0.0, 0.0));
}

Motor translator_motor_inner_product(Translator self, Motor other) {
    return Motor(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * other.g1.zzxz * vec4(-1.0, 0.0, -1.0, 0.0) + vec4(self.g0.w) * other.g1.wwwx * vec4(-1.0, 0.0, 0.0, -1.0) + self.g0.yyxx * other.g1.yxxx * vec4(-1.0, -1.0, 0.0, 0.0), vec4(self.g0.x) * other.g1 + self.g0 * vec4(other.g0.x) * vec4(0.0, 1.0, 1.0, 1.0));
}

Motor translator_motor_left_contraction(Translator self, Motor other) {
    return Motor(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * other.g1.zzxz * vec4(-1.0, 0.0, -1.0, 0.0) + vec4(self.g0.w) * other.g1.wwwx * vec4(-1.0, 0.0, 0.0, -1.0) + self.g0.yyxx * other.g1.yxxx * vec4(-1.0, -1.0, 0.0, 0.0), vec4(self.g0.x) * other.g1);
}

Translator translator_motor_right_contraction(Translator self, Motor other) {
    return Translator(vec4(self.g0.y) * vec4(other.g1.y, other.g0.x, other.g1.y, other.g1.y) * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.z) * vec4(other.g1.z, other.g1.z, other.g0.x, other.g1.z) * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * vec4(other.g1.w, other.g1.w, other.g1.w, other.g0.x) * vec4(-1.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0));
}

Scalar translator_motor_scalar_product(Translator self, Motor other) {
    return Scalar(self.g0.x * other.g0.x - self.g0.y * other.g1.y - self.g0.z * other.g1.z - self.g0.w * other.g1.w);
}

PointAndPlane translator_point_and_plane_geometric_product(Translator self, PointAndPlane other) {
    return PointAndPlane(vec4(self.g0.x) * other.g0 + vec4(self.g0.y) * vec4(other.g0.y, other.g0.x, other.g1.w, other.g1.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z, other.g1.w, other.g0.x, other.g1.y) * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.w) * vec4(other.g0.w, other.g1.z, other.g1.y, other.g0.x) * vec4(1.0, 1.0, -1.0, -1.0), vec4(self.g0.x) * other.g1 + vec4(self.g0.y) * vec4(other.g1.y, other.g1.x, other.g0.w, other.g0.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.z, other.g0.w, other.g1.x, other.g0.y) * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.w) * vec4(other.g1.w, other.g0.z, other.g0.y, other.g1.x) * vec4(1.0, 1.0, -1.0, -1.0));
}

Plane translator_point_and_plane_regressive_product(Translator self, PointAndPlane other) {
    return Plane(vec4(self.g0.z) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + self.g0.yyxx * other.g0.yxxx * vec4(-1.0, 1.0, 0.0, 0.0));
}

PointAndPlane translator_point_and_plane_outer_product(Translator self, PointAndPlane other) {
    return PointAndPlane(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * other.g1.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * other.g1.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + self.g0.xxyy * other.g1.xxwz * vec4(0.0, 0.0, 1.0, -1.0), vec4(self.g0.x) * other.g1);
}

PointAndPlane translator_point_and_plane_inner_product(Translator self, PointAndPlane other) {
    return PointAndPlane(vec4(self.g0.x) * other.g0, vec4(self.g0.x) * other.g1 + vec4(self.g0.y) * vec4(other.g1.y, other.g1.x, other.g0.w, other.g0.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.z, other.g0.w, other.g1.x, other.g0.y) * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.w) * vec4(other.g1.w, other.g0.z, other.g0.y, other.g1.x) * vec4(1.0, 1.0, -1.0, -1.0));
}

PointAndPlane translator_point_and_plane_left_contraction(Translator self, PointAndPlane other) {
    return PointAndPlane(vec4(self.g0.x) * other.g0, vec4(self.g0.x) * other.g1 + vec4(self.g0.z) * other.g0.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * other.g0.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + self.g0.xxyy * other.g0.xxwz * vec4(0.0, 0.0, 1.0, -1.0));
}

Plane translator_point_and_plane_right_contraction(Translator self, PointAndPlane other) {
    return Plane(vec4(self.g0.z) * other.g1.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.w) * other.g1.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + self.g0.yyxx * other.g1.yxxx * vec4(1.0, -1.0, 0.0, 0.0));
}

Scalar translator_squared_magnitude(Translator self) {
    return translator_translator_scalar_product(self, translator_reversal(self));
}

Scalar translator_magnitude(Translator self) {
    return Scalar(sqrt(translator_squared_magnitude(self).g0));
}

Translator translator_scale(Translator self, float other) {
    return translator_scalar_geometric_product(self, Scalar(other));
}

Translator translator_signum(Translator self) {
    return translator_scalar_geometric_product(self, Scalar(1.0 / translator_magnitude(self).g0));
}

Translator translator_inverse(Translator self) {
    return translator_scalar_geometric_product(translator_reversal(self), Scalar(1.0 / translator_squared_magnitude(self).g0));
}

Motor motor_zero() {
    return Motor(vec4(0.0), vec4(0.0));
}

Motor motor_one() {
    return Motor(vec4(1.0, 0.0, 0.0, 0.0), vec4(0.0));
}

Motor motor_neg(Motor self) {
    return Motor(self.g0 * vec4(-1.0), self.g1 * vec4(-1.0));
}

Motor motor_automorphism(Motor self) {
    return Motor(self.g0, self.g1);
}

Motor motor_reversal(Motor self) {
    return Motor(self.g0 * vec4(1.0, -1.0, -1.0, -1.0), self.g1 * vec4(1.0, -1.0, -1.0, -1.0));
}

Motor motor_conjugation(Motor self) {
    return Motor(self.g0 * vec4(1.0, -1.0, -1.0, -1.0), self.g1 * vec4(1.0, -1.0, -1.0, -1.0));
}

Motor motor_dual(Motor self) {
    return Motor(self.g1, self.g0);
}

Scalar motor_scalar_into(Motor self) {
    return Scalar(self.g0.x);
}

Motor motor_scalar_add(Motor self, Scalar other) {
    return Motor(self.g0 + vec4(other.g0) * vec4(1.0, 0.0, 0.0, 0.0), self.g1);
}

Motor motor_scalar_sub(Motor self, Scalar other) {
    return Motor(self.g0 - vec4(other.g0) * vec4(1.0, 0.0, 0.0, 0.0), self.g1);
}

Motor motor_scalar_geometric_product(Motor self, Scalar other) {
    return Motor(self.g0 * vec4(other.g0), self.g1 * vec4(other.g0));
}

Scalar motor_scalar_regressive_product(Motor self, Scalar other) {
    return Scalar(self.g1.x * other.g0);
}

Motor motor_scalar_outer_product(Motor self, Scalar other) {
    return Motor(self.g0 * vec4(other.g0), self.g1 * vec4(other.g0));
}

Motor motor_scalar_inner_product(Motor self, Scalar other) {
    return Motor(self.g0 * vec4(other.g0), self.g1 * vec4(other.g0));
}

Scalar motor_scalar_left_contraction(Motor self, Scalar other) {
    return Scalar(self.g0.x * other.g0);
}

Motor motor_scalar_right_contraction(Motor self, Scalar other) {
    return Motor(self.g0 * vec4(other.g0), self.g1 * vec4(other.g0));
}

Scalar motor_scalar_scalar_product(Motor self, Scalar other) {
    return Scalar(self.g0.x * other.g0);
}

MultiVector motor_multi_vector_add(Motor self, MultiVector other) {
    return MultiVector(self.g0 + other.g0, other.g1, other.g2, self.g1 + other.g3);
}

MultiVector motor_multi_vector_sub(Motor self, MultiVector other) {
    return MultiVector(self.g0 - other.g0, vec4(0.0) - other.g1, vec4(0.0) - other.g2, self.g1 - other.g3);
}

MultiVector motor_multi_vector_geometric_product(Motor self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g0 + vec4(self.g0.y) * other.g0.yxwz * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g0.zwxy * vec4(-1.0, -1.0, 1.0, 1.0) + vec4(self.g0.w) * other.g0.wzyx * vec4(-1.0, 1.0, -1.0, 1.0) + vec4(self.g1.x) * other.g3 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g1.y) * other.g3.yxwz * vec4(-1.0, -1.0, 1.0, -1.0) + vec4(self.g1.z) * other.g3.zwxy * vec4(-1.0, -1.0, -1.0, 1.0) + vec4(self.g1.w) * other.g3.wzyx * vec4(-1.0, 1.0, -1.0, -1.0), vec4(self.g0.x) * other.g1 + vec4(self.g0.y) * other.g1.yxwz * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g1.zwxy * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.w) * other.g1.wzyx * vec4(1.0, 1.0, -1.0, -1.0) - vec4(self.g1.x) * other.g2 + vec4(self.g1.y) * other.g2.yxwz * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g1.z) * other.g2.zwxy * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g1.w) * other.g2.wzyx * vec4(1.0, 1.0, -1.0, -1.0), vec4(self.g0.x) * other.g2 + vec4(self.g0.y) * other.g2.yxwz * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g2.zwxy * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.w) * other.g2.wzyx * vec4(1.0, 1.0, -1.0, -1.0) - vec4(self.g1.x) * other.g1 + vec4(self.g1.y) * other.g1.yxwz * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g1.z) * other.g1.zwxy * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g1.w) * other.g1.wzyx * vec4(1.0, 1.0, -1.0, -1.0), vec4(self.g0.x) * other.g3 + vec4(self.g0.y) * other.g3.yxwz * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g3.zwxy * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.w) * other.g3.wzyx * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g1.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g1.y) * other.g0.yxwz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g1.z) * other.g0.zwxy * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g1.w) * other.g0.wzyx * vec4(1.0, 1.0, -1.0, 1.0));
}

MultiVector motor_multi_vector_regressive_product(Motor self, MultiVector other) {
    return MultiVector(vec4(self.g0.y) * other.g3.yxyy * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g0.z) * other.g3.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g3.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * other.g0 + vec4(self.g1.y) * vec4(other.g0.y) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.x) * vec4(other.g3.x) * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g1.x) * other.g1 + vec4(self.g1.z) * vec4(other.g1.z) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g1.w) * vec4(other.g1.w) * vec4(-1.0, 0.0, 0.0, 0.0) + self.g1.yxxx * other.g1.yxxx * vec4(-1.0, 0.0, 0.0, 0.0), vec4(self.g0.z) * other.g1.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.w) * other.g1.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g1.x) * other.g2 + vec4(self.g1.z) * vec4(other.g2.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * vec4(other.g2.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.x, self.g1.y, self.g0.y, self.g0.y) * vec4(other.g1.x, other.g2.x, other.g1.w, other.g1.z) * vec4(0.0, 1.0, -1.0, 1.0), vec4(self.g1.x) * other.g3 + self.g1 * vec4(other.g3.x) * vec4(0.0, 1.0, 1.0, 1.0));
}

MultiVector motor_multi_vector_outer_product(Motor self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g0 + self.g0 * vec4(other.g0.x) * vec4(0.0, 1.0, 1.0, 1.0), vec4(self.g0.x) * other.g1 + vec4(self.g1.y) * other.g2.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * other.g2.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g1.w) * other.g2.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + self.g0 * vec4(other.g1.x) * vec4(0.0, -1.0, -1.0, -1.0), vec4(self.g0.x) * other.g2 + vec4(self.g0.z) * vec4(other.g2.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g2.w) * vec4(1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * other.g2.yxxx * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g0.x) * other.g3 + vec4(self.g0.z) * vec4(other.g3.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g3.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y) * other.g0.yxyy * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * other.g0.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + self.g0.yxxx * other.g3.yxxx * vec4(1.0, 0.0, 0.0, 0.0));
}

MultiVector motor_multi_vector_inner_product(Motor self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * other.g3 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g1.y) * other.g3.yxyy * vec4(-1.0, -1.0, 0.0, 0.0) + vec4(self.g1.z) * other.g3.zzxz * vec4(-1.0, 0.0, -1.0, 0.0) + vec4(self.g1.w) * other.g3.wwwx * vec4(-1.0, 0.0, 0.0, -1.0) + self.g0.yyxx * other.g0.yxxx * vec4(-1.0, 1.0, 0.0, 0.0), vec4(self.g0.x) * other.g1 + vec4(self.g0.z) * vec4(other.g1.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g1.w) * vec4(1.0, 0.0, 0.0, 0.0) - vec4(self.g1.x) * other.g2 + vec4(self.g1.y) * vec4(other.g2.y) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g2.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.w) * vec4(other.g2.w) * vec4(1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * other.g1.yxxx * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g0.x) * other.g2 + vec4(self.g0.z) * other.g2.wwxy * vec4(0.0, -1.0, -1.0, 1.0) + vec4(self.g0.w) * other.g2.zzyx * vec4(0.0, 1.0, -1.0, -1.0) - vec4(self.g1.x) * other.g1 + vec4(self.g1.y) * other.g1.xxwz * vec4(0.0, -1.0, 1.0, -1.0) + vec4(self.g1.z) * other.g1.wwxy * vec4(0.0, -1.0, -1.0, 1.0) + vec4(self.g1.w) * other.g1.zzyx * vec4(0.0, 1.0, -1.0, -1.0) + self.g0.xyyy * other.g2.xxwz * vec4(0.0, -1.0, 1.0, -1.0), vec4(self.g0.x) * other.g3 + vec4(self.g1.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + self.g0 * vec4(other.g3.x) * vec4(0.0, -1.0, -1.0, -1.0));
}

MultiVector motor_multi_vector_left_contraction(Motor self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * vec4(other.g0.z) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.w) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g1.x) * vec4(other.g3.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y) * other.g3.yxyy * vec4(-1.0, -1.0, 0.0, 0.0) + vec4(self.g1.z) * other.g3.zzxz * vec4(-1.0, 0.0, -1.0, 0.0) + vec4(self.g1.w) * other.g3.wwwx * vec4(-1.0, 0.0, 0.0, -1.0) + self.g0.yxxx * other.g0.yxxx * vec4(-1.0, 0.0, 0.0, 0.0), vec4(self.g0.x) * other.g1 + vec4(self.g0.z) * vec4(other.g1.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g1.w) * vec4(1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * other.g1.yxxx * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g0.x) * other.g2 + vec4(self.g1.y) * other.g1.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * other.g1.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g1.w) * other.g1.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + self.g0 * vec4(other.g2.x) * vec4(0.0, -1.0, -1.0, -1.0), vec4(self.g0.x) * other.g3 + self.g0 * vec4(other.g3.x) * vec4(0.0, -1.0, -1.0, -1.0));
}

MultiVector motor_multi_vector_right_contraction(Motor self, MultiVector other) {
    return MultiVector(vec4(self.g0.y) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.z) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * other.g3 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g3.y) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g3.z) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g1.w) * vec4(other.g3.w) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0), vec4(0.0) - vec4(self.g1.x) * other.g2 + vec4(self.g1.z) * vec4(other.g2.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.w) * vec4(other.g2.w) * vec4(1.0, 0.0, 0.0, 0.0) + self.g1.yxxx * other.g2.yxxx * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g0.z) * other.g2.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * other.g2.zzyz * vec4(0.0, 1.0, -1.0, 0.0) - vec4(self.g1.x) * other.g1 + vec4(self.g1.z) * vec4(other.g1.x) * vec4(0.0, 0.0, -1.0, 0.0) + vec4(self.g1.w) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.x, self.g1.y, self.g0.y, self.g0.y) * vec4(other.g2.x, other.g1.x, other.g2.w, other.g2.z) * vec4(0.0, -1.0, 1.0, -1.0), vec4(self.g1.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + self.g1 * vec4(other.g0.x) * vec4(0.0, 1.0, 1.0, 1.0));
}

Scalar motor_multi_vector_scalar_product(Motor self, MultiVector other) {
    return Scalar(self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g0.w * other.g0.w + self.g1.x * other.g3.x - self.g1.y * other.g3.y - self.g1.z * other.g3.z - self.g1.w * other.g3.w);
}

Rotor motor_rotor_into(Motor self) {
    return Rotor(self.g0);
}

Motor motor_rotor_add(Motor self, Rotor other) {
    return Motor(self.g0 + other.g0, self.g1);
}

Motor motor_rotor_sub(Motor self, Rotor other) {
    return Motor(self.g0 - other.g0, self.g1);
}

Motor motor_rotor_geometric_product(Motor self, Rotor other) {
    return Motor(vec4(self.g0.x) * other.g0 + vec4(self.g0.y) * other.g0.yxwz * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g0.zwxy * vec4(-1.0, -1.0, 1.0, 1.0) + vec4(self.g0.w) * other.g0.wzyx * vec4(-1.0, 1.0, -1.0, 1.0), vec4(self.g1.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g1.y) * other.g0.yxwz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g1.z) * other.g0.zwxy * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g1.w) * other.g0.wzyx * vec4(1.0, 1.0, -1.0, 1.0));
}

Rotor motor_rotor_regressive_product(Motor self, Rotor other) {
    return Rotor(vec4(self.g1.x) * other.g0 + vec4(self.g1.z) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + self.g1.yxxx * other.g0.yxxx * vec4(1.0, 0.0, 0.0, 0.0));
}

Motor motor_rotor_outer_product(Motor self, Rotor other) {
    return Motor(vec4(self.g0.x) * other.g0 + self.g0 * vec4(other.g0.x) * vec4(0.0, 1.0, 1.0, 1.0), vec4(self.g1.y) * other.g0.yxyy * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * other.g0.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0));
}

Motor motor_rotor_inner_product(Motor self, Rotor other) {
    return Motor(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + self.g0.yyxx * other.g0.yxxx * vec4(-1.0, 1.0, 0.0, 0.0), vec4(self.g1.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + self.g1 * vec4(other.g0.x) * vec4(0.0, 1.0, 1.0, 1.0));
}

Rotor motor_rotor_left_contraction(Motor self, Rotor other) {
    return Rotor(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * vec4(other.g0.z) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.w) * vec4(-1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * other.g0.yxxx * vec4(-1.0, 0.0, 0.0, 0.0));
}

Motor motor_rotor_right_contraction(Motor self, Rotor other) {
    return Motor(vec4(self.g0.y) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.z) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g1.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + self.g1 * vec4(other.g0.x) * vec4(0.0, 1.0, 1.0, 1.0));
}

Scalar motor_rotor_scalar_product(Motor self, Rotor other) {
    return Scalar(self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g0.w * other.g0.w);
}

PointAndPlane motor_point_geometric_product(Motor self, Point other) {
    return PointAndPlane(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * other.g0.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * other.g0.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g1.z) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g1.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g1.y, self.g1.y, self.g0.y, self.g0.y) * other.g0.yxwz * vec4(1.0, -1.0, 1.0, -1.0), vec4(self.g0.z) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, -1.0) - vec4(self.g1.x) * other.g0 + vec4(self.g1.z) * other.g0.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g1.w) * other.g0.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g0.y, self.g0.y, self.g1.y, self.g1.y) * other.g0.yxwz * vec4(1.0, -1.0, 1.0, -1.0));
}

PointAndPlane motor_point_regressive_product(Motor self, Point other) {
    return PointAndPlane(vec4(self.g1.x) * other.g0, vec4(self.g0.z) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.w) * other.g0.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g1.z) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + vec4(self.g1.y, self.g1.y, self.g0.y, self.g0.y) * other.g0.yxwz * vec4(-1.0, 1.0, -1.0, 1.0));
}

Point motor_point_outer_product(Motor self, Point other) {
    return Point(vec4(self.g0.x) * other.g0);
}

PointAndPlane motor_point_inner_product(Motor self, Point other) {
    return PointAndPlane(vec4(self.g0.x) * other.g0, vec4(self.g0.z) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, -1.0) - vec4(self.g1.x) * other.g0 + vec4(self.g1.z) * other.g0.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g1.w) * other.g0.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g0.y, self.g0.y, self.g1.y, self.g1.y) * other.g0.yxwz * vec4(1.0, -1.0, 1.0, -1.0));
}

PointAndPlane motor_point_left_contraction(Motor self, Point other) {
    return PointAndPlane(vec4(self.g0.x) * other.g0, vec4(self.g0.z) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * other.g0.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g1.w) * other.g0.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g0.y, self.g0.y, self.g1.y, self.g1.y) * other.g0.yxwz * vec4(1.0, -1.0, 1.0, -1.0));
}

Plane motor_point_right_contraction(Motor self, Point other) {
    return Plane(vec4(0.0) - vec4(self.g1.x) * other.g0);
}

IdealPoint motor_ideal_point_into(Motor self) {
    return IdealPoint(vec3(self.g1.y, self.g1.z, self.g1.w));
}

Motor motor_ideal_point_add(Motor self, IdealPoint other) {
    return Motor(self.g0, self.g1 + vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0));
}

Motor motor_ideal_point_sub(Motor self, IdealPoint other) {
    return Motor(self.g0, self.g1 - vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0));
}

Motor motor_ideal_point_geometric_product(Motor self, IdealPoint other) {
    return Motor(vec4(self.g1.y) * vec4(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.y, other.g0.z, other.g0.y, other.g0.x) * vec4(-1.0, -1.0, 0.0, 1.0) + vec4(self.g1.w) * vec4(other.g0.z, other.g0.y, other.g0.x, other.g0.z) * vec4(-1.0, 1.0, -1.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, -1.0, -1.0, -1.0), vec4(self.g0.y) * vec4(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4(1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.z, other.g0.y, other.g0.x) * vec4(1.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * vec4(other.g0.z, other.g0.y, other.g0.x, other.g0.z) * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0));
}

Translator motor_ideal_point_regressive_product(Motor self, IdealPoint other) {
    return Translator(vec4(self.g0.z) * vec4(other.g0.y) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.y, self.g1.x, self.g1.x, self.g1.x) * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z));
}

Translator motor_ideal_point_left_contraction(Motor self, IdealPoint other) {
    return Translator(vec4(self.g1.z) * vec4(other.g0.y) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.z) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y, self.g0.x, self.g0.x, self.g0.x) * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(-1.0, 1.0, 1.0, 1.0));
}

Rotor motor_ideal_point_right_contraction(Motor self, IdealPoint other) {
    return Rotor(vec4(self.g1.z) * vec4(other.g0.y) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.z) * vec4(-1.0, 0.0, 0.0, 0.0) + self.g1.yxxx * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(-1.0));
}

Scalar motor_ideal_point_scalar_product(Motor self, IdealPoint other) {
    return Scalar(0.0 - self.g1.y * other.g0.x - self.g1.z * other.g0.y - self.g1.w * other.g0.z);
}

PointAndPlane motor_plane_geometric_product(Motor self, Plane other) {
    return PointAndPlane(vec4(self.g0.z) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, -1.0) - vec4(self.g1.x) * other.g0 + vec4(self.g1.z) * other.g0.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g1.w) * other.g0.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g0.y, self.g0.y, self.g1.y, self.g1.y) * other.g0.yxwz * vec4(1.0, -1.0, 1.0, -1.0), vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * other.g0.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * other.g0.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g1.z) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g1.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g1.y, self.g1.y, self.g0.y, self.g0.y) * other.g0.yxwz * vec4(1.0, -1.0, 1.0, -1.0));
}

Plane motor_plane_regressive_product(Motor self, Plane other) {
    return Plane(vec4(self.g1.x) * other.g0);
}

PointAndPlane motor_plane_outer_product(Motor self, Plane other) {
    return PointAndPlane(vec4(self.g0.z) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * other.g0.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g1.w) * other.g0.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g0.y, self.g0.y, self.g1.y, self.g1.y) * other.g0.yxwz * vec4(1.0, -1.0, 1.0, -1.0), vec4(self.g0.x) * other.g0);
}

PointAndPlane motor_plane_inner_product(Motor self, Plane other) {
    return PointAndPlane(vec4(0.0) - vec4(self.g1.x) * other.g0, vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * other.g0.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * other.g0.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g1.z) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g1.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g1.y, self.g1.y, self.g0.y, self.g0.y) * other.g0.yxwz * vec4(1.0, -1.0, 1.0, -1.0));
}

Plane motor_plane_left_contraction(Motor self, Plane other) {
    return Plane(vec4(self.g0.x) * other.g0);
}

PointAndPlane motor_plane_right_contraction(Motor self, Plane other) {
    return PointAndPlane(vec4(0.0) - vec4(self.g1.x) * other.g0, vec4(self.g0.z) * other.g0.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * other.g0.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g1.z) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g1.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g1.y, self.g1.y, self.g0.y, self.g0.y) * other.g0.yxwz * vec4(1.0, -1.0, 1.0, -1.0));
}

Line motor_line_into(Motor self) {
    return Line(vec3(self.g1.y, self.g1.z, self.g1.w), vec3(self.g0.y, self.g0.z, self.g0.w));
}

Motor motor_line_add(Motor self, Line other) {
    return Motor(self.g0 + vec4(other.g0.x, other.g1.x, other.g1.y, other.g1.z) * vec4(0.0, 1.0, 1.0, 1.0), self.g1 + vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0));
}

Motor motor_line_sub(Motor self, Line other) {
    return Motor(self.g0 - vec4(other.g0.x, other.g1.x, other.g1.y, other.g1.z) * vec4(0.0, 1.0, 1.0, 1.0), self.g1 - vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0));
}

Motor motor_line_geometric_product(Motor self, Line other) {
    return Motor(vec4(self.g0.y) * vec4(other.g1.x, other.g1.x, other.g1.z, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.z, other.g1.y, other.g1.x) * vec4(-1.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * vec4(other.g1.z, other.g1.y, other.g1.x, other.g1.z) * vec4(-1.0, 1.0, -1.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, -1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.y, other.g0.z, other.g0.y, other.g0.x) * vec4(-1.0, -1.0, 0.0, 1.0) + vec4(self.g1.w) * vec4(other.g0.z, other.g0.y, other.g0.x, other.g0.z) * vec4(-1.0, 1.0, -1.0, 0.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.x, other.g1.y, other.g1.z) * vec4(0.0, 1.0, 1.0, 1.0), vec4(self.g0.y) * vec4(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4(1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.z, other.g0.y, other.g0.x) * vec4(1.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * vec4(other.g0.z, other.g0.y, other.g0.x, other.g0.z) * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g1.x) * vec4(other.g1.x, other.g1.x, other.g1.y, other.g1.z) * vec4(0.0, -1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g1.x, other.g1.x, other.g1.z, other.g1.y) * vec4(1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.y, other.g1.z, other.g1.y, other.g1.x) * vec4(1.0, -1.0, 0.0, 1.0) + vec4(self.g1.w) * vec4(other.g1.z, other.g1.y, other.g1.x, other.g1.z) * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0));
}

Scalar motor_line_scalar_product(Motor self, Line other) {
    return Scalar(0.0 - self.g0.y * other.g1.x - self.g0.z * other.g1.y - self.g0.w * other.g1.z - self.g1.y * other.g0.x - self.g1.z * other.g0.y - self.g1.w * other.g0.z);
}

Translator motor_translator_into(Motor self) {
    return Translator(vec4(self.g0.x, self.g1.y, self.g1.z, self.g1.w));
}

Motor motor_translator_add(Motor self, Translator other) {
    return Motor(self.g0 + vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0), self.g1 + other.g0 * vec4(0.0, 1.0, 1.0, 1.0));
}

Motor motor_translator_sub(Motor self, Translator other) {
    return Motor(self.g0 - vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0), self.g1 - other.g0 * vec4(0.0, 1.0, 1.0, 1.0));
}

Motor motor_translator_geometric_product(Motor self, Translator other) {
    return Motor(vec4(self.g1.x) * other.g0.yyzw * vec4(0.0, -1.0, -1.0, -1.0) + vec4(self.g1.y) * other.g0.yywz * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * other.g0.zwzy * vec4(-1.0, -1.0, 0.0, 1.0) + vec4(self.g1.w) * other.g0.wzyw * vec4(-1.0, 1.0, -1.0, 0.0) + self.g0 * vec4(other.g0.x), vec4(self.g0.y) * other.g0.yywz * vec4(1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * other.g0.zwzy * vec4(1.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * other.g0.wzyw * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x, self.g0.x, self.g0.x, self.g0.x) * other.g0);
}

Translator motor_translator_regressive_product(Motor self, Translator other) {
    return Translator(vec4(self.g0.z) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.x) * other.g0 + self.g0.yxxx * other.g0.yxxx * vec4(1.0, 0.0, 0.0, 0.0));
}

Motor motor_translator_outer_product(Motor self, Translator other) {
    return Motor(self.g0 * vec4(other.g0.x), vec4(self.g0.z) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + self.g0.yxxx * other.g0.yyzw);
}

Motor motor_translator_inner_product(Motor self, Translator other) {
    return Motor(vec4(self.g1.x) * other.g0.yyzw * vec4(0.0, -1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g0.y) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.w) * vec4(-1.0, 0.0, 0.0, 0.0) + self.g0 * vec4(other.g0.x), vec4(self.g1.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x, self.g0.x, self.g0.x, self.g0.x) * other.g0);
}

Translator motor_translator_left_contraction(Motor self, Translator other) {
    return Translator(vec4(self.g0.x) * other.g0 + vec4(self.g1.z) * vec4(other.g0.z) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.w) * vec4(-1.0, 0.0, 0.0, 0.0) + self.g1.yxxx * other.g0.yxxx * vec4(-1.0, 0.0, 0.0, 0.0));
}

Motor motor_translator_right_contraction(Motor self, Translator other) {
    return Motor(vec4(self.g1.x) * other.g0.yyzw * vec4(0.0, -1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g0.y) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.w) * vec4(-1.0, 0.0, 0.0, 0.0) + self.g0 * vec4(other.g0.x), self.g1 * vec4(other.g0.x));
}

Scalar motor_translator_scalar_product(Motor self, Translator other) {
    return Scalar(self.g0.x * other.g0.x - self.g1.y * other.g0.y - self.g1.z * other.g0.z - self.g1.w * other.g0.w);
}

Motor motor_motor_add(Motor self, Motor other) {
    return Motor(self.g0 + other.g0, self.g1 + other.g1);
}

Motor motor_motor_sub(Motor self, Motor other) {
    return Motor(self.g0 - other.g0, self.g1 - other.g1);
}

Motor motor_motor_mul(Motor self, Motor other) {
    return Motor(self.g0 * other.g0, self.g1 * other.g1);
}

Motor motor_motor_div(Motor self, Motor other) {
    return Motor(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.w) * vec4(1.0, 1.0, 1.0, 1.0) / vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.w) * vec4(1.0, 1.0, 1.0, 1.0), vec4(self.g1.x, self.g1.y, self.g1.z, self.g1.w) * vec4(1.0, 1.0, 1.0, 1.0) / vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.w) * vec4(1.0, 1.0, 1.0, 1.0));
}

Motor motor_motor_geometric_product(Motor self, Motor other) {
    return Motor(vec4(self.g0.x) * other.g0 + vec4(self.g0.y) * other.g0.yxwz * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g0.zwxy * vec4(-1.0, -1.0, 1.0, 1.0) + vec4(self.g0.w) * other.g0.wzyx * vec4(-1.0, 1.0, -1.0, 1.0) + vec4(self.g1.x) * other.g1 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g1.y) * other.g1.yxwz * vec4(-1.0, -1.0, 1.0, -1.0) + vec4(self.g1.z) * other.g1.zwxy * vec4(-1.0, -1.0, -1.0, 1.0) + vec4(self.g1.w) * other.g1.wzyx * vec4(-1.0, 1.0, -1.0, -1.0), vec4(self.g0.x) * other.g1 + vec4(self.g0.y) * other.g1.yxwz * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g1.zwxy * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.w) * other.g1.wzyx * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g1.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g1.y) * other.g0.yxwz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g1.z) * other.g0.zwxy * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g1.w) * other.g0.wzyx * vec4(1.0, 1.0, -1.0, 1.0));
}

Motor motor_motor_regressive_product(Motor self, Motor other) {
    return Motor(vec4(self.g0.y) * other.g1.yxyy * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g0.z) * other.g1.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g1.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * other.g0 + vec4(self.g1.y) * vec4(other.g0.y) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.x) * vec4(other.g1.x) * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g1.x) * other.g1 + self.g1 * vec4(other.g1.x) * vec4(0.0, 1.0, 1.0, 1.0));
}

Motor motor_motor_outer_product(Motor self, Motor other) {
    return Motor(vec4(self.g0.x) * other.g0 + self.g0 * vec4(other.g0.x) * vec4(0.0, 1.0, 1.0, 1.0), vec4(self.g0.x) * other.g1 + vec4(self.g0.z) * vec4(other.g1.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g1.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y) * other.g0.yxyy * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * other.g0.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + self.g0.yxxx * other.g1.yxxx * vec4(1.0, 0.0, 0.0, 0.0));
}

Motor motor_motor_inner_product(Motor self, Motor other) {
    return Motor(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * other.g1 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g1.y) * other.g1.yxyy * vec4(-1.0, -1.0, 0.0, 0.0) + vec4(self.g1.z) * other.g1.zzxz * vec4(-1.0, 0.0, -1.0, 0.0) + vec4(self.g1.w) * other.g1.wwwx * vec4(-1.0, 0.0, 0.0, -1.0) + self.g0.yyxx * other.g0.yxxx * vec4(-1.0, 1.0, 0.0, 0.0), vec4(self.g0.x) * other.g1 + vec4(self.g1.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + self.g0 * vec4(other.g1.x) * vec4(0.0, -1.0, -1.0, -1.0));
}

Motor motor_motor_left_contraction(Motor self, Motor other) {
    return Motor(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * vec4(other.g0.z) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.w) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g1.x) * vec4(other.g1.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y) * other.g1.yxyy * vec4(-1.0, -1.0, 0.0, 0.0) + vec4(self.g1.z) * other.g1.zzxz * vec4(-1.0, 0.0, -1.0, 0.0) + vec4(self.g1.w) * other.g1.wwwx * vec4(-1.0, 0.0, 0.0, -1.0) + self.g0.yxxx * other.g0.yxxx * vec4(-1.0, 0.0, 0.0, 0.0), vec4(self.g0.x) * other.g1 + self.g0 * vec4(other.g1.x) * vec4(0.0, -1.0, -1.0, -1.0));
}

Motor motor_motor_right_contraction(Motor self, Motor other) {
    return Motor(vec4(self.g0.y) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.z) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * other.g1 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g1.y) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g1.z) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g1.w) * vec4(other.g1.w) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g1.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + self.g1 * vec4(other.g0.x) * vec4(0.0, 1.0, 1.0, 1.0));
}

Scalar motor_motor_scalar_product(Motor self, Motor other) {
    return Scalar(self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g0.w * other.g0.w + self.g1.x * other.g1.x - self.g1.y * other.g1.y - self.g1.z * other.g1.z - self.g1.w * other.g1.w);
}

MultiVector motor_point_and_plane_add(Motor self, PointAndPlane other) {
    return MultiVector(self.g0, vec4(other.g1.x, other.g0.y, other.g0.z, other.g0.w), vec4(other.g0.x, other.g1.y, other.g1.z, other.g1.w), self.g1);
}

MultiVector motor_point_and_plane_sub(Motor self, PointAndPlane other) {
    return MultiVector(self.g0, vec4(0.0) - vec4(other.g1.x, other.g0.y, other.g0.z, other.g0.w), vec4(0.0) - vec4(other.g0.x, other.g1.y, other.g1.z, other.g1.w), self.g1);
}

PointAndPlane motor_point_and_plane_geometric_product(Motor self, PointAndPlane other) {
    return PointAndPlane(vec4(self.g0.x) * other.g0 + vec4(self.g0.y) * vec4(other.g1.y, other.g1.x, other.g0.w, other.g0.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.z, other.g0.w, other.g1.x, other.g0.y) * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.w) * vec4(other.g1.w, other.g0.z, other.g0.y, other.g1.x) * vec4(1.0, 1.0, -1.0, -1.0) - vec4(self.g1.x) * other.g1 + vec4(self.g1.y) * vec4(other.g0.y, other.g0.x, other.g1.w, other.g1.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.z, other.g1.w, other.g0.x, other.g1.y) * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g1.w) * vec4(other.g0.w, other.g1.z, other.g1.y, other.g0.x) * vec4(1.0, 1.0, -1.0, -1.0), vec4(self.g0.x) * other.g1 + vec4(self.g0.y) * vec4(other.g0.y, other.g0.x, other.g1.w, other.g1.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z, other.g1.w, other.g0.x, other.g1.y) * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.w) * vec4(other.g0.w, other.g1.z, other.g1.y, other.g0.x) * vec4(1.0, 1.0, -1.0, -1.0) - vec4(self.g1.x) * other.g0 + vec4(self.g1.y) * vec4(other.g1.y, other.g1.x, other.g0.w, other.g0.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.z, other.g0.w, other.g1.x, other.g0.y) * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g1.w) * vec4(other.g1.w, other.g0.z, other.g0.y, other.g1.x) * vec4(1.0, 1.0, -1.0, -1.0));
}

PointAndPlane motor_point_and_plane_regressive_product(Motor self, PointAndPlane other) {
    return PointAndPlane(vec4(self.g1.x) * other.g0, vec4(self.g0.z) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.w) * other.g0.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g1.x) * other.g1 + vec4(self.g1.z) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + vec4(self.g1.y, self.g1.y, self.g0.y, self.g0.y) * other.g0.yxwz * vec4(-1.0, 1.0, -1.0, 1.0));
}

PointAndPlane motor_point_and_plane_outer_product(Motor self, PointAndPlane other) {
    return PointAndPlane(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * other.g1.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.w) * other.g1.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * other.g1.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g1.w) * other.g1.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g0.y, self.g0.y, self.g1.y, self.g1.y) * other.g1.yxwz * vec4(1.0, -1.0, 1.0, -1.0), vec4(self.g0.x) * other.g1);
}

PointAndPlane motor_point_and_plane_inner_product(Motor self, PointAndPlane other) {
    return PointAndPlane(vec4(self.g0.x) * other.g0 - vec4(self.g1.x) * other.g1, vec4(self.g0.x) * other.g1 + vec4(self.g0.y) * vec4(other.g0.y, other.g0.x, other.g1.w, other.g1.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z, other.g1.w, other.g0.x, other.g1.y) * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.w) * vec4(other.g0.w, other.g1.z, other.g1.y, other.g0.x) * vec4(1.0, 1.0, -1.0, -1.0) - vec4(self.g1.x) * other.g0 + vec4(self.g1.y) * vec4(other.g1.y, other.g1.x, other.g0.w, other.g0.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.z, other.g0.w, other.g1.x, other.g0.y) * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g1.w) * vec4(other.g1.w, other.g0.z, other.g0.y, other.g1.x) * vec4(1.0, 1.0, -1.0, -1.0));
}

PointAndPlane motor_point_and_plane_left_contraction(Motor self, PointAndPlane other) {
    return PointAndPlane(vec4(self.g0.x) * other.g0, vec4(self.g0.x) * other.g1 + vec4(self.g0.z) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * other.g0.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g1.w) * other.g0.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g0.y, self.g0.y, self.g1.y, self.g1.y) * other.g0.yxwz * vec4(1.0, -1.0, 1.0, -1.0));
}

PointAndPlane motor_point_and_plane_right_contraction(Motor self, PointAndPlane other) {
    return PointAndPlane(vec4(0.0) - vec4(self.g1.x) * other.g1, vec4(self.g0.z) * other.g1.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * other.g1.zzyz * vec4(0.0, 1.0, -1.0, 0.0) - vec4(self.g1.x) * other.g0 + vec4(self.g1.z) * other.g1.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g1.w) * other.g1.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g1.y, self.g1.y, self.g0.y, self.g0.y) * other.g1.yxwz * vec4(1.0, -1.0, 1.0, -1.0));
}

Scalar motor_squared_magnitude(Motor self) {
    return motor_motor_scalar_product(self, motor_reversal(self));
}

Scalar motor_magnitude(Motor self) {
    return Scalar(sqrt(motor_squared_magnitude(self).g0));
}

Motor motor_scale(Motor self, float other) {
    return motor_scalar_geometric_product(self, Scalar(other));
}

Motor motor_signum(Motor self) {
    return motor_scalar_geometric_product(self, Scalar(1.0 / motor_magnitude(self).g0));
}

Motor motor_inverse(Motor self) {
    return motor_scalar_geometric_product(motor_reversal(self), Scalar(1.0 / motor_squared_magnitude(self).g0));
}

PointAndPlane point_and_plane_zero() {
    return PointAndPlane(vec4(0.0), vec4(0.0));
}

PointAndPlane point_and_plane_one() {
    return PointAndPlane(vec4(0.0), vec4(0.0));
}

PointAndPlane point_and_plane_neg(PointAndPlane self) {
    return PointAndPlane(self.g0 * vec4(-1.0), self.g1 * vec4(-1.0));
}

PointAndPlane point_and_plane_automorphism(PointAndPlane self) {
    return PointAndPlane(self.g0 * vec4(-1.0), self.g1 * vec4(-1.0));
}

PointAndPlane point_and_plane_reversal(PointAndPlane self) {
    return PointAndPlane(self.g0 * vec4(-1.0), self.g1);
}

PointAndPlane point_and_plane_conjugation(PointAndPlane self) {
    return PointAndPlane(self.g0, self.g1 * vec4(-1.0));
}

PointAndPlane point_and_plane_dual(PointAndPlane self) {
    return PointAndPlane(self.g1, self.g0 * vec4(-1.0));
}

PointAndPlane point_and_plane_scalar_geometric_product(PointAndPlane self, Scalar other) {
    return PointAndPlane(self.g0 * vec4(other.g0), self.g1 * vec4(other.g0));
}

PointAndPlane point_and_plane_scalar_outer_product(PointAndPlane self, Scalar other) {
    return PointAndPlane(self.g0 * vec4(other.g0), self.g1 * vec4(other.g0));
}

PointAndPlane point_and_plane_scalar_inner_product(PointAndPlane self, Scalar other) {
    return PointAndPlane(self.g0 * vec4(other.g0), self.g1 * vec4(other.g0));
}

PointAndPlane point_and_plane_scalar_right_contraction(PointAndPlane self, Scalar other) {
    return PointAndPlane(self.g0 * vec4(other.g0), self.g1 * vec4(other.g0));
}

MultiVector point_and_plane_multi_vector_add(PointAndPlane self, MultiVector other) {
    return MultiVector(other.g0, vec4(self.g1.x, self.g0.y, self.g0.z, self.g0.w) + other.g1, vec4(self.g0.x, self.g1.y, self.g1.z, self.g1.w) + other.g2, other.g3);
}

MultiVector point_and_plane_multi_vector_sub(PointAndPlane self, MultiVector other) {
    return MultiVector(vec4(0.0) - other.g0, vec4(self.g1.x, self.g0.y, self.g0.z, self.g0.w) - other.g1, vec4(self.g0.x, self.g1.y, self.g1.z, self.g1.w) - other.g2, vec4(0.0) - other.g3);
}

MultiVector point_and_plane_multi_vector_geometric_product(PointAndPlane self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g2 * vec4(-1.0, 1.0, 1.0, 1.0) + vec4(self.g0.y) * other.g1.yxwz * vec4(-1.0, -1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g1.zwxy * vec4(-1.0, -1.0, -1.0, 1.0) + vec4(self.g0.w) * other.g1.wzyx * vec4(-1.0, 1.0, -1.0, -1.0) + vec4(self.g1.x) * other.g1 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g1.y) * other.g2.yxwz * vec4(1.0, 1.0, -1.0, 1.0) + vec4(self.g1.z) * other.g2.zwxy * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g1.w) * other.g2.wzyx * vec4(1.0, -1.0, 1.0, 1.0), vec4(self.g0.x) * other.g3 + vec4(self.g0.y) * other.g0.yxwz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g0.zwxy * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g0.w) * other.g0.wzyx * vec4(1.0, 1.0, -1.0, 1.0) + vec4(self.g1.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g1.y) * other.g3.yxwz * vec4(-1.0, 1.0, -1.0, 1.0) + vec4(self.g1.z) * other.g3.zwxy * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.w) * other.g3.wzyx * vec4(-1.0, -1.0, 1.0, 1.0), vec4(self.g0.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g0.y) * other.g3.yxwz * vec4(-1.0, 1.0, -1.0, 1.0) + vec4(self.g0.z) * other.g3.zwxy * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.w) * other.g3.wzyx * vec4(-1.0, -1.0, 1.0, 1.0) + vec4(self.g1.x) * other.g3 + vec4(self.g1.y) * other.g0.yxwz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g1.z) * other.g0.zwxy * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g1.w) * other.g0.wzyx * vec4(1.0, 1.0, -1.0, 1.0), vec4(0.0) - vec4(self.g0.x) * other.g1 + vec4(self.g0.y) * other.g2.yxwz * vec4(-1.0, 1.0, -1.0, 1.0) + vec4(self.g0.z) * other.g2.zwxy * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.w) * other.g2.wzyx * vec4(-1.0, -1.0, 1.0, 1.0) + vec4(self.g1.x) * other.g2 + vec4(self.g1.y) * other.g1.yxwz * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g1.z) * other.g1.zwxy * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g1.w) * other.g1.wzyx * vec4(1.0, 1.0, -1.0, -1.0));
}

Scalar point_and_plane_multi_vector_scalar_product(PointAndPlane self, MultiVector other) {
    return Scalar(0.0 - self.g0.x * other.g2.x - self.g0.y * other.g1.y - self.g0.z * other.g1.z - self.g0.w * other.g1.w + self.g1.x * other.g1.x + self.g1.y * other.g2.y + self.g1.z * other.g2.z + self.g1.w * other.g2.w);
}

PointAndPlane point_and_plane_rotor_geometric_product(PointAndPlane self, Rotor other) {
    return PointAndPlane(vec4(self.g0.z) * other.g0.wwxy * vec4(0.0, -1.0, 1.0, 1.0) + vec4(self.g0.w) * other.g0.zzyx * vec4(0.0, 1.0, -1.0, 1.0) + vec4(self.g1.x) * other.g0.yyzw * vec4(0.0, -1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g0.y) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + self.g0.xyyy * other.g0.xxwz * vec4(1.0, 1.0, 1.0, -1.0), vec4(self.g0.z) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y) * other.g0.xxwz * vec4(0.0, 1.0, 1.0, -1.0) + vec4(self.g1.z) * other.g0.wwxy * vec4(0.0, -1.0, 1.0, 1.0) + vec4(self.g1.w) * other.g0.zzyx * vec4(0.0, 1.0, -1.0, 1.0) + self.g0.yxxx * other.g0.yyzw * vec4(1.0, -1.0, -1.0, -1.0));
}

PointAndPlane point_and_plane_rotor_outer_product(PointAndPlane self, Rotor other) {
    return PointAndPlane(vec4(self.g1.x) * other.g0.yyzw * vec4(0.0, -1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g0.y) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + self.g0 * vec4(other.g0.x), self.g1 * vec4(other.g0.x));
}

PointAndPlane point_and_plane_rotor_inner_product(PointAndPlane self, Rotor other) {
    return PointAndPlane(self.g0 * vec4(other.g0.x), vec4(self.g0.z) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y) * other.g0.xxwz * vec4(0.0, 1.0, 1.0, -1.0) + vec4(self.g1.z) * other.g0.wwxy * vec4(0.0, -1.0, 1.0, 1.0) + vec4(self.g1.w) * other.g0.zzyx * vec4(0.0, 1.0, -1.0, 1.0) + self.g0.yxxx * other.g0.yyzw * vec4(1.0, -1.0, -1.0, -1.0));
}

PointAndPlane point_and_plane_rotor_right_contraction(PointAndPlane self, Rotor other) {
    return PointAndPlane(self.g0 * vec4(other.g0.x), vec4(self.g0.z) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + self.g0.yxxx * other.g0.yyzw * vec4(1.0, -1.0, -1.0, -1.0));
}

Point point_and_plane_point_into(PointAndPlane self) {
    return Point(self.g0);
}

PointAndPlane point_and_plane_point_add(PointAndPlane self, Point other) {
    return PointAndPlane(self.g0 + other.g0, self.g1);
}

PointAndPlane point_and_plane_point_sub(PointAndPlane self, Point other) {
    return PointAndPlane(self.g0 - other.g0, self.g1);
}

Motor point_and_plane_point_geometric_product(PointAndPlane self, Point other) {
    return Motor(vec4(self.g0.y) * other.g0.yywz * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * other.g0.zwzy * vec4(-1.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * other.g0.wzyw * vec4(-1.0, 1.0, -1.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.x, self.g1.x, self.g1.x, self.g1.x) * other.g0 * vec4(-1.0), vec4(self.g0.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g0.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.y) * other.g0.yywz * vec4(1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * other.g0.zwzy * vec4(1.0, -1.0, 0.0, 1.0) + vec4(self.g1.w) * other.g0.wzyw * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g1.x, self.g0.x, self.g0.x, self.g0.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0));
}

Scalar point_and_plane_point_right_contraction(PointAndPlane self, Point other) {
    return Scalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g0.w * other.g0.w);
}

Scalar point_and_plane_point_scalar_product(PointAndPlane self, Point other) {
    return Scalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g0.w * other.g0.w);
}

PointAndPlane point_and_plane_ideal_point_geometric_product(PointAndPlane self, IdealPoint other) {
    return PointAndPlane(vec4(self.g0.z) * vec4(other.g0.y) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.z) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.z, other.g0.z, other.g0.z, other.g0.y) * vec4(0.0, 0.0, -1.0, 1.0) + vec4(self.g1.z) * vec4(other.g0.z, other.g0.z, other.g0.z, other.g0.x) * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.w) * vec4(other.g0.y, other.g0.y, other.g0.x, other.g0.y) * vec4(0.0, -1.0, 1.0, 0.0) + self.g0.yxxx * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(-1.0, 1.0, 1.0, 1.0), vec4(self.g0.z) * vec4(other.g0.z, other.g0.z, other.g0.z, other.g0.x) * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g0.y, other.g0.y, other.g0.x, other.g0.y) * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0) + vec4(self.g1.z) * vec4(other.g0.y) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.z) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y, self.g0.x, self.g0.y, self.g0.y) * vec4(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4(-1.0, 0.0, -1.0, 1.0));
}

Plane point_and_plane_ideal_point_regressive_product(PointAndPlane self, IdealPoint other) {
    return Plane(vec4(self.g0.z) * vec4(other.g0.y) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.z) * vec4(-1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(-1.0, 1.0, 1.0, 1.0));
}

Plane point_and_plane_ideal_point_inner_product(PointAndPlane self, IdealPoint other) {
    return Plane(vec4(self.g0.z) * vec4(other.g0.z, other.g0.z, other.g0.z, other.g0.x) * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g0.y, other.g0.y, other.g0.x, other.g0.y) * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0) + vec4(self.g1.z) * vec4(other.g0.y) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.z) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y, self.g0.x, self.g0.y, self.g0.y) * vec4(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4(-1.0, 0.0, -1.0, 1.0));
}

Plane point_and_plane_ideal_point_left_contraction(PointAndPlane self, IdealPoint other) {
    return Plane(vec4(self.g1.z) * vec4(other.g0.y) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.z) * vec4(-1.0, 0.0, 0.0, 0.0) + self.g1.yxxx * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(-1.0, 1.0, 1.0, 1.0));
}

Plane point_and_plane_plane_into(PointAndPlane self) {
    return Plane(self.g1);
}

PointAndPlane point_and_plane_plane_add(PointAndPlane self, Plane other) {
    return PointAndPlane(self.g0, self.g1 + other.g0);
}

PointAndPlane point_and_plane_plane_sub(PointAndPlane self, Plane other) {
    return PointAndPlane(self.g0, self.g1 - other.g0);
}

Motor point_and_plane_plane_geometric_product(PointAndPlane self, Plane other) {
    return Motor(vec4(self.g0.y) * vec4(other.g0.x) * vec4(0.0, -1.0, 0.0, 0.0) + vec4(self.g0.z) * vec4(other.g0.x) * vec4(0.0, 0.0, -1.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.y) * other.g0.yywz * vec4(1.0, 0.0, -1.0, 1.0) + vec4(self.g1.z) * other.g0.zwzy * vec4(1.0, 1.0, 0.0, -1.0) + vec4(self.g1.w) * other.g0.wzyw * vec4(1.0, -1.0, 1.0, 0.0) + vec4(self.g1.x, self.g0.x, self.g0.x, self.g0.x) * other.g0, vec4(self.g0.y) * other.g0.yywz * vec4(-1.0, 0.0, -1.0, 1.0) + vec4(self.g0.z) * other.g0.zwzy * vec4(-1.0, 1.0, 0.0, -1.0) + vec4(self.g0.w) * other.g0.wzyw * vec4(-1.0, -1.0, 1.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.x) * vec4(0.0, -1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.x) * vec4(0.0, 0.0, -1.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.x, self.g1.x, self.g1.x, self.g1.x) * other.g0 * vec4(-1.0, 1.0, 1.0, 1.0));
}

Scalar point_and_plane_plane_regressive_product(PointAndPlane self, Plane other) {
    return Scalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g0.w * other.g0.w);
}

Scalar point_and_plane_plane_left_contraction(PointAndPlane self, Plane other) {
    return Scalar(self.g1.x * other.g0.x + self.g1.y * other.g0.y + self.g1.z * other.g0.z + self.g1.w * other.g0.w);
}

Scalar point_and_plane_plane_scalar_product(PointAndPlane self, Plane other) {
    return Scalar(self.g1.x * other.g0.x + self.g1.y * other.g0.y + self.g1.z * other.g0.z + self.g1.w * other.g0.w);
}

PointAndPlane point_and_plane_line_geometric_product(PointAndPlane self, Line other) {
    return PointAndPlane(vec4(self.g0.y) * vec4(other.g0.x, other.g0.x, other.g1.z, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g1.z, other.g0.y, other.g1.x) * vec4(-1.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * vec4(other.g0.z, other.g1.y, other.g1.x, other.g0.z) * vec4(-1.0, 1.0, -1.0, 0.0) + vec4(self.g1.x) * vec4(other.g1.x, other.g1.x, other.g1.y, other.g1.z) * vec4(0.0, -1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g1.x, other.g1.x, other.g0.z, other.g0.y) * vec4(1.0, 0.0, -1.0, 1.0) + vec4(self.g1.z) * vec4(other.g1.y, other.g0.z, other.g1.y, other.g0.x) * vec4(1.0, 1.0, 0.0, -1.0) + vec4(self.g1.w) * vec4(other.g1.z, other.g0.y, other.g0.x, other.g1.z) * vec4(1.0, -1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0), vec4(self.g0.y) * vec4(other.g1.x, other.g1.x, other.g0.z, other.g0.y) * vec4(1.0, 0.0, -1.0, 1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g0.z, other.g1.y, other.g0.x) * vec4(1.0, 1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g1.z, other.g0.y, other.g0.x, other.g1.z) * vec4(1.0, -1.0, 1.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0) + vec4(self.g1.y) * vec4(other.g0.x, other.g0.x, other.g1.z, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.y, other.g1.z, other.g0.y, other.g1.x) * vec4(-1.0, -1.0, 0.0, 1.0) + vec4(self.g1.w) * vec4(other.g0.z, other.g1.y, other.g1.x, other.g0.z) * vec4(-1.0, 1.0, -1.0, 0.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.x, other.g1.y, other.g1.z) * vec4(0.0, -1.0, -1.0, -1.0));
}

Plane point_and_plane_line_regressive_product(PointAndPlane self, Line other) {
    return Plane(vec4(self.g0.y) * vec4(other.g0.x, other.g0.x, other.g1.z, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g1.z, other.g0.y, other.g1.x) * vec4(-1.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * vec4(other.g0.z, other.g1.y, other.g1.x, other.g0.z) * vec4(-1.0, 1.0, -1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0));
}

Point point_and_plane_line_outer_product(PointAndPlane self, Line other) {
    return Point(vec4(self.g1.y) * vec4(other.g1.x, other.g1.x, other.g0.z, other.g0.y) * vec4(1.0, 0.0, -1.0, 1.0) + vec4(self.g1.z) * vec4(other.g1.y, other.g0.z, other.g1.y, other.g0.x) * vec4(1.0, 1.0, 0.0, -1.0) + vec4(self.g1.w) * vec4(other.g1.z, other.g0.y, other.g0.x, other.g1.z) * vec4(1.0, -1.0, 1.0, 0.0) + vec4(self.g1.x) * vec4(other.g1.x, other.g1.x, other.g1.y, other.g1.z) * vec4(0.0, -1.0, -1.0, -1.0));
}

Plane point_and_plane_line_inner_product(PointAndPlane self, Line other) {
    return Plane(vec4(self.g0.y) * vec4(other.g1.x, other.g1.x, other.g0.z, other.g0.y) * vec4(1.0, 0.0, -1.0, 1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g0.z, other.g1.y, other.g0.x) * vec4(1.0, 1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g1.z, other.g0.y, other.g0.x, other.g1.z) * vec4(1.0, -1.0, 1.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0) + vec4(self.g1.y) * vec4(other.g0.x, other.g0.x, other.g1.z, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.y, other.g1.z, other.g0.y, other.g1.x) * vec4(-1.0, -1.0, 0.0, 1.0) + vec4(self.g1.w) * vec4(other.g0.z, other.g1.y, other.g1.x, other.g0.z) * vec4(-1.0, 1.0, -1.0, 0.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.x, other.g1.y, other.g1.z) * vec4(0.0, -1.0, -1.0, -1.0));
}

Plane point_and_plane_line_left_contraction(PointAndPlane self, Line other) {
    return Plane(vec4(self.g1.y) * vec4(other.g0.x, other.g0.x, other.g1.z, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.y, other.g1.z, other.g0.y, other.g1.x) * vec4(-1.0, -1.0, 0.0, 1.0) + vec4(self.g1.w) * vec4(other.g0.z, other.g1.y, other.g1.x, other.g0.z) * vec4(-1.0, 1.0, -1.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0));
}

Plane point_and_plane_line_right_contraction(PointAndPlane self, Line other) {
    return Plane(vec4(self.g0.y) * vec4(other.g1.x, other.g1.x, other.g0.z, other.g0.y) * vec4(1.0, 0.0, -1.0, 1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g0.z, other.g1.y, other.g0.x) * vec4(1.0, 1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g1.z, other.g0.y, other.g0.x, other.g1.z) * vec4(1.0, -1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.x, other.g1.y, other.g1.z) * vec4(0.0, -1.0, -1.0, -1.0));
}

PointAndPlane point_and_plane_translator_geometric_product(PointAndPlane self, Translator other) {
    return PointAndPlane(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + vec4(self.g1.z) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.w) * other.g0.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g0.y, self.g0.y, self.g1.y, self.g1.y) * other.g0.yxwz * vec4(-1.0, 1.0, -1.0, 1.0), vec4(self.g0.z) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.w) * other.g0.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g1.x) * other.g0 + vec4(self.g1.z) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + vec4(self.g1.y, self.g1.y, self.g0.y, self.g0.y) * other.g0.yxwz * vec4(-1.0, 1.0, -1.0, 1.0));
}

Plane point_and_plane_translator_regressive_product(PointAndPlane self, Translator other) {
    return Plane(vec4(self.g0.z) * vec4(other.g0.z) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.w) * vec4(-1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * other.g0.yyzw * vec4(-1.0, 1.0, 1.0, 1.0));
}

PointAndPlane point_and_plane_translator_outer_product(PointAndPlane self, Translator other) {
    return PointAndPlane(vec4(self.g1.y) * other.g0.wwwz * vec4(0.0, 0.0, -1.0, 1.0) + vec4(self.g1.z) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.w) * other.g0.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + self.g0 * vec4(other.g0.x), self.g1 * vec4(other.g0.x));
}

PointAndPlane point_and_plane_translator_inner_product(PointAndPlane self, Translator other) {
    return PointAndPlane(self.g0 * vec4(other.g0.x), vec4(self.g0.z) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.w) * other.g0.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g1.x) * other.g0 + vec4(self.g1.z) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + vec4(self.g1.y, self.g1.y, self.g0.y, self.g0.y) * other.g0.yxwz * vec4(-1.0, 1.0, -1.0, 1.0));
}

Plane point_and_plane_translator_left_contraction(PointAndPlane self, Translator other) {
    return Plane(vec4(self.g1.z) * vec4(other.g0.z) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.w) * vec4(-1.0, 0.0, 0.0, 0.0) + self.g1.yxxx * other.g0.yyzw * vec4(-1.0, 1.0, 1.0, 1.0));
}

PointAndPlane point_and_plane_translator_right_contraction(PointAndPlane self, Translator other) {
    return PointAndPlane(self.g0 * vec4(other.g0.x), vec4(self.g0.z) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.w) * other.g0.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x, self.g1.y, self.g0.y, self.g0.y) * other.g0.xxwz * vec4(1.0, 1.0, -1.0, 1.0));
}

MultiVector point_and_plane_motor_add(PointAndPlane self, Motor other) {
    return MultiVector(other.g0, vec4(self.g1.x, self.g0.y, self.g0.z, self.g0.w), vec4(self.g0.x, self.g1.y, self.g1.z, self.g1.w), other.g1);
}

MultiVector point_and_plane_motor_sub(PointAndPlane self, Motor other) {
    return MultiVector(vec4(0.0) - other.g0, vec4(self.g1.x, self.g0.y, self.g0.z, self.g0.w), vec4(self.g0.x, self.g1.y, self.g1.z, self.g1.w), vec4(0.0) - other.g1);
}

PointAndPlane point_and_plane_motor_geometric_product(PointAndPlane self, Motor other) {
    return PointAndPlane(vec4(self.g0.x) * vec4(other.g0.x, other.g1.y, other.g1.z, other.g1.w) + vec4(self.g0.y) * vec4(other.g1.y, other.g0.x, other.g0.w, other.g0.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.z, other.g0.w, other.g0.x, other.g0.y) * vec4(-1.0, -1.0, 1.0, 1.0) + vec4(self.g0.w) * vec4(other.g1.w, other.g0.z, other.g0.y, other.g0.x) * vec4(-1.0, 1.0, -1.0, 1.0) + vec4(self.g1.x) * vec4(other.g1.x, other.g0.y, other.g0.z, other.g0.w) * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g0.y, other.g1.x, other.g1.w, other.g1.z) * vec4(1.0, 1.0, -1.0, 1.0) + vec4(self.g1.z) * vec4(other.g0.z, other.g1.w, other.g1.x, other.g1.y) * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g1.w) * vec4(other.g0.w, other.g1.z, other.g1.y, other.g1.x) * vec4(1.0, -1.0, 1.0, 1.0), vec4(self.g0.x) * vec4(other.g1.x, other.g0.y, other.g0.z, other.g0.w) * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g0.y) * vec4(other.g0.y, other.g1.x, other.g1.w, other.g1.z) * vec4(1.0, 1.0, -1.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.z, other.g1.w, other.g1.x, other.g1.y) * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g0.w) * vec4(other.g0.w, other.g1.z, other.g1.y, other.g1.x) * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g1.x) * vec4(other.g0.x, other.g1.y, other.g1.z, other.g1.w) + vec4(self.g1.y) * vec4(other.g1.y, other.g0.x, other.g0.w, other.g0.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.z, other.g0.w, other.g0.x, other.g0.y) * vec4(-1.0, -1.0, 1.0, 1.0) + vec4(self.g1.w) * vec4(other.g1.w, other.g0.z, other.g0.y, other.g0.x) * vec4(-1.0, 1.0, -1.0, 1.0));
}

PointAndPlane point_and_plane_motor_regressive_product(PointAndPlane self, Motor other) {
    return PointAndPlane(self.g0 * vec4(other.g1.x), vec4(self.g0.y) * vec4(other.g1.y, other.g1.y, other.g0.w, other.g0.z) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.z, other.g0.w, other.g1.z, other.g0.y) * vec4(-1.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * vec4(other.g1.w, other.g0.z, other.g0.y, other.g1.w) * vec4(-1.0, 1.0, -1.0, 0.0) + vec4(self.g1.y) * vec4(other.g1.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g1.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x, self.g0.x, self.g0.x, self.g0.x) * other.g1);
}

PointAndPlane point_and_plane_motor_outer_product(PointAndPlane self, Motor other) {
    return PointAndPlane(vec4(self.g1.x) * other.g0.yyzw * vec4(0.0, -1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g0.y, other.g0.y, other.g1.w, other.g1.z) * vec4(1.0, 0.0, -1.0, 1.0) + vec4(self.g1.z) * vec4(other.g0.z, other.g1.w, other.g0.z, other.g1.y) * vec4(1.0, 1.0, 0.0, -1.0) + vec4(self.g1.w) * vec4(other.g0.w, other.g1.z, other.g1.y, other.g0.w) * vec4(1.0, -1.0, 1.0, 0.0) + self.g0 * vec4(other.g0.x), self.g1 * vec4(other.g0.x));
}

PointAndPlane point_and_plane_motor_inner_product(PointAndPlane self, Motor other) {
    return PointAndPlane(vec4(self.g1.x) * vec4(other.g1.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y) * vec4(other.g1.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g1.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, 1.0) + self.g0 * vec4(other.g0.x), vec4(self.g0.x) * vec4(other.g1.x, other.g0.y, other.g0.z, other.g0.w) * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g0.y) * vec4(other.g0.y, other.g1.x, other.g1.w, other.g1.z) * vec4(1.0, 1.0, -1.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.z, other.g1.w, other.g1.x, other.g1.y) * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g0.w) * vec4(other.g0.w, other.g1.z, other.g1.y, other.g1.x) * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g1.x) * vec4(other.g0.x, other.g1.y, other.g1.z, other.g1.w) + vec4(self.g1.y) * vec4(other.g1.y, other.g0.x, other.g0.w, other.g0.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.z, other.g0.w, other.g0.x, other.g0.y) * vec4(-1.0, -1.0, 1.0, 1.0) + vec4(self.g1.w) * vec4(other.g1.w, other.g0.z, other.g0.y, other.g0.x) * vec4(-1.0, 1.0, -1.0, 1.0));
}

PointAndPlane point_and_plane_motor_left_contraction(PointAndPlane self, Motor other) {
    return PointAndPlane(self.g1 * vec4(other.g1.x), vec4(self.g1.x) * other.g1.yyzw * vec4(0.0, 1.0, 1.0, 1.0) + vec4(self.g1.y) * vec4(other.g1.y, other.g1.y, other.g0.w, other.g0.z) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.z, other.g0.w, other.g1.z, other.g0.y) * vec4(-1.0, -1.0, 0.0, 1.0) + vec4(self.g1.w) * vec4(other.g1.w, other.g0.z, other.g0.y, other.g1.w) * vec4(-1.0, 1.0, -1.0, 0.0) + self.g0 * vec4(other.g1.x));
}

PointAndPlane point_and_plane_motor_right_contraction(PointAndPlane self, Motor other) {
    return PointAndPlane(self.g0 * vec4(other.g0.x), vec4(self.g0.y) * vec4(other.g0.y, other.g0.y, other.g1.w, other.g1.z) * vec4(1.0, 0.0, -1.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.z, other.g1.w, other.g0.z, other.g1.y) * vec4(1.0, 1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g0.w, other.g1.z, other.g1.y, other.g0.w) * vec4(1.0, -1.0, 1.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x, self.g0.x, self.g0.x, self.g0.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0));
}

PointAndPlane point_and_plane_point_and_plane_add(PointAndPlane self, PointAndPlane other) {
    return PointAndPlane(self.g0 + other.g0, self.g1 + other.g1);
}

PointAndPlane point_and_plane_point_and_plane_sub(PointAndPlane self, PointAndPlane other) {
    return PointAndPlane(self.g0 - other.g0, self.g1 - other.g1);
}

PointAndPlane point_and_plane_point_and_plane_mul(PointAndPlane self, PointAndPlane other) {
    return PointAndPlane(self.g0 * other.g0, self.g1 * other.g1);
}

PointAndPlane point_and_plane_point_and_plane_div(PointAndPlane self, PointAndPlane other) {
    return PointAndPlane(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.w) * vec4(1.0, 1.0, 1.0, 1.0) / vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.w) * vec4(1.0, 1.0, 1.0, 1.0), vec4(self.g1.x, self.g1.y, self.g1.z, self.g1.w) * vec4(1.0, 1.0, 1.0, 1.0) / vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.w) * vec4(1.0, 1.0, 1.0, 1.0));
}

Motor point_and_plane_point_and_plane_geometric_product(PointAndPlane self, PointAndPlane other) {
    return Motor(vec4(self.g0.x) * vec4(other.g0.x, other.g1.y, other.g1.z, other.g1.w) * vec4(-1.0, 1.0, 1.0, 1.0) + vec4(self.g0.y) * vec4(other.g0.y, other.g1.x, other.g0.w, other.g0.z) * vec4(-1.0, -1.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z, other.g0.w, other.g1.x, other.g0.y) * vec4(-1.0, -1.0, -1.0, 1.0) + vec4(self.g0.w) * vec4(other.g0.w, other.g0.z, other.g0.y, other.g1.x) * vec4(-1.0, 1.0, -1.0, -1.0) + vec4(self.g1.x) * vec4(other.g1.x, other.g0.y, other.g0.z, other.g0.w) * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g1.y, other.g0.x, other.g1.w, other.g1.z) * vec4(1.0, 1.0, -1.0, 1.0) + vec4(self.g1.z) * vec4(other.g1.z, other.g1.w, other.g0.x, other.g1.y) * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g1.w) * vec4(other.g1.w, other.g1.z, other.g1.y, other.g0.x) * vec4(1.0, -1.0, 1.0, 1.0), vec4(0.0) - vec4(self.g0.x) * vec4(other.g1.x, other.g0.y, other.g0.z, other.g0.w) + vec4(self.g0.y) * vec4(other.g1.y, other.g0.x, other.g1.w, other.g1.z) * vec4(-1.0, 1.0, -1.0, 1.0) + vec4(self.g0.z) * vec4(other.g1.z, other.g1.w, other.g0.x, other.g1.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.w) * vec4(other.g1.w, other.g1.z, other.g1.y, other.g0.x) * vec4(-1.0, -1.0, 1.0, 1.0) + vec4(self.g1.x) * vec4(other.g0.x, other.g1.y, other.g1.z, other.g1.w) + vec4(self.g1.y) * vec4(other.g0.y, other.g1.x, other.g0.w, other.g0.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.z, other.g0.w, other.g1.x, other.g0.y) * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g1.w) * vec4(other.g0.w, other.g0.z, other.g0.y, other.g1.x) * vec4(1.0, 1.0, -1.0, -1.0));
}

Scalar point_and_plane_point_and_plane_scalar_product(PointAndPlane self, PointAndPlane other) {
    return Scalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g0.w * other.g0.w + self.g1.x * other.g1.x + self.g1.y * other.g1.y + self.g1.z * other.g1.z + self.g1.w * other.g1.w);
}

Scalar point_and_plane_squared_magnitude(PointAndPlane self) {
    return point_and_plane_point_and_plane_scalar_product(self, point_and_plane_reversal(self));
}

Scalar point_and_plane_magnitude(PointAndPlane self) {
    return Scalar(sqrt(point_and_plane_squared_magnitude(self).g0));
}

PointAndPlane point_and_plane_scale(PointAndPlane self, float other) {
    return point_and_plane_scalar_geometric_product(self, Scalar(other));
}

PointAndPlane point_and_plane_signum(PointAndPlane self) {
    return point_and_plane_scalar_geometric_product(self, Scalar(1.0 / point_and_plane_magnitude(self).g0));
}

PointAndPlane point_and_plane_inverse(PointAndPlane self) {
    return point_and_plane_scalar_geometric_product(point_and_plane_reversal(self), Scalar(1.0 / point_and_plane_squared_magnitude(self).g0));
}

Rotor ideal_point_ideal_point_geometric_quotient(IdealPoint self, IdealPoint other) {
    return ideal_point_ideal_point_geometric_product(self, ideal_point_inverse(other));
}

Motor ideal_point_line_geometric_quotient(IdealPoint self, Line other) {
    return ideal_point_line_geometric_product(self, line_inverse(other));
}

Line ideal_point_line_transformation(IdealPoint self, Line other) {
    return motor_line_into(motor_ideal_point_geometric_product(ideal_point_line_geometric_product(self, other), ideal_point_reversal(self)));
}

Motor ideal_point_motor_geometric_quotient(IdealPoint self, Motor other) {
    return ideal_point_motor_geometric_product(self, motor_inverse(other));
}

Motor ideal_point_motor_transformation(IdealPoint self, Motor other) {
    return motor_ideal_point_geometric_product(ideal_point_motor_geometric_product(self, other), ideal_point_reversal(self));
}

MultiVector ideal_point_multi_vector_geometric_quotient(IdealPoint self, MultiVector other) {
    return ideal_point_multi_vector_geometric_product(self, multi_vector_inverse(other));
}

MultiVector ideal_point_multi_vector_transformation(IdealPoint self, MultiVector other) {
    return multi_vector_ideal_point_geometric_product(ideal_point_multi_vector_geometric_product(self, other), ideal_point_reversal(self));
}

PointAndPlane ideal_point_point_and_plane_geometric_quotient(IdealPoint self, PointAndPlane other) {
    return ideal_point_point_and_plane_geometric_product(self, point_and_plane_inverse(other));
}

PointAndPlane ideal_point_point_and_plane_transformation(IdealPoint self, PointAndPlane other) {
    return point_and_plane_ideal_point_geometric_product(ideal_point_point_and_plane_geometric_product(self, other), ideal_point_reversal(self));
}

IdealPoint ideal_point_scalar_geometric_quotient(IdealPoint self, Scalar other) {
    return ideal_point_scalar_geometric_product(self, scalar_inverse(other));
}

Scalar ideal_point_scalar_transformation(IdealPoint self, Scalar other) {
    return rotor_scalar_into(ideal_point_ideal_point_geometric_product(ideal_point_scalar_geometric_product(self, other), ideal_point_reversal(self)));
}

Motor line_ideal_point_geometric_quotient(Line self, IdealPoint other) {
    return line_ideal_point_geometric_product(self, ideal_point_inverse(other));
}

IdealPoint line_ideal_point_transformation(Line self, IdealPoint other) {
    return motor_ideal_point_into(motor_line_geometric_product(line_ideal_point_geometric_product(self, other), line_reversal(self)));
}

Motor line_line_geometric_quotient(Line self, Line other) {
    return line_line_geometric_product(self, line_inverse(other));
}

Line line_line_transformation(Line self, Line other) {
    return motor_line_into(motor_line_geometric_product(line_line_geometric_product(self, other), line_reversal(self)));
}

Motor line_motor_geometric_quotient(Line self, Motor other) {
    return line_motor_geometric_product(self, motor_inverse(other));
}

Motor line_motor_transformation(Line self, Motor other) {
    return motor_line_geometric_product(line_motor_geometric_product(self, other), line_reversal(self));
}

MultiVector line_multi_vector_geometric_quotient(Line self, MultiVector other) {
    return line_multi_vector_geometric_product(self, multi_vector_inverse(other));
}

MultiVector line_multi_vector_transformation(Line self, MultiVector other) {
    return multi_vector_line_geometric_product(line_multi_vector_geometric_product(self, other), line_reversal(self));
}

PointAndPlane line_plane_geometric_quotient(Line self, Plane other) {
    return line_plane_geometric_product(self, plane_inverse(other));
}

Plane line_plane_transformation(Line self, Plane other) {
    return point_and_plane_plane_into(point_and_plane_line_geometric_product(line_plane_geometric_product(self, other), line_reversal(self)));
}

PointAndPlane line_point_geometric_quotient(Line self, Point other) {
    return line_point_geometric_product(self, point_inverse(other));
}

Point line_point_transformation(Line self, Point other) {
    return point_and_plane_point_into(point_and_plane_line_geometric_product(line_point_geometric_product(self, other), line_reversal(self)));
}

PointAndPlane line_point_and_plane_geometric_quotient(Line self, PointAndPlane other) {
    return line_point_and_plane_geometric_product(self, point_and_plane_inverse(other));
}

PointAndPlane line_point_and_plane_transformation(Line self, PointAndPlane other) {
    return point_and_plane_line_geometric_product(line_point_and_plane_geometric_product(self, other), line_reversal(self));
}

Motor line_rotor_geometric_quotient(Line self, Rotor other) {
    return line_rotor_geometric_product(self, rotor_inverse(other));
}

Rotor line_rotor_transformation(Line self, Rotor other) {
    return motor_rotor_into(motor_line_geometric_product(line_rotor_geometric_product(self, other), line_reversal(self)));
}

Line line_scalar_geometric_quotient(Line self, Scalar other) {
    return line_scalar_geometric_product(self, scalar_inverse(other));
}

Scalar line_scalar_transformation(Line self, Scalar other) {
    return motor_scalar_into(line_line_geometric_product(line_scalar_geometric_product(self, other), line_reversal(self)));
}

Motor line_translator_geometric_quotient(Line self, Translator other) {
    return line_translator_geometric_product(self, translator_inverse(other));
}

Translator line_translator_transformation(Line self, Translator other) {
    return motor_translator_into(motor_line_geometric_product(line_translator_geometric_product(self, other), line_reversal(self)));
}

Motor motor_ideal_point_geometric_quotient(Motor self, IdealPoint other) {
    return motor_ideal_point_geometric_product(self, ideal_point_inverse(other));
}

IdealPoint motor_ideal_point_transformation(Motor self, IdealPoint other) {
    return motor_ideal_point_into(motor_motor_geometric_product(motor_ideal_point_geometric_product(self, other), motor_reversal(self)));
}

Motor motor_line_geometric_quotient(Motor self, Line other) {
    return motor_line_geometric_product(self, line_inverse(other));
}

Line motor_line_transformation(Motor self, Line other) {
    return motor_line_into(motor_motor_geometric_product(motor_line_geometric_product(self, other), motor_reversal(self)));
}

Motor motor_powi(Motor self, int exponent) {
    if(exponent == 0) {
        return motor_one();
    }
    Motor x = (exponent < 0) ? motor_inverse(self) : self;
    Motor y = motor_one();
    int n = abs(exponent);
    while(1 < n) {
        if((n & 1) == 1) {
            y = motor_motor_geometric_product(x, y);
        }
        x = motor_motor_geometric_product(x, x);
        n = n >> 1;
    }
    return motor_motor_geometric_product(x, y);
}

Motor motor_motor_geometric_quotient(Motor self, Motor other) {
    return motor_motor_geometric_product(self, motor_inverse(other));
}

Motor motor_motor_transformation(Motor self, Motor other) {
    return motor_motor_geometric_product(motor_motor_geometric_product(self, other), motor_reversal(self));
}

MultiVector motor_multi_vector_geometric_quotient(Motor self, MultiVector other) {
    return motor_multi_vector_geometric_product(self, multi_vector_inverse(other));
}

MultiVector motor_multi_vector_transformation(Motor self, MultiVector other) {
    return multi_vector_motor_geometric_product(motor_multi_vector_geometric_product(self, other), motor_reversal(self));
}

PointAndPlane motor_plane_geometric_quotient(Motor self, Plane other) {
    return motor_plane_geometric_product(self, plane_inverse(other));
}

Plane motor_plane_transformation(Motor self, Plane other) {
    return point_and_plane_plane_into(point_and_plane_motor_geometric_product(motor_plane_geometric_product(self, other), motor_reversal(self)));
}

PointAndPlane motor_point_geometric_quotient(Motor self, Point other) {
    return motor_point_geometric_product(self, point_inverse(other));
}

Point motor_point_transformation(Motor self, Point other) {
    return point_and_plane_point_into(point_and_plane_motor_geometric_product(motor_point_geometric_product(self, other), motor_reversal(self)));
}

PointAndPlane motor_point_and_plane_geometric_quotient(Motor self, PointAndPlane other) {
    return motor_point_and_plane_geometric_product(self, point_and_plane_inverse(other));
}

PointAndPlane motor_point_and_plane_transformation(Motor self, PointAndPlane other) {
    return point_and_plane_motor_geometric_product(motor_point_and_plane_geometric_product(self, other), motor_reversal(self));
}

Motor motor_rotor_geometric_quotient(Motor self, Rotor other) {
    return motor_rotor_geometric_product(self, rotor_inverse(other));
}

Rotor motor_rotor_transformation(Motor self, Rotor other) {
    return motor_rotor_into(motor_motor_geometric_product(motor_rotor_geometric_product(self, other), motor_reversal(self)));
}

Motor motor_scalar_geometric_quotient(Motor self, Scalar other) {
    return motor_scalar_geometric_product(self, scalar_inverse(other));
}

Scalar motor_scalar_transformation(Motor self, Scalar other) {
    return motor_scalar_into(motor_motor_geometric_product(motor_scalar_geometric_product(self, other), motor_reversal(self)));
}

Motor motor_translator_geometric_quotient(Motor self, Translator other) {
    return motor_translator_geometric_product(self, translator_inverse(other));
}

Translator motor_translator_transformation(Motor self, Translator other) {
    return motor_translator_into(motor_motor_geometric_product(motor_translator_geometric_product(self, other), motor_reversal(self)));
}

MultiVector multi_vector_ideal_point_geometric_quotient(MultiVector self, IdealPoint other) {
    return multi_vector_ideal_point_geometric_product(self, ideal_point_inverse(other));
}

IdealPoint multi_vector_ideal_point_transformation(MultiVector self, IdealPoint other) {
    return multi_vector_ideal_point_into(multi_vector_multi_vector_geometric_product(multi_vector_ideal_point_geometric_product(self, other), multi_vector_reversal(self)));
}

MultiVector multi_vector_line_geometric_quotient(MultiVector self, Line other) {
    return multi_vector_line_geometric_product(self, line_inverse(other));
}

Line multi_vector_line_transformation(MultiVector self, Line other) {
    return multi_vector_line_into(multi_vector_multi_vector_geometric_product(multi_vector_line_geometric_product(self, other), multi_vector_reversal(self)));
}

MultiVector multi_vector_motor_geometric_quotient(MultiVector self, Motor other) {
    return multi_vector_motor_geometric_product(self, motor_inverse(other));
}

Motor multi_vector_motor_transformation(MultiVector self, Motor other) {
    return multi_vector_motor_into(multi_vector_multi_vector_geometric_product(multi_vector_motor_geometric_product(self, other), multi_vector_reversal(self)));
}

MultiVector multi_vector_powi(MultiVector self, int exponent) {
    if(exponent == 0) {
        return multi_vector_one();
    }
    MultiVector x = (exponent < 0) ? multi_vector_inverse(self) : self;
    MultiVector y = multi_vector_one();
    int n = abs(exponent);
    while(1 < n) {
        if((n & 1) == 1) {
            y = multi_vector_multi_vector_geometric_product(x, y);
        }
        x = multi_vector_multi_vector_geometric_product(x, x);
        n = n >> 1;
    }
    return multi_vector_multi_vector_geometric_product(x, y);
}

MultiVector multi_vector_multi_vector_geometric_quotient(MultiVector self, MultiVector other) {
    return multi_vector_multi_vector_geometric_product(self, multi_vector_inverse(other));
}

MultiVector multi_vector_multi_vector_transformation(MultiVector self, MultiVector other) {
    return multi_vector_multi_vector_geometric_product(multi_vector_multi_vector_geometric_product(self, other), multi_vector_reversal(self));
}

MultiVector multi_vector_plane_geometric_quotient(MultiVector self, Plane other) {
    return multi_vector_plane_geometric_product(self, plane_inverse(other));
}

Plane multi_vector_plane_transformation(MultiVector self, Plane other) {
    return multi_vector_plane_into(multi_vector_multi_vector_geometric_product(multi_vector_plane_geometric_product(self, other), multi_vector_reversal(self)));
}

MultiVector multi_vector_point_geometric_quotient(MultiVector self, Point other) {
    return multi_vector_point_geometric_product(self, point_inverse(other));
}

Point multi_vector_point_transformation(MultiVector self, Point other) {
    return multi_vector_point_into(multi_vector_multi_vector_geometric_product(multi_vector_point_geometric_product(self, other), multi_vector_reversal(self)));
}

MultiVector multi_vector_point_and_plane_geometric_quotient(MultiVector self, PointAndPlane other) {
    return multi_vector_point_and_plane_geometric_product(self, point_and_plane_inverse(other));
}

PointAndPlane multi_vector_point_and_plane_transformation(MultiVector self, PointAndPlane other) {
    return multi_vector_point_and_plane_into(multi_vector_multi_vector_geometric_product(multi_vector_point_and_plane_geometric_product(self, other), multi_vector_reversal(self)));
}

MultiVector multi_vector_rotor_geometric_quotient(MultiVector self, Rotor other) {
    return multi_vector_rotor_geometric_product(self, rotor_inverse(other));
}

Rotor multi_vector_rotor_transformation(MultiVector self, Rotor other) {
    return multi_vector_rotor_into(multi_vector_multi_vector_geometric_product(multi_vector_rotor_geometric_product(self, other), multi_vector_reversal(self)));
}

MultiVector multi_vector_scalar_geometric_quotient(MultiVector self, Scalar other) {
    return multi_vector_scalar_geometric_product(self, scalar_inverse(other));
}

Scalar multi_vector_scalar_transformation(MultiVector self, Scalar other) {
    return multi_vector_scalar_into(multi_vector_multi_vector_geometric_product(multi_vector_scalar_geometric_product(self, other), multi_vector_reversal(self)));
}

MultiVector multi_vector_translator_geometric_quotient(MultiVector self, Translator other) {
    return multi_vector_translator_geometric_product(self, translator_inverse(other));
}

Translator multi_vector_translator_transformation(MultiVector self, Translator other) {
    return multi_vector_translator_into(multi_vector_multi_vector_geometric_product(multi_vector_translator_geometric_product(self, other), multi_vector_reversal(self)));
}

PointAndPlane plane_line_geometric_quotient(Plane self, Line other) {
    return plane_line_geometric_product(self, line_inverse(other));
}

Line plane_line_transformation(Plane self, Line other) {
    return motor_line_into(point_and_plane_plane_geometric_product(plane_line_geometric_product(self, other), plane_reversal(self)));
}

PointAndPlane plane_motor_geometric_quotient(Plane self, Motor other) {
    return plane_motor_geometric_product(self, motor_inverse(other));
}

Motor plane_motor_transformation(Plane self, Motor other) {
    return point_and_plane_plane_geometric_product(plane_motor_geometric_product(self, other), plane_reversal(self));
}

MultiVector plane_multi_vector_geometric_quotient(Plane self, MultiVector other) {
    return plane_multi_vector_geometric_product(self, multi_vector_inverse(other));
}

MultiVector plane_multi_vector_transformation(Plane self, MultiVector other) {
    return multi_vector_plane_geometric_product(plane_multi_vector_geometric_product(self, other), plane_reversal(self));
}

Motor plane_point_and_plane_geometric_quotient(Plane self, PointAndPlane other) {
    return plane_point_and_plane_geometric_product(self, point_and_plane_inverse(other));
}

PointAndPlane plane_point_and_plane_transformation(Plane self, PointAndPlane other) {
    return motor_plane_geometric_product(plane_point_and_plane_geometric_product(self, other), plane_reversal(self));
}

PointAndPlane plane_rotor_geometric_quotient(Plane self, Rotor other) {
    return plane_rotor_geometric_product(self, rotor_inverse(other));
}

Rotor plane_rotor_transformation(Plane self, Rotor other) {
    return motor_rotor_into(point_and_plane_plane_geometric_product(plane_rotor_geometric_product(self, other), plane_reversal(self)));
}

Plane plane_scalar_geometric_quotient(Plane self, Scalar other) {
    return plane_scalar_geometric_product(self, scalar_inverse(other));
}

PointAndPlane point_line_geometric_quotient(Point self, Line other) {
    return point_line_geometric_product(self, line_inverse(other));
}

Line point_line_transformation(Point self, Line other) {
    return motor_line_into(point_and_plane_point_geometric_product(point_line_geometric_product(self, other), point_reversal(self)));
}

PointAndPlane point_motor_geometric_quotient(Point self, Motor other) {
    return point_motor_geometric_product(self, motor_inverse(other));
}

Motor point_motor_transformation(Point self, Motor other) {
    return point_and_plane_point_geometric_product(point_motor_geometric_product(self, other), point_reversal(self));
}

MultiVector point_multi_vector_geometric_quotient(Point self, MultiVector other) {
    return point_multi_vector_geometric_product(self, multi_vector_inverse(other));
}

MultiVector point_multi_vector_transformation(Point self, MultiVector other) {
    return multi_vector_point_geometric_product(point_multi_vector_geometric_product(self, other), point_reversal(self));
}

Motor point_point_and_plane_geometric_quotient(Point self, PointAndPlane other) {
    return point_point_and_plane_geometric_product(self, point_and_plane_inverse(other));
}

PointAndPlane point_point_and_plane_transformation(Point self, PointAndPlane other) {
    return motor_point_geometric_product(point_point_and_plane_geometric_product(self, other), point_reversal(self));
}

PointAndPlane point_rotor_geometric_quotient(Point self, Rotor other) {
    return point_rotor_geometric_product(self, rotor_inverse(other));
}

Rotor point_rotor_transformation(Point self, Rotor other) {
    return motor_rotor_into(point_and_plane_point_geometric_product(point_rotor_geometric_product(self, other), point_reversal(self)));
}

Point point_scalar_geometric_quotient(Point self, Scalar other) {
    return point_scalar_geometric_product(self, scalar_inverse(other));
}

PointAndPlane point_and_plane_ideal_point_geometric_quotient(PointAndPlane self, IdealPoint other) {
    return point_and_plane_ideal_point_geometric_product(self, ideal_point_inverse(other));
}

IdealPoint point_and_plane_ideal_point_transformation(PointAndPlane self, IdealPoint other) {
    return motor_ideal_point_into(point_and_plane_point_and_plane_geometric_product(point_and_plane_ideal_point_geometric_product(self, other), point_and_plane_reversal(self)));
}

PointAndPlane point_and_plane_line_geometric_quotient(PointAndPlane self, Line other) {
    return point_and_plane_line_geometric_product(self, line_inverse(other));
}

Line point_and_plane_line_transformation(PointAndPlane self, Line other) {
    return motor_line_into(point_and_plane_point_and_plane_geometric_product(point_and_plane_line_geometric_product(self, other), point_and_plane_reversal(self)));
}

PointAndPlane point_and_plane_motor_geometric_quotient(PointAndPlane self, Motor other) {
    return point_and_plane_motor_geometric_product(self, motor_inverse(other));
}

Motor point_and_plane_motor_transformation(PointAndPlane self, Motor other) {
    return point_and_plane_point_and_plane_geometric_product(point_and_plane_motor_geometric_product(self, other), point_and_plane_reversal(self));
}

MultiVector point_and_plane_multi_vector_geometric_quotient(PointAndPlane self, MultiVector other) {
    return point_and_plane_multi_vector_geometric_product(self, multi_vector_inverse(other));
}

MultiVector point_and_plane_multi_vector_transformation(PointAndPlane self, MultiVector other) {
    return multi_vector_point_and_plane_geometric_product(point_and_plane_multi_vector_geometric_product(self, other), point_and_plane_reversal(self));
}

Motor point_and_plane_plane_geometric_quotient(PointAndPlane self, Plane other) {
    return point_and_plane_plane_geometric_product(self, plane_inverse(other));
}

Plane point_and_plane_plane_transformation(PointAndPlane self, Plane other) {
    return point_and_plane_plane_into(motor_point_and_plane_geometric_product(point_and_plane_plane_geometric_product(self, other), point_and_plane_reversal(self)));
}

Motor point_and_plane_point_geometric_quotient(PointAndPlane self, Point other) {
    return point_and_plane_point_geometric_product(self, point_inverse(other));
}

Point point_and_plane_point_transformation(PointAndPlane self, Point other) {
    return point_and_plane_point_into(motor_point_and_plane_geometric_product(point_and_plane_point_geometric_product(self, other), point_and_plane_reversal(self)));
}

Motor point_and_plane_point_and_plane_geometric_quotient(PointAndPlane self, PointAndPlane other) {
    return point_and_plane_point_and_plane_geometric_product(self, point_and_plane_inverse(other));
}

PointAndPlane point_and_plane_point_and_plane_transformation(PointAndPlane self, PointAndPlane other) {
    return motor_point_and_plane_geometric_product(point_and_plane_point_and_plane_geometric_product(self, other), point_and_plane_reversal(self));
}

PointAndPlane point_and_plane_rotor_geometric_quotient(PointAndPlane self, Rotor other) {
    return point_and_plane_rotor_geometric_product(self, rotor_inverse(other));
}

Rotor point_and_plane_rotor_transformation(PointAndPlane self, Rotor other) {
    return motor_rotor_into(point_and_plane_point_and_plane_geometric_product(point_and_plane_rotor_geometric_product(self, other), point_and_plane_reversal(self)));
}

PointAndPlane point_and_plane_scalar_geometric_quotient(PointAndPlane self, Scalar other) {
    return point_and_plane_scalar_geometric_product(self, scalar_inverse(other));
}

Scalar point_and_plane_scalar_transformation(PointAndPlane self, Scalar other) {
    return motor_scalar_into(point_and_plane_point_and_plane_geometric_product(point_and_plane_scalar_geometric_product(self, other), point_and_plane_reversal(self)));
}

PointAndPlane point_and_plane_translator_geometric_quotient(PointAndPlane self, Translator other) {
    return point_and_plane_translator_geometric_product(self, translator_inverse(other));
}

Translator point_and_plane_translator_transformation(PointAndPlane self, Translator other) {
    return motor_translator_into(point_and_plane_point_and_plane_geometric_product(point_and_plane_translator_geometric_product(self, other), point_and_plane_reversal(self)));
}

Motor rotor_line_geometric_quotient(Rotor self, Line other) {
    return rotor_line_geometric_product(self, line_inverse(other));
}

Line rotor_line_transformation(Rotor self, Line other) {
    return motor_line_into(motor_rotor_geometric_product(rotor_line_geometric_product(self, other), rotor_reversal(self)));
}

Motor rotor_motor_geometric_quotient(Rotor self, Motor other) {
    return rotor_motor_geometric_product(self, motor_inverse(other));
}

Motor rotor_motor_transformation(Rotor self, Motor other) {
    return motor_rotor_geometric_product(rotor_motor_geometric_product(self, other), rotor_reversal(self));
}

MultiVector rotor_multi_vector_geometric_quotient(Rotor self, MultiVector other) {
    return rotor_multi_vector_geometric_product(self, multi_vector_inverse(other));
}

MultiVector rotor_multi_vector_transformation(Rotor self, MultiVector other) {
    return multi_vector_rotor_geometric_product(rotor_multi_vector_geometric_product(self, other), rotor_reversal(self));
}

PointAndPlane rotor_plane_geometric_quotient(Rotor self, Plane other) {
    return rotor_plane_geometric_product(self, plane_inverse(other));
}

Plane rotor_plane_transformation(Rotor self, Plane other) {
    return point_and_plane_plane_into(point_and_plane_rotor_geometric_product(rotor_plane_geometric_product(self, other), rotor_reversal(self)));
}

PointAndPlane rotor_point_geometric_quotient(Rotor self, Point other) {
    return rotor_point_geometric_product(self, point_inverse(other));
}

Point rotor_point_transformation(Rotor self, Point other) {
    return point_and_plane_point_into(point_and_plane_rotor_geometric_product(rotor_point_geometric_product(self, other), rotor_reversal(self)));
}

PointAndPlane rotor_point_and_plane_geometric_quotient(Rotor self, PointAndPlane other) {
    return rotor_point_and_plane_geometric_product(self, point_and_plane_inverse(other));
}

PointAndPlane rotor_point_and_plane_transformation(Rotor self, PointAndPlane other) {
    return point_and_plane_rotor_geometric_product(rotor_point_and_plane_geometric_product(self, other), rotor_reversal(self));
}

Rotor rotor_powi(Rotor self, int exponent) {
    if(exponent == 0) {
        return rotor_one();
    }
    Rotor x = (exponent < 0) ? rotor_inverse(self) : self;
    Rotor y = rotor_one();
    int n = abs(exponent);
    while(1 < n) {
        if((n & 1) == 1) {
            y = rotor_rotor_geometric_product(x, y);
        }
        x = rotor_rotor_geometric_product(x, x);
        n = n >> 1;
    }
    return rotor_rotor_geometric_product(x, y);
}

Rotor rotor_rotor_geometric_quotient(Rotor self, Rotor other) {
    return rotor_rotor_geometric_product(self, rotor_inverse(other));
}

Rotor rotor_rotor_transformation(Rotor self, Rotor other) {
    return rotor_rotor_geometric_product(rotor_rotor_geometric_product(self, other), rotor_reversal(self));
}

Rotor rotor_scalar_geometric_quotient(Rotor self, Scalar other) {
    return rotor_scalar_geometric_product(self, scalar_inverse(other));
}

Scalar rotor_scalar_transformation(Rotor self, Scalar other) {
    return rotor_scalar_into(rotor_rotor_geometric_product(rotor_scalar_geometric_product(self, other), rotor_reversal(self)));
}

Motor rotor_translator_geometric_quotient(Rotor self, Translator other) {
    return rotor_translator_geometric_product(self, translator_inverse(other));
}

Translator rotor_translator_transformation(Rotor self, Translator other) {
    return motor_translator_into(motor_rotor_geometric_product(rotor_translator_geometric_product(self, other), rotor_reversal(self)));
}

IdealPoint scalar_ideal_point_geometric_quotient(Scalar self, IdealPoint other) {
    return scalar_ideal_point_geometric_product(self, ideal_point_inverse(other));
}

IdealPoint scalar_ideal_point_transformation(Scalar self, IdealPoint other) {
    return ideal_point_scalar_geometric_product(scalar_ideal_point_geometric_product(self, other), scalar_reversal(self));
}

Line scalar_line_geometric_quotient(Scalar self, Line other) {
    return scalar_line_geometric_product(self, line_inverse(other));
}

Line scalar_line_transformation(Scalar self, Line other) {
    return line_scalar_geometric_product(scalar_line_geometric_product(self, other), scalar_reversal(self));
}

Motor scalar_motor_geometric_quotient(Scalar self, Motor other) {
    return scalar_motor_geometric_product(self, motor_inverse(other));
}

Motor scalar_motor_transformation(Scalar self, Motor other) {
    return motor_scalar_geometric_product(scalar_motor_geometric_product(self, other), scalar_reversal(self));
}

MultiVector scalar_multi_vector_geometric_quotient(Scalar self, MultiVector other) {
    return scalar_multi_vector_geometric_product(self, multi_vector_inverse(other));
}

MultiVector scalar_multi_vector_transformation(Scalar self, MultiVector other) {
    return multi_vector_scalar_geometric_product(scalar_multi_vector_geometric_product(self, other), scalar_reversal(self));
}

Plane scalar_plane_geometric_quotient(Scalar self, Plane other) {
    return scalar_plane_geometric_product(self, plane_inverse(other));
}

Plane scalar_plane_transformation(Scalar self, Plane other) {
    return plane_scalar_geometric_product(scalar_plane_geometric_product(self, other), scalar_reversal(self));
}

Point scalar_point_geometric_quotient(Scalar self, Point other) {
    return scalar_point_geometric_product(self, point_inverse(other));
}

Point scalar_point_transformation(Scalar self, Point other) {
    return point_scalar_geometric_product(scalar_point_geometric_product(self, other), scalar_reversal(self));
}

PointAndPlane scalar_point_and_plane_geometric_quotient(Scalar self, PointAndPlane other) {
    return scalar_point_and_plane_geometric_product(self, point_and_plane_inverse(other));
}

PointAndPlane scalar_point_and_plane_transformation(Scalar self, PointAndPlane other) {
    return point_and_plane_scalar_geometric_product(scalar_point_and_plane_geometric_product(self, other), scalar_reversal(self));
}

Rotor scalar_rotor_geometric_quotient(Scalar self, Rotor other) {
    return scalar_rotor_geometric_product(self, rotor_inverse(other));
}

Rotor scalar_rotor_transformation(Scalar self, Rotor other) {
    return rotor_scalar_geometric_product(scalar_rotor_geometric_product(self, other), scalar_reversal(self));
}

Scalar scalar_powi(Scalar self, int exponent) {
    if(exponent == 0) {
        return scalar_one();
    }
    Scalar x = (exponent < 0) ? scalar_inverse(self) : self;
    Scalar y = scalar_one();
    int n = abs(exponent);
    while(1 < n) {
        if((n & 1) == 1) {
            y = scalar_scalar_geometric_product(x, y);
        }
        x = scalar_scalar_geometric_product(x, x);
        n = n >> 1;
    }
    return scalar_scalar_geometric_product(x, y);
}

Scalar scalar_scalar_geometric_quotient(Scalar self, Scalar other) {
    return scalar_scalar_geometric_product(self, scalar_inverse(other));
}

Scalar scalar_scalar_transformation(Scalar self, Scalar other) {
    return scalar_scalar_geometric_product(scalar_scalar_geometric_product(self, other), scalar_reversal(self));
}

Translator scalar_translator_geometric_quotient(Scalar self, Translator other) {
    return scalar_translator_geometric_product(self, translator_inverse(other));
}

Translator scalar_translator_transformation(Scalar self, Translator other) {
    return translator_scalar_geometric_product(scalar_translator_geometric_product(self, other), scalar_reversal(self));
}

Motor translator_line_geometric_quotient(Translator self, Line other) {
    return translator_line_geometric_product(self, line_inverse(other));
}

Line translator_line_transformation(Translator self, Line other) {
    return motor_line_into(motor_translator_geometric_product(translator_line_geometric_product(self, other), translator_reversal(self)));
}

Motor translator_motor_geometric_quotient(Translator self, Motor other) {
    return translator_motor_geometric_product(self, motor_inverse(other));
}

Motor translator_motor_transformation(Translator self, Motor other) {
    return motor_translator_geometric_product(translator_motor_geometric_product(self, other), translator_reversal(self));
}

MultiVector translator_multi_vector_geometric_quotient(Translator self, MultiVector other) {
    return translator_multi_vector_geometric_product(self, multi_vector_inverse(other));
}

MultiVector translator_multi_vector_transformation(Translator self, MultiVector other) {
    return multi_vector_translator_geometric_product(translator_multi_vector_geometric_product(self, other), translator_reversal(self));
}

PointAndPlane translator_point_and_plane_geometric_quotient(Translator self, PointAndPlane other) {
    return translator_point_and_plane_geometric_product(self, point_and_plane_inverse(other));
}

PointAndPlane translator_point_and_plane_transformation(Translator self, PointAndPlane other) {
    return point_and_plane_translator_geometric_product(translator_point_and_plane_geometric_product(self, other), translator_reversal(self));
}

Motor translator_rotor_geometric_quotient(Translator self, Rotor other) {
    return translator_rotor_geometric_product(self, rotor_inverse(other));
}

Rotor translator_rotor_transformation(Translator self, Rotor other) {
    return motor_rotor_into(motor_translator_geometric_product(translator_rotor_geometric_product(self, other), translator_reversal(self)));
}

Translator translator_scalar_geometric_quotient(Translator self, Scalar other) {
    return translator_scalar_geometric_product(self, scalar_inverse(other));
}

