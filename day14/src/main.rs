use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, Error};

#[derive(Debug)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

#[derive(Debug)]
struct Quadrant {
    x: (i32, i32),
    y: (i32, i32),
}

fn main() {
    let path = "./src/data.txt";
    let file = File::open(path).expect("Error opening file");
    let mut robots = get_robots_from_file(&file);
    assert_eq!(robots.len(), 500);

    // first star
    let answer = move_robots(&mut robots, (101, 103));
    println!("Answer 1: {:?}", answer);
}

fn move_robots(robots: &mut Vec<Robot>, frame: (i32, i32)) -> i32 {
    for i in 0..100 {
        for robot in &mut *robots {
            move_robot(robot, frame);
        }
    }

    println!("{:?}", robots);

    get_robots_sum_in_quadrant(robots, get_quadrant(frame))
}

fn move_robot(robot: &mut Robot, frame: (i32, i32)) -> () {
    if robot.position.1 + robot.velocity.1 < 0 {
        robot.position.1 = frame.1 + robot.position.1 + robot.velocity.1;
    } else if robot.position.1 + robot.velocity.1 > frame.1 - 1 {
        robot.position.1 = (robot.position.1 + robot.velocity.1) - frame.1;
    } else {
        robot.position.1 = robot.position.1 + robot.velocity.1;
    }

    if robot.position.0 + robot.velocity.0 > frame.0 - 1 {
        robot.position.0 = (robot.position.0 + robot.velocity.0) - frame.0;
    } else if robot.position.0 + robot.velocity.0 < 0 {
        robot.position.0 = (frame.0 + robot.position.0) + robot.velocity.0;
    } else {
        robot.position.0 = robot.position.0 + robot.velocity.0;
    }
}

fn get_robots_sum_in_quadrant(robots: &Vec<Robot>, quadrant: Vec<Quadrant>) -> i32 {
    let mut sum = 1;
    for quad in quadrant {
        let mut robot_count = 0;
        for robot in robots {
            if robot.position.0 >= quad.x.0
                && robot.position.0 <= quad.x.1
                && robot.position.1 >= quad.y.0
                && robot.position.1 <= quad.y.1
            {
                robot_count += 1;
            }
        }
        sum *= robot_count;
    }

    sum
}

fn get_quadrant(frame: (i32, i32)) -> Vec<Quadrant> {
    let quadrant_x_size = (frame.0 - 1) / 2;
    let quadrant_y_size = (frame.1 - 1) / 2;
    return vec![
        Quadrant {
            x: (0, quadrant_x_size - 1),
            y: (0, quadrant_y_size - 1),
        },
        Quadrant {
            x: (quadrant_x_size + 1, frame.0 - 1),
            y: (0, quadrant_y_size - 1),
        },
        Quadrant {
            x: (0, quadrant_x_size - 1),
            y: (quadrant_y_size + 1, frame.1 - 1),
        },
        Quadrant {
            x: (quadrant_x_size + 1, frame.0 - 1),
            y: (quadrant_y_size + 1, frame.1 - 1),
        },
    ];
}

fn get_robots_from_file(file: &File) -> Vec<Robot> {
    let mut robots = Vec::new();
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        let line = line.expect("Cannot get line");
        let parts: Vec<&str> = line.split_whitespace().collect();
        let pos_part = parts[0].strip_prefix("p=").unwrap();
        let vel_part = parts[1].strip_prefix("v=").unwrap();

        let parts: Vec<&str> = pos_part.split(',').collect();
        let px = parts[0].parse::<i32>().unwrap();
        let py: i32 = parts[1].parse::<i32>().unwrap();
        let parts: Vec<&str> = vel_part.split(',').collect();
        let vx = parts[0].parse::<i32>().unwrap();
        let vy: i32 = parts[1].parse::<i32>().unwrap();

        robots.push(Robot {
            position: (px, py),
            velocity: (vx, vy),
        });
    }
    robots
}

#[cfg(test)]
mod tests {
    use crate::Robot;

    #[test]
    fn test_basic() {
        let mut robots = vec![
            Robot {
                position: (0, 4),
                velocity: (3, -3),
            },
            Robot {
                position: (6, 3),
                velocity: (-1, -3),
            },
            Robot {
                position: (10, 3),
                velocity: (-1, 2),
            },
            Robot {
                position: (2, 0),
                velocity: (2, -1),
            },
            Robot {
                position: (0, 0),
                velocity: (1, 3),
            },
            Robot {
                position: (3, 0),
                velocity: (-2, -2),
            },
            Robot {
                position: (7, 6),
                velocity: (-1, -3),
            },
            Robot {
                position: (3, 0),
                velocity: (-1, -2),
            },
            Robot {
                position: (9, 3),
                velocity: (2, 3),
            },
            Robot {
                position: (7, 3),
                velocity: (-1, 2),
            },
            Robot {
                position: (2, 4),
                velocity: (2, -3),
            },
            Robot {
                position: (9, 5),
                velocity: (-3, -3),
            },
        ];
        let answer = super::move_robots(&mut robots, (7, 11));
        assert_eq!(answer, 12);
    }
}
