use std::f32::consts::PI;

use num::complex::{Complex32, c32};
use rui::{LocalRect, Modifiers, PaintIndex, Vger, WHITE, canvas, rui, state, vger::color::Color};

const ITER_COUNT: usize = 10000;
const TICK_SPEED: f32 = 1.0;
const INIT_SCALE: f32 = 50.0;
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

fn draw_pt(pt: Complex32, vger: &mut Vger, frame: LocalRect, scale: f32, paint: PaintIndex) {
    let [mut x, mut y] = [pt.re, pt.im];
    x *= scale;
    y *= scale;

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

fn draw_hycy(n: usize, vger: &mut Vger, frame: LocalRect, scale: f32, paint: PaintIndex) {
    for k in 0..ITER_COUNT {
        let t = k as f32 / ITER_COUNT as f32;
        let pt = H(n, 2.0 * PI * t);

        draw_pt(pt, vger, frame, scale, paint);
    }
}

fn draw_sliding(
    params: &[f32],
    n: usize,
    vger: &mut Vger,
    frame: LocalRect,
    scale: f32,
    paint: PaintIndex,
) {
    for j in 0..ITER_COUNT {
        let t = 2.0 * PI * j as f32 / ITER_COUNT as f32;
        let mut pt = H(n, t);

        for k in n..params.len() {
            pt = G(params[k], k + 1, pt);
        }
        draw_pt(pt, vger, frame, scale, paint);
    }
}

fn draw_cascade(m: usize, t: f32, vger: &mut Vger, frame: LocalRect, scale: f32) {
    for n in 1..m {
        let paint = vger.color_paint(Color::gray(n as f32 / (m + 1) as f32));

        let params = vec![t; m];
        draw_sliding(&params, n, vger, frame, scale, paint);
    }
    let paint = vger.color_paint(Color::gray(m as f32 / (m + 1) as f32));
    draw_hycy(m, vger, frame, scale, paint);
}

fn main() {
    rui(state(
        || INIT_SCALE,
        |scale, _| {
            state(
                || 1,
                move |m, _| {
                    state(
                        || 0.0,
                        move |t, _| {
                            canvas(move |cx, frame, vger| {
                                let white_paint = vger.color_paint(WHITE);
                                let magenta_paint = vger.color_paint(Color::MAGENTA);
                                let light_gray_paint = vger.color_paint(Color::gray(0.7));
                                let scale = cx[scale];

                                vger.fill_rect(frame, 0.0, white_paint);

                                draw_dashed_circ(
                                    frame.center().into(),
                                    (cx[m] + 1) as f32 * scale,
                                    1.0,
                                    200,
                                    vger,
                                    light_gray_paint,
                                );

                                draw_hycy(cx[m], vger, frame, scale, magenta_paint);
                                // draw_cascade(cx[m], cx[t], vger, frame, scale);
                            })
                            .anim(move |cx, delta| {
                                cx[t] += delta * TICK_SPEED;
                            })
                            .key(move |cx, k| match k {
                                rui::Key::ArrowLeft => {
                                    if cx[m] > 1 {
                                        cx[m] -= 1;
                                    }
                                }
                                rui::Key::ArrowRight => {
                                    cx[m] += 1;
                                }
                                _ => {}
                            })
                            .drag(move |cx, offset, gesture, btn| {
                                let Some(btn) = btn else {
                                    return;
                                };

                                match btn {
                                    rui::MouseButton::Left => {}
                                    rui::MouseButton::Right => {
                                        cx[scale] += offset.y;
                                        if cx[scale] < 1.0 {
                                            cx[scale] = 1.0;
                                        }
                                    }
                                    _ => {}
                                }
                            })
                        },
                    )
                },
            )
        },
    ));
}
