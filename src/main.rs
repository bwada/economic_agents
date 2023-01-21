mod agents;
mod io;
mod simulation;

fn main() {
    // //let test_agent = agents::make_agent();
    // //println!("{}", test_agent.is_alive());
    // let arr: [[u8; 2]; 2] = [[1,2],[3,4]];
    // let arr2: [[u8; 2]; 3] = [[1,2],[4,5],[7,8]];
    // let arr3: [[u8; 3]; 3] = [[1,2,3],[4,5,6],[7,8,9]];
    // let testy = arr[0];
    // // io::write_2d_arr(&arr,  String::from("test.txt"));
    // // io::write_2d_arr(&arr2,  String::from("test.txt"));
    // io::write_2d_arr(arr, "test1.txt");
    // io::write_2d_arr(arr2, "test2.txt");
    const NUM_AGENTS: usize = 300;
    let readout = simulation::specialization_1(NUM_AGENTS, 10);
    io::write_2d_arr(readout, "output.txt")

}
