use std::cmp::min;

// display dimensions
const SCREEN_HEIGHT: usize = 40;
const SCREEN_WIDTH: usize = 40;

const THETA_SPACING: f32 = 0.07;
const PHI_SPACING: f32 = 0.02;

const R1: f32 = 1.0;
const PI: f32 = std::f32::consts::PI;
const R2: f32 = 2.0;
const K2: f32 = 5.0;
// Calculate K1 based on screen size: the maximum x-distance occurs
// roughly at the edge of the torus, which is at x=R1+R2, z=0.  we
// want that to be displaced 3/8ths of the width of the screen, which
// is 3/4th of the way from the center to the side of the screen.
// screen_width*3/8 = K1*(R1+R2)/(K2+0)
// screen_width*K2*3/(8*(R1+R2)) = K1
const K1: f32 = SCREEN_WIDTH as f32 * K2 * 3.0 / (8.0 * (R1 + R2));

fn render_frame(a: f32, b: f32) {
    // precompute sines and cosines of A and B
    let cos_a = a.cos();
    let sin_a = a.sin();
    let cos_b = b.cos();
    let sin_b = b.sin();

    let mut output = [[b' '; SCREEN_HEIGHT]; SCREEN_WIDTH];
    let mut zbuffer = [[0.0f32; SCREEN_HEIGHT]; SCREEN_WIDTH];

    // theta goes around the cross-sectional circle of a torus
    let mut theta = 0.0;
    while theta < 2.0 * PI {
        // precompute sines and cosines of theta
        let costheta = theta.cos();
        let sintheta = theta.sin();

        // phi goes around the center of revolution of a torus
        let mut phi = 0.0;
        while phi < 2.0 * PI {
            // precompute sines and cosines of phi
            let cosphi = phi.cos();
            let sinphi = phi.sin();

            // the x,y coordinate of the circle, before revolving (factored
            // out of the above equations)
            let circlex = R2 + R1 * costheta;
            let circley = R1 * sintheta;

            // final 3D (x,y,z) coordinate after rotations, directly from
            // our math above
            let x = circlex * (cos_b * cosphi + sin_a * sin_b * sinphi) - circley * cos_a * sin_b;
            let y = circlex * (sin_b * cosphi - sin_a * cos_b * sinphi) + circley * cos_a * cos_b;
            let z = K2 + cos_a * circlex * sinphi + circley * sin_a;
            let ooz = 1.0 / z; // "one over z"

            // x and y projection.  note that y is negated here, because y
            // goes up in 3D space but down on 2D displays.
            let xp = min(
                (SCREEN_WIDTH as f32 / 2.0 + K1 * ooz * x) as usize,
                SCREEN_WIDTH - 1,
            );
            let yp = min(
                (SCREEN_HEIGHT as f32 / 2.0 - K1 * ooz * y) as usize,
                SCREEN_HEIGHT - 1,
            );

            // calculate luminance.  ugly, but correct.
            let l = cosphi * costheta * sin_b - cos_a * costheta * sinphi - sin_a * sintheta
                + cos_b * (cos_a * sintheta - costheta * sin_a * sinphi);
            // L ranges from -sqrt(2) to +sqrt(2).  If it's < 0, the surface
            // is pointing away from us, so we won't bother trying to plot it.
            if l > 0.0 {
                // test against the z-buffer.  larger 1/z means the pixel is
                // closer to the viewer than what's already plotted.
                if ooz > zbuffer[xp][yp] {
                    zbuffer[xp][yp] = ooz;
                    let luminance_index = l * 8.0;
                    // luminance_index is now in the range 0..11 (8*sqrt(2)
                    // = 11.3) now we lookup the character corresponding to the
                    // luminance and plot it in our output:
                    output[xp][yp] = b".,-~:;=!*#$@"[luminance_index as usize];
                }
            }
            phi += PHI_SPACING;
        }
        theta += THETA_SPACING;
    }

    // now, dump output[] to the screen.
    // bring cursor to "home" location, in just about any currently-used
    // terminal emulation mode
    print!("\x1b[H");
    for output in &output {
        for c in output {
            print!("{}", *c as char)
        }
        println!();
    }
}

fn main() {
    const STEP_X: f32 = 0.002;
    const STEP_Y: f32 = 0.003;
    let mut angle_x = 0.0;
    let mut angle_y = 0.0;
    loop {
        render_frame(angle_x, angle_y);
        angle_x += STEP_X;
        angle_y += STEP_Y;
    }
}
