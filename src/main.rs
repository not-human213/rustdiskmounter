use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
#[derive(Serialize, Deserialize)]
struct State {
    last_notified: u64,
}

fn main() {
    dotenv().ok();
    let uuid = env::var("uuid").expect("disk uuid not found");
    let mount_point = env::var("mount").expect("mount location not found");
    let name = String::from_utf8_lossy(
        &Command::new("blkid")
            .args(["-U", &uuid])
            .output()
            .expect("cannot get disk name")
            .stdout,
    )
    .trim()
    .to_string();

    if !Path::new(&name).exists() {
        let mut state = read_state();
        let current_time = now();
        if current_time - state.last_notified >= 43200 {
            notify();
            state.last_notified = current_time;
            write_state(&state);
        }
        return;
    }

    let mut state = read_state();
    if state.last_notified != 0 {
        state.last_notified = 0;
        write_state(&state);
    }

    if !mount(&mount_point, &uuid) {
        return;
    }
    restart_docker_container();
}

fn notify() {
    let ntfy_topic = env::var("ntfy").expect("ntfy topic not fount");
    let ntfy_url = format!("https://ntfy.sh/{ntfy_topic}");
    let ntfy_com = Command::new("curl")
        .args(["-d", "disk disconnected", &ntfy_url])
        .status()
        .unwrap();
    if ntfy_com.success() {
        println!("notified successfully");
    } else {
        println!("some error in ntfy");
    }
    return;
}
fn mount(mount_point: &str, uuid: &str) -> bool {
    let mounted = Command::new("mountpoint")
        .args(["-q", &mount_point])
        .status()
        .unwrap();

    if mounted.success() {
        println!("already mounted");
        return false;
    }
    let status = Command::new("sudo")
        .args(["mount", "-U", &uuid, &mount_point])
        .status()
        .unwrap();
    if status.success() {
        println!("mounted");
        return true;
    } else {
        println!("mount failed");
        return false;
    }
}

fn restart_docker_container() {
    let ids = Command::new("docker").args(["ps", "-q"]).output().unwrap();
    let ids = String::from_utf8_lossy(&ids.stdout);

    println!("Containers \n{}", ids);
    let container_ids: Vec<&str> = ids.lines().collect();

    if !container_ids.is_empty() {
        let output = Command::new("sudo")
            .arg("docker")
            .arg("restart")
            .args(&container_ids)
            .output()
            .unwrap();

        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
}

fn read_state() -> State {
    match fs::read_to_string("state.json") {
        Ok(contents) => serde_json::from_str(&contents).unwrap_or(State { last_notified: 0 }),
        Err(_) => State { last_notified: 0 },
    }
}

fn write_state(state: &State) {
    fs::write("state.json", serde_json::to_string_pretty(state).unwrap()).unwrap();
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
