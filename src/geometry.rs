/// Types and Functions related to point and line geometry in a 2D plane

#[deriving(Show)]
struct Point<T> {x:T, y:T,}

#[deriving(Show)]
struct Line<T> {a:Point<T>, b:Point<T>}

//impl Add<Point, Point> for Point {
////impl Add<Point<T>, Point<T>> for Point<T> {
    //fn add(&self, _rhs: &Point<T>) -> Point<T> {
        //Point<T>{self.x + _rhs.x;
                //self.y + _rhs.y }
    //}
//}

#[cfg(test)]
mod test {
	extern crate test;

#[test]
    fn test_point_show() {
        use super::Point;
        let p1:Point<f64> = Point{x:1.0, y:2.0};
        println!("p1 is {}",p1);
    }
    fn test_line_show() {
        use super::{Point,Line};
        let p1:Point<f64> = Point{x:1.0, y:2.0};
        let p2:Point<f64> = Point{x:3.0, y:4.0};
        let l = Line{a:p1,b:p2};
        println!("l is {}",l);
    }

//#[test]
    //fn test_add_points() {
        //let p3 = p1 + p2;
    //}

//#[test]
    //fn test_line_intersect() {
        //let a:Line

    //}
}
