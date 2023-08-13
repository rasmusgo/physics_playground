struct Scalar {
    // 1
    float g0;
};

struct ComplexNumber {
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

ComplexNumber scalar_complex_number_add(Scalar self, ComplexNumber other) {
    return ComplexNumber(vec2(self.g0) * vec2(1.0, 0.0) + other.g0);
}

ComplexNumber scalar_complex_number_sub(Scalar self, ComplexNumber other) {
    return ComplexNumber(vec2(self.g0) * vec2(1.0, 0.0) - other.g0);
}

ComplexNumber scalar_complex_number_geometric_product(Scalar self, ComplexNumber other) {
    return ComplexNumber(vec2(self.g0) * other.g0);
}

Scalar scalar_complex_number_regressive_product(Scalar self, ComplexNumber other) {
    return Scalar(self.g0 * other.g0.y);
}

ComplexNumber scalar_complex_number_outer_product(Scalar self, ComplexNumber other) {
    return ComplexNumber(vec2(self.g0) * other.g0);
}

ComplexNumber scalar_complex_number_inner_product(Scalar self, ComplexNumber other) {
    return ComplexNumber(vec2(self.g0) * other.g0);
}

ComplexNumber scalar_complex_number_left_contraction(Scalar self, ComplexNumber other) {
    return ComplexNumber(vec2(self.g0) * other.g0);
}

Scalar scalar_complex_number_right_contraction(Scalar self, ComplexNumber other) {
    return Scalar(self.g0 * other.g0.x);
}

Scalar scalar_complex_number_scalar_product(Scalar self, ComplexNumber other) {
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

ComplexNumber complex_number_zero() {
    return ComplexNumber(vec2(0.0));
}

ComplexNumber complex_number_one() {
    return ComplexNumber(vec2(1.0, 0.0));
}

ComplexNumber complex_number_neg(ComplexNumber self) {
    return ComplexNumber(self.g0 * vec2(-1.0));
}

ComplexNumber complex_number_automorphism(ComplexNumber self) {
    return ComplexNumber(self.g0);
}

ComplexNumber complex_number_reversal(ComplexNumber self) {
    return ComplexNumber(self.g0 * vec2(1.0, -1.0));
}

ComplexNumber complex_number_conjugation(ComplexNumber self) {
    return ComplexNumber(self.g0 * vec2(1.0, -1.0));
}

ComplexNumber complex_number_dual(ComplexNumber self) {
    return ComplexNumber(self.g0.yx);
}

Scalar complex_number_scalar_into(ComplexNumber self) {
    return Scalar(self.g0.x);
}

ComplexNumber complex_number_scalar_add(ComplexNumber self, Scalar other) {
    return ComplexNumber(self.g0 + vec2(other.g0) * vec2(1.0, 0.0));
}

ComplexNumber complex_number_scalar_sub(ComplexNumber self, Scalar other) {
    return ComplexNumber(self.g0 - vec2(other.g0) * vec2(1.0, 0.0));
}

ComplexNumber complex_number_scalar_geometric_product(ComplexNumber self, Scalar other) {
    return ComplexNumber(self.g0 * vec2(other.g0));
}

Scalar complex_number_scalar_regressive_product(ComplexNumber self, Scalar other) {
    return Scalar(self.g0.y * other.g0);
}

ComplexNumber complex_number_scalar_outer_product(ComplexNumber self, Scalar other) {
    return ComplexNumber(self.g0 * vec2(other.g0));
}

ComplexNumber complex_number_scalar_inner_product(ComplexNumber self, Scalar other) {
    return ComplexNumber(self.g0 * vec2(other.g0));
}

Scalar complex_number_scalar_left_contraction(ComplexNumber self, Scalar other) {
    return Scalar(self.g0.x * other.g0);
}

ComplexNumber complex_number_scalar_right_contraction(ComplexNumber self, Scalar other) {
    return ComplexNumber(self.g0 * vec2(other.g0));
}

Scalar complex_number_scalar_scalar_product(ComplexNumber self, Scalar other) {
    return Scalar(self.g0.x * other.g0);
}

ComplexNumber complex_number_complex_number_add(ComplexNumber self, ComplexNumber other) {
    return ComplexNumber(self.g0 + other.g0);
}

ComplexNumber complex_number_complex_number_sub(ComplexNumber self, ComplexNumber other) {
    return ComplexNumber(self.g0 - other.g0);
}

ComplexNumber complex_number_complex_number_mul(ComplexNumber self, ComplexNumber other) {
    return ComplexNumber(self.g0 * other.g0);
}

ComplexNumber complex_number_complex_number_div(ComplexNumber self, ComplexNumber other) {
    return ComplexNumber(vec2(self.g0.x, self.g0.y) * vec2(1.0, 1.0) / vec2(other.g0.x, other.g0.y) * vec2(1.0, 1.0));
}

ComplexNumber complex_number_complex_number_geometric_product(ComplexNumber self, ComplexNumber other) {
    return ComplexNumber(vec2(self.g0.x) * other.g0 + vec2(self.g0.y) * other.g0.yx * vec2(-1.0, 1.0));
}

ComplexNumber complex_number_complex_number_regressive_product(ComplexNumber self, ComplexNumber other) {
    return ComplexNumber(vec2(self.g0.y) * other.g0 + vec2(self.g0.x) * other.g0.yx * vec2(1.0, 0.0));
}

ComplexNumber complex_number_complex_number_outer_product(ComplexNumber self, ComplexNumber other) {
    return ComplexNumber(vec2(self.g0.x) * other.g0 + self.g0 * vec2(other.g0.x) * vec2(0.0, 1.0));
}

ComplexNumber complex_number_complex_number_inner_product(ComplexNumber self, ComplexNumber other) {
    return ComplexNumber(vec2(self.g0.x) * other.g0 + vec2(self.g0.y) * other.g0.yx * vec2(-1.0, 1.0));
}

ComplexNumber complex_number_complex_number_left_contraction(ComplexNumber self, ComplexNumber other) {
    return ComplexNumber(vec2(self.g0.x) * other.g0 + self.g0.yx * other.g0.yx * vec2(-1.0, 0.0));
}

ComplexNumber complex_number_complex_number_right_contraction(ComplexNumber self, ComplexNumber other) {
    return ComplexNumber(vec2(self.g0.y) * other.g0.yx * vec2(-1.0, 1.0) + vec2(self.g0.x) * vec2(other.g0.x) * vec2(1.0, 0.0));
}

Scalar complex_number_complex_number_scalar_product(ComplexNumber self, ComplexNumber other) {
    return Scalar(self.g0.x * other.g0.x - self.g0.y * other.g0.y);
}

Scalar complex_number_squared_magnitude(ComplexNumber self) {
    return complex_number_complex_number_scalar_product(self, complex_number_reversal(self));
}

Scalar complex_number_magnitude(ComplexNumber self) {
    return Scalar(sqrt(complex_number_squared_magnitude(self).g0));
}

ComplexNumber complex_number_scale(ComplexNumber self, float other) {
    return complex_number_scalar_geometric_product(self, Scalar(other));
}

ComplexNumber complex_number_signum(ComplexNumber self) {
    return complex_number_scalar_geometric_product(self, Scalar(1.0 / complex_number_magnitude(self).g0));
}

ComplexNumber complex_number_inverse(ComplexNumber self) {
    return complex_number_scalar_geometric_product(complex_number_reversal(self), Scalar(1.0 / complex_number_squared_magnitude(self).g0));
}

ComplexNumber complex_number_powi(ComplexNumber self, int exponent) {
    if(exponent == 0) {
        return complex_number_one();
    }
    ComplexNumber x = (exponent < 0) ? complex_number_inverse(self) : self;
    ComplexNumber y = complex_number_one();
    int n = abs(exponent);
    while(1 < n) {
        if((n & 1) == 1) {
            y = complex_number_complex_number_geometric_product(x, y);
        }
        x = complex_number_complex_number_geometric_product(x, x);
        n = n >> 1;
    }
    return complex_number_complex_number_geometric_product(x, y);
}

ComplexNumber complex_number_complex_number_geometric_quotient(ComplexNumber self, ComplexNumber other) {
    return complex_number_complex_number_geometric_product(self, complex_number_inverse(other));
}

ComplexNumber complex_number_complex_number_transformation(ComplexNumber self, ComplexNumber other) {
    return complex_number_complex_number_geometric_product(complex_number_complex_number_geometric_product(self, other), complex_number_reversal(self));
}

ComplexNumber complex_number_scalar_geometric_quotient(ComplexNumber self, Scalar other) {
    return complex_number_scalar_geometric_product(self, scalar_inverse(other));
}

Scalar complex_number_scalar_transformation(ComplexNumber self, Scalar other) {
    return complex_number_scalar_into(complex_number_complex_number_geometric_product(complex_number_scalar_geometric_product(self, other), complex_number_reversal(self)));
}

ComplexNumber scalar_complex_number_geometric_quotient(Scalar self, ComplexNumber other) {
    return scalar_complex_number_geometric_product(self, complex_number_inverse(other));
}

ComplexNumber scalar_complex_number_transformation(Scalar self, ComplexNumber other) {
    return complex_number_scalar_geometric_product(scalar_complex_number_geometric_product(self, other), scalar_reversal(self));
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

