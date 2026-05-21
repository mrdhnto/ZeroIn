use core::f32::consts::FRAC_PI_4;
use windows::Foundation::Numerics::Matrix3x2;
use windows::Win32::Graphics::Direct2D::Common::{D2D_POINT_2F, D2D_RECT_F};
use windows::Win32::Graphics::Direct2D::{D2D1_ELLIPSE, ID2D1DCRenderTarget, ID2D1SolidColorBrush};

use crate::config::CrosshairType;

pub fn draw(
    target: &ID2D1DCRenderTarget,
    brush: &ID2D1SolidColorBrush,
    border_brush: Option<&ID2D1SolidColorBrush>,
    ctype: CrosshairType,
    cx: f32,
    cy: f32,
    size: f32,
    thickness_h: f32,
    thickness_v: f32,
    dot_center: bool,
    border: bool,
    border_size: f32,
    space_width: f32,
    rotation: f32,
    dot_size: f32,
) {
    let intrinsic = match ctype {
        CrosshairType::Diamond => FRAC_PI_4,
        _ => 0.0,
    };
    let total = rotation.to_radians() + intrinsic;

    if total != 0.0 {
        let (sa, ca) = total.sin_cos();
        let m = Matrix3x2 {
            M11: ca, M12: sa,
            M21: -sa, M22: ca,
            M31: cx - cx * ca + cy * sa,
            M32: cy - cx * sa - cy * ca,
        };
        unsafe { target.SetTransform(&m as *const Matrix3x2); }
    }

    match ctype {
        CrosshairType::Dot => draw_dot(target, brush, border_brush, cx, cy, size, border, border_size),
        CrosshairType::Cross | CrosshairType::Diamond => {
            draw_cross(target, brush, border_brush, cx, cy, size, thickness_h, thickness_v, dot_center, border, border_size, space_width, dot_size)
        }
        CrosshairType::T => draw_t(target, brush, border_brush, cx, cy, size, thickness_h, thickness_v, dot_center, border, border_size, space_width, dot_size),
        CrosshairType::Circle => draw_circle(target, brush, border_brush, cx, cy, size, thickness_h, thickness_v, dot_center, border, border_size, space_width, dot_size),
        CrosshairType::Arrow => draw_arrow(target, brush, border_brush, cx, cy, size, thickness_h, thickness_v, dot_center, border, border_size, space_width, dot_size),
    }

    if total != 0.0 {
        let identity = Matrix3x2 {
            M11: 1.0, M12: 0.0,
            M21: 0.0, M22: 1.0,
            M31: 0.0, M32: 0.0,
        };
        unsafe { target.SetTransform(&identity as *const Matrix3x2); }
    }
}

fn draw_split_rect(
    target: &ID2D1DCRenderTarget,
    brush: &ID2D1SolidColorBrush,
    left: f32,
    top: f32,
    right: f32,
    bottom: f32,
) {
    if left < right && top < bottom {
        let _ = unsafe {
            target.FillRectangle(
                &D2D_RECT_F { left, top, right, bottom },
                brush,
            )
        };
    }
}

fn draw_dot(
    target: &ID2D1DCRenderTarget,
    brush: &ID2D1SolidColorBrush,
    border_brush: Option<&ID2D1SolidColorBrush>,
    cx: f32,
    cy: f32,
    size: f32,
    border: bool,
    border_size: f32,
) {
    let radius = size / 2.0;

    if border && border_size > 0.0 {
        if let Some(bb) = border_brush {
            let outer = radius + border_size;
            let _ = unsafe {
                target.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F { x: cx, y: cy },
                        radiusX: outer,
                        radiusY: outer,
                    },
                    bb,
                )
            };
        }
    }

    let _ = unsafe {
        target.FillEllipse(
            &D2D1_ELLIPSE {
                point: D2D_POINT_2F { x: cx, y: cy },
                radiusX: radius,
                radiusY: radius,
            },
            brush,
        )
    };
}

