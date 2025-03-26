use std::f32::consts::PI;

use num::complex::{Complex32, c32};
use rui::{LocalRect, Modifiers, PaintIndex, Vger, WHITE, canvas, rui, state, vger::color::Color};

const ITER_COUNT: usize = 10000;
const TICK_SPEED: f32 = 1.0;
const SCALE: f32 = 50.0;
const N: usize = 7;

fn expi(t: f32) -> Complex32 {
    (Complex32::i() * t).exp()
}

fn F(varphi: &[f32]) -> Complex32 {
    let sum_exp: Complex32 = varphi.iter().copied().map(expi).sum();

    let sum_phi: f32 = varphi.iter().copied().sum();

    sum_exp + expi(-sum_phi)
}

fn H(n: usize, t: f32) -> Complex32 {
    let varphi = vec![t; n];
    F(&varphi)
}

fn G(phi: f32, n: usize, pt: Complex32) -> Complex32 {
    expi(phi) * pt + expi(-(n as f32) * phi)
}

fn draw_pt(pt: Complex32, vger: &mut Vger, frame: LocalRect, paint: PaintIndex) {
    let [mut x, mut y] = [pt.re, pt.im];
    x *= SCALE;
    y *= SCALE;

    x += frame.center().x;
    y += frame.center().y;

    vger.fill_circle([x, y], 1.0, paint);
}

fn draw_dashed_circ(
    center: [f32; 2],
    radius: f32,
    thickness: f32,
    dash_len: usize,
    vger: &mut Vger,
    paint: PaintIndex,
) {
    for k in 0..ITER_COUNT {
        if k % dash_len < dash_len / 2 {
            let t = 2.0 * PI * k as f32 / ITER_COUNT as f32;
            let mut pt = radius * expi(t);
            pt += c32(center[0], center[1]);

            let pt = [pt.re, pt.im];
            vger.fill_circle(pt, thickness, paint);
        }
    }
}

fn draw_hycy(n: usize, vger: &mut Vger, frame: LocalRect, paint: PaintIndex) {
    for k in 0..ITER_COUNT {
        let t = k as f32 / ITER_COUNT as f32;
        let pt = H(n, 2.0 * PI * t);

        draw_pt(pt, vger, frame, paint);
    }
}

fn draw_sliding(params: &[f32], n: usize, vger: &mut Vger, frame: LocalRect, paint: PaintIndex) {
    for j in 0..ITER_COUNT {
        let t = 2.0 * PI * j as f32 / ITER_COUNT as f32;
        let mut pt = H(n, t);

        for k in n..params.len() {
            pt = G(params[k], k + 1, pt);
        }
        draw_pt(pt, vger, frame, paint);
    }
}

fn draw_cascade(t: f32, vger: &mut Vger, frame: LocalRect) {
    for n in 1..N {
        let paint = vger.color_paint(Color::gray(n as f32 / (N + 1) as f32));

        let params = vec![t; N];
        draw_sliding(&params, n, vger, frame, paint);
    }
    let paint = vger.color_paint(Color::gray(N as f32 / (N + 1) as f32));
    draw_hycy(N, vger, frame, paint);
}

fn main() {
    rui(state(
        || 0.0,
        |t, _| {
            canvas(move |cx, frame, vger| {
                let white_paint = vger.color_paint(WHITE);
                let magenta_paint = vger.color_paint(Color::MAGENTA);
                let light_gray_paint = vger.color_paint(Color::gray(0.7));

                vger.fill_rect(frame, 0.0, white_paint);

                draw_dashed_circ(
                    frame.center().into(),
                    (N + 1) as f32 * SCALE,
                    1.0,
                    200,
                    vger,
                    light_gray_paint,
                );

                // draw_hycy(N, vger, frame, magenta_paint);
                draw_cascade(cx[t], vger, frame);
            })
            .anim(move |cx, delta| {
                cx[t] += delta * TICK_SPEED;
            })
        },
    ));
}
