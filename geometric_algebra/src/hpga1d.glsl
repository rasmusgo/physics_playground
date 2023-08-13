struct Scalar {
    // 1
    float g0;
};

struct SplitComplexNumber {
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

SplitComplexNumber scalar_split_complex_number_add(Scalar self, SplitComplexNumber other) {
    return SplitComplexNumber(vec2(self.g0) * vec2(1.0, 0.0) + other.g0);
}

SplitComplexNumber scalar_split_complex_number_sub(Scalar self, SplitComplexNumber other) {
    return SplitComplexNumber(vec2(self.g0) * vec2(1.0, 0.0) - other.g0);
}

SplitComplexNumber scalar_split_complex_number_geometric_product(Scalar self, SplitComplexNumber other) {
    return SplitComplexNumber(vec2(self.g0) * other.g0);
}

Scalar scalar_split_complex_number_regressive_product(Scalar self, SplitComplexNumber other) {
    return Scalar(self.g0 * other.g0.y);
}

SplitComplexNumber scalar_split_complex_number_outer_product(Scalar self, SplitComplexNumber other) {
    return SplitComplexNumber(vec2(self.g0) * other.g0);
}

SplitComplexNumber scalar_split_complex_number_inner_product(Scalar self, SplitComplexNumber other) {
    return SplitComplexNumber(vec2(self.g0) * other.g0);
}

SplitComplexNumber scalar_split_complex_number_left_contraction(Scalar self, SplitComplexNumber other) {
    return SplitComplexNumber(vec2(self.g0) * other.g0);
}

Scalar scalar_split_complex_number_right_contraction(Scalar self, SplitComplexNumber other) {
    return Scalar(self.g0 * other.g0.x);
}

Scalar scalar_split_complex_number_scalar_product(Scalar self, SplitComplexNumber other) {
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

SplitComplexNumber split_complex_number_zero() {
    return SplitComplexNumber(vec2(0.0));
}

SplitComplexNumber split_complex_number_one() {
    return SplitComplexNumber(vec2(1.0, 0.0));
}

SplitComplexNumber split_complex_number_neg(SplitComplexNumber self) {
    return SplitComplexNumber(self.g0 * vec2(-1.0));
}

SplitComplexNumber split_complex_number_automorphism(SplitComplexNumber self) {
    return SplitComplexNumber(self.g0);
}

SplitComplexNumber split_complex_number_reversal(SplitComplexNumber self) {
    return SplitComplexNumber(self.g0 * vec2(1.0, -1.0));
}

SplitComplexNumber split_complex_number_conjugation(SplitComplexNumber self) {
    return SplitComplexNumber(self.g0 * vec2(1.0, -1.0));
}

SplitComplexNumber split_complex_number_dual(SplitComplexNumber self) {
    return SplitComplexNumber(self.g0.yx);
}

Scalar split_complex_number_scalar_into(SplitComplexNumber self) {
    return Scalar(self.g0.x);
}

SplitComplexNumber split_complex_number_scalar_add(SplitComplexNumber self, Scalar other) {
    return SplitComplexNumber(self.g0 + vec2(other.g0) * vec2(1.0, 0.0));
}

SplitComplexNumber split_complex_number_scalar_sub(SplitComplexNumber self, Scalar other) {
    return SplitComplexNumber(self.g0 - vec2(other.g0) * vec2(1.0, 0.0));
}

SplitComplexNumber split_complex_number_scalar_geometric_product(SplitComplexNumber self, Scalar other) {
    return SplitComplexNumber(self.g0 * vec2(other.g0));
}

Scalar split_complex_number_scalar_regressive_product(SplitComplexNumber self, Scalar other) {
    return Scalar(self.g0.y * other.g0);
}

SplitComplexNumber split_complex_number_scalar_outer_product(SplitComplexNumber self, Scalar other) {
    return SplitComplexNumber(self.g0 * vec2(other.g0));
}

SplitComplexNumber split_complex_number_scalar_inner_product(SplitComplexNumber self, Scalar other) {
    return SplitComplexNumber(self.g0 * vec2(other.g0));
}

Scalar split_complex_number_scalar_left_contraction(SplitComplexNumber self, Scalar other) {
    return Scalar(self.g0.x * other.g0);
}

SplitComplexNumber split_complex_number_scalar_right_contraction(SplitComplexNumber self, Scalar other) {
    return SplitComplexNumber(self.g0 * vec2(other.g0));
}

Scalar split_complex_number_scalar_scalar_product(SplitComplexNumber self, Scalar other) {
    return Scalar(self.g0.x * other.g0);
}

SplitComplexNumber split_complex_number_split_complex_number_add(SplitComplexNumber self, SplitComplexNumber other) {
    return SplitComplexNumber(self.g0 + other.g0);
}

SplitComplexNumber split_complex_number_split_complex_number_sub(SplitComplexNumber self, SplitComplexNumber other) {
    return SplitComplexNumber(self.g0 - other.g0);
}

SplitComplexNumber split_complex_number_split_complex_number_mul(SplitComplexNumber self, SplitComplexNumber other) {
    return SplitComplexNumber(self.g0 * other.g0);
}

SplitComplexNumber split_complex_number_split_complex_number_div(SplitComplexNumber self, SplitComplexNumber other) {
    return SplitComplexNumber(vec2(self.g0.x, self.g0.y) * vec2(1.0, 1.0) / vec2(other.g0.x, other.g0.y) * vec2(1.0, 1.0));
}

SplitComplexNumber split_complex_number_split_complex_number_geometric_product(SplitComplexNumber self, SplitComplexNumber other) {
    return SplitComplexNumber(vec2(self.g0.x) * other.g0 + vec2(self.g0.y) * other.g0.yx);
}

SplitComplexNumber split_complex_number_split_complex_number_regressive_product(SplitComplexNumber self, SplitComplexNumber other) {
    return SplitComplexNumber(vec2(self.g0.y) * other.g0 + vec2(self.g0.x) * other.g0.yx * vec2(1.0, 0.0));
}

SplitComplexNumber split_complex_number_split_complex_number_outer_product(SplitComplexNumber self, SplitComplexNumber other) {
    return SplitComplexNumber(vec2(self.g0.x) * other.g0 + self.g0 * vec2(other.g0.x) * vec2(0.0, 1.0));
}

SplitComplexNumber split_complex_number_split_complex_number_inner_product(SplitComplexNumber self, SplitComplexNumber other) {
    return SplitComplexNumber(vec2(self.g0.x) * other.g0 + vec2(self.g0.y) * other.g0.yx);
}

SplitComplexNumber split_complex_number_split_complex_number_left_contraction(SplitComplexNumber self, SplitComplexNumber other) {
    return SplitComplexNumber(vec2(self.g0.x) * other.g0 + self.g0.yx * other.g0.yx * vec2(1.0, 0.0));
}

SplitComplexNumber split_complex_number_split_complex_number_right_contraction(SplitComplexNumber self, SplitComplexNumber other) {
    return SplitComplexNumber(vec2(self.g0.y) * other.g0.yx + vec2(self.g0.x) * vec2(other.g0.x) * vec2(1.0, 0.0));
}

Scalar split_complex_number_split_complex_number_scalar_product(SplitComplexNumber self, SplitComplexNumber other) {
    return Scalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y);
}

Scalar split_complex_number_squared_magnitude(SplitComplexNumber self) {
    return split_complex_number_split_complex_number_scalar_product(self, split_complex_number_reversal(self));
}

Scalar split_complex_number_magnitude(SplitComplexNumber self) {
    return Scalar(sqrt(split_complex_number_squared_magnitude(self).g0));
}

SplitComplexNumber split_complex_number_scale(SplitComplexNumber self, float other) {
    return split_complex_number_scalar_geometric_product(self, Scalar(other));
}

SplitComplexNumber split_complex_number_signum(SplitComplexNumber self) {
    return split_complex_number_scalar_geometric_product(self, Scalar(1.0 / split_complex_number_magnitude(self).g0));
}

SplitComplexNumber split_complex_number_inverse(SplitComplexNumber self) {
    return split_complex_number_scalar_geometric_product(split_complex_number_reversal(self), Scalar(1.0 / split_complex_number_squared_magnitude(self).g0));
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

SplitComplexNumber scalar_split_complex_number_geometric_quotient(Scalar self, SplitComplexNumber other) {
    return scalar_split_complex_number_geometric_product(self, split_complex_number_inverse(other));
}

SplitComplexNumber scalar_split_complex_number_transformation(Scalar self, SplitComplexNumber other) {
    return split_complex_number_scalar_geometric_product(scalar_split_complex_number_geometric_product(self, other), scalar_reversal(self));
}

SplitComplexNumber split_complex_number_scalar_geometric_quotient(SplitComplexNumber self, Scalar other) {
    return split_complex_number_scalar_geometric_product(self, scalar_inverse(other));
}

Scalar split_complex_number_scalar_transformation(SplitComplexNumber self, Scalar other) {
    return split_complex_number_scalar_into(split_complex_number_split_complex_number_geometric_product(split_complex_number_scalar_geometric_product(self, other), split_complex_number_reversal(self)));
}

SplitComplexNumber split_complex_number_powi(SplitComplexNumber self, int exponent) {
    if(exponent == 0) {
        return split_complex_number_one();
    }
    SplitComplexNumber x = (exponent < 0) ? split_complex_number_inverse(self) : self;
    SplitComplexNumber y = split_complex_number_one();
    int n = abs(exponent);
    while(1 < n) {
        if((n & 1) == 1) {
            y = split_complex_number_split_complex_number_geometric_product(x, y);
        }
        x = split_complex_number_split_complex_number_geometric_product(x, x);
        n = n >> 1;
    }
    return split_complex_number_split_complex_number_geometric_product(x, y);
}

SplitComplexNumber split_complex_number_split_complex_number_geometric_quotient(SplitComplexNumber self, SplitComplexNumber other) {
    return split_complex_number_split_complex_number_geometric_product(self, split_complex_number_inverse(other));
}

SplitComplexNumber split_complex_number_split_complex_number_transformation(SplitComplexNumber self, SplitComplexNumber other) {
    return split_complex_number_split_complex_number_geometric_product(split_complex_number_split_complex_number_geometric_product(self, other), split_complex_number_reversal(self));
}

