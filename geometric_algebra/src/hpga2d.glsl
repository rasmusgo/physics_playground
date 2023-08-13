struct Scalar {
    // 1
    float g0;
};

struct MultiVector {
    // 1, e12, e1, e2
    vec4 g0;
    // e0, e012, e01, -e02
    vec4 g1;
};

struct Rotor {
    // 1, e12
    vec2 g0;
};

struct Point {
    // e12, e01, -e02
    vec3 g0;
};

struct IdealPoint {
    // e01, -e02
    vec2 g0;
};

struct Plane {
    // e0, e2, e1
    vec3 g0;
};

struct Translator {
    // 1, e01, -e02
    vec3 g0;
};

struct Motor {
    // 1, e12, e01, -e02
    vec4 g0;
};

struct MotorDual {
    // e012, e0, e2, e1
    vec4 g0;
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
    return MultiVector(vec4(self.g0) * vec4(1.0, 0.0, 0.0, 0.0) + other.g0, other.g1);
}

MultiVector scalar_multi_vector_sub(Scalar self, MultiVector other) {
    return MultiVector(vec4(self.g0) * vec4(1.0, 0.0, 0.0, 0.0) - other.g0, vec4(0.0) - other.g1);
}

MultiVector scalar_multi_vector_geometric_product(Scalar self, MultiVector other) {
    return MultiVector(vec4(self.g0) * other.g0, vec4(self.g0) * other.g1);
}

Scalar scalar_multi_vector_regressive_product(Scalar self, MultiVector other) {
    return Scalar(self.g0 * other.g1.y);
}

MultiVector scalar_multi_vector_outer_product(Scalar self, MultiVector other) {
    return MultiVector(vec4(self.g0) * other.g0, vec4(self.g0) * other.g1);
}

MultiVector scalar_multi_vector_inner_product(Scalar self, MultiVector other) {
    return MultiVector(vec4(self.g0) * other.g0, vec4(self.g0) * other.g1);
}

MultiVector scalar_multi_vector_left_contraction(Scalar self, MultiVector other) {
    return MultiVector(vec4(self.g0) * other.g0, vec4(self.g0) * other.g1);
}

Scalar scalar_multi_vector_right_contraction(Scalar self, MultiVector other) {
    return Scalar(self.g0 * other.g0.x);
}

Scalar scalar_multi_vector_scalar_product(Scalar self, MultiVector other) {
    return Scalar(self.g0 * other.g0.x);
}

Rotor scalar_rotor_add(Scalar self, Rotor other) {
    return Rotor(vec2(self.g0) * vec2(1.0, 0.0) + other.g0);
}

Rotor scalar_rotor_sub(Scalar self, Rotor other) {
    return Rotor(vec2(self.g0) * vec2(1.0, 0.0) - other.g0);
}

Rotor scalar_rotor_geometric_product(Scalar self, Rotor other) {
    return Rotor(vec2(self.g0) * other.g0);
}

Rotor scalar_rotor_outer_product(Scalar self, Rotor other) {
    return Rotor(vec2(self.g0) * other.g0);
}

Rotor scalar_rotor_inner_product(Scalar self, Rotor other) {
    return Rotor(vec2(self.g0) * other.g0);
}

Rotor scalar_rotor_left_contraction(Scalar self, Rotor other) {
    return Rotor(vec2(self.g0) * other.g0);
}

Scalar scalar_rotor_right_contraction(Scalar self, Rotor other) {
    return Scalar(self.g0 * other.g0.x);
}

Scalar scalar_rotor_scalar_product(Scalar self, Rotor other) {
    return Scalar(self.g0 * other.g0.x);
}

Motor scalar_point_add(Scalar self, Point other) {
    return Motor(vec4(self.g0) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0));
}

Motor scalar_point_sub(Scalar self, Point other) {
    return Motor(vec4(self.g0) * vec4(1.0, 0.0, 0.0, 0.0) - vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0));
}

Point scalar_point_geometric_product(Scalar self, Point other) {
    return Point(vec3(self.g0) * other.g0);
}

Point scalar_point_outer_product(Scalar self, Point other) {
    return Point(vec3(self.g0) * other.g0);
}

Point scalar_point_inner_product(Scalar self, Point other) {
    return Point(vec3(self.g0) * other.g0);
}

Point scalar_point_left_contraction(Scalar self, Point other) {
    return Point(vec3(self.g0) * other.g0);
}

Translator scalar_ideal_point_add(Scalar self, IdealPoint other) {
    return Translator(vec3(self.g0) * vec3(1.0, 0.0, 0.0) + vec3(other.g0.x, other.g0.x, other.g0.y) * vec3(0.0, 1.0, 1.0));
}

Translator scalar_ideal_point_sub(Scalar self, IdealPoint other) {
    return Translator(vec3(self.g0) * vec3(1.0, 0.0, 0.0) - vec3(other.g0.x, other.g0.x, other.g0.y) * vec3(0.0, 1.0, 1.0));
}

IdealPoint scalar_ideal_point_geometric_product(Scalar self, IdealPoint other) {
    return IdealPoint(vec2(self.g0) * other.g0);
}

IdealPoint scalar_ideal_point_outer_product(Scalar self, IdealPoint other) {
    return IdealPoint(vec2(self.g0) * other.g0);
}

IdealPoint scalar_ideal_point_inner_product(Scalar self, IdealPoint other) {
    return IdealPoint(vec2(self.g0) * other.g0);
}

IdealPoint scalar_ideal_point_left_contraction(Scalar self, IdealPoint other) {
    return IdealPoint(vec2(self.g0) * other.g0);
}

Plane scalar_plane_geometric_product(Scalar self, Plane other) {
    return Plane(vec3(self.g0) * other.g0);
}

Plane scalar_plane_outer_product(Scalar self, Plane other) {
    return Plane(vec3(self.g0) * other.g0);
}

Plane scalar_plane_inner_product(Scalar self, Plane other) {
    return Plane(vec3(self.g0) * other.g0);
}

Plane scalar_plane_left_contraction(Scalar self, Plane other) {
    return Plane(vec3(self.g0) * other.g0);
}

Translator scalar_translator_add(Scalar self, Translator other) {
    return Translator(vec3(self.g0) * vec3(1.0, 0.0, 0.0) + other.g0);
}

Translator scalar_translator_sub(Scalar self, Translator other) {
    return Translator(vec3(self.g0) * vec3(1.0, 0.0, 0.0) - other.g0);
}

Translator scalar_translator_geometric_product(Scalar self, Translator other) {
    return Translator(vec3(self.g0) * other.g0);
}

Translator scalar_translator_outer_product(Scalar self, Translator other) {
    return Translator(vec3(self.g0) * other.g0);
}

Translator scalar_translator_inner_product(Scalar self, Translator other) {
    return Translator(vec3(self.g0) * other.g0);
}

Translator scalar_translator_left_contraction(Scalar self, Translator other) {
    return Translator(vec3(self.g0) * other.g0);
}

Scalar scalar_translator_right_contraction(Scalar self, Translator other) {
    return Scalar(self.g0 * other.g0.x);
}

Scalar scalar_translator_scalar_product(Scalar self, Translator other) {
    return Scalar(self.g0 * other.g0.x);
}

Motor scalar_motor_add(Scalar self, Motor other) {
    return Motor(vec4(self.g0) * vec4(1.0, 0.0, 0.0, 0.0) + other.g0);
}

Motor scalar_motor_sub(Scalar self, Motor other) {
    return Motor(vec4(self.g0) * vec4(1.0, 0.0, 0.0, 0.0) - other.g0);
}

Motor scalar_motor_geometric_product(Scalar self, Motor other) {
    return Motor(vec4(self.g0) * other.g0);
}

Motor scalar_motor_outer_product(Scalar self, Motor other) {
    return Motor(vec4(self.g0) * other.g0);
}

Motor scalar_motor_inner_product(Scalar self, Motor other) {
    return Motor(vec4(self.g0) * other.g0);
}

Motor scalar_motor_left_contraction(Scalar self, Motor other) {
    return Motor(vec4(self.g0) * other.g0);
}

Scalar scalar_motor_right_contraction(Scalar self, Motor other) {
    return Scalar(self.g0 * other.g0.x);
}

Scalar scalar_motor_scalar_product(Scalar self, Motor other) {
    return Scalar(self.g0 * other.g0.x);
}

MotorDual scalar_motor_dual_geometric_product(Scalar self, MotorDual other) {
    return MotorDual(vec4(self.g0) * other.g0);
}

Scalar scalar_motor_dual_regressive_product(Scalar self, MotorDual other) {
    return Scalar(self.g0 * other.g0.x);
}

MotorDual scalar_motor_dual_outer_product(Scalar self, MotorDual other) {
    return MotorDual(vec4(self.g0) * other.g0);
}

MotorDual scalar_motor_dual_inner_product(Scalar self, MotorDual other) {
    return MotorDual(vec4(self.g0) * other.g0);
}

MotorDual scalar_motor_dual_left_contraction(Scalar self, MotorDual other) {
    return MotorDual(vec4(self.g0) * other.g0);
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
    return MultiVector(vec4(0.0), vec4(0.0));
}

MultiVector multi_vector_one() {
    return MultiVector(vec4(1.0, 0.0, 0.0, 0.0), vec4(0.0));
}

MultiVector multi_vector_neg(MultiVector self) {
    return MultiVector(self.g0 * vec4(-1.0), self.g1 * vec4(-1.0));
}

MultiVector multi_vector_automorphism(MultiVector self) {
    return MultiVector(self.g0 * vec4(1.0, 1.0, -1.0, -1.0), self.g1 * vec4(-1.0, -1.0, 1.0, 1.0));
}

MultiVector multi_vector_reversal(MultiVector self) {
    return MultiVector(self.g0 * vec4(1.0, -1.0, 1.0, 1.0), self.g1 * vec4(1.0, -1.0, -1.0, -1.0));
}

MultiVector multi_vector_conjugation(MultiVector self) {
    return MultiVector(self.g0 * vec4(1.0, -1.0, -1.0, -1.0), self.g1 * vec4(-1.0, 1.0, -1.0, -1.0));
}

MultiVector multi_vector_dual(MultiVector self) {
    return MultiVector(self.g1.yxwz, self.g0.yxwz);
}

Scalar multi_vector_scalar_into(MultiVector self) {
    return Scalar(self.g0.x);
}

MultiVector multi_vector_scalar_add(MultiVector self, Scalar other) {
    return MultiVector(self.g0 + vec4(other.g0) * vec4(1.0, 0.0, 0.0, 0.0), self.g1);
}

MultiVector multi_vector_scalar_sub(MultiVector self, Scalar other) {
    return MultiVector(self.g0 - vec4(other.g0) * vec4(1.0, 0.0, 0.0, 0.0), self.g1);
}

MultiVector multi_vector_scalar_geometric_product(MultiVector self, Scalar other) {
    return MultiVector(self.g0 * vec4(other.g0), self.g1 * vec4(other.g0));
}

Scalar multi_vector_scalar_regressive_product(MultiVector self, Scalar other) {
    return Scalar(self.g1.y * other.g0);
}

MultiVector multi_vector_scalar_outer_product(MultiVector self, Scalar other) {
    return MultiVector(self.g0 * vec4(other.g0), self.g1 * vec4(other.g0));
}

MultiVector multi_vector_scalar_inner_product(MultiVector self, Scalar other) {
    return MultiVector(self.g0 * vec4(other.g0), self.g1 * vec4(other.g0));
}

Scalar multi_vector_scalar_left_contraction(MultiVector self, Scalar other) {
    return Scalar(self.g0.x * other.g0);
}

MultiVector multi_vector_scalar_right_contraction(MultiVector self, Scalar other) {
    return MultiVector(self.g0 * vec4(other.g0), self.g1 * vec4(other.g0));
}

Scalar multi_vector_scalar_scalar_product(MultiVector self, Scalar other) {
    return Scalar(self.g0.x * other.g0);
}

MultiVector multi_vector_multi_vector_add(MultiVector self, MultiVector other) {
    return MultiVector(self.g0 + other.g0, self.g1 + other.g1);
}

MultiVector multi_vector_multi_vector_sub(MultiVector self, MultiVector other) {
    return MultiVector(self.g0 - other.g0, self.g1 - other.g1);
}

MultiVector multi_vector_multi_vector_mul(MultiVector self, MultiVector other) {
    return MultiVector(self.g0 * other.g0, self.g1 * other.g1);
}

MultiVector multi_vector_multi_vector_div(MultiVector self, MultiVector other) {
    return MultiVector(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.w) * vec4(1.0, 1.0, 1.0, 1.0) / vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.w) * vec4(1.0, 1.0, 1.0, 1.0), vec4(self.g1.x, self.g1.y, self.g1.z, self.g1.w) * vec4(1.0, 1.0, 1.0, 1.0) / vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.w) * vec4(1.0, 1.0, 1.0, 1.0));
}

MultiVector multi_vector_multi_vector_geometric_product(MultiVector self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g0 + vec4(self.g0.y) * other.g0.yxwz * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g0.zwxy + vec4(self.g0.w) * other.g0.wzyx * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g1.x) * other.g1 * vec4(-1.0, -1.0, -1.0, 1.0) + vec4(self.g1.y) * other.g1.yxwz * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g1.z) * other.g1.zwxy * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g1.w) * other.g1.wzyx * vec4(1.0, 1.0, 1.0, -1.0), vec4(self.g0.x) * other.g1 + vec4(self.g0.y) * other.g1.yxwz * vec4(-1.0, 1.0, -1.0, 1.0) + vec4(self.g0.z) * other.g1.zwxy * vec4(-1.0, 1.0, -1.0, 1.0) + vec4(self.g0.w) * other.g1.wzyx + vec4(self.g1.x) * other.g0 * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g1.y) * other.g0.yxwz * vec4(-1.0, 1.0, 1.0, 1.0) + vec4(self.g1.z) * other.g0.zwxy * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g1.w) * other.g0.wzyx * vec4(-1.0, 1.0, 1.0, 1.0));
}

MultiVector multi_vector_multi_vector_regressive_product(MultiVector self, MultiVector other) {
    return MultiVector(vec4(self.g0.y) * other.g1 * vec4(1.0, 1.0, -1.0, 1.0) + vec4(self.g0.z) * other.g1.wwyw * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g1.zzzy * vec4(1.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * vec4(other.g0.y) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y) * other.g0 + vec4(self.g1.z) * other.g0.wwyw * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * other.g0.zzzy * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g0.x) * other.g1.yxxx * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g1.y) * other.g1 + vec4(self.g1.z) * other.g1.wwyw * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * other.g1.zzzy * vec4(1.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * other.g1.yxxx * vec4(1.0, 0.0, 0.0, 0.0));
}

MultiVector multi_vector_multi_vector_outer_product(MultiVector self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * other.g0.wwxw * vec4(0.0, 1.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0.zzzx * vec4(0.0, -1.0, 0.0, 1.0) + self.g0.xyxx * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0), vec4(self.g0.x) * other.g1 + vec4(self.g0.z) * other.g1.wwxw * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g0.w) * other.g1.zzzx * vec4(0.0, 1.0, 0.0, 1.0) + vec4(self.g1.x) * other.g0 * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g1.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * other.g0.wwxw * vec4(0.0, 1.0, 1.0, 0.0) + vec4(self.g1.w) * other.g0.zzzx * vec4(0.0, 1.0, 0.0, 1.0) + self.g0.xyxx * vec4(other.g1.x) * vec4(0.0, 1.0, 0.0, 0.0));
}

MultiVector multi_vector_multi_vector_inner_product(MultiVector self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g0 + vec4(self.g0.y) * other.g0.yxwz * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.w) * other.g0.wwyx * vec4(1.0, 0.0, -1.0, 1.0) + vec4(self.g1.x) * other.g1 * vec4(-1.0, -1.0, -1.0, 1.0) + vec4(self.g1.y) * other.g1.yxwz * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g1.z) * other.g1.zzxy * vec4(1.0, 0.0, 1.0, 1.0) + vec4(self.g1.w) * other.g1.wwyx * vec4(1.0, 0.0, 1.0, -1.0) + self.g0.zxzz * other.g0.zxxy * vec4(1.0, 0.0, 1.0, 1.0), vec4(self.g0.x) * other.g1 + vec4(self.g0.z) * other.g1.zzzy * vec4(-1.0, 0.0, 0.0, 1.0) + vec4(self.g0.w) * other.g1.wwyw * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y) * other.g0.yxwz * vec4(-1.0, 1.0, 1.0, 1.0) + vec4(self.g1.z) * other.g0.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + self.g0.yxxx * other.g1.yxxx * vec4(-1.0, 0.0, 0.0, 0.0));
}

MultiVector multi_vector_multi_vector_left_contraction(MultiVector self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * other.g0.zzzy * vec4(1.0, 0.0, 0.0, 1.0) + vec4(self.g0.w) * other.g0.wwyw * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g1.x) * other.g1 * vec4(-1.0, -1.0, -1.0, 1.0) + vec4(self.g1.y) * vec4(other.g1.y) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.z) * other.g1.zzzy * vec4(1.0, 0.0, 0.0, 1.0) + vec4(self.g1.w) * other.g1.wwyw * vec4(1.0, 0.0, 1.0, 0.0) + self.g0.yxxx * other.g0.yxxx * vec4(-1.0, 0.0, 0.0, 0.0), vec4(self.g0.x) * other.g1 + vec4(self.g0.z) * other.g1.zzzy * vec4(-1.0, 0.0, 0.0, 1.0) + vec4(self.g0.w) * other.g1.wwyw * vec4(1.0, 0.0, 1.0, 0.0) + self.g0.yxxx * other.g1.yxxx * vec4(-1.0, 0.0, 0.0, 0.0));
}

MultiVector multi_vector_multi_vector_right_contraction(MultiVector self, MultiVector other) {
    return MultiVector(vec4(self.g0.y) * other.g0.yxwz * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g0.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * vec4(other.g1.x) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y) * other.g1.yxwz * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g1.z) * other.g1.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * other.g1.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g0.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g1.y) * other.g0.yxwz * vec4(-1.0, 1.0, 1.0, 1.0) + vec4(self.g1.z) * other.g0.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0));
}

Scalar multi_vector_multi_vector_scalar_product(MultiVector self, MultiVector other) {
    return Scalar(self.g0.x * other.g0.x - self.g0.y * other.g0.y + self.g0.z * other.g0.z + self.g0.w * other.g0.w - self.g1.x * other.g1.x + self.g1.y * other.g1.y + self.g1.z * other.g1.z + self.g1.w * other.g1.w);
}

Rotor multi_vector_rotor_into(MultiVector self) {
    return Rotor(vec2(self.g0.x, self.g0.y));
}

MultiVector multi_vector_rotor_add(MultiVector self, Rotor other) {
    return MultiVector(self.g0 + vec4(other.g0.x, other.g0.y, other.g0.x, other.g0.x) * vec4(1.0, 1.0, 0.0, 0.0), self.g1);
}

MultiVector multi_vector_rotor_sub(MultiVector self, Rotor other) {
    return MultiVector(self.g0 - vec4(other.g0.x, other.g0.y, other.g0.x, other.g0.x) * vec4(1.0, 1.0, 0.0, 0.0), self.g1);
}

MultiVector multi_vector_rotor_geometric_product(MultiVector self, Rotor other) {
    return MultiVector(vec4(self.g0.y) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g0.y) * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.y, other.g0.y, other.g0.y, other.g0.x) * vec4(0.0, 0.0, -1.0, 1.0) + self.g0.xxzz * vec4(other.g0.x, other.g0.y, other.g0.x, other.g0.y), vec4(self.g1.y) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g0.y) * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.y, other.g0.y, other.g0.y, other.g0.x) * vec4(0.0, 0.0, 1.0, 1.0) + self.g1.xxzz * vec4(other.g0.x, other.g0.y, other.g0.x, other.g0.y) * vec4(1.0, 1.0, 1.0, -1.0));
}

MultiVector multi_vector_rotor_outer_product(MultiVector self, Rotor other) {
    return MultiVector(vec4(self.g0.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + self.g0.xxzw * vec4(other.g0.x, other.g0.y, other.g0.x, other.g0.x), vec4(self.g1.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + self.g1.xxzw * vec4(other.g0.x, other.g0.y, other.g0.x, other.g0.x));
}

MultiVector multi_vector_rotor_inner_product(MultiVector self, Rotor other) {
    return MultiVector(vec4(self.g0.y) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g0.y) * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.y, other.g0.y, other.g0.y, other.g0.x) * vec4(0.0, 0.0, -1.0, 1.0) + self.g0.xxzz * vec4(other.g0.x, other.g0.y, other.g0.x, other.g0.y), vec4(self.g1.y) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g0.y) * vec4(-1.0, 1.0, 0.0, 0.0) + self.g1.xxzw * vec4(other.g0.x) * vec4(1.0, 0.0, 1.0, 1.0));
}

MultiVector multi_vector_rotor_right_contraction(MultiVector self, Rotor other) {
    return MultiVector(vec4(self.g0.y) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g0.y) * vec4(-1.0, 1.0, 0.0, 0.0) + self.g0.xxzw * vec4(other.g0.x) * vec4(1.0, 0.0, 1.0, 1.0), vec4(self.g1.y) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g0.y) * vec4(-1.0, 1.0, 0.0, 0.0) + self.g1.xxzw * vec4(other.g0.x) * vec4(1.0, 0.0, 1.0, 1.0));
}

Scalar multi_vector_rotor_scalar_product(MultiVector self, Rotor other) {
    return Scalar(self.g0.x * other.g0.x - self.g0.y * other.g0.y);
}

Point multi_vector_point_into(MultiVector self) {
    return Point(vec3(self.g0.y, self.g1.z, self.g1.w));
}

