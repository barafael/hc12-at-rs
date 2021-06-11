# Rust embedded-hal driver for hc12 radio module

[![.github/workflows/rust.yml](https://github.com/barafael/hc12-at-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/barafael/hc12-at-rs/actions/workflows/rust.yml)

Driver for the hc-12 radio transceiver serial module.

For driver, see hc12-at/. For example running on raspberry pi, see hc12-example-raspi/.

# IMPORTANT NOTE

In some countries/regions, some of the technically valid configurations of this module are legally prohibited.
For example, in germany transmission power above 25mW or hc12 channels other than 1, 2, 3, 4 are prohibited.
The very good reason being that we all want to be able to reliably use our equipment.

## Blog post
https://barafael.github.io/A-Platform-Agnostic-Driver-for-the-HC12-serial-radio-module/
