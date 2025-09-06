struct FirstClass {
}
impl FirstClass {
    fn first_method(&self, a_number: i32) -> i32 {
        return a_number+8;
    }
}

/* 
 * basic X, Y position in 2D space
 */
struct Position2D {
    x: i32,
    y: i32,
}

/* 
 * X, Y, Z position in 3D space
 */
/*
struct Position3D {
    x: i32,
    y: i32,
    z: i32,
}
*/

struct Shape2D {
    grid_size: Position2D,
    vertices: Vec<Position2D>,
}
impl Shape2D {
    fn draw(&self) {
        /*
        // get grid size from maximum X and Y values
        let mut max_x = 0;
        let mut max_y = 0;
        for vertice in &self.vertices {
            if vertice.x > max_x { max_x = vertice.x; }
            if vertice.y > max_y { max_y = vertice.y; }
        }
        */

        // loop through every line (Y position)

        // get the shape's min and max Y
        let mut min_y: i32 = 0;
        let mut max_y: i32 = 0;
        let mut min_y_set: bool = false;
        for cursor_y in 0..=self.grid_size.y {
            for vertice in &self.vertices {
                if cursor_y == vertice.y {
                    if !min_y_set {
                        min_y = vertice.y;
                        min_y_set = true;
                    }

                    max_y = vertice.y;
                }
            }
        }

        let mut min_x: i32 = 0;
        let mut max_x: i32 = 0;
        for cursor_y in 0..=self.grid_size.y {
            // within every line, loop through every horizontal position (X)

            // first, get a line's min and max X so that we can fill up the shape
            let mut min_x_set: bool = false;
            for vertice in &self.vertices {
                if cursor_y == vertice.y {
                    if !min_x_set { min_x = vertice.x; min_x_set = true; }
                    if vertice.x > max_x { max_x = vertice.x; }
                }
                // we preserve the min_x if a line has no vertices
                // that way, it remembers
            }

            for cursor_x in 0..=self.grid_size.x {
                let mut found_vert = false;

                // on each position, check all vertices to see if any X&Y position matches
                for vertice in &self.vertices {
                    if cursor_x == vertice.x && cursor_y == vertice.y {
                        // position match! print a character
                        print!("o");
                        found_vert = true;
                    }
                }

                // fill up empty space when position doesn't match
                if !found_vert { 
                    if 
                        cursor_x > min_x && cursor_x < max_x
                        /*
                        &&
                        cursor_y > min_y && cursor_y < max_y 
                        */
                    {
                        // we're inside the shape
                        if cursor_y == min_y || cursor_y == max_y {
                            // we're at the top or bottom edge
                            print!("-");
                        } else {
                            // we're somewhere inbetween the left and right edge
                            print!(".");
                        }
                    } else if cursor_x == min_x || cursor_x == max_x {
                        print!("|");
                    } else {
                        // we're in blank space
                        print!(" ");
                    }
                }
            }
            print!(" minX:{min_x},maxX:{max_x}");
            println!();
        }

        println!("min_y: {min_y}, max_y: {max_y}");

        for vertice in &self.vertices {
            println!("vertice [X:{0}, Y:{1}]", vertice.x, vertice.y);
        }
    }
}

fn main() {
    let first_instance = FirstClass{};
    // println!("{}", first_instance.first_method(8));

    println!("line");
    let line = Shape2D{
        grid_size: Position2D{x:30, y:0},
        vertices: vec![
            Position2D{x:5, y:0},
            Position2D{x:25, y:0},
        ]
    };
    line.draw();
    println!();

    println!("square");
    let square = Shape2D{
        grid_size: Position2D{x:30, y:10},
        vertices: vec![
            Position2D{x:0, y:0},
            Position2D{x:30, y:0},
            Position2D{x:0, y:10},
            Position2D{x:30, y:10},
        ]
    };
    square.draw();
    println!();

    println!("weird shape lol");
    let weird_shape = Shape2D{
        grid_size: Position2D{x:10, y:5},
        vertices: vec![
            Position2D{x:2, y:0},
            Position2D{x:4, y: 0},
            Position2D{x:2, y:3},
            Position2D{x:5, y:3},
            Position2D{x:1, y:5},
            Position2D{x:10, y:5},
        ]
    };
    weird_shape.draw();
}