MultiVector multi_vector_point_add(MultiVector self, Point other) {
    return MultiVector(self.g0 + vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0), self.g1 + vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 0.0, 1.0, 1.0));
}

MultiVector multi_vector_point_sub(MultiVector self, Point other) {
    return MultiVector(self.g0 - vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0), self.g1 - vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 0.0, 1.0, 1.0));
}

MultiVector multi_vector_point_geometric_product(MultiVector self, Point other) {
    return MultiVector(vec4(self.g1.x) * vec4(other.g0.y, other.g0.y, other.g0.y, other.g0.z) * vec4(0.0, 0.0, -1.0, 1.0) + vec4(self.g1.y) * vec4(other.g0.z, other.g0.z, other.g0.z, other.g0.y) * vec4(0.0, 0.0, 1.0, 1.0) + vec4(self.g1.z) * vec4(other.g0.y, other.g0.z, other.g0.y, other.g0.y) * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.z, other.g0.y, other.g0.z, other.g0.z) * vec4(1.0, 1.0, 0.0, 0.0) + self.g0.yxwz * vec4(other.g0.x) * vec4(-1.0, 1.0, -1.0, 1.0), vec4(self.g0.y) * vec4(other.g0.z, other.g0.z, other.g0.z, other.g0.y) * vec4(0.0, 0.0, -1.0, 1.0) + vec4(self.g0.w) * vec4(other.g0.z, other.g0.y, other.g0.z, other.g0.z) * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.x) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + self.g0.zzxx * vec4(other.g0.y, other.g0.z, other.g0.y, other.g0.z) * vec4(-1.0, 1.0, 1.0, 1.0));
}

Scalar multi_vector_point_scalar_product(MultiVector self, Point other) {
    return Scalar(0.0 - self.g0.y * other.g0.x + self.g1.z * other.g0.y + self.g1.w * other.g0.z);
}

IdealPoint multi_vector_ideal_point_into(MultiVector self) {
    return IdealPoint(vec2(self.g1.z, self.g1.w));
}

MultiVector multi_vector_ideal_point_add(MultiVector self, IdealPoint other) {
    return MultiVector(self.g0, self.g1 + vec4(other.g0.x, other.g0.x, other.g0.x, other.g0.y) * vec4(0.0, 0.0, 1.0, 1.0));
}

MultiVector multi_vector_ideal_point_sub(MultiVector self, IdealPoint other) {
    return MultiVector(self.g0, self.g1 - vec4(other.g0.x, other.g0.x, other.g0.x, other.g0.y) * vec4(0.0, 0.0, 1.0, 1.0));
}

MultiVector multi_vector_ideal_point_geometric_product(MultiVector self, IdealPoint other) {
    return MultiVector(vec4(self.g1.y) * vec4(other.g0.y, other.g0.y, other.g0.y, other.g0.x) * vec4(0.0, 0.0, 1.0, 1.0) + vec4(self.g1.w) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g0.y) * vec4(1.0, 1.0, 0.0, 0.0) + self.g1.zzxx * vec4(other.g0.x, other.g0.y, other.g0.x, other.g0.y) * vec4(1.0, -1.0, -1.0, 1.0), vec4(self.g0.y) * vec4(other.g0.y, other.g0.y, other.g0.y, other.g0.x) * vec4(0.0, 0.0, -1.0, 1.0) + vec4(self.g0.w) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g0.y) * vec4(1.0, 1.0, 0.0, 0.0) + self.g0.zzxx * vec4(other.g0.x, other.g0.y, other.g0.x, other.g0.y) * vec4(-1.0, 1.0, 1.0, 1.0));
}

Scalar multi_vector_ideal_point_scalar_product(MultiVector self, IdealPoint other) {
    return Scalar(self.g1.z * other.g0.x + self.g1.w * other.g0.y);
}

Plane multi_vector_plane_into(MultiVector self) {
    return Plane(vec3(self.g1.x, self.g0.w, self.g0.z));
}

MultiVector multi_vector_plane_add(MultiVector self, Plane other) {
    return MultiVector(self.g0 + vec4(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4(0.0, 0.0, 1.0, 1.0), self.g1 + vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0));
}

MultiVector multi_vector_plane_sub(MultiVector self, Plane other) {
    return MultiVector(self.g0 - vec4(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4(0.0, 0.0, 1.0, 1.0), self.g1 - vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0));
}

MultiVector multi_vector_plane_geometric_product(MultiVector self, Plane other) {
    return MultiVector(vec4(self.g0.y) * vec4(other.g0.y, other.g0.y, other.g0.y, other.g0.z) * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.w) * vec4(other.g0.y, other.g0.z, other.g0.y, other.g0.y) * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.x) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.x) * vec4(0.0, -1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0) + self.g0.zzxx * vec4(other.g0.z, other.g0.y, other.g0.z, other.g0.y), vec4(self.g1.x) * vec4(other.g0.z, other.g0.z, other.g0.z, other.g0.y) * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.y) * vec4(other.g0.y, other.g0.y, other.g0.y, other.g0.z) * vec4(0.0, 0.0, 1.0, 1.0) + vec4(self.g1.z) * vec4(other.g0.z, other.g0.y, other.g0.z, other.g0.z) * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.y, other.g0.z, other.g0.y, other.g0.y) * vec4(-1.0, 1.0, 0.0, 0.0) + self.g0 * vec4(other.g0.x) * vec4(1.0, 1.0, -1.0, 1.0));
}

Scalar multi_vector_plane_scalar_product(MultiVector self, Plane other) {
    return Scalar(self.g0.z * other.g0.z + self.g0.w * other.g0.y - self.g1.x * other.g0.x);
}

Translator multi_vector_translator_into(MultiVector self) {
    return Translator(vec3(self.g0.x, self.g1.z, self.g1.w));
}

MultiVector multi_vector_translator_add(MultiVector self, Translator other) {
    return MultiVector(self.g0 + vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0), self.g1 + vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 0.0, 1.0, 1.0));
}

MultiVector multi_vector_translator_sub(MultiVector self, Translator other) {
    return MultiVector(self.g0 - vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0), self.g1 - vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 0.0, 1.0, 1.0));
}

MultiVector multi_vector_translator_geometric_product(MultiVector self, Translator other) {
    return MultiVector(vec4(self.g1.x) * vec4(other.g0.y, other.g0.y, other.g0.y, other.g0.z) * vec4(0.0, 0.0, -1.0, 1.0) + vec4(self.g1.y) * vec4(other.g0.z, other.g0.z, other.g0.z, other.g0.y) * vec4(0.0, 0.0, 1.0, 1.0) + vec4(self.g1.z) * vec4(other.g0.y, other.g0.z, other.g0.y, other.g0.y) * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.z, other.g0.y, other.g0.z, other.g0.z) * vec4(1.0, 1.0, 0.0, 0.0) + self.g0 * vec4(other.g0.x), vec4(self.g0.y) * vec4(other.g0.z, other.g0.z, other.g0.z, other.g0.y) * vec4(0.0, 0.0, -1.0, 1.0) + vec4(self.g0.w) * vec4(other.g0.z, other.g0.y, other.g0.z, other.g0.z) * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + self.g0.zzxx * vec4(other.g0.y, other.g0.z, other.g0.y, other.g0.z) * vec4(-1.0, 1.0, 1.0, 1.0));
}

MultiVector multi_vector_translator_outer_product(MultiVector self, Translator other) {
    return MultiVector(self.g0 * vec4(other.g0.x), vec4(self.g0.w) * vec4(other.g0.y) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x, self.g0.z, self.g0.x, self.g0.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g0.z));
}

MultiVector multi_vector_translator_inner_product(MultiVector self, Translator other) {
    return MultiVector(vec4(self.g1.x) * vec4(other.g0.y, other.g0.y, other.g0.y, other.g0.z) * vec4(0.0, 0.0, -1.0, 1.0) + vec4(self.g1.y) * vec4(other.g0.z, other.g0.z, other.g0.z, other.g0.y) * vec4(0.0, 0.0, 1.0, 1.0) + vec4(self.g1.z) * vec4(other.g0.y) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + self.g0 * vec4(other.g0.x), vec4(self.g0.w) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.z, self.g1.y, self.g0.x, self.g0.x) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g0.z) * vec4(-1.0, 1.0, 1.0, 1.0));
}

MultiVector multi_vector_translator_right_contraction(MultiVector self, Translator other) {
    return MultiVector(vec4(self.g1.y) * vec4(other.g0.z, other.g0.z, other.g0.z, other.g0.y) * vec4(0.0, 0.0, 1.0, 1.0) + vec4(self.g1.z) * vec4(other.g0.y) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + self.g0 * vec4(other.g0.x), self.g1 * vec4(other.g0.x));
}

Scalar multi_vector_translator_scalar_product(MultiVector self, Translator other) {
    return Scalar(self.g0.x * other.g0.x + self.g1.z * other.g0.y + self.g1.w * other.g0.z);
}

Motor multi_vector_motor_into(MultiVector self) {
    return Motor(vec4(self.g0.x, self.g0.y, self.g1.z, self.g1.w));
}

MultiVector multi_vector_motor_add(MultiVector self, Motor other) {
    return MultiVector(self.g0 + other.g0.xyxx * vec4(1.0, 1.0, 0.0, 0.0), self.g1 + other.g0.xxzw * vec4(0.0, 0.0, 1.0, 1.0));
}

MultiVector multi_vector_motor_sub(MultiVector self, Motor other) {
    return MultiVector(self.g0 - other.g0.xyxx * vec4(1.0, 1.0, 0.0, 0.0), self.g1 - other.g0.xxzw * vec4(0.0, 0.0, 1.0, 1.0));
}

MultiVector multi_vector_motor_geometric_product(MultiVector self, Motor other) {
    return MultiVector(vec4(self.g0.y) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.w) * other.g0.yyyx * vec4(0.0, 0.0, -1.0, 1.0) + vec4(self.g1.x) * other.g0.zzzw * vec4(0.0, 0.0, -1.0, 1.0) + vec4(self.g1.y) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, 1.0) + vec4(self.g1.z) * other.g0.zwzz * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g1.w) * other.g0.wzww * vec4(1.0, 1.0, 0.0, 0.0) + self.g0.xxzz * other.g0.xyxy, vec4(self.g0.y) * other.g0.wwwz * vec4(0.0, 0.0, -1.0, 1.0) + vec4(self.g0.w) * other.g0.wzww * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g1.x) * other.g0.xyxx * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g1.y) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * other.g0.xxxy * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.w) * other.g0.yyyx * vec4(0.0, 0.0, 1.0, 1.0) + self.g0.zzxx * other.g0.zwzw * vec4(-1.0, 1.0, 1.0, 1.0));
}

MultiVector multi_vector_motor_outer_product(MultiVector self, Motor other) {
    return MultiVector(vec4(self.g0.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + self.g0.xxzw * other.g0.xyxx, vec4(self.g0.w) * vec4(other.g0.z) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g1.x) * other.g0.xyxx * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + self.g0.xzxx * other.g0.xwzw * vec4(0.0, 1.0, 1.0, 1.0));
}

MultiVector multi_vector_motor_inner_product(MultiVector self, Motor other) {
    return MultiVector(vec4(self.g0.y) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.w) * other.g0.yyyx * vec4(0.0, 0.0, -1.0, 1.0) + vec4(self.g1.x) * other.g0.zzzw * vec4(0.0, 0.0, -1.0, 1.0) + vec4(self.g1.y) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, 1.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + self.g0.xxzz * other.g0.xyxy, vec4(self.g0.w) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + self.g0.zxxx * other.g0.zxzw * vec4(-1.0, 0.0, 1.0, 1.0));
}

MultiVector multi_vector_motor_right_contraction(MultiVector self, Motor other) {
    return MultiVector(vec4(self.g0.y) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.y) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, 1.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + self.g0.xxzw * vec4(other.g0.x) * vec4(1.0, 0.0, 1.0, 1.0), vec4(self.g1.y) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + self.g1.xxzw * vec4(other.g0.x) * vec4(1.0, 0.0, 1.0, 1.0));
}

Scalar multi_vector_motor_scalar_product(MultiVector self, Motor other) {
    return Scalar(self.g0.x * other.g0.x - self.g0.y * other.g0.y + self.g1.z * other.g0.z + self.g1.w * other.g0.w);
}

MotorDual multi_vector_motor_dual_into(MultiVector self) {
    return MotorDual(vec4(self.g1.y, self.g1.x, self.g0.w, self.g0.z));
}

MultiVector multi_vector_motor_dual_add(MultiVector self, MotorDual other) {
    return MultiVector(self.g0 + other.g0.xxwz * vec4(0.0, 0.0, 1.0, 1.0), self.g1 + other.g0.yxxx * vec4(1.0, 1.0, 0.0, 0.0));
}

MultiVector multi_vector_motor_dual_sub(MultiVector self, MotorDual other) {
    return MultiVector(self.g0 - other.g0.xxwz * vec4(0.0, 0.0, 1.0, 1.0), self.g1 - other.g0.yxxx * vec4(1.0, 1.0, 0.0, 0.0));
}

MultiVector multi_vector_motor_dual_geometric_product(MultiVector self, MotorDual other) {
    return MultiVector(vec4(self.g0.y) * other.g0.zzzw * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.w) * other.g0.zwzz * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g1.x) * other.g0.yxyy * vec4(-1.0, -1.0, 0.0, 0.0) + vec4(self.g1.y) * other.g0.xyxx * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g1.z) * other.g0.yyyx * vec4(0.0, 0.0, 1.0, 1.0) + vec4(self.g1.w) * other.g0.xxxy * vec4(0.0, 0.0, 1.0, -1.0) + self.g0.zzxx * other.g0.wzwz, vec4(self.g0.y) * other.g0.xyxx * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.w) * other.g0.xxxy * vec4(0.0, 0.0, 1.0, 1.0) + vec4(self.g1.x) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.y) * other.g0.zzzw * vec4(0.0, 0.0, 1.0, 1.0) + vec4(self.g1.z) * other.g0.wzww * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g1.w) * other.g0.zwzz * vec4(-1.0, 1.0, 0.0, 0.0) + self.g0.xxzz * other.g0.yxyx * vec4(1.0, 1.0, -1.0, 1.0));
}

MultiVector multi_vector_motor_dual_regressive_product(MultiVector self, MotorDual other) {
    return MultiVector(vec4(self.g0.y) * other.g0.yxyy * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g1.y) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, 1.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + self.g0.xxzw * vec4(other.g0.x) * vec4(1.0, 0.0, 1.0, 1.0), vec4(self.g1.y) * other.g0.yxyy * vec4(1.0, 1.0, 0.0, 0.0) + self.g1.xxzw * vec4(other.g0.x) * vec4(1.0, 0.0, 1.0, 1.0));
}

MultiVector multi_vector_motor_dual_inner_product(MultiVector self, MotorDual other) {
    return MultiVector(vec4(self.g0.y) * other.g0.zzzw * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.w) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.x) * other.g0.yxyy * vec4(-1.0, -1.0, 0.0, 0.0) + vec4(self.g1.y) * other.g0.xyxx * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g1.z) * other.g0.yyyx * vec4(0.0, 0.0, 1.0, 1.0) + vec4(self.g1.w) * other.g0.xxxy * vec4(0.0, 0.0, 1.0, -1.0) + self.g0.zxxx * other.g0.wxwz * vec4(1.0, 0.0, 1.0, 1.0), vec4(self.g0.y) * vec4(other.g0.x) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y) * other.g0.zzzw * vec4(0.0, 0.0, 1.0, 1.0) + vec4(self.g1.z) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.z) * vec4(-1.0, 0.0, 0.0, 0.0) + self.g0.xxwz * other.g0.yxxx);
}

MultiVector multi_vector_motor_dual_left_contraction(MultiVector self, MotorDual other) {
    return MultiVector(vec4(self.g0.w) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.x) * other.g0.yxyy * vec4(-1.0, -1.0, 0.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + self.g0.zxxx * other.g0.wxwz * vec4(1.0, 0.0, 1.0, 1.0), vec4(self.g0.y) * vec4(other.g0.x) * vec4(-1.0, 0.0, 0.0, 0.0) + self.g0.xxwz * other.g0.yxxx);
}

Scalar multi_vector_motor_dual_scalar_product(MultiVector self, MotorDual other) {
    return Scalar(self.g0.z * other.g0.w + self.g0.w * other.g0.z - self.g1.x * other.g0.y + self.g1.y * other.g0.x);
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
    return Rotor(vec2(0.0));
}

Rotor rotor_one() {
    return Rotor(vec2(1.0, 0.0));
}

Rotor rotor_neg(Rotor self) {
    return Rotor(self.g0 * vec2(-1.0));
}

Rotor rotor_automorphism(Rotor self) {
    return Rotor(self.g0);
}

Rotor rotor_reversal(Rotor self) {
    return Rotor(self.g0 * vec2(1.0, -1.0));
}

Rotor rotor_conjugation(Rotor self) {
    return Rotor(self.g0 * vec2(1.0, -1.0));
}

Scalar rotor_scalar_into(Rotor self) {
    return Scalar(self.g0.x);
}

Rotor rotor_scalar_add(Rotor self, Scalar other) {
    return Rotor(self.g0 + vec2(other.g0) * vec2(1.0, 0.0));
}

Rotor rotor_scalar_sub(Rotor self, Scalar other) {
    return Rotor(self.g0 - vec2(other.g0) * vec2(1.0, 0.0));
}

Rotor rotor_scalar_geometric_product(Rotor self, Scalar other) {
    return Rotor(self.g0 * vec2(other.g0));
}

Rotor rotor_scalar_outer_product(Rotor self, Scalar other) {
    return Rotor(self.g0 * vec2(other.g0));
}

Rotor rotor_scalar_inner_product(Rotor self, Scalar other) {
    return Rotor(self.g0 * vec2(other.g0));
}

Scalar rotor_scalar_left_contraction(Rotor self, Scalar other) {
    return Scalar(self.g0.x * other.g0);
}

Rotor rotor_scalar_right_contraction(Rotor self, Scalar other) {
    return Rotor(self.g0 * vec2(other.g0));
}

Scalar rotor_scalar_scalar_product(Rotor self, Scalar other) {
    return Scalar(self.g0.x * other.g0);
}

MultiVector rotor_multi_vector_add(Rotor self, MultiVector other) {
    return MultiVector(vec4(self.g0.x, self.g0.y, self.g0.x, self.g0.x) * vec4(1.0, 1.0, 0.0, 0.0) + other.g0, other.g1);
}

MultiVector rotor_multi_vector_sub(Rotor self, MultiVector other) {
    return MultiVector(vec4(self.g0.x, self.g0.y, self.g0.x, self.g0.x) * vec4(1.0, 1.0, 0.0, 0.0) - other.g0, vec4(0.0) - other.g1);
}

MultiVector rotor_multi_vector_geometric_product(Rotor self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g0 + vec4(self.g0.y) * other.g0.yxwz * vec4(-1.0, 1.0, 1.0, -1.0), vec4(self.g0.x) * other.g1 + vec4(self.g0.y) * other.g1.yxwz * vec4(-1.0, 1.0, -1.0, 1.0));
}

MultiVector rotor_multi_vector_outer_product(Rotor self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g0 + vec4(self.g0.x, self.g0.y, self.g0.x, self.g0.x) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0), vec4(self.g0.x) * other.g1 + vec4(self.g0.x, self.g0.y, self.g0.x, self.g0.x) * vec4(other.g1.x) * vec4(0.0, 1.0, 0.0, 0.0));
}

MultiVector rotor_multi_vector_inner_product(Rotor self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g0 + vec4(self.g0.y) * other.g0.yxwz * vec4(-1.0, 1.0, 1.0, -1.0), vec4(self.g0.x) * other.g1 + vec4(self.g0.y, self.g0.x, self.g0.x, self.g0.x) * other.g1.yxxx * vec4(-1.0, 0.0, 0.0, 0.0));
}

MultiVector rotor_multi_vector_left_contraction(Rotor self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g0 + vec4(self.g0.y, self.g0.x, self.g0.x, self.g0.x) * other.g0.yxxx * vec4(-1.0, 0.0, 0.0, 0.0), vec4(self.g0.x) * other.g1 + vec4(self.g0.y, self.g0.x, self.g0.x, self.g0.x) * other.g1.yxxx * vec4(-1.0, 0.0, 0.0, 0.0));
}

Scalar rotor_multi_vector_scalar_product(Rotor self, MultiVector other) {
    return Scalar(self.g0.x * other.g0.x - self.g0.y * other.g0.y);
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
    return Rotor(vec2(self.g0.x, self.g0.y) * vec2(1.0, 1.0) / vec2(other.g0.x, other.g0.y) * vec2(1.0, 1.0));
}

Rotor rotor_rotor_geometric_product(Rotor self, Rotor other) {
    return Rotor(vec2(self.g0.x) * other.g0 + vec2(self.g0.y) * other.g0.yx * vec2(-1.0, 1.0));
}

Rotor rotor_rotor_outer_product(Rotor self, Rotor other) {
    return Rotor(vec2(self.g0.x) * other.g0 + self.g0 * vec2(other.g0.x) * vec2(0.0, 1.0));
}

Rotor rotor_rotor_inner_product(Rotor self, Rotor other) {
    return Rotor(vec2(self.g0.x) * other.g0 + vec2(self.g0.y) * other.g0.yx * vec2(-1.0, 1.0));
}

Rotor rotor_rotor_left_contraction(Rotor self, Rotor other) {
    return Rotor(vec2(self.g0.x) * other.g0 + self.g0.yx * other.g0.yx * vec2(-1.0, 0.0));
}

