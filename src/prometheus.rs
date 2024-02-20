mod script;

pub const prometheus_yml: &str = r#"global:
scrape_interval: 15s

scrape_configs:
- job_name: node
static_configs:
- targets: ['localhost:9100']"#:

pub fn init_node_exporter() {
  const node_exporter_bash = r#"#!/bin/bash
export prom_ver="2.45.3"
export os="linux"
export arch="amd64"
prom_folder="$HOME/.prometheus"
prometheus_port=9090

init_prometheus() {
  if command -v node_exporter &> /dev/null; then
    echo "node_exporter is installed. Pass"
  else
    echo "node_exporter is not installed. Installing..."
    mkdir -p $prom_folder
    pushd $prom_folder
    curl -O https://github.com/prometheus/node_exporter/releases/download/v$prom_ver/node_exporter-$prom_ver.$os-$arch.tar.gz
    file_name=node_exporter-$prom_ver.$os-$arch.tar.gz
    tar xvfz $file_name -C $prom_folder
    cd $file_name
    cp ./node_exporter /usr/local/bin
    popd
    # render config yml
    echo "global:
  scrape_interval: 15s

scrape_configs:
- job_name: node
  static_configs:
  - targets: ['localhost:9100']" > $prom_folder/prometheus.yml
  fi
}

start_service() {
  # start node_exporter
  nohup node_exporter &
}

init_prometheus
start_service
"#;
  script::execute_bash_command(node_exporter_bash);
}