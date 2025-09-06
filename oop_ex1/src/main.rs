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
    vertices: Vec<Pos2D>,
}
impl Shape2D {
    pub fn new() -> Self {
        return Self {
            vertices: vec![],
        }
    }

    pub fn add_vertice(&mut self, x: u8, y: u8) -> &mut Self {
        self.vertices.push(Pos2D{x,y});
        return self
    }

    fn render(&self) {
        /*
         * get the shape's max X and Y (to determine grid size)
         * as well as min Y (to determine where the shape starts)
         */
        let mut grid_max_x: u8 = 0;
        let mut grid_max_y: u8 = 0;
        let mut grid_min_y: u8 = 0;
        let mut grid_min_y_found: bool = false;
        for vertice in &self.vertices {
            if !grid_min_y_found { grid_min_y = vertice.y; grid_min_y_found = true; }

            if vertice.x > grid_max_x { grid_max_x = vertice.x; }
            if vertice.y > grid_max_y { grid_max_y = vertice.y; }
        }

        let mut line_min_x: u8 = 0;
        let mut line_max_x: u8 = 0;
        // loop through every line (Y) of the grid
        for cursor_y in 0..=grid_max_y {
            // within every line, loop through every horizontal position (X)

            // first, get a line's min and max X so that we can fill up the shape
            let mut line_min_x_found: bool = false;
            for vertice in &self.vertices {
                if cursor_y == vertice.y {
                    if !line_min_x_found { line_min_x = vertice.x; line_min_x_found = true; }
                    if vertice.x != line_max_x { line_max_x = vertice.x; }
                }
                // we preserve the line_min_x if a line has no vertices
                // that way, it remembers
            }

            for cursor_x in 0..=grid_max_x {
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
                    if cursor_x >= line_min_x && cursor_x <= line_max_x {
                        // we're inside the shape
                        //
                        if cursor_y == grid_min_y || cursor_y == grid_max_y {
                            // we're at the top or bottom edge
                            print!("-");
                        }
                        else if cursor_x == line_min_x || cursor_x == line_max_x {
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

            // print!(" minX:{line_min_x},maxX:{line_max_x}");
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

    println!();

    println!("line:");
    let mut line = Shape2D::new();
    line
        .add_vertice(1,0)
        .add_vertice(30,0)
    .render();
    println!();

    println!("square:");
    let mut square = Shape2D::new();
    square
        .add_vertice(1,0)
        .add_vertice(10,0)
        .add_vertice(1,3)
        .add_vertice(10,3)
    .render();
    println!();

    println!("triangle:");
    let mut triangle = Shape2D::new();
    triangle
        .add_vertice(5,0)
        .add_vertice(0,3)
        .add_vertice(10,3)
    .render();
    println!();

    println!("hexagon:");
    let mut hexagon = Shape2D::new();
    hexagon
        // top edge
        .add_vertice(10,0)
        .add_vertice(20,0)

        // slope (top left)
        .add_vertice(7,1)
        .add_vertice(3,2)
        // slope (bottom left)
        .add_vertice(3,7)
        .add_vertice(7,8)

        // left edge
        .add_vertice(0, 3)
        .add_vertice(0, 6)

        // slope (top right)
        .add_vertice(23,1)
        .add_vertice(27,2)
        // slope (bottom right)
        .add_vertice(27,7)
        .add_vertice(23,8)

        // right edge
        .add_vertice(30, 3)
        .add_vertice(30, 6)

        // bottom edge
        .add_vertice(10, 9)
        .add_vertice(20, 9)
    .render();
    println!("super hexagon?!");
    println!();

    println!("bottle");
    let mut bottle = Shape2D::new();
    bottle
        // bottle opening
        .add_vertice(4,0)
        .add_vertice(6,0)
        .add_vertice(4,3)
        .add_vertice(6,3)

        // slope
        .add_vertice(3,4)
        .add_vertice(7,4)
        .add_vertice(1,5)
        .add_vertice(9,5)

        // body
        .add_vertice(1,10)
        .add_vertice(9,10)
    .render();
    println!();

    println!("letter R");
    let mut letter_r = Shape2D::new();
    letter_r
        .add_vertice(0,0)
        .add_vertice(15,0)

        .add_vertice(0,1)
        .add_vertice(15,1)

        .add_vertice(0,2)
        .add_vertice(4,2)

        .add_vertice(0,7)
        .add_vertice(4,7)
    .render();
}
