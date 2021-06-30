use termion::raw::IntoRawMode;
use std::io::{Read, Write, stdout};
use termion::async_stdin;
use rand::Rng;
use std::{thread, time};

enum Directions {
    Left,
    Right,
    Up,
    Down,
    None,
}

const ROWS: usize = 20;
const COLS: usize = 20;


fn main() {
    let interval = time::Duration::from_millis(100);
    let mut snake: Vec<(usize, usize)> = Vec::new();
    snake.push((ROWS/2, COLS/2)); //first cell of the snake
    let out = stdout();
    let mut out = out.lock()
                     .into_raw_mode()
                     .unwrap();
    let mut stdin = async_stdin().bytes();
    write!(out,
            "{}{}",
            termion::clear::All,
            termion::cursor::Goto(1,1))
            .unwrap();
    //this is true if the apple has not been eaten yet; false otherwise
    let mut apple_state: bool = true;
    //setting the default direction. that is, no direction at all.
    let mut direction: Directions = Directions::None;
    let mut points: usize = 0;
    let mut apple: (usize, usize) = gen_apple();
    //game loop
    loop {
        //moving the snake one step forward based on its direction
        match direction {
            Directions::Up => {
                snake.insert(0, (snake[0].0, snake[0].1 - 1));
                snake.pop();
            },
            Directions::Down => {
                snake.insert(0, (snake[0].0, snake[0].1 + 1));
                snake.pop();
            },
            Directions::Left => {
                snake.insert(0, (snake[0].0 - 1, snake[0].1));
                snake.pop();
            },
            Directions::Right => {
                snake.insert(0, (snake[0].0 + 1, snake[0].1));
                snake.pop();
            },
            _ => {},
        }

        let ch = stdin.next();
        //checking the collision between the head of the snake and the apple
        if (snake[0].0 ==  apple.0) && (snake[0].1 == apple.1) {
            apple_state = false;
        }
        //instantiating the apple if not already there
        if apple_state == false {
            apple = gen_apple();
            apple_state = true;
            grow_snake(&mut snake, &direction);
            points = points + 1;
        }
        //checking on what keys are pressed
        match ch {
            Some(Ok(b'q')) => {
                //end the game
                break;
            },
            Some(Ok(b'w')) => {
                //if the snake is currently going right or left, make it go up
                //if the snake is currently going down or up, do nothing
                match direction {
                    Directions::Down => {},
                    Directions::Up => {},
                    Directions::Left => {
                        snake.insert(0, (snake[0].0, snake[0].1 - 1));
                        snake.pop();
                        direction = Directions::Up;
                    },
                    Directions::Right => {
                        snake.insert(0, (snake[0].0, snake[0].1 - 1));
                        snake.pop();
                        direction = Directions::Up;
                    },
                    Directions::None => {
                        snake.insert(0, (snake[0].0, snake[0].1 - 1));
                        snake.pop();
                        direction = Directions::Up;
                    },
                }
            },
            Some(Ok(b'a')) => {
                //if the snake is currently going up, or down, make it go right
                //if the snake is currently going right, or left, do nothing
                match direction {
                    Directions::Down => {
                        snake.insert(0, (snake[0].0 - 1, snake[0].1));
                        snake.pop();
                        direction = Directions::Left;
                    },
                    Directions::Up => {
                        snake.insert(0, (snake[0].0 - 1, snake[0].1));
                        snake.pop();
                        direction = Directions::Left;
                    },
                    Directions::Left => {},
                    Directions::Right => {},
                    _ => {},
                }
            },
            Some(Ok(b's')) => {
                //if the snake is currently going right or left, make it go down
                //if the snake is currently going down or up, do nothing
                match direction {
                    Directions::Down => {},
                    Directions::Up => {},
                    Directions::Left => {
                        snake.insert(0, (snake[0].0, snake[0].1 + 1));
                        snake.pop();
                        direction = Directions::Down;
                    },
                    Directions::Right => {
                        snake.insert(0, (snake[0].0, snake[0].1 + 1));
                        snake.pop();
                        direction = Directions::Down;
                    },
                    _ => {
                        snake.insert(0, (snake[0].0, snake[0].1 + 1));
                        snake.pop();
                        direction = Directions::Down;
                    },
                }
            },
            Some(Ok(b'd')) => {
                //if the snake is currently going up or down, make it go left
                //if the snake is currently going right or left, do nothing
                match direction {
                    Directions::Down => {
                        snake.insert(0, (snake[0].0 + 1, snake[0].1));
                        snake.pop();
                        direction = Directions::Right;
                    },
                    Directions::Up => {
                        snake.insert(0, (snake[0].0 + 1, snake[0].1));
                        snake.pop();
                        direction = Directions::Right;
                    },
                    Directions::Left => {},
                    Directions::Right => {},
                    _ => {},
                }
            },
            _ => {},
        };
        out.flush().unwrap();
        write!(out, "{}{}",
                termion::clear::All,
                termion::cursor::Goto(1, 1))
                .unwrap();
        out.flush().unwrap();
        write!(out, "{}points: {}",
                termion::cursor::Goto(1, 1),
                points)
                .unwrap();
        write!(out, "{}snake x: {}, snake y: {}",
                termion::cursor::Goto(1, 2),
                snake[0].0,
                snake[0].1)
                .unwrap();

        write!(out, "{}apple x: {}, apple y {}",
                termion::cursor::Goto(1, 3),
                apple.0,
                apple.1)
                .unwrap();
        print_grid(&snake, &apple);
        out.flush().unwrap();
        //asking for a character in input
        thread::sleep(interval);
    }

}


fn gen_apple() -> (usize, usize) {
    (rand::thread_rng().gen_range(1..ROWS),
     rand::thread_rng().gen_range(1..COLS))
}


fn print_apple(apple: &(usize, usize)) {
    print!("\x1b[{};{}H@", apple.1, apple.0);
}


fn print_grid(snake: &Vec<(usize, usize)>, apple: &(usize, usize))
{
    print_apple(apple);
    for i in 0..snake.len() {
        print!("\x1b[{};{}Ho", snake[i].1, snake[i].0);
    }
}


fn grow_snake(snake: &mut Vec<(usize, usize)>, direction: &Directions) {
    match snake.len() {
        1 => {
            match direction {
                Directions::Up => {
                    snake.push((snake[0].0, snake[0].1 + 1));
                },
                Directions::Down => {
                    snake.push((snake[0].0, snake[0].1 - 1));
                },
                Directions::Left => {
                    snake.push((snake[0].0 + 1, snake[0].1));
                },
                Directions::Right => {
                    snake.push((snake[0].0 - 1, snake[0].1));
                },
                _ => {}, //unused, it's impossible for the snake to not
                //have a direction
            }
        }
        _ => {
            let last_snake_cell = snake[snake.len() - 1];
            let snd_last_snake_cell = snake[snake.len() - 2];
            //they're both (x, y) 2-uples.
            //if they're laid horizontally...
            if last_snake_cell.1 == snd_last_snake_cell.1 {
                //...add one in the line, with x equal to the one of the last cell - 1
                snake.push((last_snake_cell.0 - 1, last_snake_cell.1));
            //else if they're laid vertically...
            } else {
                //...add one in the vertical line, with y equal to the one of the last
                //cell + 1, since the terminal cells grow downward.
                snake.push((last_snake_cell.0, last_snake_cell.1 + 1));
            }
        },
    }
}
