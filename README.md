# Shelly4b Exporter
 
 Allows Prometheus to scrape poer consumption values from Shelly 4b relay (https://shelly.cloud/products/shelly-4pro-smart-home-automation-relay/).
 
 It takes values from HTTP API on device, check their documentation here https://shelly-api-docs.shelly.cloud/#shelly4pro-status for more info.
 
 ## Sample Prometheus scrape config
 
 ```
  - job_name: 'Shelly4b'
    static_configs:
      - targets: ['127.0.0.1:9048']
    params:
      device_address: ['127.0.0.1:80']
```
...exporter needs to respond to `http://127.0.0.1:9048/metrics?device_address=127.0.0.1:80` with sample metrics:
```
# HELP shelly4b_relay_meter_power Current real AC power being drawn, in Watts
# TYPE shelly4b_relay_meter_power gauge
shelly4b_relay_meter_power{name="None",pos="1"} 151.9
shelly4b_relay_meter_power{name="None",pos="2"} 1541.1
shelly4b_relay_meter_power{name="None",pos="3"} 27
shelly4b_relay_meter_power{name="None",pos="0"} 1005.5
# HELP shelly4b_relay_meter_total Total energy consumed by the attached electrical appliance in Watt-minute
# TYPE shelly4b_relay_meter_total gauge
shelly4b_relay_meter_total{name="None",pos="1"} 1531191
shelly4b_relay_meter_total{name="None",pos="2"} 15149448
shelly4b_relay_meter_total{name="None",pos="3"} 296835
shelly4b_relay_meter_total{name="None",pos="0"} 9902481
```

### Port Setting
set `SHELLY4B_EXPORTER_PORT` envionment variable to port number you desire, defaults to `9048`.

### Pre-Compiled Binaries
see Release section for pre-compiled binaries like this one [v0.1.0](https://github.com/jbremmer/shelly4b_exporter/releases/tag/v0.1.0)
