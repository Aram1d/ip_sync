# Ip-Sync

IPSync is a tool for synchronizing your public IP address with an AWS Route 53 DNS record. It ensures that the DNS record for your domain always reflects your current external IP, checking for changes and updating as necessary.

## Features

- Monitors your public IP and compares it to the current DNS record.
- Updates AWS Route 53 DNS records automatically when the IP changes.
- Configurable polling intervals and DNS TTL values.

## Prerequisites

- AWS Route 53 hosted zone and domain configured.
- Required utilities installed: dig and upnpc.
- `dig` and `upnpc` utilities for IP resolution

## Installation

Todo: systemd services, package builder.
