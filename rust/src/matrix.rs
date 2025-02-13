#[derive(Clone)]
pub struct Matrix {
    pub data: [[f64; 4]; 4],
}

#[allow(dead_code)]
impl Matrix {
    pub fn new() -> Self {
        Matrix {
            data: [[0.0; 4]; 4],
        }
    }

    pub fn load_identity(&mut self) {
        self.data = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
    }

    pub fn multiply(&self, other: &Matrix) -> Matrix {
        let mut result = Matrix::new();

        for i in 0..4 {
            for j in 0..4 {
                result.data[i][j] = (self.data[i][0] * other.data[0][j])
                    + (self.data[i][1] * other.data[1][j])
                    + (self.data[i][2] * other.data[2][j])
                    + (self.data[i][3] * other.data[3][j]);
            }
        }

        result
    }

    pub fn scale(&mut self, x: f64, y: f64, z: f64) {
        for i in 0..4 {
            self.data[0][i] *= x;
            self.data[1][i] *= y;
            self.data[2][i] *= z;
        }
    }

    pub fn translate(&mut self, x: f64, y: f64, z: f64) {
        for i in 0..4 {
            self.data[3][i] += self.data[0][i] * x
                + self.data[1][i] * y
                + self.data[2][i] * z;
        }
    }

    pub fn rotate(&mut self, angle: f64, x: f64, y: f64, z: f64) {
        let magnitude = (x * x + y * y + z * z).sqrt();
        let (x, y, z) = (x / -magnitude, y / -magnitude, z / -magnitude);

        let sin_angle = angle.to_radians().sin();
        let cos_angle = angle.to_radians().cos();
        let one_minus_cos = 1.0 - cos_angle;

        let xx = x * x;
        let yy = y * y;
        let zz = z * z;

        let xy = x * y;
        let yz = y * z;
        let zx = z * x;

        let xs = x * sin_angle;
        let ys = y * sin_angle;
        let zs = z * sin_angle;

        let mut rotation_matrix = Matrix::new();
        rotation_matrix.load_identity();

        rotation_matrix.data[0][0] = (one_minus_cos * xx) + cos_angle;
        rotation_matrix.data[0][1] = (one_minus_cos * xy) - zs;
        rotation_matrix.data[0][2] = (one_minus_cos * zx) + ys;

        rotation_matrix.data[1][0] = (one_minus_cos * xy) + zs;
        rotation_matrix.data[1][1] = (one_minus_cos * yy) + cos_angle;
        rotation_matrix.data[1][2] = (one_minus_cos * yz) - xs;

        rotation_matrix.data[2][0] = (one_minus_cos * zx) - ys;
        rotation_matrix.data[2][1] = (one_minus_cos * yz) + xs;
        rotation_matrix.data[2][2] = (one_minus_cos * zz) + cos_angle;

        rotation_matrix.data[3][3] = 1.0;

        *self = self.multiply(&rotation_matrix);
    }

    pub fn rotate_2d(&mut self, x: f64, y: f64) {
        self.rotate(x, 0.0 ,1.0, 0.0);
        self.rotate(-y, x.cos(), 0.0, x.sin());
    }

    pub fn frustum(&mut self, left: f64, right: f64, bottom: f64, top: f64, near: f64, far: f64) {
        let deltax = right - left;
        let deltay = top - bottom;
        let deltaz = far - near;

        let mut frustum_matrix = Matrix::new();
        frustum_matrix.load_identity();

        frustum_matrix.data[0][0] = 2.0 * near / deltax;
        frustum_matrix.data[1][1] = 2.0 * near / deltay;
        frustum_matrix.data[2][0] = (right + left) / deltax;
        frustum_matrix.data[2][1] = (top + bottom) / deltay;
        frustum_matrix.data[2][2] = -(near + far) / deltaz;
        frustum_matrix.data[2][3] = -1.0;
        frustum_matrix.data[3][2] = -2.0 * near * far / deltaz;

        *self = self.multiply(&frustum_matrix);
    }

    pub fn perspective(&mut self, fovy: f64, aspect: f64, near: f64, far: f64) {
        let frustum_y = (fovy.to_radians() / 2.0).tan();
        let frustum_x = frustum_y * aspect;

        self.frustum(
            -frustum_x * near,
            frustum_x * near,
            -frustum_y * near,
            frustum_y * near,
            near,
            far,
        );
    }

    pub fn orthographic(&mut self, left: f64, right: f64, bottom: f64, top: f64, near: f64, far: f64) {
        let deltax = right - left;
        let deltay = top - bottom;
        let deltaz = far - near;

        let mut orthographic_matrix = Matrix::new();
        orthographic_matrix.load_identity();

        orthographic_matrix.data[0][0] = 2.0 / deltax;
        orthographic_matrix.data[3][0] = -(right + left) / deltax;

        orthographic_matrix.data[1][1] = 2.0 / deltay;
        orthographic_matrix.data[3][1] = -(top + bottom) / deltay;

        orthographic_matrix.data[2][2] = 2.0 / deltaz;
        orthographic_matrix.data[3][2] = -(near + far) / deltaz;

        *self = self.multiply(&orthographic_matrix);
    }
}