Rotor rotor_rotor_right_contraction(Rotor self, Rotor other) {
    return Rotor(vec2(self.g0.y) * other.g0.yx * vec2(-1.0, 1.0) + vec2(self.g0.x) * vec2(other.g0.x) * vec2(1.0, 0.0));
}

Scalar rotor_rotor_scalar_product(Rotor self, Rotor other) {
    return Scalar(self.g0.x * other.g0.x - self.g0.y * other.g0.y);
}

Motor rotor_point_add(Rotor self, Point other) {
    return Motor(vec4(self.g0.x, self.g0.y, self.g0.x, self.g0.x) * vec4(1.0, 1.0, 0.0, 0.0) + vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0));
}

Motor rotor_point_sub(Rotor self, Point other) {
    return Motor(vec4(self.g0.x, self.g0.y, self.g0.x, self.g0.x) * vec4(1.0, 1.0, 0.0, 0.0) - vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0));
}

Motor rotor_point_geometric_product(Rotor self, Point other) {
    return Motor(vec4(self.g0.y) * vec4(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4(-1.0, 0.0, -1.0, 1.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0));
}

Point rotor_point_outer_product(Rotor self, Point other) {
    return Point(vec3(self.g0.x) * other.g0);
}

Motor rotor_point_inner_product(Rotor self, Point other) {
    return Motor(vec4(self.g0.y, self.g0.x, self.g0.x, self.g0.x) * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(-1.0, 1.0, 1.0, 1.0));
}

Motor rotor_point_left_contraction(Rotor self, Point other) {
    return Motor(vec4(self.g0.y, self.g0.x, self.g0.x, self.g0.x) * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(-1.0, 1.0, 1.0, 1.0));
}

Scalar rotor_point_right_contraction(Rotor self, Point other) {
    return Scalar(0.0 - self.g0.y * other.g0.x);
}

Scalar rotor_point_scalar_product(Rotor self, Point other) {
    return Scalar(0.0 - self.g0.y * other.g0.x);
}

Motor rotor_ideal_point_add(Rotor self, IdealPoint other) {
    return Motor(vec4(self.g0.x, self.g0.y, self.g0.x, self.g0.x) * vec4(1.0, 1.0, 0.0, 0.0) + vec4(other.g0.x, other.g0.x, other.g0.x, other.g0.y) * vec4(0.0, 0.0, 1.0, 1.0));
}

Motor rotor_ideal_point_sub(Rotor self, IdealPoint other) {
    return Motor(vec4(self.g0.x, self.g0.y, self.g0.x, self.g0.x) * vec4(1.0, 1.0, 0.0, 0.0) - vec4(other.g0.x, other.g0.x, other.g0.x, other.g0.y) * vec4(0.0, 0.0, 1.0, 1.0));
}

IdealPoint rotor_ideal_point_geometric_product(Rotor self, IdealPoint other) {
    return IdealPoint(vec2(self.g0.x) * other.g0 + vec2(self.g0.y) * other.g0.yx * vec2(-1.0, 1.0));
}

IdealPoint rotor_ideal_point_outer_product(Rotor self, IdealPoint other) {
    return IdealPoint(vec2(self.g0.x) * other.g0);
}

IdealPoint rotor_ideal_point_inner_product(Rotor self, IdealPoint other) {
    return IdealPoint(vec2(self.g0.x) * other.g0);
}

IdealPoint rotor_ideal_point_left_contraction(Rotor self, IdealPoint other) {
    return IdealPoint(vec2(self.g0.x) * other.g0);
}

MotorDual rotor_plane_geometric_product(Rotor self, Plane other) {
    return MotorDual(vec4(self.g0.y) * vec4(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4(1.0, 0.0, -1.0, 1.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0));
}

Scalar rotor_plane_regressive_product(Rotor self, Plane other) {
    return Scalar(self.g0.y * other.g0.x);
}

MotorDual rotor_plane_outer_product(Rotor self, Plane other) {
    return MotorDual(vec4(self.g0.y, self.g0.x, self.g0.x, self.g0.x) * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z));
}

Plane rotor_plane_inner_product(Rotor self, Plane other) {
    return Plane(vec3(self.g0.x) * other.g0 + vec3(self.g0.x, self.g0.y, self.g0.y) * other.g0.xzy * vec3(0.0, -1.0, 1.0));
}

Plane rotor_plane_left_contraction(Rotor self, Plane other) {
    return Plane(vec3(self.g0.x) * other.g0);
}

Motor rotor_translator_add(Rotor self, Translator other) {
    return Motor(vec4(self.g0.x, self.g0.y, self.g0.x, self.g0.x) * vec4(1.0, 1.0, 0.0, 0.0) + vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(1.0, 0.0, 1.0, 1.0));
}

Motor rotor_translator_sub(Rotor self, Translator other) {
    return Motor(vec4(self.g0.x, self.g0.y, self.g0.x, self.g0.x) * vec4(1.0, 1.0, 0.0, 0.0) - vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(1.0, 0.0, 1.0, 1.0));
}

Motor rotor_translator_geometric_product(Rotor self, Translator other) {
    return Motor(vec4(self.g0.y) * vec4(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4(0.0, 1.0, -1.0, 1.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(1.0, 0.0, 1.0, 1.0));
}

Motor rotor_translator_outer_product(Rotor self, Translator other) {
    return Motor(vec4(self.g0.x, self.g0.y, self.g0.x, self.g0.x) * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z));
}

Motor rotor_translator_inner_product(Rotor self, Translator other) {
    return Motor(vec4(self.g0.x, self.g0.y, self.g0.x, self.g0.x) * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z));
}

Translator rotor_translator_left_contraction(Rotor self, Translator other) {
    return Translator(vec3(self.g0.x) * other.g0);
}

Rotor rotor_translator_right_contraction(Rotor self, Translator other) {
    return Rotor(self.g0 * vec2(other.g0.x));
}

Scalar rotor_translator_scalar_product(Rotor self, Translator other) {
    return Scalar(self.g0.x * other.g0.x);
}

Motor rotor_motor_add(Rotor self, Motor other) {
    return Motor(vec4(self.g0.x, self.g0.y, self.g0.x, self.g0.x) * vec4(1.0, 1.0, 0.0, 0.0) + other.g0);
}

Motor rotor_motor_sub(Rotor self, Motor other) {
    return Motor(vec4(self.g0.x, self.g0.y, self.g0.x, self.g0.x) * vec4(1.0, 1.0, 0.0, 0.0) - other.g0);
}

Motor rotor_motor_geometric_product(Rotor self, Motor other) {
    return Motor(vec4(self.g0.x) * other.g0 + vec4(self.g0.y) * other.g0.yxwz * vec4(-1.0, 1.0, -1.0, 1.0));
}

Motor rotor_motor_outer_product(Rotor self, Motor other) {
    return Motor(vec4(self.g0.x) * other.g0 + vec4(self.g0.x, self.g0.y, self.g0.x, self.g0.x) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0));
}

Motor rotor_motor_inner_product(Rotor self, Motor other) {
    return Motor(vec4(self.g0.x) * other.g0 + vec4(self.g0.y, self.g0.y, self.g0.x, self.g0.x) * other.g0.yxxx * vec4(-1.0, 1.0, 0.0, 0.0));
}

Motor rotor_motor_left_contraction(Rotor self, Motor other) {
    return Motor(vec4(self.g0.x) * other.g0 + vec4(self.g0.y, self.g0.x, self.g0.x, self.g0.x) * other.g0.yxxx * vec4(-1.0, 0.0, 0.0, 0.0));
}

Rotor rotor_motor_right_contraction(Rotor self, Motor other) {
    return Rotor(vec2(self.g0.y) * vec2(other.g0.y, other.g0.x) * vec2(-1.0, 1.0) + vec2(self.g0.x) * vec2(other.g0.x) * vec2(1.0, 0.0));
}

Scalar rotor_motor_scalar_product(Rotor self, Motor other) {
    return Scalar(self.g0.x * other.g0.x - self.g0.y * other.g0.y);
}

MotorDual rotor_motor_dual_geometric_product(Rotor self, MotorDual other) {
    return MotorDual(vec4(self.g0.x) * other.g0 + vec4(self.g0.y) * other.g0.yxwz * vec4(1.0, -1.0, -1.0, 1.0));
}

Rotor rotor_motor_dual_regressive_product(Rotor self, MotorDual other) {
    return Rotor(vec2(self.g0.y) * vec2(other.g0.y, other.g0.x) + vec2(self.g0.x) * vec2(other.g0.x) * vec2(1.0, 0.0));
}

MotorDual rotor_motor_dual_outer_product(Rotor self, MotorDual other) {
    return MotorDual(vec4(self.g0.x) * other.g0 + vec4(self.g0.y, self.g0.x, self.g0.x, self.g0.x) * other.g0.yxxx * vec4(1.0, 0.0, 0.0, 0.0));
}

MotorDual rotor_motor_dual_inner_product(Rotor self, MotorDual other) {
    return MotorDual(vec4(self.g0.x) * other.g0 + vec4(self.g0.x, self.g0.y, self.g0.y, self.g0.y) * other.g0.xxwz * vec4(0.0, -1.0, -1.0, 1.0));
}

MotorDual rotor_motor_dual_left_contraction(Rotor self, MotorDual other) {
    return MotorDual(vec4(self.g0.x) * other.g0 + vec4(self.g0.x, self.g0.y, self.g0.x, self.g0.x) * vec4(other.g0.x) * vec4(0.0, -1.0, 0.0, 0.0));
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
    return Point(vec3(0.0));
}

Point point_one() {
    return Point(vec3(0.0));
}

Point point_neg(Point self) {
    return Point(self.g0 * vec3(-1.0));
}

Point point_automorphism(Point self) {
    return Point(self.g0);
}

Point point_reversal(Point self) {
    return Point(self.g0 * vec3(-1.0));
}

Point point_conjugation(Point self) {
    return Point(self.g0 * vec3(-1.0));
}

Plane point_dual(Point self) {
    return Plane(self.g0);
}

Motor point_scalar_add(Point self, Scalar other) {
    return Motor(vec4(self.g0.x, self.g0.x, self.g0.y, self.g0.z) * vec4(0.0, 1.0, 1.0, 1.0) + vec4(other.g0) * vec4(1.0, 0.0, 0.0, 0.0));
}

Motor point_scalar_sub(Point self, Scalar other) {
    return Motor(vec4(self.g0.x, self.g0.x, self.g0.y, self.g0.z) * vec4(0.0, 1.0, 1.0, 1.0) - vec4(other.g0) * vec4(1.0, 0.0, 0.0, 0.0));
}

Point point_scalar_geometric_product(Point self, Scalar other) {
    return Point(self.g0 * vec3(other.g0));
}

Point point_scalar_outer_product(Point self, Scalar other) {
    return Point(self.g0 * vec3(other.g0));
}

Point point_scalar_inner_product(Point self, Scalar other) {
    return Point(self.g0 * vec3(other.g0));
}

Point point_scalar_right_contraction(Point self, Scalar other) {
    return Point(self.g0 * vec3(other.g0));
}

MultiVector point_multi_vector_add(Point self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + other.g0, vec4(self.g0.x, self.g0.x, self.g0.y, self.g0.z) * vec4(0.0, 0.0, 1.0, 1.0) + other.g1);
}

MultiVector point_multi_vector_sub(Point self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) - other.g0, vec4(self.g0.x, self.g0.x, self.g0.y, self.g0.z) * vec4(0.0, 0.0, 1.0, 1.0) - other.g1);
}

MultiVector point_multi_vector_geometric_product(Point self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g0.yxwz * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.y) * other.g1.zwxy * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g0.z) * other.g1.wzyx * vec4(1.0, 1.0, 1.0, -1.0), vec4(self.g0.x) * other.g1.yxwz * vec4(-1.0, 1.0, -1.0, 1.0) + vec4(self.g0.y) * other.g0.zwxy * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g0.wzyx * vec4(-1.0, 1.0, 1.0, 1.0));
}

Scalar point_multi_vector_scalar_product(Point self, MultiVector other) {
    return Scalar(0.0 - self.g0.x * other.g0.y + self.g0.y * other.g1.z + self.g0.z * other.g1.w);
}

Motor point_rotor_add(Point self, Rotor other) {
    return Motor(vec4(self.g0.x, self.g0.x, self.g0.y, self.g0.z) * vec4(0.0, 1.0, 1.0, 1.0) + vec4(other.g0.x, other.g0.y, other.g0.x, other.g0.x) * vec4(1.0, 1.0, 0.0, 0.0));
}

Motor point_rotor_sub(Point self, Rotor other) {
    return Motor(vec4(self.g0.x, self.g0.x, self.g0.y, self.g0.z) * vec4(0.0, 1.0, 1.0, 1.0) - vec4(other.g0.x, other.g0.y, other.g0.x, other.g0.x) * vec4(1.0, 1.0, 0.0, 0.0));
}

Motor point_rotor_geometric_product(Point self, Rotor other) {
    return Motor(vec4(self.g0.z) * vec4(other.g0.y, other.g0.y, other.g0.y, other.g0.x) * vec4(0.0, 0.0, 1.0, 1.0) + vec4(self.g0.x, self.g0.x, self.g0.y, self.g0.y) * vec4(other.g0.y, other.g0.x, other.g0.x, other.g0.y) * vec4(-1.0, 1.0, 1.0, -1.0));
}

Point point_rotor_outer_product(Point self, Rotor other) {
    return Point(self.g0 * vec3(other.g0.x));
}

Motor point_rotor_inner_product(Point self, Rotor other) {
    return Motor(vec4(self.g0.x, self.g0.x, self.g0.y, self.g0.z) * vec4(other.g0.y, other.g0.x, other.g0.x, other.g0.x) * vec4(-1.0, 1.0, 1.0, 1.0));
}

Scalar point_rotor_left_contraction(Point self, Rotor other) {
    return Scalar(0.0 - self.g0.x * other.g0.y);
}

Motor point_rotor_right_contraction(Point self, Rotor other) {
    return Motor(vec4(self.g0.x, self.g0.x, self.g0.y, self.g0.z) * vec4(other.g0.y, other.g0.x, other.g0.x, other.g0.x) * vec4(-1.0, 1.0, 1.0, 1.0));
}

Scalar point_rotor_scalar_product(Point self, Rotor other) {
    return Scalar(0.0 - self.g0.x * other.g0.y);
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
    return Point(vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(1.0, 1.0, 1.0) / vec3(other.g0.x, other.g0.y, other.g0.z) * vec3(1.0, 1.0, 1.0));
}

Motor point_point_geometric_product(Point self, Point other) {
    return Motor(vec4(self.g0.y) * vec4(other.g0.y, other.g0.z, other.g0.y, other.g0.x) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z, other.g0.y, other.g0.x, other.g0.z) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4(-1.0, 0.0, -1.0, 1.0));
}

Plane point_point_regressive_product(Point self, Point other) {
    return Plane(vec3(self.g0.y) * other.g0.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * other.g0.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x) * other.g0.xzy * vec3(0.0, 1.0, -1.0));
}

Scalar point_point_inner_product(Point self, Point other) {
    return Scalar(0.0 - self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z);
}

Scalar point_point_left_contraction(Point self, Point other) {
    return Scalar(0.0 - self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z);
}

Scalar point_point_right_contraction(Point self, Point other) {
    return Scalar(0.0 - self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z);
}

Scalar point_point_scalar_product(Point self, Point other) {
    return Scalar(0.0 - self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z);
}

IdealPoint point_ideal_point_into(Point self) {
    return IdealPoint(vec2(self.g0.y, self.g0.z));
}

Point point_ideal_point_add(Point self, IdealPoint other) {
    return Point(self.g0 + vec3(other.g0.x, other.g0.x, other.g0.y) * vec3(0.0, 1.0, 1.0));
}

Point point_ideal_point_sub(Point self, IdealPoint other) {
    return Point(self.g0 - vec3(other.g0.x, other.g0.x, other.g0.y) * vec3(0.0, 1.0, 1.0));
}

Motor point_ideal_point_geometric_product(Point self, IdealPoint other) {
    return Motor(vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g0.y) * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g0.y, self.g0.y, self.g0.x, self.g0.x) * vec4(other.g0.x, other.g0.y, other.g0.y, other.g0.x) * vec4(1.0, -1.0, -1.0, 1.0));
}

Plane point_ideal_point_regressive_product(Point self, IdealPoint other) {
    return Plane(vec3(self.g0.z) * vec3(other.g0.x) * vec3(1.0, 0.0, 0.0) + self.g0.yxx * vec3(other.g0.y, other.g0.y, other.g0.x) * vec3(-1.0, 1.0, -1.0));
}

Scalar point_ideal_point_inner_product(Point self, IdealPoint other) {
    return Scalar(self.g0.y * other.g0.x + self.g0.z * other.g0.y);
}

Scalar point_ideal_point_left_contraction(Point self, IdealPoint other) {
    return Scalar(self.g0.y * other.g0.x + self.g0.z * other.g0.y);
}

Scalar point_ideal_point_right_contraction(Point self, IdealPoint other) {
    return Scalar(self.g0.y * other.g0.x + self.g0.z * other.g0.y);
}

Scalar point_ideal_point_scalar_product(Point self, IdealPoint other) {
    return Scalar(self.g0.y * other.g0.x + self.g0.z * other.g0.y);
}

MotorDual point_plane_geometric_product(Point self, Plane other) {
    return MotorDual(vec4(self.g0.y) * vec4(other.g0.y, other.g0.z, other.g0.y, other.g0.x) * vec4(1.0, 1.0, 0.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.z, other.g0.y, other.g0.x, other.g0.z) * vec4(1.0, -1.0, -1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4(1.0, 0.0, -1.0, 1.0));
}

Scalar point_plane_regressive_product(Point self, Plane other) {
    return Scalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z);
}

Plane point_plane_inner_product(Point self, Plane other) {
    return Plane(vec3(self.g0.y) * other.g0.zzx * vec3(1.0, 0.0, 1.0) + vec3(self.g0.z) * other.g0.yxy * vec3(-1.0, -1.0, 0.0) + vec3(self.g0.x) * other.g0.xzy * vec3(0.0, -1.0, 1.0));
}

Plane point_plane_right_contraction(Point self, Plane other) {
    return Plane(vec3(self.g0.y) * other.g0.zzx * vec3(1.0, 0.0, 1.0) + vec3(self.g0.z) * other.g0.yxy * vec3(-1.0, -1.0, 0.0) + vec3(self.g0.x) * other.g0.xzy * vec3(0.0, -1.0, 1.0));
}

Motor point_translator_add(Point self, Translator other) {
    return Motor(vec4(self.g0.x, self.g0.x, self.g0.y, self.g0.z) * vec4(0.0, 1.0, 1.0, 1.0) + vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(1.0, 0.0, 1.0, 1.0));
}

Motor point_translator_sub(Point self, Translator other) {
    return Motor(vec4(self.g0.x, self.g0.x, self.g0.y, self.g0.z) * vec4(0.0, 1.0, 1.0, 1.0) - vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(1.0, 0.0, 1.0, 1.0));
}

