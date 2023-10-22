#![allow(unused_must_use)]

mod paint;
mod misc;

use num_cpus;
use std::process::Command;
use serde_json::Value;
use misc::*;

// For each physical CPU parse its temperature from lm-sensors output then apply color and print.
fn main() {
    loop {
        let cpus = num_cpus::get_physical();
        let output = Command::new("sensors")
            .arg("--no-adapter")
            .arg("-j")
            .output()
            .expect("lm-sensors not found.")
            .stdout;
        clearscreen();
        println!("CPU temperature: \n");
        let json: Value = serde_json::from_str(std::str::from_utf8(&output).unwrap()).expect("fail");
        for cpu in 0..cpus {
            let cpudata = gencore(cpu);
            let celsius: f64 = json["coretemp-isa-0000"][&cpudata[0]][&cpudata[1]].to_string().parse::<f64>().unwrap() as f64;
            println!("Core {}: {} °C", cpu, paint::apply(celsius));
        }
        // lm-sensors update every second, there is no point in lowering this number.
        sleep(1000);
    }
}

fn gencore(ncpu: usize) -> Vec<String> {
    let core: String = format!("{}{}", "Core ", ncpu);
    let degs: String = format!("temp{}_input", ncpu + 2);
    return vec![core, degs];
}


