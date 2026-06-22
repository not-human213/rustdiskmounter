use std::process::Command;
use std::path::Path; 
use std::env;
use dotenvy::dotenv;

fn notify(){
	let ntfy_topic = env::var("ntfy").expect("ntfy topic not fount");
	let ntfy_url = format!("https://ntfy.sh/{ntfy_topic}");
	let ntfy_com = Command::new("curl")
	.args(["-d", "disk disconnected", &ntfy_url])
	.status().unwrap();
	if ntfy_com.success(){
	println!("notified successfully");
	}
	else{
	println!("some error in ntfy");}
	return;
}
fn main() {

 dotenv().ok();
 let uuid = env::var("uuid")
	.expect("disk uuid not found");
 let mount_point = env::var("mount")
	.expect("mount location not found");
 let name = String::from_utf8_lossy(&Command::new("blkid")
	.args(["-U", &uuid]).output().expect("cannot get disk name").stdout,)
	.trim().to_string();
if !Path::new(&name).exists(){
	notify();
return;
}
let mounted = Command::new("mountpoint").args(["-q", &mount_point]).status().unwrap();

if mounted.success(){
	println!("already mounted");
return;
	}
let status = Command::new("sudo").args(["mount", "-U" ,&uuid, &mount_point]).status().unwrap();
if status.success()
{
	println!("mounted");
}
else{
	println!("mount failed");
	return;
}
let ids = Command::new("docker").args(["ps", "-q"]).output().unwrap();
let ids = String::from_utf8_lossy(&ids.stdout);

println!("Containers \n{}",ids);
let container_ids: Vec<&str> = ids.lines().collect();

if !container_ids.is_empty(){

	let output = Command::new("sudo")
	.arg("docker")
	.arg("restart")
	.args(&container_ids)
	.output()
	.unwrap();

println!("{}",String::from_utf8_lossy(&output.stdout));
}
}