Motor point_translator_geometric_product(Point self, Translator other) {
    return Motor(vec4(self.g0.y) * vec4(other.g0.y, other.g0.z, other.g0.x, other.g0.y) * vec4(1.0, -1.0, 1.0, 0.0) + vec4(self.g0.z) * vec4(other.g0.z, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4(0.0, 1.0, -1.0, 1.0));
}

Plane point_translator_regressive_product(Point self, Translator other) {
    return Plane(vec3(self.g0.z) * vec3(other.g0.y) * vec3(1.0, 0.0, 0.0) + self.g0.yxx * other.g0.zzy * vec3(-1.0, 1.0, -1.0));
}

Point point_translator_outer_product(Point self, Translator other) {
    return Point(self.g0 * vec3(other.g0.x));
}

Motor point_translator_inner_product(Point self, Translator other) {
    return Motor(vec4(self.g0.z) * vec4(other.g0.z, other.g0.z, other.g0.z, other.g0.x) * vec4(1.0, 0.0, 0.0, 1.0) + vec4(self.g0.y, self.g0.x, self.g0.y, self.g0.x) * vec4(other.g0.y, other.g0.x, other.g0.x, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

Scalar point_translator_left_contraction(Point self, Translator other) {
    return Scalar(self.g0.y * other.g0.y + self.g0.z * other.g0.z);
}

Motor point_translator_right_contraction(Point self, Translator other) {
    return Motor(vec4(self.g0.z) * vec4(other.g0.z, other.g0.z, other.g0.z, other.g0.x) * vec4(1.0, 0.0, 0.0, 1.0) + vec4(self.g0.y, self.g0.x, self.g0.y, self.g0.x) * vec4(other.g0.y, other.g0.x, other.g0.x, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

Scalar point_translator_scalar_product(Point self, Translator other) {
    return Scalar(self.g0.y * other.g0.y + self.g0.z * other.g0.z);
}

Motor point_motor_add(Point self, Motor other) {
    return Motor(vec4(self.g0.x, self.g0.x, self.g0.y, self.g0.z) * vec4(0.0, 1.0, 1.0, 1.0) + other.g0);
}

Motor point_motor_sub(Point self, Motor other) {
    return Motor(vec4(self.g0.x, self.g0.x, self.g0.y, self.g0.z) * vec4(0.0, 1.0, 1.0, 1.0) - other.g0);
}

Motor point_motor_geometric_product(Point self, Motor other) {
    return Motor(vec4(self.g0.x) * other.g0.yxwz * vec4(-1.0, 1.0, -1.0, 1.0) + vec4(self.g0.y) * other.g0.zwxy * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g0.wzyx);
}

Plane point_motor_regressive_product(Point self, Motor other) {
    return Plane(vec3(self.g0.y) * vec3(other.g0.w, other.g0.w, other.g0.y) * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * vec3(other.g0.z, other.g0.y, other.g0.z) * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x) * vec3(other.g0.x, other.g0.w, other.g0.z) * vec3(0.0, 1.0, -1.0));
}

Point point_motor_outer_product(Point self, Motor other) {
    return Point(self.g0 * vec3(other.g0.x));
}

Motor point_motor_inner_product(Point self, Motor other) {
    return Motor(vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g0.z) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * other.g0.yxxx * vec4(-1.0, 1.0, 0.0, 0.0));
}

Scalar point_motor_left_contraction(Point self, Motor other) {
    return Scalar(0.0 - self.g0.x * other.g0.y + self.g0.y * other.g0.z + self.g0.z * other.g0.w);
}

Motor point_motor_right_contraction(Point self, Motor other) {
    return Motor(vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g0.z) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * other.g0.yxxx * vec4(-1.0, 1.0, 0.0, 0.0));
}

Scalar point_motor_scalar_product(Point self, Motor other) {
    return Scalar(0.0 - self.g0.x * other.g0.y + self.g0.y * other.g0.z + self.g0.z * other.g0.w);
}

MotorDual point_motor_dual_geometric_product(Point self, MotorDual other) {
    return MotorDual(vec4(self.g0.x) * other.g0.yxwz * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.y) * other.g0.zwxy + vec4(self.g0.z) * other.g0.wzyx * vec4(1.0, -1.0, -1.0, 1.0));
}

Motor point_motor_dual_regressive_product(Point self, MotorDual other) {
    return Motor(vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g0.z) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * other.g0.yxxx * vec4(1.0, 1.0, 0.0, 0.0));
}

Plane point_motor_dual_inner_product(Point self, MotorDual other) {
    return Plane(vec3(self.g0.x) * vec3(other.g0.x, other.g0.w, other.g0.z) * vec3(-1.0, -1.0, 1.0) + vec3(self.g0.y) * vec3(other.g0.w, other.g0.x, other.g0.y) + vec3(self.g0.z) * vec3(other.g0.z, other.g0.y, other.g0.x) * vec3(-1.0, -1.0, 1.0));
}

Plane point_motor_dual_left_contraction(Point self, MotorDual other) {
    return Plane(self.g0 * vec3(other.g0.x) * vec3(-1.0, 1.0, 1.0));
}

Plane point_motor_dual_right_contraction(Point self, MotorDual other) {
    return Plane(vec3(self.g0.y) * vec3(other.g0.w, other.g0.w, other.g0.y) * vec3(1.0, 0.0, 1.0) + vec3(self.g0.z) * vec3(other.g0.z, other.g0.y, other.g0.z) * vec3(-1.0, -1.0, 0.0) + vec3(self.g0.x) * vec3(other.g0.x, other.g0.w, other.g0.z) * vec3(0.0, -1.0, 1.0));
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
    return IdealPoint(vec2(0.0));
}

IdealPoint ideal_point_one() {
    return IdealPoint(vec2(0.0));
}

IdealPoint ideal_point_neg(IdealPoint self) {
    return IdealPoint(self.g0 * vec2(-1.0));
}

IdealPoint ideal_point_automorphism(IdealPoint self) {
    return IdealPoint(self.g0);
}

IdealPoint ideal_point_reversal(IdealPoint self) {
    return IdealPoint(self.g0 * vec2(-1.0));
}

IdealPoint ideal_point_conjugation(IdealPoint self) {
    return IdealPoint(self.g0 * vec2(-1.0));
}

Translator ideal_point_scalar_add(IdealPoint self, Scalar other) {
    return Translator(vec3(self.g0.x, self.g0.x, self.g0.y) * vec3(0.0, 1.0, 1.0) + vec3(other.g0) * vec3(1.0, 0.0, 0.0));
}

Translator ideal_point_scalar_sub(IdealPoint self, Scalar other) {
    return Translator(vec3(self.g0.x, self.g0.x, self.g0.y) * vec3(0.0, 1.0, 1.0) - vec3(other.g0) * vec3(1.0, 0.0, 0.0));
}

IdealPoint ideal_point_scalar_geometric_product(IdealPoint self, Scalar other) {
    return IdealPoint(self.g0 * vec2(other.g0));
}

IdealPoint ideal_point_scalar_outer_product(IdealPoint self, Scalar other) {
    return IdealPoint(self.g0 * vec2(other.g0));
}

IdealPoint ideal_point_scalar_inner_product(IdealPoint self, Scalar other) {
    return IdealPoint(self.g0 * vec2(other.g0));
}

IdealPoint ideal_point_scalar_right_contraction(IdealPoint self, Scalar other) {
    return IdealPoint(self.g0 * vec2(other.g0));
}

MultiVector ideal_point_multi_vector_add(IdealPoint self, MultiVector other) {
    return MultiVector(other.g0, vec4(self.g0.x, self.g0.x, self.g0.x, self.g0.y) * vec4(0.0, 0.0, 1.0, 1.0) + other.g1);
}

MultiVector ideal_point_multi_vector_sub(IdealPoint self, MultiVector other) {
    return MultiVector(vec4(0.0) - other.g0, vec4(self.g0.x, self.g0.x, self.g0.x, self.g0.y) * vec4(0.0, 0.0, 1.0, 1.0) - other.g1);
}

MultiVector ideal_point_multi_vector_geometric_product(IdealPoint self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g1.zwxy * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g0.y) * other.g1.wzyx * vec4(1.0, 1.0, 1.0, -1.0), vec4(self.g0.x) * other.g0.zwxy * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g0.y) * other.g0.wzyx * vec4(-1.0, 1.0, 1.0, 1.0));
}

Scalar ideal_point_multi_vector_scalar_product(IdealPoint self, MultiVector other) {
    return Scalar(self.g0.x * other.g1.z + self.g0.y * other.g1.w);
}

Motor ideal_point_rotor_add(IdealPoint self, Rotor other) {
    return Motor(vec4(self.g0.x, self.g0.x, self.g0.x, self.g0.y) * vec4(0.0, 0.0, 1.0, 1.0) + vec4(other.g0.x, other.g0.y, other.g0.x, other.g0.x) * vec4(1.0, 1.0, 0.0, 0.0));
}

Motor ideal_point_rotor_sub(IdealPoint self, Rotor other) {
    return Motor(vec4(self.g0.x, self.g0.x, self.g0.x, self.g0.y) * vec4(0.0, 0.0, 1.0, 1.0) - vec4(other.g0.x, other.g0.y, other.g0.x, other.g0.x) * vec4(1.0, 1.0, 0.0, 0.0));
}

IdealPoint ideal_point_rotor_geometric_product(IdealPoint self, Rotor other) {
    return IdealPoint(vec2(self.g0.x) * other.g0 * vec2(1.0, -1.0) + vec2(self.g0.y) * other.g0.yx);
}

IdealPoint ideal_point_rotor_outer_product(IdealPoint self, Rotor other) {
    return IdealPoint(self.g0 * vec2(other.g0.x));
}

IdealPoint ideal_point_rotor_inner_product(IdealPoint self, Rotor other) {
    return IdealPoint(self.g0 * vec2(other.g0.x));
}

IdealPoint ideal_point_rotor_right_contraction(IdealPoint self, Rotor other) {
    return IdealPoint(self.g0 * vec2(other.g0.x));
}

Point ideal_point_point_add(IdealPoint self, Point other) {
    return Point(vec3(self.g0.x, self.g0.x, self.g0.y) * vec3(0.0, 1.0, 1.0) + other.g0);
}

Point ideal_point_point_sub(IdealPoint self, Point other) {
    return Point(vec3(self.g0.x, self.g0.x, self.g0.y) * vec3(0.0, 1.0, 1.0) - other.g0);
}

Motor ideal_point_point_geometric_product(IdealPoint self, Point other) {
    return Motor(vec4(self.g0.y) * vec4(other.g0.z, other.g0.y, other.g0.x, other.g0.z) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.y, other.g0.z, other.g0.x, other.g0.x) * vec4(1.0, -1.0, 0.0, -1.0));
}

Plane ideal_point_point_regressive_product(IdealPoint self, Point other) {
    return Plane(vec3(self.g0.y) * other.g0.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x) * other.g0.zxx * vec3(-1.0, 0.0, 1.0));
}

Scalar ideal_point_point_inner_product(IdealPoint self, Point other) {
    return Scalar(self.g0.x * other.g0.y + self.g0.y * other.g0.z);
}

Scalar ideal_point_point_left_contraction(IdealPoint self, Point other) {
    return Scalar(self.g0.x * other.g0.y + self.g0.y * other.g0.z);
}

Scalar ideal_point_point_right_contraction(IdealPoint self, Point other) {
    return Scalar(self.g0.x * other.g0.y + self.g0.y * other.g0.z);
}

Scalar ideal_point_point_scalar_product(IdealPoint self, Point other) {
    return Scalar(self.g0.x * other.g0.y + self.g0.y * other.g0.z);
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
    return IdealPoint(vec2(self.g0.x, self.g0.y) * vec2(1.0, 1.0) / vec2(other.g0.x, other.g0.y) * vec2(1.0, 1.0));
}

Rotor ideal_point_ideal_point_geometric_product(IdealPoint self, IdealPoint other) {
    return Rotor(vec2(self.g0.x) * other.g0 * vec2(1.0, -1.0) + vec2(self.g0.y) * other.g0.yx);
}

Scalar ideal_point_ideal_point_inner_product(IdealPoint self, IdealPoint other) {
    return Scalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y);
}

Scalar ideal_point_ideal_point_left_contraction(IdealPoint self, IdealPoint other) {
    return Scalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y);
}

Scalar ideal_point_ideal_point_right_contraction(IdealPoint self, IdealPoint other) {
    return Scalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y);
}

Scalar ideal_point_ideal_point_scalar_product(IdealPoint self, IdealPoint other) {
    return Scalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y);
}

MotorDual ideal_point_plane_geometric_product(IdealPoint self, Plane other) {
    return MotorDual(vec4(self.g0.y) * vec4(other.g0.z, other.g0.y, other.g0.x, other.g0.z) * vec4(1.0, -1.0, -1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.y, other.g0.z, other.g0.x, other.g0.x) * vec4(1.0, 1.0, 0.0, 1.0));
}

Scalar ideal_point_plane_regressive_product(IdealPoint self, Plane other) {
    return Scalar(self.g0.x * other.g0.y + self.g0.y * other.g0.z);
}

Plane ideal_point_plane_inner_product(IdealPoint self, Plane other) {
    return Plane(vec3(self.g0.y) * other.g0.yxy * vec3(-1.0, -1.0, 0.0) + vec3(self.g0.x) * other.g0.zxx * vec3(1.0, 0.0, 1.0));
}

Plane ideal_point_plane_right_contraction(IdealPoint self, Plane other) {
    return Plane(vec3(self.g0.y) * other.g0.yxy * vec3(-1.0, -1.0, 0.0) + vec3(self.g0.x) * other.g0.zxx * vec3(1.0, 0.0, 1.0));
}

Translator ideal_point_translator_add(IdealPoint self, Translator other) {
    return Translator(vec3(self.g0.x, self.g0.x, self.g0.y) * vec3(0.0, 1.0, 1.0) + other.g0);
}

Translator ideal_point_translator_sub(IdealPoint self, Translator other) {
    return Translator(vec3(self.g0.x, self.g0.x, self.g0.y) * vec3(0.0, 1.0, 1.0) - other.g0);
}

Motor ideal_point_translator_geometric_product(IdealPoint self, Translator other) {
    return Motor(vec4(self.g0.y) * vec4(other.g0.z, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g0.y, other.g0.z, other.g0.x, other.g0.x) * vec4(1.0, -1.0, 1.0, 0.0));
}

IdealPoint ideal_point_translator_outer_product(IdealPoint self, Translator other) {
    return IdealPoint(self.g0 * vec2(other.g0.x));
}

Translator ideal_point_translator_inner_product(IdealPoint self, Translator other) {
    return Translator(vec3(self.g0.y) * other.g0.zzx * vec3(1.0, 0.0, 1.0) + vec3(self.g0.x) * other.g0.yxx * vec3(1.0, 1.0, 0.0));
}

Scalar ideal_point_translator_left_contraction(IdealPoint self, Translator other) {
    return Scalar(self.g0.x * other.g0.y + self.g0.y * other.g0.z);
}

Translator ideal_point_translator_right_contraction(IdealPoint self, Translator other) {
    return Translator(vec3(self.g0.y) * other.g0.zzx * vec3(1.0, 0.0, 1.0) + vec3(self.g0.x) * other.g0.yxx * vec3(1.0, 1.0, 0.0));
}

Scalar ideal_point_translator_scalar_product(IdealPoint self, Translator other) {
    return Scalar(self.g0.x * other.g0.y + self.g0.y * other.g0.z);
}

Motor ideal_point_motor_add(IdealPoint self, Motor other) {
    return Motor(vec4(self.g0.x, self.g0.x, self.g0.x, self.g0.y) * vec4(0.0, 0.0, 1.0, 1.0) + other.g0);
}

Motor ideal_point_motor_sub(IdealPoint self, Motor other) {
    return Motor(vec4(self.g0.x, self.g0.x, self.g0.x, self.g0.y) * vec4(0.0, 0.0, 1.0, 1.0) - other.g0);
}

Motor ideal_point_motor_geometric_product(IdealPoint self, Motor other) {
    return Motor(vec4(self.g0.x) * other.g0.zwxy * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * other.g0.wzyx);
}

Plane ideal_point_motor_regressive_product(IdealPoint self, Motor other) {
    return Plane(vec3(self.g0.y) * vec3(other.g0.z, other.g0.y, other.g0.z) * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x) * vec3(other.g0.w, other.g0.x, other.g0.y) * vec3(-1.0, 0.0, 1.0));
}

IdealPoint ideal_point_motor_outer_product(IdealPoint self, Motor other) {
    return IdealPoint(self.g0 * vec2(other.g0.x));
}

Translator ideal_point_motor_inner_product(IdealPoint self, Motor other) {
    return Translator(vec3(self.g0.y) * vec3(other.g0.w, other.g0.w, other.g0.x) * vec3(1.0, 0.0, 1.0) + vec3(self.g0.x) * vec3(other.g0.z, other.g0.x, other.g0.x) * vec3(1.0, 1.0, 0.0));
}

Scalar ideal_point_motor_left_contraction(IdealPoint self, Motor other) {
    return Scalar(self.g0.x * other.g0.z + self.g0.y * other.g0.w);
}

Translator ideal_point_motor_right_contraction(IdealPoint self, Motor other) {
    return Translator(vec3(self.g0.y) * vec3(other.g0.w, other.g0.w, other.g0.x) * vec3(1.0, 0.0, 1.0) + vec3(self.g0.x) * vec3(other.g0.z, other.g0.x, other.g0.x) * vec3(1.0, 1.0, 0.0));
}

Scalar ideal_point_motor_scalar_product(IdealPoint self, Motor other) {
    return Scalar(self.g0.x * other.g0.z + self.g0.y * other.g0.w);
}

MotorDual ideal_point_motor_dual_geometric_product(IdealPoint self, MotorDual other) {
    return MotorDual(vec4(self.g0.x) * other.g0.zwxy + vec4(self.g0.y) * other.g0.wzyx * vec4(1.0, -1.0, -1.0, 1.0));
}

Translator ideal_point_motor_dual_regressive_product(IdealPoint self, MotorDual other) {
    return Translator(vec3(self.g0.y) * vec3(other.g0.w, other.g0.w, other.g0.x) * vec3(1.0, 0.0, 1.0) + vec3(self.g0.x) * vec3(other.g0.z, other.g0.x, other.g0.x) * vec3(1.0, 1.0, 0.0));
}

Plane ideal_point_motor_dual_inner_product(IdealPoint self, MotorDual other) {
    return Plane(vec3(self.g0.x) * vec3(other.g0.w, other.g0.x, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, other.g0.y, other.g0.x) * vec3(-1.0, -1.0, 1.0));
}

Plane ideal_point_motor_dual_right_contraction(IdealPoint self, MotorDual other) {
    return Plane(vec3(self.g0.y) * vec3(other.g0.z, other.g0.y, other.g0.z) * vec3(-1.0, -1.0, 0.0) + vec3(self.g0.x) * vec3(other.g0.w, other.g0.x, other.g0.y) * vec3(1.0, 0.0, 1.0));
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
    return Plane(vec3(0.0));
}

Plane plane_one() {
    return Plane(vec3(0.0));
}

Plane plane_neg(Plane self) {
    return Plane(self.g0 * vec3(-1.0));
}

Plane plane_automorphism(Plane self) {
    return Plane(self.g0 * vec3(-1.0));
}

Plane plane_reversal(Plane self) {
    return Plane(self.g0);
}

Plane plane_conjugation(Plane self) {
    return Plane(self.g0 * vec3(-1.0));
}

Point plane_dual(Plane self) {
    return Point(self.g0);
}

Plane plane_scalar_geometric_product(Plane self, Scalar other) {
    return Plane(self.g0 * vec3(other.g0));
}

Plane plane_scalar_outer_product(Plane self, Scalar other) {
    return Plane(self.g0 * vec3(other.g0));
}

Plane plane_scalar_inner_product(Plane self, Scalar other) {
    return Plane(self.g0 * vec3(other.g0));
}

Plane plane_scalar_right_contraction(Plane self, Scalar other) {
    return Plane(self.g0 * vec3(other.g0));
}

MultiVector plane_multi_vector_add(Plane self, MultiVector other) {
    return MultiVector(vec4(self.g0.x, self.g0.x, self.g0.z, self.g0.y) * vec4(0.0, 0.0, 1.0, 1.0) + other.g0, vec4(self.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + other.g1);
}

MultiVector plane_multi_vector_sub(Plane self, MultiVector other) {
    return MultiVector(vec4(self.g0.x, self.g0.x, self.g0.z, self.g0.y) * vec4(0.0, 0.0, 1.0, 1.0) - other.g0, vec4(self.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) - other.g1);
}

MultiVector plane_multi_vector_geometric_product(Plane self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g1 * vec4(-1.0, -1.0, -1.0, 1.0) + vec4(self.g0.y) * other.g0.wzyx * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.z) * other.g0.zwxy, vec4(self.g0.x) * other.g0 * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g0.y) * other.g1.wzyx + vec4(self.g0.z) * other.g1.zwxy * vec4(-1.0, 1.0, -1.0, 1.0));
}

Scalar plane_multi_vector_scalar_product(Plane self, MultiVector other) {
    return Scalar(0.0 - self.g0.x * other.g1.x + self.g0.y * other.g0.w + self.g0.z * other.g0.z);
}

MotorDual plane_rotor_geometric_product(Plane self, Rotor other) {
    return MotorDual(vec4(self.g0.z) * vec4(other.g0.y, other.g0.y, other.g0.y, other.g0.x) * vec4(0.0, 0.0, 1.0, 1.0) + vec4(self.g0.x, self.g0.x, self.g0.y, self.g0.y) * vec4(other.g0.y, other.g0.x, other.g0.x, other.g0.y) * vec4(1.0, 1.0, 1.0, -1.0));
}

Scalar plane_rotor_regressive_product(Plane self, Rotor other) {
    return Scalar(self.g0.x * other.g0.y);
}

MotorDual plane_rotor_outer_product(Plane self, Rotor other) {
    return MotorDual(vec4(self.g0.x, self.g0.x, self.g0.y, self.g0.z) * vec4(other.g0.y, other.g0.x, other.g0.x, other.g0.x));
}

Plane plane_rotor_inner_product(Plane self, Rotor other) {
    return Plane(vec3(self.g0.z) * vec3(other.g0.y, other.g0.y, other.g0.x) * vec3(0.0, 1.0, 1.0) + self.g0.xyy * vec3(other.g0.x, other.g0.x, other.g0.y) * vec3(1.0, 1.0, -1.0));
}

Plane plane_rotor_right_contraction(Plane self, Rotor other) {
    return Plane(self.g0 * vec3(other.g0.x));
}

MotorDual plane_point_geometric_product(Plane self, Point other) {
    return MotorDual(vec4(self.g0.y) * vec4(other.g0.y, other.g0.z, other.g0.y, other.g0.x) * vec4(1.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z, other.g0.y, other.g0.x, other.g0.z) * vec4(1.0, -1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4(1.0, 0.0, 1.0, -1.0));
}

Scalar plane_point_regressive_product(Plane self, Point other) {
    return Scalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z);
}

Plane plane_point_inner_product(Plane self, Point other) {
    return Plane(vec3(self.g0.y) * other.g0.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * other.g0.yxy * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * other.g0.xzy * vec3(0.0, 1.0, -1.0));
}

Plane plane_point_left_contraction(Plane self, Point other) {
    return Plane(vec3(self.g0.y) * other.g0.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * other.g0.yxy * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * other.g0.xzy * vec3(0.0, 1.0, -1.0));
}

MotorDual plane_ideal_point_geometric_product(Plane self, IdealPoint other) {
    return MotorDual(vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g0.y) * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g0.y, self.g0.y, self.g0.x, self.g0.x) * vec4(other.g0.x, other.g0.y, other.g0.y, other.g0.x) * vec4(1.0, 1.0, 1.0, -1.0));
}

Scalar plane_ideal_point_regressive_product(Plane self, IdealPoint other) {
    return Scalar(self.g0.y * other.g0.x + self.g0.z * other.g0.y);
}

Plane plane_ideal_point_inner_product(Plane self, IdealPoint other) {
    return Plane(vec3(self.g0.z) * vec3(other.g0.x) * vec3(-1.0, 0.0, 0.0) + self.g0.yxx * vec3(other.g0.y, other.g0.y, other.g0.x) * vec3(1.0, 1.0, -1.0));
}

