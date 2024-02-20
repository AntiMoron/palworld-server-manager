use commander_rust::{command, entry, option, run, Cli};
use dialoguer::{Confirm, Input, Password};
use std::{fs};
use std::collections::HashMap;
use num_cpus;
mod util;
mod script;
mod docker_compose;

fn get_cpu_count_minus_half() -> f32 {
    let cpus = num_cpus::get();
    let minus_half = (cpus as f32) - 0.5;
    minus_half
}


fn create_cli_folder() {
    let folder_path = util::get_cli_base_folder();
    util::create_folder(&folder_path)
}

fn create_user_folder(user_name: &str) {
    let folder_path = util::get_user_folder(user_name);
    util::create_folder(&folder_path)
}

#[command(deploy, "create a new server of Palworld")]
fn deploy() {
    let public_server: bool = Confirm::new()
        .with_prompt("Do you want it public server?")
        .default(false)
        .interact()
        .unwrap();

    let user_name = "default_server";

    let server_name: String = Input::new()
        .with_prompt("Set your server's name(Length no longer than 20 chars)")
        .validate_with(|input: &String| {
            if input.chars().count() > 20 {
                Err("Name should have length no longer than 20 chars".to_string())
            } else {
                Ok(())
            }
        })
        .interact()
        .unwrap();

    let server_player = Input::new()
        .with_prompt("Set your server's max players (default 16)")
        .default(16)
        .interact()
        .unwrap();

    let server_cpus = Input::new()
        .with_prompt("Set how many cpu you use")
        .default(format!("{}", get_cpu_count_minus_half()))
        .interact()
        .unwrap();

    let server_password: String = Password::new()
        .with_prompt("Set your server's password (press enter to skip)")
        .with_confirmation(
            "Please re-input your server's password to make sure you don't have typo",
            "Password not matched with last one",
        )
        .allow_empty_password(true)
        .interact()
        .unwrap();

    let admin_password: String = Password::new()
        .with_prompt(
            "Set your admin password. You can have permissions, e.g. to teleport to other players",
        )
        .with_confirmation(
            "Please re-input your server's password to make sure you don't have typo",
            "Password not matched with last one",
        )
        .interact()
        .unwrap();

    println!("Public server: {}", public_server);
    println!("Server password: {}", server_password);
    println!("Admin password: {}", admin_password);
    println!("Server name: {}", server_name);
    println!("Server cpus: {}", server_cpus);
    println!("Server players: {}", server_player);
    // create a folder
    create_cli_folder();
    // create a folder for user
    create_user_folder(&user_name);
    // generate docker-compose.yml
    let user_folder = util::get_user_folder(&user_name);
    let player = server_player.to_string();
    // init docker
    script::init_docker();
    // create prometheus config
    let prom_dir = format!("{}/.prometheus/prometheus.yml", &util::get_home_dir());

    // create docker compose file
    docker_compose::generate_docker_compose_for_dir(
        &user_folder,
        HashMap::from([
            ("CPUS", server_cpus),
            ("UID" , user_name.to_string()),
            ("RCON_PORT", util::get_random_safe_port().to_string()),
            ("PORT", util::get_random_safe_port().to_string()),
            ("PLAYERS", player),
            ("ADMIN_PWD", admin_password),
            ("SERVER_NAME", server_name),
            ("SERVER_PWD", server_password),
            ("PROM_CONF_FILE", prom_dir),
        ]),
    );
    // start containers
    script::execute_bash_command(format!("cd {} && sudo docker compose up -d", &user_folder));
}

// options here are public, options above `#[command]` are private
#[option(-s, --silently <silently>, "won't display anything")]
#[entry]
fn main() {
    let brief = "One key deploy server of Palworld.";
    // Run it now
    let app = run!();
}
