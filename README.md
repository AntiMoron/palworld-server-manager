## Palworld-Server-Manager

Make deploying and managing Palworld dedicated server easily on Linux servers.

### You can:

- Delpoy server by user id that you defined. E.g. You input some phone number for your buyer, then a docker container named 'palworld-{yourinput}' will occur in your container list.
- Easily manage server resource limits(memory, memory swap, cpus), sometimes modifying `docker-compose.yml` can be annoying.
- Resource management
- Faster deployment for server hoster, cache all gaming file on storage services.

### Dependencies:

Thanks to those great projects:

- [https://github.com/zaigie/palworld-server-tool](https://github.com/zaigie/palworld-server-tool)
- [https://github.com/thijsvanloef/palworld-server-docker](https://github.com/thijsvanloef/palworld-server-docker)

### Usage

```bash
palworld-cli deploy
```

Follow the instruction, you will don

prometheuse/node_exporter will be started to collect CPU/disk usage status on port 9100.
prometheuse will be on port 9090

Here's a handy url that shows common metrices like cpu status,disk status....

http://127.0.0.1:9090/graph?g0.expr=rate(node_cpu_seconds_total%7Bmode%3D%22system%22%7D%5B1m%5D)&g0.tab=0&g0.stacked=0&g0.show_exemplars=0&g0.range_input=1h&g1.expr=node_filesystem_avail_bytes&g1.tab=0&g1.stacked=0&g1.show_exemplars=0&g1.range_input=1h&g2.expr=rate(node_network_receive_bytes_total%5B1m%5D)&g2.tab=0&g2.stacked=0&g2.show_exemplars=0&g2.range_input=1h