Plane plane_ideal_point_left_contraction(Plane self, IdealPoint other) {
    return Plane(vec3(self.g0.z) * vec3(other.g0.x) * vec3(-1.0, 0.0, 0.0) + self.g0.yxx * vec3(other.g0.y, other.g0.y, other.g0.x) * vec3(1.0, 1.0, -1.0));
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
    return Plane(vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(1.0, 1.0, 1.0) / vec3(other.g0.x, other.g0.y, other.g0.z) * vec3(1.0, 1.0, 1.0));
}

Motor plane_plane_geometric_product(Plane self, Plane other) {
    return Motor(vec4(self.g0.y) * vec4(other.g0.y, other.g0.z, other.g0.y, other.g0.x) * vec4(1.0, -1.0, 0.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.z, other.g0.y, other.g0.x, other.g0.z) * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0));
}

Point plane_plane_outer_product(Plane self, Plane other) {
    return Point(vec3(self.g0.y) * other.g0.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * other.g0.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x) * other.g0.xzy * vec3(0.0, 1.0, -1.0));
}

Scalar plane_plane_inner_product(Plane self, Plane other) {
    return Scalar(0.0 - self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z);
}

Scalar plane_plane_left_contraction(Plane self, Plane other) {
    return Scalar(0.0 - self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z);
}

Scalar plane_plane_right_contraction(Plane self, Plane other) {
    return Scalar(0.0 - self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z);
}

Scalar plane_plane_scalar_product(Plane self, Plane other) {
    return Scalar(0.0 - self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z);
}

MotorDual plane_translator_geometric_product(Plane self, Translator other) {
    return MotorDual(vec4(self.g0.y) * vec4(other.g0.y, other.g0.z, other.g0.x, other.g0.y) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.z) * vec4(other.g0.z, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, -1.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4(0.0, 1.0, 1.0, -1.0));
}

Scalar plane_translator_regressive_product(Plane self, Translator other) {
    return Scalar(self.g0.y * other.g0.y + self.g0.z * other.g0.z);
}

MotorDual plane_translator_outer_product(Plane self, Translator other) {
    return MotorDual(vec4(self.g0.z) * vec4(other.g0.z, other.g0.z, other.g0.z, other.g0.x) * vec4(1.0, 0.0, 0.0, 1.0) + vec4(self.g0.y, self.g0.x, self.g0.y, self.g0.x) * vec4(other.g0.y, other.g0.x, other.g0.x, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

Plane plane_translator_inner_product(Plane self, Translator other) {
    return Plane(vec3(self.g0.x) * other.g0.xzy * vec3(1.0, 1.0, -1.0) + vec3(self.g0.z) * other.g0.yyx * vec3(-1.0, 0.0, 1.0) + self.g0.yyx * other.g0.zxx * vec3(1.0, 1.0, 0.0));
}

Plane plane_translator_left_contraction(Plane self, Translator other) {
    return Plane(vec3(self.g0.z) * vec3(other.g0.y) * vec3(-1.0, 0.0, 0.0) + self.g0.yxx * other.g0.zzy * vec3(1.0, 1.0, -1.0));
}

Plane plane_translator_right_contraction(Plane self, Translator other) {
    return Plane(self.g0 * vec3(other.g0.x));
}

MotorDual plane_motor_geometric_product(Plane self, Motor other) {
    return MotorDual(vec4(self.g0.x) * other.g0.yxwz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g0.y) * other.g0.zwxy * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g0.wzyx * vec4(1.0, -1.0, 1.0, 1.0));
}

Scalar plane_motor_regressive_product(Plane self, Motor other) {
    return Scalar(self.g0.x * other.g0.y + self.g0.y * other.g0.z + self.g0.z * other.g0.w);
}

MotorDual plane_motor_outer_product(Plane self, Motor other) {
    return MotorDual(vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g0.z) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * other.g0.yxxx * vec4(1.0, 1.0, 0.0, 0.0));
}

Plane plane_motor_inner_product(Plane self, Motor other) {
    return Plane(vec3(self.g0.x) * vec3(other.g0.x, other.g0.w, other.g0.z) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.y) * vec3(other.g0.w, other.g0.x, other.g0.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.z) * vec3(other.g0.z, other.g0.y, other.g0.x) * vec3(-1.0, 1.0, 1.0));
}

Plane plane_motor_left_contraction(Plane self, Motor other) {
    return Plane(vec3(self.g0.y) * vec3(other.g0.w, other.g0.w, other.g0.y) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * vec3(other.g0.z, other.g0.y, other.g0.z) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * vec3(other.g0.x, other.g0.w, other.g0.z) * vec3(0.0, 1.0, -1.0));
}

Plane plane_motor_right_contraction(Plane self, Motor other) {
    return Plane(self.g0 * vec3(other.g0.x));
}

MotorDual plane_motor_dual_add(Plane self, MotorDual other) {
    return MotorDual(vec4(self.g0.x, self.g0.x, self.g0.y, self.g0.z) * vec4(0.0, 1.0, 1.0, 1.0) + other.g0);
}

MotorDual plane_motor_dual_sub(Plane self, MotorDual other) {
    return MotorDual(vec4(self.g0.x, self.g0.x, self.g0.y, self.g0.z) * vec4(0.0, 1.0, 1.0, 1.0) - other.g0);
}

Motor plane_motor_dual_geometric_product(Plane self, MotorDual other) {
    return Motor(vec4(self.g0.x) * other.g0.yxwz * vec4(-1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * other.g0.zwxy * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g0.z) * other.g0.wzyx * vec4(1.0, 1.0, -1.0, 1.0));
}

Plane plane_motor_dual_regressive_product(Plane self, MotorDual other) {
    return Plane(self.g0 * vec3(other.g0.x));
}

Point plane_motor_dual_outer_product(Plane self, MotorDual other) {
    return Point(vec3(self.g0.y) * vec3(other.g0.w, other.g0.w, other.g0.y) * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * vec3(other.g0.z, other.g0.y, other.g0.z) * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x) * vec3(other.g0.x, other.g0.w, other.g0.z) * vec3(0.0, 1.0, -1.0));
}

Motor plane_motor_dual_inner_product(Plane self, MotorDual other) {
    return Motor(vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g0.z) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * other.g0.yxxx * vec4(-1.0, -1.0, 0.0, 0.0));
}

Motor plane_motor_dual_left_contraction(Plane self, MotorDual other) {
    return Motor(vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g0.z) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * other.g0.yxxx * vec4(-1.0, -1.0, 0.0, 0.0));
}

Scalar plane_motor_dual_right_contraction(Plane self, MotorDual other) {
    return Scalar(0.0 - self.g0.x * other.g0.y + self.g0.y * other.g0.z + self.g0.z * other.g0.w);
}

Scalar plane_motor_dual_scalar_product(Plane self, MotorDual other) {
    return Scalar(0.0 - self.g0.x * other.g0.y + self.g0.y * other.g0.z + self.g0.z * other.g0.w);
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

Translator translator_zero() {
    return Translator(vec3(0.0));
}

Translator translator_one() {
    return Translator(vec3(1.0, 0.0, 0.0));
}

Translator translator_neg(Translator self) {
    return Translator(self.g0 * vec3(-1.0));
}

Translator translator_automorphism(Translator self) {
    return Translator(self.g0);
}

Translator translator_reversal(Translator self) {
    return Translator(self.g0 * vec3(1.0, -1.0, -1.0));
}

Translator translator_conjugation(Translator self) {
    return Translator(self.g0 * vec3(1.0, -1.0, -1.0));
}

Scalar translator_scalar_into(Translator self) {
    return Scalar(self.g0.x);
}

Translator translator_scalar_add(Translator self, Scalar other) {
    return Translator(self.g0 + vec3(other.g0) * vec3(1.0, 0.0, 0.0));
}

Translator translator_scalar_sub(Translator self, Scalar other) {
    return Translator(self.g0 - vec3(other.g0) * vec3(1.0, 0.0, 0.0));
}

Translator translator_scalar_geometric_product(Translator self, Scalar other) {
    return Translator(self.g0 * vec3(other.g0));
}

Translator translator_scalar_outer_product(Translator self, Scalar other) {
    return Translator(self.g0 * vec3(other.g0));
}

Translator translator_scalar_inner_product(Translator self, Scalar other) {
    return Translator(self.g0 * vec3(other.g0));
}

Scalar translator_scalar_left_contraction(Translator self, Scalar other) {
    return Scalar(self.g0.x * other.g0);
}

Translator translator_scalar_right_contraction(Translator self, Scalar other) {
    return Translator(self.g0 * vec3(other.g0));
}

Scalar translator_scalar_scalar_product(Translator self, Scalar other) {
    return Scalar(self.g0.x * other.g0);
}

MultiVector translator_multi_vector_add(Translator self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + other.g0, vec4(self.g0.x, self.g0.x, self.g0.y, self.g0.z) * vec4(0.0, 0.0, 1.0, 1.0) + other.g1);
}

MultiVector translator_multi_vector_sub(Translator self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) - other.g0, vec4(self.g0.x, self.g0.x, self.g0.y, self.g0.z) * vec4(0.0, 0.0, 1.0, 1.0) - other.g1);
}

MultiVector translator_multi_vector_geometric_product(Translator self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g0 + vec4(self.g0.y) * other.g1.zwxy * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g0.z) * other.g1.wzyx * vec4(1.0, 1.0, 1.0, -1.0), vec4(self.g0.x) * other.g1 + vec4(self.g0.y) * other.g0.zwxy * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g0.wzyx * vec4(-1.0, 1.0, 1.0, 1.0));
}

MultiVector translator_multi_vector_outer_product(Translator self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g0, vec4(self.g0.x) * other.g1 + vec4(self.g0.z) * other.g0.zzzx * vec4(0.0, 1.0, 0.0, 1.0) + vec4(self.g0.x, self.g0.y, self.g0.y, self.g0.x) * other.g0.xwxx * vec4(0.0, 1.0, 1.0, 0.0));
}

MultiVector translator_multi_vector_inner_product(Translator self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * other.g1.wwyx * vec4(1.0, 0.0, 1.0, -1.0) + vec4(self.g0.y, self.g0.x, self.g0.y, self.g0.y) * other.g1.zxxy * vec4(1.0, 0.0, 1.0, 1.0), vec4(self.g0.x) * other.g1 + vec4(self.g0.z) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + vec4(self.g0.y, self.g0.x, self.g0.y, self.g0.x) * other.g0.zxxx * vec4(1.0, 0.0, 1.0, 0.0));
}

MultiVector translator_multi_vector_left_contraction(Translator self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * other.g1.wwyw * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g0.y, self.g0.x, self.g0.x, self.g0.y) * other.g1.zxxy * vec4(1.0, 0.0, 0.0, 1.0), vec4(self.g0.x) * other.g1);
}

Scalar translator_multi_vector_scalar_product(Translator self, MultiVector other) {
    return Scalar(self.g0.x * other.g0.x + self.g0.y * other.g1.z + self.g0.z * other.g1.w);
}

Motor translator_rotor_add(Translator self, Rotor other) {
    return Motor(vec4(self.g0.x, self.g0.x, self.g0.y, self.g0.z) * vec4(1.0, 0.0, 1.0, 1.0) + vec4(other.g0.x, other.g0.y, other.g0.x, other.g0.x) * vec4(1.0, 1.0, 0.0, 0.0));
}

Motor translator_rotor_sub(Translator self, Rotor other) {
    return Motor(vec4(self.g0.x, self.g0.x, self.g0.y, self.g0.z) * vec4(1.0, 0.0, 1.0, 1.0) - vec4(other.g0.x, other.g0.y, other.g0.x, other.g0.x) * vec4(1.0, 1.0, 0.0, 0.0));
}

Motor translator_rotor_geometric_product(Translator self, Rotor other) {
    return Motor(vec4(self.g0.z) * vec4(other.g0.y, other.g0.y, other.g0.y, other.g0.x) * vec4(0.0, 0.0, 1.0, 1.0) + vec4(self.g0.x, self.g0.x, self.g0.y, self.g0.y) * vec4(other.g0.x, other.g0.y, other.g0.x, other.g0.y) * vec4(1.0, 1.0, 1.0, -1.0));
}

Motor translator_rotor_outer_product(Translator self, Rotor other) {
    return Motor(vec4(self.g0.x, self.g0.x, self.g0.y, self.g0.z) * vec4(other.g0.x, other.g0.y, other.g0.x, other.g0.x));
}

Motor translator_rotor_inner_product(Translator self, Rotor other) {
    return Motor(vec4(self.g0.x, self.g0.x, self.g0.y, self.g0.z) * vec4(other.g0.x, other.g0.y, other.g0.x, other.g0.x));
}

Rotor translator_rotor_left_contraction(Translator self, Rotor other) {
    return Rotor(vec2(self.g0.x) * other.g0);
}

Translator translator_rotor_right_contraction(Translator self, Rotor other) {
    return Translator(self.g0 * vec3(other.g0.x));
}

Scalar translator_rotor_scalar_product(Translator self, Rotor other) {
    return Scalar(self.g0.x * other.g0.x);
}

Motor translator_point_add(Translator self, Point other) {
    return Motor(vec4(self.g0.x, self.g0.x, self.g0.y, self.g0.z) * vec4(1.0, 0.0, 1.0, 1.0) + vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0));
}

Motor translator_point_sub(Translator self, Point other) {
    return Motor(vec4(self.g0.x, self.g0.x, self.g0.y, self.g0.z) * vec4(1.0, 0.0, 1.0, 1.0) - vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0));
}

Motor translator_point_geometric_product(Translator self, Point other) {
    return Motor(vec4(self.g0.y) * vec4(other.g0.y, other.g0.z, other.g0.y, other.g0.x) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z, other.g0.y, other.g0.x, other.g0.z) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0));
}

Plane translator_point_regressive_product(Translator self, Point other) {
    return Plane(vec3(self.g0.z) * other.g0.yxy * vec3(1.0, -1.0, 0.0) + self.g0.yxy * other.g0.zxx * vec3(-1.0, 0.0, 1.0));
}

Point translator_point_outer_product(Translator self, Point other) {
    return Point(vec3(self.g0.x) * other.g0);
}

Motor translator_point_inner_product(Translator self, Point other) {
    return Motor(vec4(self.g0.z) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.y, self.g0.x, self.g0.x, self.g0.x) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g0.z));
}

Motor translator_point_left_contraction(Translator self, Point other) {
    return Motor(vec4(self.g0.z) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.y, self.g0.x, self.g0.x, self.g0.x) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g0.z));
}

Scalar translator_point_right_contraction(Translator self, Point other) {
    return Scalar(self.g0.y * other.g0.y + self.g0.z * other.g0.z);
}

Scalar translator_point_scalar_product(Translator self, Point other) {
    return Scalar(self.g0.y * other.g0.y + self.g0.z * other.g0.z);
}

IdealPoint translator_ideal_point_into(Translator self) {
    return IdealPoint(vec2(self.g0.y, self.g0.z));
}

Translator translator_ideal_point_add(Translator self, IdealPoint other) {
    return Translator(self.g0 + vec3(other.g0.x, other.g0.x, other.g0.y) * vec3(0.0, 1.0, 1.0));
}

Translator translator_ideal_point_sub(Translator self, IdealPoint other) {
    return Translator(self.g0 - vec3(other.g0.x, other.g0.x, other.g0.y) * vec3(0.0, 1.0, 1.0));
}

Motor translator_ideal_point_geometric_product(Translator self, IdealPoint other) {
    return Motor(vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g0.y) * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g0.y, self.g0.y, self.g0.x, self.g0.x) * vec4(other.g0.x, other.g0.y, other.g0.x, other.g0.y) * vec4(1.0, -1.0, 1.0, 1.0));
}

IdealPoint translator_ideal_point_outer_product(Translator self, IdealPoint other) {
    return IdealPoint(vec2(self.g0.x) * other.g0);
}

Translator translator_ideal_point_inner_product(Translator self, IdealPoint other) {
    return Translator(vec3(self.g0.z) * vec3(other.g0.y) * vec3(1.0, 0.0, 0.0) + self.g0.yxx * vec3(other.g0.x, other.g0.x, other.g0.y));
}

Translator translator_ideal_point_left_contraction(Translator self, IdealPoint other) {
    return Translator(vec3(self.g0.z) * vec3(other.g0.y) * vec3(1.0, 0.0, 0.0) + self.g0.yxx * vec3(other.g0.x, other.g0.x, other.g0.y));
}

Scalar translator_ideal_point_right_contraction(Translator self, IdealPoint other) {
    return Scalar(self.g0.y * other.g0.x + self.g0.z * other.g0.y);
}

Scalar translator_ideal_point_scalar_product(Translator self, IdealPoint other) {
    return Scalar(self.g0.y * other.g0.x + self.g0.z * other.g0.y);
}

MotorDual translator_plane_geometric_product(Translator self, Plane other) {
    return MotorDual(vec4(self.g0.y) * vec4(other.g0.y, other.g0.z, other.g0.y, other.g0.x) * vec4(1.0, 1.0, 0.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.z, other.g0.y, other.g0.x, other.g0.z) * vec4(1.0, -1.0, -1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0));
}

Scalar translator_plane_regressive_product(Translator self, Plane other) {
    return Scalar(self.g0.y * other.g0.y + self.g0.z * other.g0.z);
}

MotorDual translator_plane_outer_product(Translator self, Plane other) {
    return MotorDual(vec4(self.g0.z) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.y, self.g0.x, self.g0.x, self.g0.x) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g0.z));
}

Plane translator_plane_inner_product(Translator self, Plane other) {
    return Plane(vec3(self.g0.x) * other.g0 + vec3(self.g0.z) * other.g0.yxy * vec3(-1.0, -1.0, 0.0) + self.g0.yxy * other.g0.zxx * vec3(1.0, 0.0, 1.0));
}

Plane translator_plane_left_contraction(Translator self, Plane other) {
    return Plane(vec3(self.g0.x) * other.g0);
}

Plane translator_plane_right_contraction(Translator self, Plane other) {
    return Plane(vec3(self.g0.z) * other.g0.yxy * vec3(-1.0, -1.0, 0.0) + self.g0.yxy * other.g0.zxx * vec3(1.0, 0.0, 1.0));
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
    return Translator(vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(1.0, 1.0, 1.0) / vec3(other.g0.x, other.g0.y, other.g0.z) * vec3(1.0, 1.0, 1.0));
}

Motor translator_translator_geometric_product(Translator self, Translator other) {
    return Motor(vec4(self.g0.y) * vec4(other.g0.y, other.g0.z, other.g0.x, other.g0.y) * vec4(1.0, -1.0, 1.0, 0.0) + vec4(self.g0.z) * vec4(other.g0.z, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(1.0, 0.0, 1.0, 1.0));
}

Translator translator_translator_outer_product(Translator self, Translator other) {
    return Translator(vec3(self.g0.x) * other.g0 + self.g0 * vec3(other.g0.x) * vec3(0.0, 1.0, 1.0));
}

Translator translator_translator_inner_product(Translator self, Translator other) {
    return Translator(vec3(self.g0.x) * other.g0 + vec3(self.g0.z) * other.g0.zzx * vec3(1.0, 0.0, 1.0) + self.g0.yyx * other.g0.yxx * vec3(1.0, 1.0, 0.0));
}

Translator translator_translator_left_contraction(Translator self, Translator other) {
    return Translator(vec3(self.g0.x) * other.g0 + vec3(self.g0.z) * vec3(other.g0.z) * vec3(1.0, 0.0, 0.0) + self.g0.yxx * other.g0.yxx * vec3(1.0, 0.0, 0.0));
}

Translator translator_translator_right_contraction(Translator self, Translator other) {
    return Translator(vec3(self.g0.y) * other.g0.yxy * vec3(1.0, 1.0, 0.0) + vec3(self.g0.z) * other.g0.zzx * vec3(1.0, 0.0, 1.0) + vec3(self.g0.x) * vec3(other.g0.x) * vec3(1.0, 0.0, 0.0));
}

Scalar translator_translator_scalar_product(Translator self, Translator other) {
    return Scalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z);
}

Motor translator_motor_add(Translator self, Motor other) {
    return Motor(vec4(self.g0.x, self.g0.x, self.g0.y, self.g0.z) * vec4(1.0, 0.0, 1.0, 1.0) + other.g0);
}

Motor translator_motor_sub(Translator self, Motor other) {
    return Motor(vec4(self.g0.x, self.g0.x, self.g0.y, self.g0.z) * vec4(1.0, 0.0, 1.0, 1.0) - other.g0);
}

Motor translator_motor_geometric_product(Translator self, Motor other) {
    return Motor(vec4(self.g0.x) * other.g0 + vec4(self.g0.y) * other.g0.zwxy * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g0.wzyx);
}

Plane translator_motor_regressive_product(Translator self, Motor other) {
    return Plane(vec3(self.g0.z) * vec3(other.g0.z, other.g0.y, other.g0.z) * vec3(1.0, -1.0, 0.0) + self.g0.yxy * vec3(other.g0.w, other.g0.x, other.g0.y) * vec3(-1.0, 0.0, 1.0));
}

Motor translator_motor_outer_product(Translator self, Motor other) {
    return Motor(vec4(self.g0.x) * other.g0 + vec4(self.g0.x, self.g0.x, self.g0.y, self.g0.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 1.0));
}

Motor translator_motor_inner_product(Translator self, Motor other) {
    return Motor(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + vec4(self.g0.y, self.g0.x, self.g0.y, self.g0.x) * other.g0.zxxx * vec4(1.0, 0.0, 1.0, 0.0));
}

Motor translator_motor_left_contraction(Translator self, Motor other) {
    return Motor(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.y, self.g0.x, self.g0.x, self.g0.x) * other.g0.zxxx * vec4(1.0, 0.0, 0.0, 0.0));
}

