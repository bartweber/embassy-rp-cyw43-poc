# POC - How Embassy enables wireless communication for the Raspberry Pi Pico

A simple project demonstrating the use of the `embassy` crate to communicate with the Raspberry Pi Pico via WiFi.
For one of my projects I want to use the Pico for measurements. The measurement data is to be sent to a gateway using
the CYW43 WiFi module. Most of the firmware is just a copy of the
[Embassy examples](https://github.com/embassy-rs/embassy/blob/main/examples/rp/src/bin/wifi_tcp_server.rs).
All credits go to Dario Nieuwenhuis and contributors. The protocol is mostly a copy from the
[demo repo](https://github.com/bedroombuilds/pico-usb) of Michael Kefeder's presentation at the Rust Zürisee Meetup
March 2023. Great thanks to him for his [great talk](https://youtu.be/KECu_piSM5s?si=6T-fWqiHLXb51VCr).
This simple project, could be a starting point for further development.

## Wireless ping pong using Embassy & Postcard

With the help of Embassy, on the Raspberry Pi Pico we set up a server that listens for incoming messages
(in this case a.k.a. postcards 😉). On the other side, we have a client that sends messages to the server.
The server then responds to the client with a "Pong" message.

## Requirements

- Raspberry Pi Pico
- Raspberry Pi Debug Probe (or any other debugger)
- some micro-usb cables
- `probe-rs` installed on your computer (see their website for the details: [probe.rs](https://probe.rs)
- `cargo` and `rustup` installed on your computer

## Installation and setup

1. Clone the repository: `git clone https://github.com/bartweber/embassy-rp-cyw43-poc.git`
1. Power up your Raspberry Pi Pico.
1. Connect your Raspberry Pi Pico to your computer with a debugger, for example, a Raspberry Pi Debug Probe.
1. Configure the WiFi settings in `firmware/config.toml`. An example configuration is in the
   file `firmware/config.toml.example`.
1. Open two terminals.
   - In one terminal, flash the firmware to the Raspberry Pi Pico by running: `cd firmware && cargo run`.
   - In the other terminal, run the client side code by: `cd clien && cargo run`

If all is set up well, the RP Pico will try to connect to your Wi-Fi network and the client will start sending "Ping"
messages to the server. The server will respond with "Pong" messages.
