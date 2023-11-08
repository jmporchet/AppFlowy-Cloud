# Monitoring Grafana
A simple guide to deploy the monitoring tools for AppFlowy Cloud

## Components
- Listed in previous directory: `docker-compose-monitor.yml`

## Quick Start
- This assumes that you have already deployed AppFlowy.
- Copy the `dev.env` as `.env` if you have not done so
- Change the `GF_SECURITY_ADMIN_USER` and `GF_SECURITY_ADMIN_PASSWORD` field as necessary
- In the previous directory, open a terminal and put the following command:
```
docker compose --file docker-compose-monitor.yml up -d
```
- Quick check: `docker ps -a`

## Web UI Setup
- After deployment, you will have a Grafana dashboard server, sign in with your username and password defined previously
- There no dashboard at the start, you would need to add preconfigured dashboard or configure your own.

### Steps to import
- Go to myhost/dashboard/import
- Fill in dashboard ID (e.g. 1860 for node-exporter-full)

### Recommended Publicly available Dashboard
- https://grafana.com/grafana/dashboards/13946-docker-cadvisor/
- https://grafana.com/grafana/dashboards/1860-node-exporter-full/
- https://grafana.com/grafana/dashboards/12708-nginx/

### AppFlowy Cloud dashboard
- Go to myhost/dashboard/import
- Fill in json model from file (same directory as this doc)
  - `appflowy_cloud_prometheus_grafana_dashboard.json`