struct Scalar {
    // 1
    float g0;
};

struct DualNumber {
    // 1, e01
    vec2 g0;
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

DualNumber scalar_dual_number_add(Scalar self, DualNumber other) {
    return DualNumber(vec2(self.g0) * vec2(1.0, 0.0) + other.g0);
}

DualNumber scalar_dual_number_sub(Scalar self, DualNumber other) {
    return DualNumber(vec2(self.g0) * vec2(1.0, 0.0) - other.g0);
}

DualNumber scalar_dual_number_geometric_product(Scalar self, DualNumber other) {
    return DualNumber(vec2(self.g0) * other.g0);
}

Scalar scalar_dual_number_regressive_product(Scalar self, DualNumber other) {
    return Scalar(self.g0 * other.g0.y);
}

DualNumber scalar_dual_number_outer_product(Scalar self, DualNumber other) {
    return DualNumber(vec2(self.g0) * other.g0);
}

DualNumber scalar_dual_number_inner_product(Scalar self, DualNumber other) {
    return DualNumber(vec2(self.g0) * other.g0);
}

DualNumber scalar_dual_number_left_contraction(Scalar self, DualNumber other) {
    return DualNumber(vec2(self.g0) * other.g0);
}

Scalar scalar_dual_number_right_contraction(Scalar self, DualNumber other) {
    return Scalar(self.g0 * other.g0.x);
}

Scalar scalar_dual_number_scalar_product(Scalar self, DualNumber other) {
    return Scalar(self.g0 * other.g0.x);
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

DualNumber dual_number_zero() {
    return DualNumber(vec2(0.0));
}

DualNumber dual_number_one() {
    return DualNumber(vec2(1.0, 0.0));
}

DualNumber dual_number_neg(DualNumber self) {
    return DualNumber(self.g0 * vec2(-1.0));
}

DualNumber dual_number_automorphism(DualNumber self) {
    return DualNumber(self.g0);
}

DualNumber dual_number_reversal(DualNumber self) {
    return DualNumber(self.g0 * vec2(1.0, -1.0));
}

DualNumber dual_number_conjugation(DualNumber self) {
    return DualNumber(self.g0 * vec2(1.0, -1.0));
}

DualNumber dual_number_dual(DualNumber self) {
    return DualNumber(self.g0.yx);
}

Scalar dual_number_scalar_into(DualNumber self) {
    return Scalar(self.g0.x);
}

DualNumber dual_number_scalar_add(DualNumber self, Scalar other) {
    return DualNumber(self.g0 + vec2(other.g0) * vec2(1.0, 0.0));
}

DualNumber dual_number_scalar_sub(DualNumber self, Scalar other) {
    return DualNumber(self.g0 - vec2(other.g0) * vec2(1.0, 0.0));
}

DualNumber dual_number_scalar_geometric_product(DualNumber self, Scalar other) {
    return DualNumber(self.g0 * vec2(other.g0));
}

Scalar dual_number_scalar_regressive_product(DualNumber self, Scalar other) {
    return Scalar(self.g0.y * other.g0);
}

DualNumber dual_number_scalar_outer_product(DualNumber self, Scalar other) {
    return DualNumber(self.g0 * vec2(other.g0));
}

DualNumber dual_number_scalar_inner_product(DualNumber self, Scalar other) {
    return DualNumber(self.g0 * vec2(other.g0));
}

Scalar dual_number_scalar_left_contraction(DualNumber self, Scalar other) {
    return Scalar(self.g0.x * other.g0);
}

DualNumber dual_number_scalar_right_contraction(DualNumber self, Scalar other) {
    return DualNumber(self.g0 * vec2(other.g0));
}

Scalar dual_number_scalar_scalar_product(DualNumber self, Scalar other) {
    return Scalar(self.g0.x * other.g0);
}

DualNumber dual_number_dual_number_add(DualNumber self, DualNumber other) {
    return DualNumber(self.g0 + other.g0);
}

DualNumber dual_number_dual_number_sub(DualNumber self, DualNumber other) {
    return DualNumber(self.g0 - other.g0);
}

DualNumber dual_number_dual_number_mul(DualNumber self, DualNumber other) {
    return DualNumber(self.g0 * other.g0);
}

DualNumber dual_number_dual_number_div(DualNumber self, DualNumber other) {
    return DualNumber(vec2(self.g0.x, self.g0.y) * vec2(1.0, 1.0) / vec2(other.g0.x, other.g0.y) * vec2(1.0, 1.0));
}

DualNumber dual_number_dual_number_geometric_product(DualNumber self, DualNumber other) {
    return DualNumber(vec2(self.g0.x) * other.g0 + self.g0 * vec2(other.g0.x) * vec2(0.0, 1.0));
}

DualNumber dual_number_dual_number_regressive_product(DualNumber self, DualNumber other) {
    return DualNumber(vec2(self.g0.y) * other.g0 + vec2(self.g0.x) * other.g0.yx * vec2(1.0, 0.0));
}

DualNumber dual_number_dual_number_outer_product(DualNumber self, DualNumber other) {
    return DualNumber(vec2(self.g0.x) * other.g0 + self.g0 * vec2(other.g0.x) * vec2(0.0, 1.0));
}

DualNumber dual_number_dual_number_inner_product(DualNumber self, DualNumber other) {
    return DualNumber(vec2(self.g0.x) * other.g0 + self.g0 * vec2(other.g0.x) * vec2(0.0, 1.0));
}

DualNumber dual_number_dual_number_left_contraction(DualNumber self, DualNumber other) {
    return DualNumber(vec2(self.g0.x) * other.g0);
}

DualNumber dual_number_dual_number_right_contraction(DualNumber self, DualNumber other) {
    return DualNumber(self.g0 * vec2(other.g0.x));
}

Scalar dual_number_dual_number_scalar_product(DualNumber self, DualNumber other) {
    return Scalar(self.g0.x * other.g0.x);
}

Scalar dual_number_squared_magnitude(DualNumber self) {
    return dual_number_dual_number_scalar_product(self, dual_number_reversal(self));
}

Scalar dual_number_magnitude(DualNumber self) {
    return Scalar(sqrt(dual_number_squared_magnitude(self).g0));
}

DualNumber dual_number_scale(DualNumber self, float other) {
    return dual_number_scalar_geometric_product(self, Scalar(other));
}

DualNumber dual_number_signum(DualNumber self) {
    return dual_number_scalar_geometric_product(self, Scalar(1.0 / dual_number_magnitude(self).g0));
}

DualNumber dual_number_inverse(DualNumber self) {
    return dual_number_scalar_geometric_product(dual_number_reversal(self), Scalar(1.0 / dual_number_squared_magnitude(self).g0));
}

DualNumber dual_number_powi(DualNumber self, int exponent) {
    if(exponent == 0) {
        return dual_number_one();
    }
    DualNumber x = (exponent < 0) ? dual_number_inverse(self) : self;
    DualNumber y = dual_number_one();
    int n = abs(exponent);
    while(1 < n) {
        if((n & 1) == 1) {
            y = dual_number_dual_number_geometric_product(x, y);
        }
        x = dual_number_dual_number_geometric_product(x, x);
        n = n >> 1;
    }
    return dual_number_dual_number_geometric_product(x, y);
}

DualNumber dual_number_dual_number_geometric_quotient(DualNumber self, DualNumber other) {
    return dual_number_dual_number_geometric_product(self, dual_number_inverse(other));
}

DualNumber dual_number_dual_number_transformation(DualNumber self, DualNumber other) {
    return dual_number_dual_number_geometric_product(dual_number_dual_number_geometric_product(self, other), dual_number_reversal(self));
}

DualNumber dual_number_scalar_geometric_quotient(DualNumber self, Scalar other) {
    return dual_number_scalar_geometric_product(self, scalar_inverse(other));
}

Scalar dual_number_scalar_transformation(DualNumber self, Scalar other) {
    return dual_number_scalar_into(dual_number_dual_number_geometric_product(dual_number_scalar_geometric_product(self, other), dual_number_reversal(self)));
}

DualNumber scalar_dual_number_geometric_quotient(Scalar self, DualNumber other) {
    return scalar_dual_number_geometric_product(self, dual_number_inverse(other));
}

DualNumber scalar_dual_number_transformation(Scalar self, DualNumber other) {
    return dual_number_scalar_geometric_product(scalar_dual_number_geometric_product(self, other), scalar_reversal(self));
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

