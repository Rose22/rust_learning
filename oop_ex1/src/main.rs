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
    fn render(&self) {
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
                            print!(".");
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

/*
 * pixel grid
 */
struct Pix2D {
    grid_size: Pos2D,
    grid: Vec<Pos2D>,
}
impl Pix2D {
    pub fn new(x: u8, y: u8) -> Self {
        return Self {
            grid_size: Pos2D{x, y},
            grid: vec![],
        }
    }

    fn fill(&mut self, y: u8, x: u8) -> &mut Self {
        if x > self.grid_size.x || y > self.grid_size.y {
            panic!("out of grid bounds");
        }
        self.grid.push(Pos2D{x,y});
        return self;
    }

    /*
    fn fill_list(&mut self, list: Vec<(u8, u8)>) {
        for pos in list {
            self.fill(pos.1, pos.0);
        }
    }
    */

    fn fill_line(&mut self, y: u8, from_x: u8, to_x: u8) -> &mut Self {
        for x in from_x..=to_x {
            self.fill(y,x);
        }

        return self;
    }

    fn render(&mut self) {
        for y in 0..=self.grid_size.y {
            for x in 0..=self.grid_size.x {
                let mut found: bool = false;
                for pixel in &self.grid {
                    if pixel.x == x && pixel.y == y {
                        print!("o");
                        found = true;
                    }
                }
                if !found { print!("."); }
            }
            println!();
        }
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
    line.render();
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
    square.render();
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
    triangle.render();
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
    weird_shape.render();
    println!();

    println!("mario!");
    let mut mario = Pix2D::new(15,13);
    mario
        // cap
        .fill_line(0, 5, 9)
        .fill_line(1, 4, 12)

        // head
        .fill_line(2, 4, 10)
        .fill_line(3, 3, 12)
        .fill_line(4, 3, 13)
        .fill_line(5, 3, 12)
        .fill_line(6, 5, 11)

        // body
        .fill_line(7, 1, 12)
        .fill_line(8, 1, 11)
        .fill_line(9, 2, 11)

        // hand & arm
        .fill_line(0, 12, 14)
        .fill_line(1, 13, 14)
        .fill_line(2, 12, 14)
        .fill_line(3, 13, 14)
        .fill(4, 14)
        .fill(5, 13)
        .fill_line(6, 12, 13)

        // legs
        .fill(7, 15)
        .fill_line(8, 14, 15)
        .fill_line(9, 12, 15)
        .fill_line(10, 4, 15)
        .fill_line(11, 1, 15)
        .fill_line(12, 0, 9)
        .fill_line(13, 0, 2)
    ;
    mario.render();
}
