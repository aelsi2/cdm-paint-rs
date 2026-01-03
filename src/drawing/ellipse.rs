use super::DrawingCtx;
use crate::graphics::Color;
use crate::graphics::Point;
use core::cmp;

trait EllipseImpl {
    fn draw_pixel(ctx: &mut DrawingCtx, center: Point, point: Point, plus_off: Point, color: Color);
}

struct FilledEllipseImpl;
struct OutlineEllipseImpl;

impl EllipseImpl for OutlineEllipseImpl {
    fn draw_pixel(ctx: &mut DrawingCtx, center: Point, point: Point, plus_off: Point, color: Color) {
        let (xc, yc) = center.into();
        let (x, y) = point.into();
        let pt_plus_x = xc + x + plus_off.x() as i16;
        let pt_minus_x = xc - x;
        let pt_plus_y = yc + y + plus_off.y() as i16;
        let pt_minus_y = yc - y;
        ctx.draw_pixel_impl(Point::new(pt_plus_x, pt_plus_y), color);
        ctx.draw_pixel_impl(Point::new(pt_minus_x, pt_plus_y), color);
        ctx.draw_pixel_impl(Point::new(pt_plus_x, pt_minus_y), color);
        ctx.draw_pixel_impl(Point::new(pt_minus_x, pt_minus_y), color);
    }
}

impl EllipseImpl for FilledEllipseImpl {
    fn draw_pixel(ctx: &mut DrawingCtx, center: Point, point: Point, plus_off: Point, color: Color) {
        let (xc, yc) = center.into();
        let (x, y) = point.into();
        let pt_plus_x = xc + x + plus_off.x() as i16;
        let pt_minus_x = xc - x;
        let pt_plus_y = yc + y + plus_off.y() as i16;
        let pt_minus_y = yc - y;
        ctx.draw_horizontal_line(
            Point::new(pt_minus_x, pt_plus_y),
            Point::new(pt_plus_x, pt_plus_y),
            color,
        );
        ctx.draw_horizontal_line(
            Point::new(pt_minus_x, pt_minus_y),
            Point::new(pt_plus_x, pt_minus_y),
            color,
        );
    }
}

impl DrawingCtx {
    fn draw_ellipse<T: EllipseImpl>(&mut self, p1: Point, p2: Point, color: Color) {
        let x1 = cmp::min(p1.x(), p2.x());
        let x2 = cmp::max(p1.x(), p2.x());
        let y1 = cmp::min(p1.y(), p2.y());
        let y2 = cmp::max(p1.y(), p2.y());

        let rx = (x2 - x1) / 2;
        let ry = (y2 - y1) / 2;

        if rx == 0 || ry == 0 {
            self.draw_filled_rect(p1, p2, color);
            return;
        }
        self.mark_dirty_range(y1 as usize, y2 as usize);

        let center = Point::new((x1 + x2) / 2, (y1 + y2) / 2);
        let plus_off = Point::new((x1 ^ x2) & 1, (y1 ^ y2) & 1);
        let ryry = ry.pow(2);
        let rxrx = rx.pow(2);

        let mut x = 0;
        let mut y = ry;
        let mut dx = 0;
        let mut dy = rxrx * y * 2;

        let mut d1 = ryry - rxrx * ry + rxrx / 4;
        while dx < dy {
            let point = Point::new(x, y);
            T::draw_pixel(self, center, point, plus_off, color);

            if d1 < 0 {
                x += 1;
                dx += ryry * 2;
                d1 += dx + ryry;
            } else {
                x += 1;
                y -= 1;
                dx += ryry * 2;
                dy -= rxrx * 2;
                d1 += dx - dy + ryry;
            }
        }

        let mut d2 = ryry * ((x * 2 + 1).pow(2) / 4)
            + rxrx * ((y - 1).pow(2) - ryry);
        while y >= 0 {
            let point = Point::new(x, y);
            T::draw_pixel(self, center, point, plus_off, color);

            if d2 > 0 {
                y -= 1;
                dy -= rxrx * 2;
                d2 += rxrx - dy;
            } else {
                y -= 1;
                x += 1;
                dx += ryry * 2;
                dy -= rxrx * 2;
                d2 += dx - dy + rxrx;
            }
        }
    }

    pub fn draw_outline_ellipse(&mut self, p1: Point, p2: Point, color: Color) {
        self.draw_ellipse::<OutlineEllipseImpl>(p1, p2, color);
    }
    
    pub fn draw_filled_ellipse(&mut self, p1: Point, p2: Point, color: Color) {
        self.draw_ellipse::<FilledEllipseImpl>(p1, p2, color);
    }
}