fn draw_cross(
    target: &ID2D1DCRenderTarget,
    brush: &ID2D1SolidColorBrush,
    border_brush: Option<&ID2D1SolidColorBrush>,
    cx: f32,
    cy: f32,
    size: f32,
    thickness_h: f32,
    thickness_v: f32,
    dot_center: bool,
    border: bool,
    border_size: f32,
    space_width: f32,
    dot_size: f32,
) { unsafe {
    let half = size / 2.0;
    let half_t_h = thickness_h / 2.0;
    let half_t_v = thickness_v / 2.0;
    let sw = space_width.min(half);
    let bs = border_size;

    if border && bs > 0.0 {
        if let Some(bb) = border_brush {
            draw_split_rect(target, bb, cx - half - bs, cy - half_t_v - bs, cx - sw + bs, cy + half_t_v + bs);
            draw_split_rect(target, bb, cx + sw - bs, cy - half_t_v - bs, cx + half + bs, cy + half_t_v + bs);
            draw_split_rect(target, bb, cx - half_t_h - bs, cy - half - bs, cx + half_t_h + bs, cy - sw + bs);
            draw_split_rect(target, bb, cx - half_t_h - bs, cy + sw - bs, cx + half_t_h + bs, cy + half + bs);
            if dot_center {
                let _ = target.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F { x: cx, y: cy },
                        radiusX: dot_size + bs,
                        radiusY: dot_size + bs,
                    },
                    bb,
                );
            }
        }
    }

    draw_split_rect(target, brush, cx - half, cy - half_t_v, cx - sw, cy + half_t_v);
    draw_split_rect(target, brush, cx + sw, cy - half_t_v, cx + half, cy + half_t_v);
    draw_split_rect(target, brush, cx - half_t_h, cy - half, cx + half_t_h, cy - sw);
    draw_split_rect(target, brush, cx - half_t_h, cy + sw, cx + half_t_h, cy + half);

    if dot_center {
        let _ = target.FillEllipse(
            &D2D1_ELLIPSE {
                point: D2D_POINT_2F { x: cx, y: cy },
                radiusX: dot_size,
                radiusY: dot_size,
            },
            brush,
        );
    }
}}

fn draw_t(
    target: &ID2D1DCRenderTarget,
    brush: &ID2D1SolidColorBrush,
    border_brush: Option<&ID2D1SolidColorBrush>,
    cx: f32,
    cy: f32,
    size: f32,
    thickness_h: f32,
    thickness_v: f32,
    dot_center: bool,
    border: bool,
    border_size: f32,
    space_width: f32,
    dot_size: f32,
) { unsafe {
    let half = size / 2.0;
    let half_t_h = thickness_h / 2.0;
    let half_t_v = thickness_v / 2.0;
    let sw = space_width.min(half);
    let bs = border_size;

    if border && bs > 0.0 {
        if let Some(bb) = border_brush {
            draw_split_rect(target, bb, cx - half - bs, cy - half_t_v - bs, cx - sw + bs, cy + half_t_v + bs);
            draw_split_rect(target, bb, cx + sw - bs, cy - half_t_v - bs, cx + half + bs, cy + half_t_v + bs);
            draw_split_rect(target, bb, cx - half_t_h - bs, cy - half_t_h - bs, cx + half_t_h + bs, cy - sw + bs);
            draw_split_rect(target, bb, cx - half_t_h - bs, cy + sw - bs, cx + half_t_h + bs, cy + half + bs);
            if dot_center {
                let _ = target.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F { x: cx, y: cy },
                        radiusX: dot_size + bs,
                        radiusY: dot_size + bs,
                    },
                    bb,
                );
            }
        }
    }

    draw_split_rect(target, brush, cx - half, cy - half_t_v, cx - sw, cy + half_t_v);
    draw_split_rect(target, brush, cx + sw, cy - half_t_v, cx + half, cy + half_t_v);
    draw_split_rect(target, brush, cx - half_t_h, cy - half_t_h, cx + half_t_h, cy - sw);
    draw_split_rect(target, brush, cx - half_t_h, cy + sw, cx + half_t_h, cy + half);

    if dot_center {
        let _ = target.FillEllipse(
            &D2D1_ELLIPSE {
                point: D2D_POINT_2F { x: cx, y: cy },
                radiusX: dot_size,
                radiusY: dot_size,
            },
            brush,
        );
    }
}}

fn draw_circle(
    target: &ID2D1DCRenderTarget,
    brush: &ID2D1SolidColorBrush,
    border_brush: Option<&ID2D1SolidColorBrush>,
    cx: f32,
    cy: f32,
    size: f32,
    thickness_h: f32,
    _thickness_v: f32,
    dot_center: bool,
    border: bool,
    border_size: f32,
    space_width: f32,
    dot_size: f32,
) { unsafe {
    let outer_r = size / 2.0;
    let stroke = thickness_h;
    let bs = border_size;
    let outline_mode = border;

    if bs > 0.0 {
        if let Some(bb) = border_brush {
            if outline_mode {
                let border_stroke = stroke + bs * 2.0;
                let _ = target.DrawEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F { x: cx, y: cy },
                        radiusX: outer_r + bs,
                        radiusY: outer_r + bs,
                    },
                    bb,
                    border_stroke,
                    None,
                );
            } else if dot_center {
                let _ = target.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F { x: cx, y: cy },
                        radiusX: dot_size + bs,
                        radiusY: dot_size + bs,
                    },
                    bb,
                );
            } else {
                let _ = target.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F { x: cx, y: cy },
                        radiusX: outer_r + bs,
                        radiusY: outer_r + bs,
                    },
                    bb,
                );
            }
            if dot_center {
                let _ = target.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F { x: cx, y: cy },
                        radiusX: dot_size + bs,
                        radiusY: dot_size + bs,
                    },
                    bb,
                );
            }
        }
    }

    if outline_mode && dot_center {
        let max_dot = (outer_r - stroke / 2.0 - space_width).max(0.0);
        let actual_dot_r = dot_size.min(max_dot);
        let _ = target.DrawEllipse(
            &D2D1_ELLIPSE {
                point: D2D_POINT_2F { x: cx, y: cy },
                radiusX: outer_r,
                radiusY: outer_r,
            },
            brush,
            stroke,
            None,
        );
        if actual_dot_r > 0.0 {
            let _ = target.FillEllipse(
                &D2D1_ELLIPSE {
                    point: D2D_POINT_2F { x: cx, y: cy },
                    radiusX: actual_dot_r,
                    radiusY: actual_dot_r,
                },
                brush,
            );
        }
    } else if outline_mode && !dot_center {
        let _ = target.DrawEllipse(
            &D2D1_ELLIPSE {
                point: D2D_POINT_2F { x: cx, y: cy },
                radiusX: outer_r,
                radiusY: outer_r,
            },
            brush,
            stroke,
            None,
        );
    } else if !outline_mode && dot_center {
        let _ = target.FillEllipse(
            &D2D1_ELLIPSE {
                point: D2D_POINT_2F { x: cx, y: cy },
                radiusX: dot_size,
                radiusY: dot_size,
            },
            brush,
        );
    } else {
        let _ = target.FillEllipse(
            &D2D1_ELLIPSE {
                point: D2D_POINT_2F { x: cx, y: cy },
                radiusX: outer_r,
                radiusY: outer_r,
            },
            brush,
        );
    }
}}

