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
        for y in 0..=self.grid_size.y {
            // within every line, loop through every horizontal position (X)
            for x in 0..=self.grid_size.x {
                let mut printed = false;

                // on each position, check all vertices to see if any X&Y position matches
                for vertice in &self.vertices {
                    if x == vertice.x && y == vertice.y {
                        // position match! print a character
                        
                        // use previous position to determine what character to draw
                        print!("x");
                        printed = true;
                    }
                }

                // fill up empty space when position doesn't match
                if !printed { 
                    print!(".");
                }
            }
            println!();
        }

        for vertice in &self.vertices {
            println!("vertice [{0}, {1}]", vertice.x, vertice.y);
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
            Position2D{x:30, y:10},
            Position2D{x:0, y:10},
        ]
    };
    square.draw();
    println!();

    println!("weird shape lol");
    let weird_shape = Shape2D{
        grid_size: Position2D{x:10, y:5},
        vertices: vec![
            Position2D{x:0, y:0},
            Position2D{x: 3, y: 0},
            Position2D{x:5, y:3},
            Position2D{x:0, y:3},
            Position2D{x:10, y:5},
        ]
    };
    weird_shape.draw();
}
