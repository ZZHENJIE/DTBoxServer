# DTBox Server

## Introduction (Beginner Project)

A stock data platform with real-time market data, screening tools, and user management.

## Example Configuration
```json
{
  "server": {
    "port": 8080,
    "host": "0.0.0.0",
    "web_dir": ""
  },
  "database_url": "sqlite:./dtbox.db?mode=rwc",
  "jwt": {
    "secret": "",
    "access_token_expire_minutes": 15,
    "refresh_token_expire_days": 7
  },
  "rate_limiter": {
    "max_requests": 100,
    "window_seconds": 60
  },
  "logging": {
    "level": "info",
    "log_dir": ".logs"
  },
  "data_source": {
    "finviz_api_key": "",
    "alpaca": {
      "api_key": "",
      "api_secret": ""
    }
  }
}
```
