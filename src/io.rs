use std::fs::File;
use std::io::{Write, Error};


pub fn write_2d_arr<Matrix: AsRef<[Row]>, Row: AsRef<[i8]>>(features: Matrix, file: &str) {
    let mut output = File::create(file).unwrap();
    for row in features.as_ref() {
        for cell in row.as_ref() {
            write!(output, "{} ", cell).unwrap();
        }
        write!(output, "\n").unwrap();
    };
}

pub fn read_2d_arr<Matrix: AsRef<[Row]>, Row: AsRef<[i8]>>(features: Matrix, file: &str, ) {

}