Translator translator_motor_right_contraction(Translator self, Motor other) {
    return Translator(vec3(self.g0.y) * vec3(other.g0.z, other.g0.x, other.g0.z) * vec3(1.0, 1.0, 0.0) + vec3(self.g0.z) * vec3(other.g0.w, other.g0.w, other.g0.x) * vec3(1.0, 0.0, 1.0) + vec3(self.g0.x) * vec3(other.g0.x) * vec3(1.0, 0.0, 0.0));
}

Scalar translator_motor_scalar_product(Translator self, Motor other) {
    return Scalar(self.g0.x * other.g0.x + self.g0.y * other.g0.z + self.g0.z * other.g0.w);
}

MotorDual translator_motor_dual_geometric_product(Translator self, MotorDual other) {
    return MotorDual(vec4(self.g0.x) * other.g0 + vec4(self.g0.y) * other.g0.zwxy + vec4(self.g0.z) * other.g0.wzyx * vec4(1.0, -1.0, -1.0, 1.0));
}

Translator translator_motor_dual_regressive_product(Translator self, MotorDual other) {
    return Translator(vec3(self.g0.y) * vec3(other.g0.z, other.g0.x, other.g0.z) * vec3(1.0, 1.0, 0.0) + vec3(self.g0.z) * vec3(other.g0.w, other.g0.w, other.g0.x) * vec3(1.0, 0.0, 1.0) + vec3(self.g0.x) * vec3(other.g0.x) * vec3(1.0, 0.0, 0.0));
}

MotorDual translator_motor_dual_outer_product(Translator self, MotorDual other) {
    return MotorDual(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.y, self.g0.x, self.g0.x, self.g0.x) * other.g0.zxxx * vec4(1.0, 0.0, 0.0, 0.0));
}

MotorDual translator_motor_dual_inner_product(Translator self, MotorDual other) {
    return MotorDual(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * other.g0.zzyx * vec4(0.0, -1.0, -1.0, 1.0) + vec4(self.g0.x, self.g0.y, self.g0.y, self.g0.y) * other.g0.xwxy * vec4(0.0, 1.0, 1.0, 1.0));
}

MotorDual translator_motor_dual_left_contraction(Translator self, MotorDual other) {
    return MotorDual(vec4(self.g0.x) * other.g0 + vec4(self.g0.x, self.g0.x, self.g0.y, self.g0.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 1.0));
}

Plane translator_motor_dual_right_contraction(Translator self, MotorDual other) {
    return Plane(vec3(self.g0.z) * vec3(other.g0.z, other.g0.y, other.g0.z) * vec3(-1.0, -1.0, 0.0) + self.g0.yxy * vec3(other.g0.w, other.g0.x, other.g0.y) * vec3(1.0, 0.0, 1.0));
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
    return Motor(vec4(0.0));
}

Motor motor_one() {
    return Motor(vec4(1.0, 0.0, 0.0, 0.0));
}

Motor motor_neg(Motor self) {
    return Motor(self.g0 * vec4(-1.0));
}

Motor motor_automorphism(Motor self) {
    return Motor(self.g0);
}

Motor motor_reversal(Motor self) {
    return Motor(self.g0 * vec4(1.0, -1.0, -1.0, -1.0));
}

Motor motor_conjugation(Motor self) {
    return Motor(self.g0 * vec4(1.0, -1.0, -1.0, -1.0));
}

MotorDual motor_dual(Motor self) {
    return MotorDual(self.g0);
}

Scalar motor_scalar_into(Motor self) {
    return Scalar(self.g0.x);
}

Motor motor_scalar_add(Motor self, Scalar other) {
    return Motor(self.g0 + vec4(other.g0) * vec4(1.0, 0.0, 0.0, 0.0));
}

Motor motor_scalar_sub(Motor self, Scalar other) {
    return Motor(self.g0 - vec4(other.g0) * vec4(1.0, 0.0, 0.0, 0.0));
}

Motor motor_scalar_geometric_product(Motor self, Scalar other) {
    return Motor(self.g0 * vec4(other.g0));
}

Motor motor_scalar_outer_product(Motor self, Scalar other) {
    return Motor(self.g0 * vec4(other.g0));
}

Motor motor_scalar_inner_product(Motor self, Scalar other) {
    return Motor(self.g0 * vec4(other.g0));
}

Scalar motor_scalar_left_contraction(Motor self, Scalar other) {
    return Scalar(self.g0.x * other.g0);
}

Motor motor_scalar_right_contraction(Motor self, Scalar other) {
    return Motor(self.g0 * vec4(other.g0));
}

Scalar motor_scalar_scalar_product(Motor self, Scalar other) {
    return Scalar(self.g0.x * other.g0);
}

MultiVector motor_multi_vector_add(Motor self, MultiVector other) {
    return MultiVector(self.g0.xyxx * vec4(1.0, 1.0, 0.0, 0.0) + other.g0, self.g0.xxzw * vec4(0.0, 0.0, 1.0, 1.0) + other.g1);
}

MultiVector motor_multi_vector_sub(Motor self, MultiVector other) {
    return MultiVector(self.g0.xyxx * vec4(1.0, 1.0, 0.0, 0.0) - other.g0, self.g0.xxzw * vec4(0.0, 0.0, 1.0, 1.0) - other.g1);
}

MultiVector motor_multi_vector_geometric_product(Motor self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g0 + vec4(self.g0.y) * other.g0.yxwz * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g1.zwxy * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g0.w) * other.g1.wzyx * vec4(1.0, 1.0, 1.0, -1.0), vec4(self.g0.x) * other.g1 + vec4(self.g0.y) * other.g1.yxwz * vec4(-1.0, 1.0, -1.0, 1.0) + vec4(self.g0.z) * other.g0.zwxy * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g0.w) * other.g0.wzyx * vec4(-1.0, 1.0, 1.0, 1.0));
}

MultiVector motor_multi_vector_outer_product(Motor self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g0 + self.g0.xyxx * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0), vec4(self.g0.x) * other.g1 + vec4(self.g0.z) * other.g0.wwxw * vec4(0.0, 1.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0.zzzx * vec4(0.0, 1.0, 0.0, 1.0) + self.g0.xyxx * vec4(other.g1.x) * vec4(0.0, 1.0, 0.0, 0.0));
}

MultiVector motor_multi_vector_inner_product(Motor self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g0 + vec4(self.g0.y) * other.g0.yxwz * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.w) * other.g1.wwyx * vec4(1.0, 0.0, 1.0, -1.0) + self.g0.zxzz * other.g1.zxxy * vec4(1.0, 0.0, 1.0, 1.0), vec4(self.g0.x) * other.g1 + vec4(self.g0.z) * other.g0.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + self.g0.yxxx * other.g1.yxxx * vec4(-1.0, 0.0, 0.0, 0.0));
}

MultiVector motor_multi_vector_left_contraction(Motor self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * other.g1.zzzy * vec4(1.0, 0.0, 0.0, 1.0) + vec4(self.g0.w) * other.g1.wwyw * vec4(1.0, 0.0, 1.0, 0.0) + self.g0.yxxx * other.g0.yxxx * vec4(-1.0, 0.0, 0.0, 0.0), vec4(self.g0.x) * other.g1 + self.g0.yxxx * other.g1.yxxx * vec4(-1.0, 0.0, 0.0, 0.0));
}

Scalar motor_multi_vector_scalar_product(Motor self, MultiVector other) {
    return Scalar(self.g0.x * other.g0.x - self.g0.y * other.g0.y + self.g0.z * other.g1.z + self.g0.w * other.g1.w);
}

Rotor motor_rotor_into(Motor self) {
    return Rotor(vec2(self.g0.x, self.g0.y));
}

Motor motor_rotor_add(Motor self, Rotor other) {
    return Motor(self.g0 + vec4(other.g0.x, other.g0.y, other.g0.x, other.g0.x) * vec4(1.0, 1.0, 0.0, 0.0));
}

Motor motor_rotor_sub(Motor self, Rotor other) {
    return Motor(self.g0 - vec4(other.g0.x, other.g0.y, other.g0.x, other.g0.x) * vec4(1.0, 1.0, 0.0, 0.0));
}

Motor motor_rotor_geometric_product(Motor self, Rotor other) {
    return Motor(vec4(self.g0.y) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g0.y) * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.y, other.g0.y, other.g0.y, other.g0.x) * vec4(0.0, 0.0, 1.0, 1.0) + self.g0.xxzz * vec4(other.g0.x, other.g0.y, other.g0.x, other.g0.y) * vec4(1.0, 1.0, 1.0, -1.0));
}

Motor motor_rotor_outer_product(Motor self, Rotor other) {
    return Motor(vec4(self.g0.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + self.g0.xxzw * vec4(other.g0.x, other.g0.y, other.g0.x, other.g0.x));
}

Motor motor_rotor_inner_product(Motor self, Rotor other) {
    return Motor(vec4(self.g0.y) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g0.y) * vec4(-1.0, 1.0, 0.0, 0.0) + self.g0.xxzw * vec4(other.g0.x, other.g0.y, other.g0.x, other.g0.x));
}

Rotor motor_rotor_left_contraction(Motor self, Rotor other) {
    return Rotor(vec2(self.g0.x) * other.g0 + vec2(self.g0.y, self.g0.x) * other.g0.yx * vec2(-1.0, 0.0));
}

Motor motor_rotor_right_contraction(Motor self, Rotor other) {
    return Motor(vec4(self.g0.y) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g0.y) * vec4(-1.0, 1.0, 0.0, 0.0) + self.g0.xxzw * vec4(other.g0.x) * vec4(1.0, 0.0, 1.0, 1.0));
}

Scalar motor_rotor_scalar_product(Motor self, Rotor other) {
    return Scalar(self.g0.x * other.g0.x - self.g0.y * other.g0.y);
}

Point motor_point_into(Motor self) {
    return Point(vec3(self.g0.y, self.g0.z, self.g0.w));
}

Motor motor_point_add(Motor self, Point other) {
    return Motor(self.g0 + vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0));
}

Motor motor_point_sub(Motor self, Point other) {
    return Motor(self.g0 - vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0));
}

Motor motor_point_geometric_product(Motor self, Point other) {
    return Motor(vec4(self.g0.y) * vec4(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4(-1.0, 0.0, -1.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.z, other.g0.y, other.g0.x) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g0.z, other.g0.y, other.g0.x, other.g0.z) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0));
}

Plane motor_point_regressive_product(Motor self, Point other) {
    return Plane(vec3(self.g0.z) * other.g0.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.w) * other.g0.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x, self.g0.y, self.g0.y) * other.g0.xzy * vec3(0.0, 1.0, -1.0));
}

Point motor_point_outer_product(Motor self, Point other) {
    return Point(vec3(self.g0.x) * other.g0);
}

Motor motor_point_inner_product(Motor self, Point other) {
    return Motor(vec4(self.g0.z) * vec4(other.g0.y) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(-1.0, 1.0, 1.0, 1.0));
}

Motor motor_point_left_contraction(Motor self, Point other) {
    return Motor(vec4(self.g0.z) * vec4(other.g0.y) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(-1.0, 1.0, 1.0, 1.0));
}

Scalar motor_point_right_contraction(Motor self, Point other) {
    return Scalar(0.0 - self.g0.y * other.g0.x + self.g0.z * other.g0.y + self.g0.w * other.g0.z);
}

Scalar motor_point_scalar_product(Motor self, Point other) {
    return Scalar(0.0 - self.g0.y * other.g0.x + self.g0.z * other.g0.y + self.g0.w * other.g0.z);
}

IdealPoint motor_ideal_point_into(Motor self) {
    return IdealPoint(vec2(self.g0.z, self.g0.w));
}

Motor motor_ideal_point_add(Motor self, IdealPoint other) {
    return Motor(self.g0 + vec4(other.g0.x, other.g0.x, other.g0.x, other.g0.y) * vec4(0.0, 0.0, 1.0, 1.0));
}

Motor motor_ideal_point_sub(Motor self, IdealPoint other) {
    return Motor(self.g0 - vec4(other.g0.x, other.g0.x, other.g0.x, other.g0.y) * vec4(0.0, 0.0, 1.0, 1.0));
}

Motor motor_ideal_point_geometric_product(Motor self, IdealPoint other) {
    return Motor(vec4(self.g0.y) * vec4(other.g0.y, other.g0.y, other.g0.y, other.g0.x) * vec4(0.0, 0.0, -1.0, 1.0) + vec4(self.g0.w) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g0.y) * vec4(1.0, 1.0, 0.0, 0.0) + self.g0.zzxx * vec4(other.g0.x, other.g0.y, other.g0.x, other.g0.y) * vec4(1.0, -1.0, 1.0, 1.0));
}

Plane motor_ideal_point_regressive_product(Motor self, IdealPoint other) {
    return Plane(vec3(self.g0.w) * vec3(other.g0.x) * vec3(1.0, 0.0, 0.0) + vec3(self.g0.z, self.g0.y, self.g0.y) * vec3(other.g0.y, other.g0.y, other.g0.x) * vec3(-1.0, 1.0, -1.0));
}

IdealPoint motor_ideal_point_outer_product(Motor self, IdealPoint other) {
    return IdealPoint(vec2(self.g0.x) * other.g0);
}

Translator motor_ideal_point_inner_product(Motor self, IdealPoint other) {
    return Translator(vec3(self.g0.w) * vec3(other.g0.y) * vec3(1.0, 0.0, 0.0) + vec3(self.g0.z, self.g0.x, self.g0.x) * vec3(other.g0.x, other.g0.x, other.g0.y));
}

Translator motor_ideal_point_left_contraction(Motor self, IdealPoint other) {
    return Translator(vec3(self.g0.w) * vec3(other.g0.y) * vec3(1.0, 0.0, 0.0) + vec3(self.g0.z, self.g0.x, self.g0.x) * vec3(other.g0.x, other.g0.x, other.g0.y));
}

Scalar motor_ideal_point_right_contraction(Motor self, IdealPoint other) {
    return Scalar(self.g0.z * other.g0.x + self.g0.w * other.g0.y);
}

Scalar motor_ideal_point_scalar_product(Motor self, IdealPoint other) {
    return Scalar(self.g0.z * other.g0.x + self.g0.w * other.g0.y);
}

MotorDual motor_plane_geometric_product(Motor self, Plane other) {
    return MotorDual(vec4(self.g0.y) * vec4(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4(1.0, 0.0, -1.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.z, other.g0.y, other.g0.x) * vec4(1.0, 1.0, 0.0, 1.0) + vec4(self.g0.w) * vec4(other.g0.z, other.g0.y, other.g0.x, other.g0.z) * vec4(1.0, -1.0, -1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0));
}

Scalar motor_plane_regressive_product(Motor self, Plane other) {
    return Scalar(self.g0.y * other.g0.x + self.g0.z * other.g0.y + self.g0.w * other.g0.z);
}

MotorDual motor_plane_outer_product(Motor self, Plane other) {
    return MotorDual(vec4(self.g0.z) * vec4(other.g0.y) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z));
}

Plane motor_plane_inner_product(Motor self, Plane other) {
    return Plane(vec3(self.g0.x) * other.g0 + vec3(self.g0.z) * other.g0.zzx * vec3(1.0, 0.0, 1.0) + vec3(self.g0.w) * other.g0.yxy * vec3(-1.0, -1.0, 0.0) + vec3(self.g0.x, self.g0.y, self.g0.y) * other.g0.xzy * vec3(0.0, -1.0, 1.0));
}

Plane motor_plane_left_contraction(Motor self, Plane other) {
    return Plane(vec3(self.g0.x) * other.g0);
}

Plane motor_plane_right_contraction(Motor self, Plane other) {
    return Plane(vec3(self.g0.z) * other.g0.zzx * vec3(1.0, 0.0, 1.0) + vec3(self.g0.w) * other.g0.yxy * vec3(-1.0, -1.0, 0.0) + vec3(self.g0.x, self.g0.y, self.g0.y) * other.g0.xzy * vec3(0.0, -1.0, 1.0));
}

Translator motor_translator_into(Motor self) {
    return Translator(vec3(self.g0.x, self.g0.z, self.g0.w));
}

Motor motor_translator_add(Motor self, Translator other) {
    return Motor(self.g0 + vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(1.0, 0.0, 1.0, 1.0));
}

Motor motor_translator_sub(Motor self, Translator other) {
    return Motor(self.g0 - vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(1.0, 0.0, 1.0, 1.0));
}

Motor motor_translator_geometric_product(Motor self, Translator other) {
    return Motor(vec4(self.g0.y) * vec4(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4(0.0, 1.0, -1.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.z, other.g0.x, other.g0.y) * vec4(1.0, -1.0, 1.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.z, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(1.0, 0.0, 1.0, 1.0));
}

Plane motor_translator_regressive_product(Motor self, Translator other) {
    return Plane(vec3(self.g0.w) * vec3(other.g0.y) * vec3(1.0, 0.0, 0.0) + vec3(self.g0.z, self.g0.y, self.g0.y) * other.g0.zzy * vec3(-1.0, 1.0, -1.0));
}

Motor motor_translator_outer_product(Motor self, Translator other) {
    return Motor(vec4(self.g0.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + self.g0.xyxx * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z));
}

Motor motor_translator_inner_product(Motor self, Translator other) {
    return Motor(vec4(self.g0.z) * vec4(other.g0.y, other.g0.y, other.g0.x, other.g0.y) * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.z, other.g0.z, other.g0.z, other.g0.x) * vec4(1.0, 0.0, 0.0, 1.0) + self.g0.xyxx * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z));
}

Translator motor_translator_left_contraction(Motor self, Translator other) {
    return Translator(vec3(self.g0.x) * other.g0 + vec3(self.g0.w) * vec3(other.g0.z) * vec3(1.0, 0.0, 0.0) + vec3(self.g0.z, self.g0.x, self.g0.x) * other.g0.yxx * vec3(1.0, 0.0, 0.0));
}

Motor motor_translator_right_contraction(Motor self, Translator other) {
    return Motor(vec4(self.g0.z) * vec4(other.g0.y, other.g0.y, other.g0.x, other.g0.y) * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.z, other.g0.z, other.g0.z, other.g0.x) * vec4(1.0, 0.0, 0.0, 1.0) + self.g0.xyxx * vec4(other.g0.x) * vec4(1.0, 1.0, 0.0, 0.0));
}

Scalar motor_translator_scalar_product(Motor self, Translator other) {
    return Scalar(self.g0.x * other.g0.x + self.g0.z * other.g0.y + self.g0.w * other.g0.z);
}

Motor motor_motor_add(Motor self, Motor other) {
    return Motor(self.g0 + other.g0);
}

Motor motor_motor_sub(Motor self, Motor other) {
    return Motor(self.g0 - other.g0);
}

Motor motor_motor_mul(Motor self, Motor other) {
    return Motor(self.g0 * other.g0);
}

Motor motor_motor_div(Motor self, Motor other) {
    return Motor(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.w) * vec4(1.0, 1.0, 1.0, 1.0) / vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.w) * vec4(1.0, 1.0, 1.0, 1.0));
}

Motor motor_motor_geometric_product(Motor self, Motor other) {
    return Motor(vec4(self.g0.x) * other.g0 + vec4(self.g0.y) * other.g0.yxwz * vec4(-1.0, 1.0, -1.0, 1.0) + vec4(self.g0.z) * other.g0.zwxy * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.w) * other.g0.wzyx);
}

Plane motor_motor_regressive_product(Motor self, Motor other) {
    return Plane(vec3(self.g0.z) * vec3(other.g0.w, other.g0.w, other.g0.y) * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.w) * vec3(other.g0.z, other.g0.y, other.g0.z) * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x, self.g0.y, self.g0.y) * vec3(other.g0.x, other.g0.w, other.g0.z) * vec3(0.0, 1.0, -1.0));
}

Motor motor_motor_outer_product(Motor self, Motor other) {
    return Motor(vec4(self.g0.x) * other.g0 + self.g0 * vec4(other.g0.x) * vec4(0.0, 1.0, 1.0, 1.0));
}

Motor motor_motor_inner_product(Motor self, Motor other) {
    return Motor(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * other.g0.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + self.g0.yyxx * other.g0.yxxx * vec4(-1.0, 1.0, 0.0, 0.0));
}

Motor motor_motor_left_contraction(Motor self, Motor other) {
    return Motor(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * other.g0.yxxx * vec4(-1.0, 0.0, 0.0, 0.0));
}

Motor motor_motor_right_contraction(Motor self, Motor other) {
    return Motor(vec4(self.g0.y) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.z) * other.g0.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0));
}

Scalar motor_motor_scalar_product(Motor self, Motor other) {
    return Scalar(self.g0.x * other.g0.x - self.g0.y * other.g0.y + self.g0.z * other.g0.z + self.g0.w * other.g0.w);
}

MultiVector motor_motor_dual_add(Motor self, MotorDual other) {
    return MultiVector(self.g0.xyxx * vec4(1.0, 1.0, 0.0, 0.0) + other.g0.xxwz * vec4(0.0, 0.0, 1.0, 1.0), self.g0.xxzw * vec4(0.0, 0.0, 1.0, 1.0) + other.g0.yxxx * vec4(1.0, 1.0, 0.0, 0.0));
}

MultiVector motor_motor_dual_sub(Motor self, MotorDual other) {
    return MultiVector(self.g0.xyxx * vec4(1.0, 1.0, 0.0, 0.0) - other.g0.xxwz * vec4(0.0, 0.0, 1.0, 1.0), self.g0.xxzw * vec4(0.0, 0.0, 1.0, 1.0) - other.g0.yxxx * vec4(1.0, 1.0, 0.0, 0.0));
}

