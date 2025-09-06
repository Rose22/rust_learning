/* 
 * basic X, Y position in 2D space
 */
struct Pos2D {
    x: u8,
    y: u8,
}

/* 
 * a 2D shape
 */
struct Shape2D {
    grid_size: Pos2D,
    vertices: Vec<Pos2D>,
}

impl Shape2D {
    fn draw(&self) {
        // get the shape's min and max Y
        let mut min_y: u8 = 0;
        let mut max_y: u8 = 0;
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

        let mut min_x: u8 = 0;
        let mut max_x: u8 = 0;
        // loop through every line (Y) of the grid
        for cursor_y in 0..=self.grid_size.y {
            // within every line, loop through every horizontal position (X)

            // first, get a line's min and max X so that we can fill up the shape
            let mut min_x_set: bool = false;
            for vertice in &self.vertices {
                if cursor_y == vertice.y {
                    if !min_x_set { min_x = vertice.x; min_x_set = true; }
                    if vertice.x != max_x { max_x = vertice.x; }
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
                        cursor_x >= min_x && cursor_x <= max_x
                        &&
                        cursor_y >= min_y && cursor_y <= max_y 
                    {
                        // we're inside the shape
                        if cursor_y == min_y || cursor_y == max_y {
                            // we're at the top or bottom edge
                            print!("-");
                        }
                        else if cursor_x == min_x || cursor_x == max_x {
                            // we're at the left or right edge
                            print!("|");
                        }
                        else {
                            // we're somewhere inbetween the left and right edge
                            print!("~");
                        }
                    } else {
                        // we're in blank space
                        print!(" ");
                    }
                }
            }

            // print!(" minX:{min_x},maxX:{max_x}");
            println!();
        }

        /*
        println!("min_y: {min_y}, max_y: {max_y}");

        for vertice in &self.vertices {
            println!("vertice [X:{0}, Y:{1}]", vertice.x, vertice.y);
        }
        */
    }
}

fn main() {
    let line = Shape2D{
        grid_size: Pos2D{x:30, y:0},
        vertices: vec![
            Pos2D{x:5, y:0},
            Pos2D{x:25, y:0},
        ]
    };
    println!("line:");
    line.draw();
    println!();

    let square = Shape2D{
        grid_size: Pos2D{x:30, y:10},
        vertices: vec![
            Pos2D{x:5, y:0},
            Pos2D{x:20, y:0},
            Pos2D{x:5, y:10},
            Pos2D{x:20, y:10},
        ]
    };
    println!("square:");
    square.draw();
    println!();

    let triangle = Shape2D{
        grid_size: Pos2D{x:30, y:5},
        vertices: vec![
            Pos2D{x:15, y:0},
            Pos2D{x:0, y:5},
            Pos2D{x:30, y:5},
        ]
    };
    println!("triangle:");
    triangle.draw();
    println!();

    let weird_shape = Shape2D{
        grid_size: Pos2D{x:10, y:5},
        vertices: vec![
            Pos2D{x:2, y:0},
            Pos2D{x:4, y: 0},
            Pos2D{x:2, y:3},
            Pos2D{x:5, y:3},
            Pos2D{x:1, y:5},
            Pos2D{x:10, y:5},
        ]
    };
    println!("weird shape lol:");
    weird_shape.draw();
    println!();

    let mario = Shape2D{
        grid_size: Pos2D{x:20, y:10},
        vertices: vec![
            // head
            Pos2D{x:5,y:0},
            Pos2D{x:9,y:0},

            Pos2D{x:4,y:1},
            Pos2D{x:13,y:1},

            Pos2D{x:4,y:2},
            Pos2D{x:10,y:2},

            Pos2D{x:3,y:3},
            Pos2D{x:13,y:3},

            Pos2D{x:3,y:4},
            Pos2D{x:14,y:4},

            Pos2D{x:3,y:5},
            Pos2D{x:12,y:5},

            Pos2D{x:5,y:6},
            Pos2D{x:11,y:6},

            // body
            Pos2D{x:1,y:7},
            Pos2D{x:12,y:7},

            Pos2D{x:1,y:8},
            Pos2D{x:11,y:8},
        ]
    };
    println!("mario! ..sorta");
    println!();
    mario.draw();
}
