use std::process::Command;
use std::path::Path; 

fn main() {
 let name = "/dev/sdb1";
 let mount_point = "/mnt/disk";
 if !Path::new(name).exists(){
	println!("not present");
return;
}
let mounted = Command::new("mountpoint").args(["-q", mount_point]).status().unwrap();

if mounted.success(){
	println!("already mounted");
return;
	}
let status = Command::new("sudo").args(["mount", name, mount_point]).status().unwrap();

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
if status.success(){
println!("mounted");
}
else{
println!("mount failed");
}
}	

