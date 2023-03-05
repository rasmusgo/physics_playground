# Collisions

Use a spatial hash grid to find collision candidates.
Collide points against implicit surfaces (fields).
The implicit surfaces (fields) can be signed distance fields.
Rely on points being sampled dense enough instead of colliding fields against other fields.

Remember a reference point from the initial contact expressed in the coordinate system of the field.
Simulate stiction by pulling colliding points towards position of this reference point.
Adjust the reference point if the distance to the actual point is too large in order to simulate slip.

Reaction force is correction divided by timestep squared:
$$ F_c = \frac{corr}{dt²} $$

Damping can be done by interpolating towards the average velocity.

# Animations

Vary template shapes over time.

# Audio

Model the objects as spring and damper systems with template matching.
Use automatic differentiation to find the system matrix for a reference shape (typically relaxed).
Create a state transition matrix for a small timestep (eg. 44.1 khz) by multiplying the state transition matrix by the timestep size and then taking the matrix exponential of it.
