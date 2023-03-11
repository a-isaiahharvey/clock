use std::io::{self, Write};

use clock::stopwatch::{format_time, Stopwatch};

fn main() {
    let mut stopwatch = Stopwatch::new();

    loop {
        let mut input = String::new();

        print!("Enter a command (start, stop, lap, quit): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "start" => {
                stopwatch.start();
                println!("Stopwatch started!");
            }
            "stop" => {
                stopwatch.stop();
                println!("Stopwatch stopped!");
            }
            "lap" => {
                stopwatch.add_lap();
                let lap_times = stopwatch.lap_times();

                // Print table header
                println!("{:<10}  {:<15}  {:<15}", "Lap", "Split Time", "Total Time");

                // Print table rows
                for lap in lap_times {
                    println!(
                        "{:<10}  {:<15}  {:<15}",
                        lap.lap_number(),
                        format_time(lap.split_time()),
                        format_time(lap.total_time())
                    );
                }
            }
            "quit" => break,
            _ => println!("Invalid command."),
        }
    }
}
