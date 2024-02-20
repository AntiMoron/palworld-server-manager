use std::process::Command;

fn execute_bash_command(command: &str) {
  let output = Command::new("bash")
      .arg("-c")
      .arg(command)
      .output();

  if let Ok(output) = output {
      if output.status.success() {
          let stdout = String::from_utf8_lossy(&output.stdout);
          println!("Command execution succeeded:\n{}", stdout);
      } else {
          let stderr = String::from_utf8_lossy(&output.stderr);
          println!("Command execution failed:\n{}", stderr);
      }
  } else {
      println!("Failed to execute command");
  }
}

pub const INIT_DOCKER: &str = r#"
#!/bin/bash
init_docker() {
  if command -v docker &> /dev/null; then
      echo "Docker is installed. Pass"
  else
      echo "Docker is not installed. Installing..."
      sudo apt-get update -y
      sudo apt-get install ca-certificates curl gnupg -y
      sudo install -m 0755 -d /etc/apt/keyrings
      curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg
      sudo chmod a+r /etc/apt/keyrings/docker.gpg

      # Add the repository to Apt sources:
      echo \
        "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/ubuntu \
        $(. /etc/os-release && echo "$VERSION_CODENAME") stable" | \
        sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
      sudo apt-get update -y
      sudo apt-get install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin -y
      sudo docker pull thijsvanloef/palworld-server-docker:latest
  fi
}

init_docker
"#;

pub fn init_docker() {
  execute_bash_command(&INIT_DOCKER);
}