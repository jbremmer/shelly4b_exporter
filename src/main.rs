use env_logger::{Builder, Env};
use log::{debug, error, info};
use std::env;
use std::string::String;

use serde::Deserialize;

use url::Url;

use reqwest;
use reqwest::Error;
use tiny_http::{Response, Server};

use prometheus::{Encoder, Gauge, Opts, Registry, TextEncoder};

#[derive(Deserialize, Debug)]
struct Relay {
    name: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Meter {
    power: f64,
    total: f64,
}

#[derive(Deserialize, Debug)]
struct Sehlly4bSettings {
    relays: Vec<Relay>,
    meters: Vec<Meter>,
}

fn main() -> Result<(), Error> {
    Builder::from_env(Env::default().default_filter_or("info")).init();

    let default_exporter_port = "9048";
    let exporter_port =
        env::var("SHELLY4B_EXPORTER_PORT").unwrap_or(default_exporter_port.to_string());
    match env::var("SHELLY4B_EXPORTER_PORT") {
        Ok(port) => debug!("SHELLY4B_EXPORTER_PORT variable set to {}", port),
        Err(_e) => debug!(
            "No SHELLY4B_EXPORTER_PORT environment variable defined, using default port = {}",
            default_exporter_port
        ),
    };

    let server = Server::http(format!("0.0.0.0:{}", exporter_port)).unwrap();

    info!("Shelly4b Exporter started at port {}", exporter_port);

    for request in server.incoming_requests() {
        let full_url = format!("http://127.0.0.1/{}", request.url());
        let parsed_url = Url::parse(&full_url).unwrap();

        let mut pairs = parsed_url.query_pairs();

        let response;
        if pairs.count() == 1 {
            let param = pairs.next().unwrap();

            if param.0 == "device_address" {
                let shelly_url = format!("http://{}/settings", param.1);
                let shelly_settings: Sehlly4bSettings =
                    reqwest::blocking::get(&shelly_url)?.json()?;

                let r = Registry::new();

                for (pos, relay) in shelly_settings.relays.iter().enumerate() {
                    let gauge_opts = Opts::new(
                        "shelly4b_relay_meter_power",
                        "Current real AC power being drawn, in Watts",
                    )
                    .const_label("pos", pos.to_string())
                    .const_label("name", relay.name.as_ref().unwrap_or(&"None".to_string()));
                    let gauge = Gauge::with_opts(gauge_opts).unwrap();

                    r.register(Box::new(gauge.clone())).unwrap();

                    gauge.set(shelly_settings.meters.get(pos).unwrap().power);

                    let total_gauge_opts = Opts::new(
                        "shelly4b_relay_meter_total",
                        "Total energy consumed by the attached electrical appliance in Watt-minute",
                    )
                    .const_label("pos", pos.to_string())
                    .const_label("name", relay.name.as_ref().unwrap_or(&"None".to_string()));
                    let total_gauge = Gauge::with_opts(total_gauge_opts).unwrap();

                    r.register(Box::new(total_gauge.clone())).unwrap();

                    total_gauge.set(shelly_settings.meters.get(pos).unwrap().total);
                }

                let metric_families = r.gather();
                let mut buffer = Vec::<u8>::new();
                let encoder = TextEncoder::new();
                encoder.encode(&metric_families, &mut buffer).unwrap();

                response = Response::from_string(String::from_utf8(buffer.clone()).unwrap());
            } else {
                let err_msg = "Wrong request, 'device_address' parameter is mising from url.";
                error!("{}", err_msg);
                response = Response::from_string(err_msg);
            }
        } else {
            let err_msg = "Wrong request, too many parameters in url.";
            error!("{}", err_msg);
            response = Response::from_string(err_msg);
        }

        request
            .respond(response)
            .map_err(|err| error!("{}", err))
            .ok();
    }

    Ok(())
}