fn draw_arrow(
    target: &ID2D1DCRenderTarget,
    brush: &ID2D1SolidColorBrush,
    border_brush: Option<&ID2D1SolidColorBrush>,
    cx: f32,
    cy: f32,
    size: f32,
    thickness_h: f32,
    thickness_v: f32,
    dot_center: bool,
    border: bool,
    border_size: f32,
    space_width: f32,
    dot_size: f32,
) { unsafe {
    let half = size / 2.0;
    let sw = space_width.min(half);
    let arm = (half - sw) * 0.55;
    let half_t_h = thickness_h / 2.0;
    let half_t_v = thickness_v / 2.0;
    let bs = border_size;

    // Arrow line pairs: 0-3 are "horizontal" origin (use thickness_v), 4-7 are "vertical" origin (use thickness_h)
    let lines_h: [(D2D_POINT_2F, D2D_POINT_2F); 4] = [
        (D2D_POINT_2F { x: cx + sw, y: cy - half_t_v }, D2D_POINT_2F { x: cx + sw + arm, y: cy - arm }),
        (D2D_POINT_2F { x: cx + sw, y: cy + half_t_v }, D2D_POINT_2F { x: cx + sw + arm, y: cy + arm }),
        (D2D_POINT_2F { x: cx - sw, y: cy - half_t_v }, D2D_POINT_2F { x: cx - sw - arm, y: cy - arm }),
        (D2D_POINT_2F { x: cx - sw, y: cy + half_t_v }, D2D_POINT_2F { x: cx - sw - arm, y: cy + arm }),
    ];
    let lines_v: [(D2D_POINT_2F, D2D_POINT_2F); 4] = [
        (D2D_POINT_2F { x: cx - half_t_h, y: cy - sw }, D2D_POINT_2F { x: cx - arm, y: cy - sw - arm }),
        (D2D_POINT_2F { x: cx + half_t_h, y: cy - sw }, D2D_POINT_2F { x: cx + arm, y: cy - sw - arm }),
        (D2D_POINT_2F { x: cx - half_t_h, y: cy + sw }, D2D_POINT_2F { x: cx - arm, y: cy + sw + arm }),
        (D2D_POINT_2F { x: cx + half_t_h, y: cy + sw }, D2D_POINT_2F { x: cx + arm, y: cy + sw + arm }),
    ];

    if border && bs > 0.0 {
        if let Some(bb) = border_brush {
            let th = thickness_v + bs * 2.0;
            let tv = thickness_h + bs * 2.0;
            for &(p1, p2) in &lines_h {
                let _ = target.DrawLine(p1, p2, bb, th, None);
            }
            for &(p1, p2) in &lines_v {
                let _ = target.DrawLine(p1, p2, bb, tv, None);
            }
            if dot_center {
                let _ = target.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F { x: cx, y: cy },
                        radiusX: dot_size + bs,
                        radiusY: dot_size + bs,
                    },
                    bb,
                );
            }
        }
    }

    for &(p1, p2) in &lines_h {
        let _ = target.DrawLine(p1, p2, brush, thickness_v, None);
    }
    for &(p1, p2) in &lines_v {
        let _ = target.DrawLine(p1, p2, brush, thickness_h, None);
    }

    if dot_center {
        let _ = target.FillEllipse(
            &D2D1_ELLIPSE {
                point: D2D_POINT_2F { x: cx, y: cy },
                radiusX: dot_size,
                radiusY: dot_size,
            },
            brush,
        );
    }
}}