MotorDual motor_motor_dual_geometric_product(Motor self, MotorDual other) {
    return MotorDual(vec4(self.g0.x) * other.g0 + vec4(self.g0.y) * other.g0.yxwz * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.z) * other.g0.zwxy + vec4(self.g0.w) * other.g0.wzyx * vec4(1.0, -1.0, -1.0, 1.0));
}

Motor motor_motor_dual_regressive_product(Motor self, MotorDual other) {
    return Motor(vec4(self.g0.y) * other.g0.yxyy * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g0.z) * other.g0.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0));
}

MotorDual motor_motor_dual_outer_product(Motor self, MotorDual other) {
    return MotorDual(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * other.g0.yxxx * vec4(1.0, 0.0, 0.0, 0.0));
}

MotorDual motor_motor_dual_inner_product(Motor self, MotorDual other) {
    return MotorDual(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * other.g0.wwxy * vec4(0.0, 1.0, 1.0, 1.0) + vec4(self.g0.w) * other.g0.zzyx * vec4(0.0, -1.0, -1.0, 1.0) + self.g0.xyyy * other.g0.xxwz * vec4(0.0, -1.0, -1.0, 1.0));
}

MotorDual motor_motor_dual_left_contraction(Motor self, MotorDual other) {
    return MotorDual(vec4(self.g0.x) * other.g0 + self.g0 * vec4(other.g0.x) * vec4(0.0, -1.0, 1.0, 1.0));
}

Plane motor_motor_dual_right_contraction(Motor self, MotorDual other) {
    return Plane(vec3(self.g0.z) * vec3(other.g0.w, other.g0.w, other.g0.y) * vec3(1.0, 0.0, 1.0) + vec3(self.g0.w) * vec3(other.g0.z, other.g0.y, other.g0.z) * vec3(-1.0, -1.0, 0.0) + vec3(self.g0.x, self.g0.y, self.g0.y) * vec3(other.g0.x, other.g0.w, other.g0.z) * vec3(0.0, -1.0, 1.0));
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

MotorDual motor_dual_zero() {
    return MotorDual(vec4(0.0));
}

MotorDual motor_dual_one() {
    return MotorDual(vec4(0.0));
}

MotorDual motor_dual_neg(MotorDual self) {
    return MotorDual(self.g0 * vec4(-1.0));
}

MotorDual motor_dual_automorphism(MotorDual self) {
    return MotorDual(self.g0 * vec4(-1.0));
}

MotorDual motor_dual_reversal(MotorDual self) {
    return MotorDual(self.g0 * vec4(-1.0, 1.0, 1.0, 1.0));
}

MotorDual motor_dual_conjugation(MotorDual self) {
    return MotorDual(self.g0 * vec4(1.0, -1.0, -1.0, -1.0));
}

Motor motor_dual_dual(MotorDual self) {
    return Motor(self.g0);
}

MotorDual motor_dual_scalar_geometric_product(MotorDual self, Scalar other) {
    return MotorDual(self.g0 * vec4(other.g0));
}

Scalar motor_dual_scalar_regressive_product(MotorDual self, Scalar other) {
    return Scalar(self.g0.x * other.g0);
}

MotorDual motor_dual_scalar_outer_product(MotorDual self, Scalar other) {
    return MotorDual(self.g0 * vec4(other.g0));
}

MotorDual motor_dual_scalar_inner_product(MotorDual self, Scalar other) {
    return MotorDual(self.g0 * vec4(other.g0));
}

MotorDual motor_dual_scalar_right_contraction(MotorDual self, Scalar other) {
    return MotorDual(self.g0 * vec4(other.g0));
}

MultiVector motor_dual_multi_vector_add(MotorDual self, MultiVector other) {
    return MultiVector(self.g0.xxwz * vec4(0.0, 0.0, 1.0, 1.0) + other.g0, self.g0.yxxx * vec4(1.0, 1.0, 0.0, 0.0) + other.g1);
}

MultiVector motor_dual_multi_vector_sub(MotorDual self, MultiVector other) {
    return MultiVector(self.g0.xxwz * vec4(0.0, 0.0, 1.0, 1.0) - other.g0, self.g0.yxxx * vec4(1.0, 1.0, 0.0, 0.0) - other.g1);
}

MultiVector motor_dual_multi_vector_geometric_product(MotorDual self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g1.yxwz * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g0.y) * other.g1 * vec4(-1.0, -1.0, -1.0, 1.0) + vec4(self.g0.z) * other.g0.wzyx * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.w) * other.g0.zwxy, vec4(self.g0.x) * other.g0.yxwz * vec4(-1.0, 1.0, 1.0, 1.0) + vec4(self.g0.y) * other.g0 * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g1.wzyx + vec4(self.g0.w) * other.g1.zwxy * vec4(-1.0, 1.0, -1.0, 1.0));
}

MultiVector motor_dual_multi_vector_regressive_product(MotorDual self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * other.g1.zzzy * vec4(1.0, 0.0, 0.0, 1.0) + vec4(self.g0.w) * other.g1.wwyw * vec4(1.0, 0.0, 1.0, 0.0) + self.g0.yxxx * other.g0.yxxx * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g0.x) * other.g1 + self.g0.yxxx * other.g1.yxxx * vec4(1.0, 0.0, 0.0, 0.0));
}

MultiVector motor_dual_multi_vector_inner_product(MotorDual self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g1.yxwz * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g0.y) * other.g1 * vec4(-1.0, -1.0, -1.0, 1.0) + vec4(self.g0.w) * other.g0.zzxy * vec4(1.0, 0.0, 1.0, 1.0) + self.g0.zxzz * other.g0.wxyx * vec4(1.0, 0.0, -1.0, 1.0), vec4(self.g0.x) * other.g0.yxwz * vec4(-1.0, 1.0, 1.0, 1.0) + vec4(self.g0.z) * other.g1.wwyw * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g1.zzzy * vec4(-1.0, 0.0, 0.0, 1.0) + self.g0.yxxx * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0));
}

MultiVector motor_dual_multi_vector_right_contraction(MotorDual self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g1.yxwz * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g0.z) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + vec4(self.g0.w) * other.g0.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + self.g0.yxxx * vec4(other.g1.x) * vec4(-1.0, 0.0, 0.0, 0.0), vec4(self.g0.x) * other.g0.yxwz * vec4(-1.0, 1.0, 1.0, 1.0) + self.g0.yxxx * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0));
}

Scalar motor_dual_multi_vector_scalar_product(MotorDual self, MultiVector other) {
    return Scalar(self.g0.x * other.g1.y - self.g0.y * other.g1.x + self.g0.z * other.g0.w + self.g0.w * other.g0.z);
}

MotorDual motor_dual_rotor_geometric_product(MotorDual self, Rotor other) {
    return MotorDual(vec4(self.g0.y) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g0.y) * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.y, other.g0.y, other.g0.y, other.g0.x) * vec4(0.0, 0.0, 1.0, 1.0) + self.g0.xxzz * vec4(other.g0.x, other.g0.y, other.g0.x, other.g0.y) * vec4(1.0, -1.0, 1.0, -1.0));
}

Rotor motor_dual_rotor_regressive_product(MotorDual self, Rotor other) {
    return Rotor(vec2(self.g0.x) * other.g0 + vec2(self.g0.y, self.g0.x) * other.g0.yx * vec2(1.0, 0.0));
}

MotorDual motor_dual_rotor_outer_product(MotorDual self, Rotor other) {
    return MotorDual(vec4(self.g0.y) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g0.y) * vec4(1.0, 1.0, 0.0, 0.0) + self.g0.xxzw * vec4(other.g0.x) * vec4(1.0, 0.0, 1.0, 1.0));
}

MotorDual motor_dual_rotor_inner_product(MotorDual self, Rotor other) {
    return MotorDual(vec4(self.g0.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.y, other.g0.y, other.g0.y, other.g0.x) * vec4(0.0, 0.0, 1.0, 1.0) + self.g0.xxzz * vec4(other.g0.x, other.g0.y, other.g0.x, other.g0.y) * vec4(1.0, -1.0, 1.0, -1.0));
}

MotorDual motor_dual_rotor_right_contraction(MotorDual self, Rotor other) {
    return MotorDual(vec4(self.g0.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + self.g0.xxzw * vec4(other.g0.x, other.g0.y, other.g0.x, other.g0.x) * vec4(1.0, -1.0, 1.0, 1.0));
}

MotorDual motor_dual_point_geometric_product(MotorDual self, Point other) {
    return MotorDual(vec4(self.g0.y) * vec4(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4(1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.z, other.g0.y, other.g0.x) * vec4(1.0, 1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g0.z, other.g0.y, other.g0.x, other.g0.z) * vec4(1.0, -1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, -1.0, 1.0, 1.0));
}

Motor motor_dual_point_regressive_product(MotorDual self, Point other) {
    return Motor(vec4(self.g0.z) * vec4(other.g0.y) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z));
}

Plane motor_dual_point_inner_product(MotorDual self, Point other) {
    return Plane(vec3(self.g0.x) * other.g0 * vec3(-1.0, 1.0, 1.0) + vec3(self.g0.z) * other.g0.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g0.w) * other.g0.yxy * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x, self.g0.y, self.g0.y) * other.g0.xzy * vec3(0.0, 1.0, -1.0));
}

Plane motor_dual_point_left_contraction(MotorDual self, Point other) {
    return Plane(vec3(self.g0.z) * other.g0.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g0.w) * other.g0.yxy * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x, self.g0.y, self.g0.y) * other.g0.xzy * vec3(0.0, 1.0, -1.0));
}

Plane motor_dual_point_right_contraction(MotorDual self, Point other) {
    return Plane(vec3(self.g0.x) * other.g0 * vec3(-1.0, 1.0, 1.0));
}

MotorDual motor_dual_ideal_point_geometric_product(MotorDual self, IdealPoint other) {
    return MotorDual(vec4(self.g0.y) * vec4(other.g0.y, other.g0.y, other.g0.y, other.g0.x) * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.w) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g0.y) * vec4(1.0, -1.0, 0.0, 0.0) + self.g0.zzxx * vec4(other.g0.x, other.g0.y, other.g0.x, other.g0.y));
}

Translator motor_dual_ideal_point_regressive_product(MotorDual self, IdealPoint other) {
    return Translator(vec3(self.g0.w) * vec3(other.g0.y) * vec3(1.0, 0.0, 0.0) + vec3(self.g0.z, self.g0.x, self.g0.x) * vec3(other.g0.x, other.g0.x, other.g0.y));
}

Plane motor_dual_ideal_point_inner_product(MotorDual self, IdealPoint other) {
    return Plane(vec3(self.g0.y) * vec3(other.g0.y, other.g0.y, other.g0.x) * vec3(0.0, 1.0, -1.0) + vec3(self.g0.w) * vec3(other.g0.x) * vec3(-1.0, 0.0, 0.0) + vec3(self.g0.z, self.g0.x, self.g0.x) * vec3(other.g0.y, other.g0.x, other.g0.y));
}

Plane motor_dual_ideal_point_left_contraction(MotorDual self, IdealPoint other) {
    return Plane(vec3(self.g0.w) * vec3(other.g0.x) * vec3(-1.0, 0.0, 0.0) + vec3(self.g0.z, self.g0.y, self.g0.y) * vec3(other.g0.y, other.g0.y, other.g0.x) * vec3(1.0, 1.0, -1.0));
}

Plane motor_dual_plane_into(MotorDual self) {
    return Plane(vec3(self.g0.y, self.g0.z, self.g0.w));
}

MotorDual motor_dual_plane_add(MotorDual self, Plane other) {
    return MotorDual(self.g0 + vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0));
}

MotorDual motor_dual_plane_sub(MotorDual self, Plane other) {
    return MotorDual(self.g0 - vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0));
}

Motor motor_dual_plane_geometric_product(MotorDual self, Plane other) {
    return Motor(vec4(self.g0.y) * vec4(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.z, other.g0.y, other.g0.x) * vec4(1.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * vec4(other.g0.z, other.g0.y, other.g0.x, other.g0.z) * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, -1.0, 1.0, 1.0));
}

Plane motor_dual_plane_regressive_product(MotorDual self, Plane other) {
    return Plane(vec3(self.g0.x) * other.g0);
}

Point motor_dual_plane_outer_product(MotorDual self, Plane other) {
    return Point(vec3(self.g0.z) * other.g0.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.w) * other.g0.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x, self.g0.y, self.g0.y) * other.g0.xzy * vec3(0.0, 1.0, -1.0));
}

Motor motor_dual_plane_inner_product(MotorDual self, Plane other) {
    return Motor(vec4(self.g0.z) * vec4(other.g0.y) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(-1.0, -1.0, 1.0, 1.0));
}

Scalar motor_dual_plane_left_contraction(MotorDual self, Plane other) {
    return Scalar(0.0 - self.g0.y * other.g0.x + self.g0.z * other.g0.y + self.g0.w * other.g0.z);
}

Motor motor_dual_plane_right_contraction(MotorDual self, Plane other) {
    return Motor(vec4(self.g0.z) * vec4(other.g0.y) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(-1.0, -1.0, 1.0, 1.0));
}

Scalar motor_dual_plane_scalar_product(MotorDual self, Plane other) {
    return Scalar(0.0 - self.g0.y * other.g0.x + self.g0.z * other.g0.y + self.g0.w * other.g0.z);
}

MotorDual motor_dual_translator_geometric_product(MotorDual self, Translator other) {
    return MotorDual(vec4(self.g0.y) * vec4(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4(0.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.z, other.g0.x, other.g0.y) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.z, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, -1.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(1.0, 0.0, 1.0, 1.0));
}

Translator motor_dual_translator_regressive_product(MotorDual self, Translator other) {
    return Translator(vec3(self.g0.x) * other.g0 + vec3(self.g0.w) * vec3(other.g0.z) * vec3(1.0, 0.0, 0.0) + vec3(self.g0.z, self.g0.x, self.g0.x) * other.g0.yxx * vec3(1.0, 0.0, 0.0));
}

MotorDual motor_dual_translator_outer_product(MotorDual self, Translator other) {
    return MotorDual(vec4(self.g0.z) * vec4(other.g0.y, other.g0.y, other.g0.x, other.g0.y) * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.z, other.g0.z, other.g0.z, other.g0.x) * vec4(1.0, 0.0, 0.0, 1.0) + self.g0.xyxx * vec4(other.g0.x) * vec4(1.0, 1.0, 0.0, 0.0));
}

MotorDual motor_dual_translator_inner_product(MotorDual self, Translator other) {
    return MotorDual(vec4(self.g0.y) * vec4(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4(0.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g0.z) * vec4(0.0, 1.0, 1.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.y, other.g0.y, other.g0.y, other.g0.x) * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(1.0, 0.0, 1.0, 1.0));
}

Plane motor_dual_translator_left_contraction(MotorDual self, Translator other) {
    return Plane(vec3(self.g0.w) * vec3(other.g0.y) * vec3(-1.0, 0.0, 0.0) + vec3(self.g0.z, self.g0.y, self.g0.y) * other.g0.zzy * vec3(1.0, 1.0, -1.0));
}

MotorDual motor_dual_translator_right_contraction(MotorDual self, Translator other) {
    return MotorDual(vec4(self.g0.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + self.g0.xyxx * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z));
}

MultiVector motor_dual_motor_add(MotorDual self, Motor other) {
    return MultiVector(self.g0.xxwz * vec4(0.0, 0.0, 1.0, 1.0) + other.g0.xyxx * vec4(1.0, 1.0, 0.0, 0.0), self.g0.yxxx * vec4(1.0, 1.0, 0.0, 0.0) + other.g0.xxzw * vec4(0.0, 0.0, 1.0, 1.0));
}

MultiVector motor_dual_motor_sub(MotorDual self, Motor other) {
    return MultiVector(self.g0.xxwz * vec4(0.0, 0.0, 1.0, 1.0) - other.g0.xyxx * vec4(1.0, 1.0, 0.0, 0.0), self.g0.yxxx * vec4(1.0, 1.0, 0.0, 0.0) - other.g0.xxzw * vec4(0.0, 0.0, 1.0, 1.0));
}

MotorDual motor_dual_motor_geometric_product(MotorDual self, Motor other) {
    return MotorDual(vec4(self.g0.x) * other.g0 * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g0.y) * other.g0.yxwz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g0.zwxy * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g0.w) * other.g0.wzyx * vec4(1.0, -1.0, 1.0, 1.0));
}

Motor motor_dual_motor_regressive_product(MotorDual self, Motor other) {
    return Motor(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * other.g0.yxxx * vec4(1.0, 0.0, 0.0, 0.0));
}

MotorDual motor_dual_motor_outer_product(MotorDual self, Motor other) {
    return MotorDual(vec4(self.g0.y) * other.g0.yxyy * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g0.z) * other.g0.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0));
}

MotorDual motor_dual_motor_inner_product(MotorDual self, Motor other) {
    return MotorDual(vec4(self.g0.x) * other.g0 * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g0.z) * other.g0.wwxy * vec4(0.0, 1.0, 1.0, -1.0) + vec4(self.g0.w) * other.g0.zzyx * vec4(0.0, -1.0, 1.0, 1.0) + self.g0.xyyy * other.g0.xxwz * vec4(0.0, 1.0, 1.0, -1.0));
}

Plane motor_dual_motor_left_contraction(MotorDual self, Motor other) {
    return Plane(vec3(self.g0.z) * vec3(other.g0.w, other.g0.w, other.g0.y) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.w) * vec3(other.g0.z, other.g0.y, other.g0.z) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x, self.g0.y, self.g0.y) * vec3(other.g0.x, other.g0.w, other.g0.z) * vec3(0.0, 1.0, -1.0));
}

MotorDual motor_dual_motor_right_contraction(MotorDual self, Motor other) {
    return MotorDual(vec4(self.g0.x) * other.g0 * vec4(1.0, -1.0, 1.0, 1.0) + self.g0 * vec4(other.g0.x) * vec4(0.0, 1.0, 1.0, 1.0));
}

MotorDual motor_dual_motor_dual_add(MotorDual self, MotorDual other) {
    return MotorDual(self.g0 + other.g0);
}

MotorDual motor_dual_motor_dual_sub(MotorDual self, MotorDual other) {
    return MotorDual(self.g0 - other.g0);
}

MotorDual motor_dual_motor_dual_mul(MotorDual self, MotorDual other) {
    return MotorDual(self.g0 * other.g0);
}

MotorDual motor_dual_motor_dual_div(MotorDual self, MotorDual other) {
    return MotorDual(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.w) * vec4(1.0, 1.0, 1.0, 1.0) / vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.w) * vec4(1.0, 1.0, 1.0, 1.0));
}

Motor motor_dual_motor_dual_geometric_product(MotorDual self, MotorDual other) {
    return Motor(vec4(self.g0.x) * other.g0 * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g0.y) * other.g0.yxwz * vec4(-1.0, -1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g0.zwxy * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g0.w) * other.g0.wzyx * vec4(1.0, 1.0, -1.0, 1.0));
}

MotorDual motor_dual_motor_dual_regressive_product(MotorDual self, MotorDual other) {
    return MotorDual(vec4(self.g0.x) * other.g0 + self.g0 * vec4(other.g0.x) * vec4(0.0, 1.0, 1.0, 1.0));
}

Point motor_dual_motor_dual_outer_product(MotorDual self, MotorDual other) {
    return Point(vec3(self.g0.z) * vec3(other.g0.w, other.g0.w, other.g0.y) * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.w) * vec3(other.g0.z, other.g0.y, other.g0.z) * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x, self.g0.y, self.g0.y) * vec3(other.g0.x, other.g0.w, other.g0.z) * vec3(0.0, 1.0, -1.0));
}

Motor motor_dual_motor_dual_inner_product(MotorDual self, MotorDual other) {
    return Motor(vec4(self.g0.x) * other.g0 * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g0.z) * other.g0.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + self.g0.yyxx * other.g0.yxxx * vec4(-1.0, -1.0, 0.0, 0.0));
}

Motor motor_dual_motor_dual_left_contraction(MotorDual self, MotorDual other) {
    return Motor(vec4(self.g0.y) * other.g0.yxyy * vec4(-1.0, -1.0, 0.0, 0.0) + vec4(self.g0.z) * other.g0.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0));
}

Motor motor_dual_motor_dual_right_contraction(MotorDual self, MotorDual other) {
    return Motor(vec4(self.g0.x) * other.g0 * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * other.g0.yxxx * vec4(-1.0, 0.0, 0.0, 0.0));
}

Scalar motor_dual_motor_dual_scalar_product(MotorDual self, MotorDual other) {
    return Scalar(self.g0.x * other.g0.x - self.g0.y * other.g0.y + self.g0.z * other.g0.z + self.g0.w * other.g0.w);
}

Scalar motor_dual_squared_magnitude(MotorDual self) {
    return motor_dual_motor_dual_scalar_product(self, motor_dual_reversal(self));
}

Scalar motor_dual_magnitude(MotorDual self) {
    return Scalar(sqrt(motor_dual_squared_magnitude(self).g0));
}

MotorDual motor_dual_scale(MotorDual self, float other) {
    return motor_dual_scalar_geometric_product(self, Scalar(other));
}

MotorDual motor_dual_signum(MotorDual self) {
    return motor_dual_scalar_geometric_product(self, Scalar(1.0 / motor_dual_magnitude(self).g0));
}

MotorDual motor_dual_inverse(MotorDual self) {
    return motor_dual_scalar_geometric_product(motor_dual_reversal(self), Scalar(1.0 / motor_dual_squared_magnitude(self).g0));
}

Rotor ideal_point_ideal_point_geometric_quotient(IdealPoint self, IdealPoint other) {
    return ideal_point_ideal_point_geometric_product(self, ideal_point_inverse(other));
}

