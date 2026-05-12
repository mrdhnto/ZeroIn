use windows::Win32::Graphics::Direct2D::Common::{D2D_RECT_F, D2D_POINT_2F};
use windows::Win32::Graphics::Direct2D::{D2D1_ELLIPSE, ID2D1DCRenderTarget, ID2D1SolidColorBrush};

use crate::config::CrosshairType;

pub fn draw(
    target: &ID2D1DCRenderTarget,
    brush: &ID2D1SolidColorBrush,
    ctype: CrosshairType,
    cx: f32,
    cy: f32,
    size: f32,
    thickness: f32,
    dot_center: bool,
    border: bool,
    space_width: f32,
) {
    match ctype {
        CrosshairType::Dot => draw_dot(target, brush, cx, cy, size),
        CrosshairType::Cross => draw_cross(target, brush, cx, cy, size, thickness, dot_center, space_width),
        CrosshairType::T => draw_t(target, brush, cx, cy, size, thickness, dot_center, space_width),
        CrosshairType::Circle => draw_circle(target, brush, cx, cy, size, thickness, dot_center, border, space_width),
    }
}

fn draw_dot(
    target: &ID2D1DCRenderTarget,
    brush: &ID2D1SolidColorBrush,
    cx: f32,
    cy: f32,
    size: f32,
) {
    let radius = size / 2.0;
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

fn draw_cross(
    target: &ID2D1DCRenderTarget,
    brush: &ID2D1SolidColorBrush,
    cx: f32,
    cy: f32,
    size: f32,
    thickness: f32,
    dot_center: bool,
    space_width: f32,
) {
    let half = size / 2.0;
    let half_t = thickness / 2.0;
    let sw = space_width.min(half);

    unsafe {
        draw_split_rect(target, brush, cx - half, cy - half_t, cx - sw, cy + half_t);
        draw_split_rect(target, brush, cx + sw, cy - half_t, cx + half, cy + half_t);
        draw_split_rect(target, brush, cx - half_t, cy - half, cx + half_t, cy - sw);
        draw_split_rect(target, brush, cx - half_t, cy + sw, cx + half_t, cy + half);

        if dot_center {
            let _ = target.FillEllipse(
                &D2D1_ELLIPSE {
                    point: D2D_POINT_2F { x: cx, y: cy },
                    radiusX: 1.5,
                    radiusY: 1.5,
                },
                brush,
            );
        }
    }
}

fn draw_t(
    target: &ID2D1DCRenderTarget,
    brush: &ID2D1SolidColorBrush,
    cx: f32,
    cy: f32,
    size: f32,
    thickness: f32,
    dot_center: bool,
    space_width: f32,
) {
    let half = size / 2.0;
    let half_t = thickness / 2.0;
    let sw = space_width.min(half);

    unsafe {
        draw_split_rect(target, brush, cx - half, cy - half_t, cx - sw, cy + half_t);
        draw_split_rect(target, brush, cx + sw, cy - half_t, cx + half, cy + half_t);
        draw_split_rect(target, brush, cx - half_t, cy - half_t, cx + half_t, cy - sw);
        draw_split_rect(target, brush, cx - half_t, cy + sw, cx + half_t, cy + half);

        if dot_center {
            let _ = target.FillEllipse(
                &D2D1_ELLIPSE {
                    point: D2D_POINT_2F { x: cx, y: cy },
                    radiusX: 1.5,
                    radiusY: 1.5,
                },
                brush,
            );
        }
    }
}

fn draw_circle(
    target: &ID2D1DCRenderTarget,
    brush: &ID2D1SolidColorBrush,
    cx: f32,
    cy: f32,
    size: f32,
    thickness: f32,
    dot_center: bool,
    border: bool,
    space_width: f32,
) {
    let outer_r = size / 2.0;
    let inner_dot_r = (thickness * 0.5).max(1.5);

    unsafe {
        if border && dot_center {
            let max_dot = (outer_r - thickness / 2.0 - space_width).max(0.0);
            let actual_dot_r = inner_dot_r.min(max_dot);

            let _ = target.DrawEllipse(
                &D2D1_ELLIPSE {
                    point: D2D_POINT_2F { x: cx, y: cy },
                    radiusX: outer_r,
                    radiusY: outer_r,
                },
                brush,
                thickness,
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
        } else if border && !dot_center {
            let _ = target.DrawEllipse(
                &D2D1_ELLIPSE {
                    point: D2D_POINT_2F { x: cx, y: cy },
                    radiusX: outer_r,
                    radiusY: outer_r,
                },
                brush,
                thickness,
                None,
            );
        } else if !border && dot_center {
            let _ = target.FillEllipse(
                &D2D1_ELLIPSE {
                    point: D2D_POINT_2F { x: cx, y: cy },
                    radiusX: inner_dot_r,
                    radiusY: inner_dot_r,
                },
                brush,
            );
        } else {
            let _ = target.FillEllipse(
                &D2D1_ELLIPSE {
                    point: D2D_POINT_2F { x: cx, y: cy },
                    radiusX: 1.5,
                    radiusY: 1.5,
                },
                brush,
            );
        }
    }
}
