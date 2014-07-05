use std::num::abs;

// idea: have visual debugging of algorithm
// demonstrate mapping between algorithm, variables, and the actual visual
// output, step by step.

fn main()
{
    // A simple integer calculator:
    // `+` or `-` means add or subtract by 1
    // `*` or `/` means multiply or divide by 2

    let program = "+ + * - /";
    let mut accumulator = 0i;

    for token in program.chars() {
        match token {
            '+' => accumulator += 1,
            '-' => accumulator -= 1,
            '*' => accumulator *= 2,
            '/' => accumulator /= 2,
            _ => { /* ignore everything else */ }
        }
    }

    println!("The program \"{}\" calculates the value {}",
              program, accumulator);
    println!("LINE!");
    plotLine(0,0,5,10);
    println!("CIRCLE!");
    plotCircle(5, 5, 3);
}

fn setPixel(x: int, y: int)
{
  println!("Setting pixel ({}, {})", x, y);
}

fn plotLine(mut x0: int, mut y0: int, x1: int, y1: int)
{
   let dx:int =  abs(x1 - x0);
   let sx:int = if x0 < x1 { 1 } else { -1 };
   let dy:int = -1*abs(y1 - y0);
   let sy:int = if y0 < y1 { 1 } else { -1 };
   let mut err:int = dx+dy; /* error value e_xy */

   loop {  /* loop */
      setPixel(x0, y0);
      if x0 == x1 && y0 == y1 { break }

      let e2 = 2*err;
      if e2 >= dy {
        err += dy;
        x0 += sx;
      } /* e_xy+e_x > 0 */
      if e2 <= dx {
        err += dx;
        y0 += sy;
      } /* e_xy+e_y < 0 */
   }
}

fn plotCircle(xm: int, ym: int, mut r: int)
{
   let mut x:int = -r;
   let mut y:int = 0;
   let mut err:int =  2 - 2*r; /* II. Quadrant */
   loop {
      setPixel(xm - x, ym + y); /*   I. Quadrant */
      setPixel(xm - y, ym - x); /*  II. Quadrant */
      setPixel(xm + x, ym - y); /* III. Quadrant */
      setPixel(xm + y, ym + x); /*  IV. Quadrant */
      r = err;
      if r <= y {
        /* e_xy+e_y < 0 */
        y += 1;
        err += y*2 + 1;
      }
      if r > x || err > y {
        /* e_xy+e_x > 0 or no 2nd y-step */
        x += 1;
        err += x*2 + 1;
      }
      
      if x >= 0 { break }
   }
}