IdealPoint ideal_point_ideal_point_transformation(IdealPoint self, IdealPoint other) {
    return rotor_ideal_point_geometric_product(ideal_point_ideal_point_geometric_product(self, other), ideal_point_reversal(self));
}

Motor ideal_point_motor_geometric_quotient(IdealPoint self, Motor other) {
    return ideal_point_motor_geometric_product(self, motor_inverse(other));
}

Motor ideal_point_motor_transformation(IdealPoint self, Motor other) {
    return motor_ideal_point_geometric_product(ideal_point_motor_geometric_product(self, other), ideal_point_reversal(self));
}

MotorDual ideal_point_motor_dual_geometric_quotient(IdealPoint self, MotorDual other) {
    return ideal_point_motor_dual_geometric_product(self, motor_dual_inverse(other));
}

MotorDual ideal_point_motor_dual_transformation(IdealPoint self, MotorDual other) {
    return motor_dual_ideal_point_geometric_product(ideal_point_motor_dual_geometric_product(self, other), ideal_point_reversal(self));
}

MultiVector ideal_point_multi_vector_geometric_quotient(IdealPoint self, MultiVector other) {
    return ideal_point_multi_vector_geometric_product(self, multi_vector_inverse(other));
}

MultiVector ideal_point_multi_vector_transformation(IdealPoint self, MultiVector other) {
    return multi_vector_ideal_point_geometric_product(ideal_point_multi_vector_geometric_product(self, other), ideal_point_reversal(self));
}

MotorDual ideal_point_plane_geometric_quotient(IdealPoint self, Plane other) {
    return ideal_point_plane_geometric_product(self, plane_inverse(other));
}

Plane ideal_point_plane_transformation(IdealPoint self, Plane other) {
    return motor_dual_plane_into(motor_dual_ideal_point_geometric_product(ideal_point_plane_geometric_product(self, other), ideal_point_reversal(self)));
}

Motor ideal_point_point_geometric_quotient(IdealPoint self, Point other) {
    return ideal_point_point_geometric_product(self, point_inverse(other));
}

Point ideal_point_point_transformation(IdealPoint self, Point other) {
    return motor_point_into(motor_ideal_point_geometric_product(ideal_point_point_geometric_product(self, other), ideal_point_reversal(self)));
}

IdealPoint ideal_point_rotor_geometric_quotient(IdealPoint self, Rotor other) {
    return ideal_point_rotor_geometric_product(self, rotor_inverse(other));
}

Rotor ideal_point_rotor_transformation(IdealPoint self, Rotor other) {
    return ideal_point_ideal_point_geometric_product(ideal_point_rotor_geometric_product(self, other), ideal_point_reversal(self));
}

IdealPoint ideal_point_scalar_geometric_quotient(IdealPoint self, Scalar other) {
    return ideal_point_scalar_geometric_product(self, scalar_inverse(other));
}

Scalar ideal_point_scalar_transformation(IdealPoint self, Scalar other) {
    return rotor_scalar_into(ideal_point_ideal_point_geometric_product(ideal_point_scalar_geometric_product(self, other), ideal_point_reversal(self)));
}

Motor ideal_point_translator_geometric_quotient(IdealPoint self, Translator other) {
    return ideal_point_translator_geometric_product(self, translator_inverse(other));
}

Translator ideal_point_translator_transformation(IdealPoint self, Translator other) {
    return motor_translator_into(motor_ideal_point_geometric_product(ideal_point_translator_geometric_product(self, other), ideal_point_reversal(self)));
}

Motor motor_ideal_point_geometric_quotient(Motor self, IdealPoint other) {
    return motor_ideal_point_geometric_product(self, ideal_point_inverse(other));
}

IdealPoint motor_ideal_point_transformation(Motor self, IdealPoint other) {
    return motor_ideal_point_into(motor_motor_geometric_product(motor_ideal_point_geometric_product(self, other), motor_reversal(self)));
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

MotorDual motor_motor_dual_geometric_quotient(Motor self, MotorDual other) {
    return motor_motor_dual_geometric_product(self, motor_dual_inverse(other));
}

MotorDual motor_motor_dual_transformation(Motor self, MotorDual other) {
    return motor_dual_motor_geometric_product(motor_motor_dual_geometric_product(self, other), motor_reversal(self));
}

MultiVector motor_multi_vector_geometric_quotient(Motor self, MultiVector other) {
    return motor_multi_vector_geometric_product(self, multi_vector_inverse(other));
}

MultiVector motor_multi_vector_transformation(Motor self, MultiVector other) {
    return multi_vector_motor_geometric_product(motor_multi_vector_geometric_product(self, other), motor_reversal(self));
}

MotorDual motor_plane_geometric_quotient(Motor self, Plane other) {
    return motor_plane_geometric_product(self, plane_inverse(other));
}

Plane motor_plane_transformation(Motor self, Plane other) {
    return motor_dual_plane_into(motor_dual_motor_geometric_product(motor_plane_geometric_product(self, other), motor_reversal(self)));
}

Motor motor_point_geometric_quotient(Motor self, Point other) {
    return motor_point_geometric_product(self, point_inverse(other));
}

Point motor_point_transformation(Motor self, Point other) {
    return motor_point_into(motor_motor_geometric_product(motor_point_geometric_product(self, other), motor_reversal(self)));
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

MotorDual motor_dual_ideal_point_geometric_quotient(MotorDual self, IdealPoint other) {
    return motor_dual_ideal_point_geometric_product(self, ideal_point_inverse(other));
}

IdealPoint motor_dual_ideal_point_transformation(MotorDual self, IdealPoint other) {
    return motor_ideal_point_into(motor_dual_motor_dual_geometric_product(motor_dual_ideal_point_geometric_product(self, other), motor_dual_reversal(self)));
}

MotorDual motor_dual_motor_geometric_quotient(MotorDual self, Motor other) {
    return motor_dual_motor_geometric_product(self, motor_inverse(other));
}

Motor motor_dual_motor_transformation(MotorDual self, Motor other) {
    return motor_dual_motor_dual_geometric_product(motor_dual_motor_geometric_product(self, other), motor_dual_reversal(self));
}

Motor motor_dual_motor_dual_geometric_quotient(MotorDual self, MotorDual other) {
    return motor_dual_motor_dual_geometric_product(self, motor_dual_inverse(other));
}

MotorDual motor_dual_motor_dual_transformation(MotorDual self, MotorDual other) {
    return motor_motor_dual_geometric_product(motor_dual_motor_dual_geometric_product(self, other), motor_dual_reversal(self));
}

MultiVector motor_dual_multi_vector_geometric_quotient(MotorDual self, MultiVector other) {
    return motor_dual_multi_vector_geometric_product(self, multi_vector_inverse(other));
}

MultiVector motor_dual_multi_vector_transformation(MotorDual self, MultiVector other) {
    return multi_vector_motor_dual_geometric_product(motor_dual_multi_vector_geometric_product(self, other), motor_dual_reversal(self));
}

Motor motor_dual_plane_geometric_quotient(MotorDual self, Plane other) {
    return motor_dual_plane_geometric_product(self, plane_inverse(other));
}

Plane motor_dual_plane_transformation(MotorDual self, Plane other) {
    return motor_dual_plane_into(motor_motor_dual_geometric_product(motor_dual_plane_geometric_product(self, other), motor_dual_reversal(self)));
}

MotorDual motor_dual_point_geometric_quotient(MotorDual self, Point other) {
    return motor_dual_point_geometric_product(self, point_inverse(other));
}

Point motor_dual_point_transformation(MotorDual self, Point other) {
    return motor_point_into(motor_dual_motor_dual_geometric_product(motor_dual_point_geometric_product(self, other), motor_dual_reversal(self)));
}

MotorDual motor_dual_rotor_geometric_quotient(MotorDual self, Rotor other) {
    return motor_dual_rotor_geometric_product(self, rotor_inverse(other));
}

Rotor motor_dual_rotor_transformation(MotorDual self, Rotor other) {
    return motor_rotor_into(motor_dual_motor_dual_geometric_product(motor_dual_rotor_geometric_product(self, other), motor_dual_reversal(self)));
}

MotorDual motor_dual_scalar_geometric_quotient(MotorDual self, Scalar other) {
    return motor_dual_scalar_geometric_product(self, scalar_inverse(other));
}

Scalar motor_dual_scalar_transformation(MotorDual self, Scalar other) {
    return motor_scalar_into(motor_dual_motor_dual_geometric_product(motor_dual_scalar_geometric_product(self, other), motor_dual_reversal(self)));
}

MotorDual motor_dual_translator_geometric_quotient(MotorDual self, Translator other) {
    return motor_dual_translator_geometric_product(self, translator_inverse(other));
}

Translator motor_dual_translator_transformation(MotorDual self, Translator other) {
    return motor_translator_into(motor_dual_motor_dual_geometric_product(motor_dual_translator_geometric_product(self, other), motor_dual_reversal(self)));
}

MultiVector multi_vector_ideal_point_geometric_quotient(MultiVector self, IdealPoint other) {
    return multi_vector_ideal_point_geometric_product(self, ideal_point_inverse(other));
}

IdealPoint multi_vector_ideal_point_transformation(MultiVector self, IdealPoint other) {
    return multi_vector_ideal_point_into(multi_vector_multi_vector_geometric_product(multi_vector_ideal_point_geometric_product(self, other), multi_vector_reversal(self)));
}

MultiVector multi_vector_motor_geometric_quotient(MultiVector self, Motor other) {
    return multi_vector_motor_geometric_product(self, motor_inverse(other));
}

Motor multi_vector_motor_transformation(MultiVector self, Motor other) {
    return multi_vector_motor_into(multi_vector_multi_vector_geometric_product(multi_vector_motor_geometric_product(self, other), multi_vector_reversal(self)));
}

MultiVector multi_vector_motor_dual_geometric_quotient(MultiVector self, MotorDual other) {
    return multi_vector_motor_dual_geometric_product(self, motor_dual_inverse(other));
}

MotorDual multi_vector_motor_dual_transformation(MultiVector self, MotorDual other) {
    return multi_vector_motor_dual_into(multi_vector_multi_vector_geometric_product(multi_vector_motor_dual_geometric_product(self, other), multi_vector_reversal(self)));
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

MotorDual plane_ideal_point_geometric_quotient(Plane self, IdealPoint other) {
    return plane_ideal_point_geometric_product(self, ideal_point_inverse(other));
}

IdealPoint plane_ideal_point_transformation(Plane self, IdealPoint other) {
    return motor_ideal_point_into(motor_dual_plane_geometric_product(plane_ideal_point_geometric_product(self, other), plane_reversal(self)));
}

MotorDual plane_motor_geometric_quotient(Plane self, Motor other) {
    return plane_motor_geometric_product(self, motor_inverse(other));
}

Motor plane_motor_transformation(Plane self, Motor other) {
    return motor_dual_plane_geometric_product(plane_motor_geometric_product(self, other), plane_reversal(self));
}

Motor plane_motor_dual_geometric_quotient(Plane self, MotorDual other) {
    return plane_motor_dual_geometric_product(self, motor_dual_inverse(other));
}

MotorDual plane_motor_dual_transformation(Plane self, MotorDual other) {
    return motor_plane_geometric_product(plane_motor_dual_geometric_product(self, other), plane_reversal(self));
}

MultiVector plane_multi_vector_geometric_quotient(Plane self, MultiVector other) {
    return plane_multi_vector_geometric_product(self, multi_vector_inverse(other));
}

MultiVector plane_multi_vector_transformation(Plane self, MultiVector other) {
    return multi_vector_plane_geometric_product(plane_multi_vector_geometric_product(self, other), plane_reversal(self));
}

Motor plane_plane_geometric_quotient(Plane self, Plane other) {
    return plane_plane_geometric_product(self, plane_inverse(other));
}

Plane plane_plane_transformation(Plane self, Plane other) {
    return motor_dual_plane_into(motor_plane_geometric_product(plane_plane_geometric_product(self, other), plane_reversal(self)));
}

MotorDual plane_point_geometric_quotient(Plane self, Point other) {
    return plane_point_geometric_product(self, point_inverse(other));
}

Point plane_point_transformation(Plane self, Point other) {
    return motor_point_into(motor_dual_plane_geometric_product(plane_point_geometric_product(self, other), plane_reversal(self)));
}

MotorDual plane_rotor_geometric_quotient(Plane self, Rotor other) {
    return plane_rotor_geometric_product(self, rotor_inverse(other));
}

Rotor plane_rotor_transformation(Plane self, Rotor other) {
    return motor_rotor_into(motor_dual_plane_geometric_product(plane_rotor_geometric_product(self, other), plane_reversal(self)));
}

Plane plane_scalar_geometric_quotient(Plane self, Scalar other) {
    return plane_scalar_geometric_product(self, scalar_inverse(other));
}

Scalar plane_scalar_transformation(Plane self, Scalar other) {
    return motor_scalar_into(plane_plane_geometric_product(plane_scalar_geometric_product(self, other), plane_reversal(self)));
}

MotorDual plane_translator_geometric_quotient(Plane self, Translator other) {
    return plane_translator_geometric_product(self, translator_inverse(other));
}

Translator plane_translator_transformation(Plane self, Translator other) {
    return motor_translator_into(motor_dual_plane_geometric_product(plane_translator_geometric_product(self, other), plane_reversal(self)));
}

Motor point_ideal_point_geometric_quotient(Point self, IdealPoint other) {
    return point_ideal_point_geometric_product(self, ideal_point_inverse(other));
}

IdealPoint point_ideal_point_transformation(Point self, IdealPoint other) {
    return motor_ideal_point_into(motor_point_geometric_product(point_ideal_point_geometric_product(self, other), point_reversal(self)));
}

Motor point_motor_geometric_quotient(Point self, Motor other) {
    return point_motor_geometric_product(self, motor_inverse(other));
}

Motor point_motor_transformation(Point self, Motor other) {
    return motor_point_geometric_product(point_motor_geometric_product(self, other), point_reversal(self));
}

MotorDual point_motor_dual_geometric_quotient(Point self, MotorDual other) {
    return point_motor_dual_geometric_product(self, motor_dual_inverse(other));
}

MotorDual point_motor_dual_transformation(Point self, MotorDual other) {
    return motor_dual_point_geometric_product(point_motor_dual_geometric_product(self, other), point_reversal(self));
}

MultiVector point_multi_vector_geometric_quotient(Point self, MultiVector other) {
    return point_multi_vector_geometric_product(self, multi_vector_inverse(other));
}

MultiVector point_multi_vector_transformation(Point self, MultiVector other) {
    return multi_vector_point_geometric_product(point_multi_vector_geometric_product(self, other), point_reversal(self));
}

MotorDual point_plane_geometric_quotient(Point self, Plane other) {
    return point_plane_geometric_product(self, plane_inverse(other));
}

Plane point_plane_transformation(Point self, Plane other) {
    return motor_dual_plane_into(motor_dual_point_geometric_product(point_plane_geometric_product(self, other), point_reversal(self)));
}

Motor point_point_geometric_quotient(Point self, Point other) {
    return point_point_geometric_product(self, point_inverse(other));
}

Point point_point_transformation(Point self, Point other) {
    return motor_point_into(motor_point_geometric_product(point_point_geometric_product(self, other), point_reversal(self)));
}

Motor point_rotor_geometric_quotient(Point self, Rotor other) {
    return point_rotor_geometric_product(self, rotor_inverse(other));
}

Rotor point_rotor_transformation(Point self, Rotor other) {
    return motor_rotor_into(motor_point_geometric_product(point_rotor_geometric_product(self, other), point_reversal(self)));
}

Point point_scalar_geometric_quotient(Point self, Scalar other) {
    return point_scalar_geometric_product(self, scalar_inverse(other));
}

Scalar point_scalar_transformation(Point self, Scalar other) {
    return motor_scalar_into(point_point_geometric_product(point_scalar_geometric_product(self, other), point_reversal(self)));
}

Motor point_translator_geometric_quotient(Point self, Translator other) {
    return point_translator_geometric_product(self, translator_inverse(other));
}

Translator point_translator_transformation(Point self, Translator other) {
    return motor_translator_into(motor_point_geometric_product(point_translator_geometric_product(self, other), point_reversal(self)));
}

IdealPoint rotor_ideal_point_geometric_quotient(Rotor self, IdealPoint other) {
    return rotor_ideal_point_geometric_product(self, ideal_point_inverse(other));
}

IdealPoint rotor_ideal_point_transformation(Rotor self, IdealPoint other) {
    return ideal_point_rotor_geometric_product(rotor_ideal_point_geometric_product(self, other), rotor_reversal(self));
}

Motor rotor_motor_geometric_quotient(Rotor self, Motor other) {
    return rotor_motor_geometric_product(self, motor_inverse(other));
}

Motor rotor_motor_transformation(Rotor self, Motor other) {
    return motor_rotor_geometric_product(rotor_motor_geometric_product(self, other), rotor_reversal(self));
}

MotorDual rotor_motor_dual_geometric_quotient(Rotor self, MotorDual other) {
    return rotor_motor_dual_geometric_product(self, motor_dual_inverse(other));
}

MotorDual rotor_motor_dual_transformation(Rotor self, MotorDual other) {
    return motor_dual_rotor_geometric_product(rotor_motor_dual_geometric_product(self, other), rotor_reversal(self));
}

MultiVector rotor_multi_vector_geometric_quotient(Rotor self, MultiVector other) {
    return rotor_multi_vector_geometric_product(self, multi_vector_inverse(other));
}

MultiVector rotor_multi_vector_transformation(Rotor self, MultiVector other) {
    return multi_vector_rotor_geometric_product(rotor_multi_vector_geometric_product(self, other), rotor_reversal(self));
}

MotorDual rotor_plane_geometric_quotient(Rotor self, Plane other) {
    return rotor_plane_geometric_product(self, plane_inverse(other));
}

Plane rotor_plane_transformation(Rotor self, Plane other) {
    return motor_dual_plane_into(motor_dual_rotor_geometric_product(rotor_plane_geometric_product(self, other), rotor_reversal(self)));
}

Motor rotor_point_geometric_quotient(Rotor self, Point other) {
    return rotor_point_geometric_product(self, point_inverse(other));
}

Point rotor_point_transformation(Rotor self, Point other) {
    return motor_point_into(motor_rotor_geometric_product(rotor_point_geometric_product(self, other), rotor_reversal(self)));
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

Motor scalar_motor_geometric_quotient(Scalar self, Motor other) {
    return scalar_motor_geometric_product(self, motor_inverse(other));
}

Motor scalar_motor_transformation(Scalar self, Motor other) {
    return motor_scalar_geometric_product(scalar_motor_geometric_product(self, other), scalar_reversal(self));
}

MotorDual scalar_motor_dual_geometric_quotient(Scalar self, MotorDual other) {
    return scalar_motor_dual_geometric_product(self, motor_dual_inverse(other));
}

MotorDual scalar_motor_dual_transformation(Scalar self, MotorDual other) {
    return motor_dual_scalar_geometric_product(scalar_motor_dual_geometric_product(self, other), scalar_reversal(self));
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

Motor translator_ideal_point_geometric_quotient(Translator self, IdealPoint other) {
    return translator_ideal_point_geometric_product(self, ideal_point_inverse(other));
}

IdealPoint translator_ideal_point_transformation(Translator self, IdealPoint other) {
    return motor_ideal_point_into(motor_translator_geometric_product(translator_ideal_point_geometric_product(self, other), translator_reversal(self)));
}

Motor translator_motor_geometric_quotient(Translator self, Motor other) {
    return translator_motor_geometric_product(self, motor_inverse(other));
}

Motor translator_motor_transformation(Translator self, Motor other) {
    return motor_translator_geometric_product(translator_motor_geometric_product(self, other), translator_reversal(self));
}

MotorDual translator_motor_dual_geometric_quotient(Translator self, MotorDual other) {
    return translator_motor_dual_geometric_product(self, motor_dual_inverse(other));
}

MotorDual translator_motor_dual_transformation(Translator self, MotorDual other) {
    return motor_dual_translator_geometric_product(translator_motor_dual_geometric_product(self, other), translator_reversal(self));
}

MultiVector translator_multi_vector_geometric_quotient(Translator self, MultiVector other) {
    return translator_multi_vector_geometric_product(self, multi_vector_inverse(other));
}

MultiVector translator_multi_vector_transformation(Translator self, MultiVector other) {
    return multi_vector_translator_geometric_product(translator_multi_vector_geometric_product(self, other), translator_reversal(self));
}

MotorDual translator_plane_geometric_quotient(Translator self, Plane other) {
    return translator_plane_geometric_product(self, plane_inverse(other));
}

Plane translator_plane_transformation(Translator self, Plane other) {
    return motor_dual_plane_into(motor_dual_translator_geometric_product(translator_plane_geometric_product(self, other), translator_reversal(self)));
}

Motor translator_point_geometric_quotient(Translator self, Point other) {
    return translator_point_geometric_product(self, point_inverse(other));
}

Point translator_point_transformation(Translator self, Point other) {
    return motor_point_into(motor_translator_geometric_product(translator_point_geometric_product(self, other), translator_reversal(self)));
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

Scalar translator_scalar_transformation(Translator self, Scalar other) {
    return motor_scalar_into(translator_translator_geometric_product(translator_scalar_geometric_product(self, other), translator_reversal(self)));
}

Motor translator_translator_geometric_quotient(Translator self, Translator other) {
    return translator_translator_geometric_product(self, translator_inverse(other));
}

Translator translator_translator_transformation(Translator self, Translator other) {
    return motor_translator_into(motor_translator_geometric_product(translator_translator_geometric_product(self, other), translator_reversal(self)));
}

