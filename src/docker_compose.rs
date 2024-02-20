use std::{collections::HashMap, fs::File, io::Write};

/**
 * CPUS, 3.5
 * UID, 1
 * PORT, 8211
 * RCON_PORT, 27015
 * PLAYERS, 16
 * ADMIN_PWD, ilovepals
 * SERVER_NAME, My Palworld Server
 * SERVER_PWD, ""
 * PROM_CONF_FILE, $HOME/.prometheus/prometheus.yml
 */
pub const TEMPLATE: &str = r#"# docker-compose.yml
services:
  palworld:
    image: thijsvanloef/palworld-server-docker:latest
    cpus: {CPUS}
    mem_limit: 13g
    memswap_limit: 6g
    restart: unless-stopped
    container_name: palworld-{UID}
    ports:
      - {PORT}:8211/udp
      - {RCON_PORT}:27015/udp
      - {RCON_PORT}:27015
    environment:
      - PUID=1000
      - PGID=1000
      - PORT=8211
      - PLAYERS={PLAYERS}
      - MULTITHREADING=true
      - RCON_ENABLED=true
      - RCON_PORT=27015
      - ADMIN_PASSWORD={ADMIN_PWD}
      - COMMUNITY=false
      - SERVER_NAME={SERVER_NAME}
      - SERVER_PASSWORD={SERVER_PWD}
    volumes:
      - /palworld:/palworld/
  admin:
    image: jokerwho/palworld-server-tool:latest
    cpus: 0.5
    network_mode: host
    mem_limit: 256M
    memswap_limit: 4g
    restart: unless-stopped
    container_name: palworld-{UID}-admin
    environment:
      - WEB__PASSWORD=
      - RCON__ADDRESS=127.0.0.1:{RCON_PORT}
      - RCON__PASSWORD={ADMIN_PWD}
      - SAVE__PATH="/game"
      - SAVE__SYNC_INTERVAL=120
  prom:
    image: prom/prometheus
    volumes:
      - type: bind
        source: {PROM_CONF_FILE}
        target: /etc/prometheus/prometheus.yml
    ports:
      - 9090:9090
"#;

pub fn generate_docker_compose_for_dir(dir: &str, options: HashMap<&str, String>) {
    let mut file = File::create(format!("{}/docker-compose.yml", dir)).unwrap();
    let mut template = TEMPLATE.to_string();
    for (key, value) in options {
        template = template.replace(&format!("{{{}}}", key), &value);
    }
    file.write_all(template.as_bytes()).unwrap();
}
