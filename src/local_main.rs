use std::fs::{self, File};
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

mod bfs_alg;

fn get_data(path: String) -> Vec<Vec<char>> {
    // fn get_data(path: String) -> Option<Vec<Vec<char>>> {
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    let mut data = Vec::new();
    for row in contents.split("\n") {
        data.push(row.chars().collect());
    }
    data
}

fn write_data(
    data: &Vec<Vec<char>>,
    min_path: &Vec<(usize, usize)>,
    path: String,
) -> std::io::Result<()> {
    let mut data1 = data.clone();
    for (i, j) in min_path {
        data1[*i][*j] = '.'
    }
    let mut result: String = "".to_string();
    for row in data1 {
        let str_row = row
            .iter()
            .fold(String::new(), |acc, &arg| acc + &arg.to_string());
        result.push_str(&str_row);
        result.push_str("\n");
    }
    let mut f = File::create(path)?;
    f.write_all(result.as_bytes())?;
    Ok(())
}

fn write_data_1(
    data: &Vec<Vec<char>>,
    points: &Vec<(usize, usize)>,
    path: String,
) -> std::io::Result<()> {
    let mut data1 = data.clone();
    for (i, j) in points {
        data1[*i][*j] = '*'
    }
    let mut result: String = "".to_string();
    for row in data1 {
        let str_row = row
            .iter()
            .fold(String::new(), |acc, &arg| acc + &arg.to_string());
        result.push_str(&str_row);
        result.push_str("\n");
    }
    let mut f = File::create(path)?;
    f.write_all(result.as_bytes())?;
    Ok(())
}

fn find_item(data: &Vec<Vec<char>>, item: char) -> Option<(usize, usize)> {
    for (i, row) in data.iter().enumerate() {
        for (j, el) in row.iter().enumerate() {
            if *el == item {
                return Some((i, j));
            }
        }
    }
    None
}

fn min_path_find() {
    let data = get_data("./src/fbs-03.txt".to_string());
    let start = find_item(&data, 'S').unwrap();
    let end = find_item(&data, 'T').unwrap();

    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let path = bfs_alg::bfs(&data, start, end, |item| item == 'W');
    let now1 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap() - now;

    println!("Size: {:?}, {:?}", data.len(), data[0].len());
    println!("start: {:?}", start);
    println!("end: {:?}", end);

    println!("bfs calculate: {:?}", now1);
    let path11 = match path {
        Some(v) => v,
        None => Vec::new(),
    };
    println!("Found path.len: {:?}", path11.len());
    write_data(&data, &path11, "./src/fbs-03-responce.txt".to_string()).unwrap();
}

fn matrix_print(matrix: &Vec<Vec<f64>>) {
    // for row in matrix {
    //     for col in row {
    //         print!("{:.2}\t", col);
    //     }
    //     print!("\n");
    // }
    println!("{}", serde_json::to_string(matrix).ok().unwrap());
    // for row in matrix {
    //     for col in row {
    //         print!("{:}\t", col);
    //     }
    //     print!("\n");
    // }
}

// fn main() {
//     min_path_find()
// }

// extern crate time;
mod distance_matrix_calculate;
mod travelling_salesman;
fn main_1() {
    let data = get_data("./src/geometry.txt".to_string());
    let points = &[
        (69, 19),
        (2, 1),
        (2, 7),
        (2, 14),
        (2, 22),
        (2, 31),
        (2, 40),
        (5, 17),
        (5, 20),
        (5, 35),
        (5, 44),
        (8, 3),
        (8, 13),
        (8, 30),
        (8, 39),
        (11, 1),
        (11, 17),
        (11, 36),
        (11, 46),
        (14, 4),
        (14, 16),
        (14, 31),
        (14, 42),
        (17, 13),
        (17, 22),
        (17, 36),
        (17, 45),
        (23, 3),
        // (23, 15),
        // (23, 36),
        // (23, 37),
        // (26, 19),
        // (26, 27),
        // (26, 46),
        // (29, 6),
        // (29, 28),
        // (29, 38),
        // (37, 52),
        // (37, 62),
        // (37, 68),
        // (40, 68),
        // (43, 58),
        // (46, 53),
        // (46, 64),
        // (49, 57),
        // (49, 66),
        // (52, 53),
        // (52, 60),
        // (55, 56),
        // (55, 67),
        // (67, 55),
        // (67, 65),
    ];

    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let matrix = distance_matrix_calculate::get_distances_matrix(points, |point1, point2| {
        let path = bfs_alg::bfs(&data, *point1, *point2, |item| item == 'W');
        match path {
            Some(v) => v.len() as f64,
            None => -1.0,
        }
    });
    let now1 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap() - now;
    // write_data_1(
    //     &data,
    //     &points.to_vec(),
    //     "./src/fbs-03-responce.txt".to_string(),
    // )
    // .unwrap();
    println!("points: {:?}", points.len());
    println!("get_distances_matrix: {:?}", now1);
    matrix_print(&matrix);
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let tour = travelling_salesman::simulated_annealing::solve_by_distance_matrix(
        &matrix,
        time::Duration::seconds(1),
    );
    let now1 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap() - now;
    println!(
        "simulated_annealing::simulated_annealing::solve: {:?}",
        now1
    );
    println!("Tour distance: {}, route: {:?}", tour.distance, tour.route);
}
