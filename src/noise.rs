use rand::Rng;
use std::{thread, time::Duration};

const PERMUTATION: [u8; 256] = [
    151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30, 69,
    142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94, 252, 219,
    203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174, 20, 125, 136, 171, 168, 68, 175,
    74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60, 211, 133, 230,
    220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1, 216, 80, 73, 209,
    76, 132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86, 164, 100, 109, 198,
    173, 186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118, 126, 255, 82, 85, 212,
    207, 206, 59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170, 213, 119, 248, 152, 2,
    44, 154, 163, 70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253, 19, 98, 108, 110,
    79, 113, 224, 232, 178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193, 238, 210, 144,
    12, 191, 179, 162, 241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31, 181, 199, 106,
    157, 184, 84, 204, 176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93, 222, 114, 67,
    29, 24, 72, 243, 141, 128, 195, 78, 66, 215, 61, 156, 180,
];

pub(crate) struct PerlinNoise {
    p: [u8; 512],
}

impl PerlinNoise {
    pub(crate) fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut p = [0; 512];
        for i in 0..256 {
            p[i] = i as u8;
        }
        for i in 0..256 {
            let j = rng.gen_range(0..256);
            p.swap(i, j);
        }
        for i in 0..256 {
            p[i + 256] = p[i];
        }
        PerlinNoise { p }
    }

    fn fade(t: f64) -> f64 {
        t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
    }

    fn lerp(t: f64, a: f64, b: f64) -> f64 {
        a + t * (b - a)
    }

    fn grad(hash: u8, x: f64, y: f64, z: f64) -> f64 {
        let h = hash & 15;
        let u = if h < 8 { x } else { y };
        let v = if h < 4 {
            y
        } else if h == 12 || h == 14 {
            x
        } else {
            z
        };
        let s = if (h & 1) == 0 { 1.0 } else { -1.0 };
        let t = if (h & 2) == 0 { 1.0 } else { -1.0 };
        s * u + t * v
    }

    pub fn noise(&self, x: f64, y: f64, z: f64) -> f64 {
        let xi = x.floor() as usize & 255;
        let yi = y.floor() as usize & 255;
        let zi = z.floor() as usize & 255;

        let x = x - x.floor();
        let y = y - y.floor();
        let z = z - z.floor();

        let u = Self::fade(x);
        let v = Self::fade(y);
        let w = Self::fade(z);

        let a = self.p[xi] as usize + yi;
        let aa = self.p[a] as usize + zi;
        let ab = self.p[a + 1] as usize + zi;
        let b = self.p[xi + 1] as usize + yi;
        let ba = self.p[b] as usize + zi;
        let bb = self.p[b + 1] as usize + zi;

        Self::lerp(
            w,
            Self::lerp(
                v,
                Self::lerp(
                    u,
                    Self::grad(self.p[aa], x, y, z),
                    Self::grad(self.p[ba], x - 1.0, y, z),
                ),
                Self::lerp(
                    u,
                    Self::grad(self.p[ab], x, y - 1.0, z),
                    Self::grad(self.p[bb], x - 1.0, y - 1.0, z),
                ),
            ),
            Self::lerp(
                v,
                Self::lerp(
                    u,
                    Self::grad(self.p[aa + 1], x, y, z - 1.0),
                    Self::grad(self.p[ba + 1], x - 1.0, y, z - 1.0),
                ),
                Self::lerp(
                    u,
                    Self::grad(self.p[ab + 1], x, y - 1.0, z - 1.0),
                    Self::grad(self.p[bb + 1], x - 1.0, y - 1.0, z - 1.0),
                ),
            ),
        )
    }
}
