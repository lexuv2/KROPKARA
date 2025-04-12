

fn interpolate (a0:f64, a1:f64, w:f64) -> f64 {
    (a1 - a0) * ((w * (w * 6.0 - 15.0) + 10.0) * w * w * w) + a0
}

fn random_gradient (ix:i64, iy:i64) -> [f64; 2] {
    let w:u64 = 32;
    let s:u64 = 16;
    let mut a:u64 = ix as u64;
    let mut b:u64 = iy as u64;
    
    a = a.overflowing_mul(3284157443).0;
    b ^= a << s | a >> w-s;
    b = b.overflowing_mul(1911520717).0;
    a ^= b << s | b >> w-s;
    a = a.overflowing_mul(2048419325).0;
    let randi = (a as f64) * (3.14159265 / 9223372036854775807.5 );
    let v = [randi.cos(), randi.sin()];
    v
}

fn dot_grid_gradient(ix:i64, iy:i64, x:f64, y:f64) -> f64 {
    let gradient = random_gradient(ix, iy);
    let dx = x - (ix as f64);
    let dy = y - (iy as f64);
    dx*gradient[0] + dy*gradient[1]
}

pub fn perlin(x:f64, y:f64) -> f64 {
    let x0 = x.floor() as i64;
    let x1 = x0 + 1;
    let y0 = y.floor() as i64;
    let y1 = y0 + 1;
    let sx = x - (x0 as f64);
    let sy = y - (y0 as f64);

    let n0 = dot_grid_gradient(x0, y0, x, y);
    let n1 = dot_grid_gradient(x1, y0, x, y);
    let ix0 = interpolate(n0, n1, sx);
    let n0 = dot_grid_gradient(x0, y1, x, y);
    let n1 = dot_grid_gradient(x1, y1, x, y);
    let ix1 = interpolate(n0, n1, sx);
    
    interpolate(ix0, ix1, sy)
}
