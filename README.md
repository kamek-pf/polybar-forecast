## Weather forcast for Polybar

This is a simple weather forecast module for Polybar, you need Weather Icons and Material Icons for this to work properly.
Both are available in the AUR: 
- Weather Icons (https://aur.archlinux.org/packages/ttf-weather-icons/)
- Material Icons (https://aur.archlinux.org/packages/ttf-material-icons/)

### Configuration
Look at the example TOML configuration file.

```toml
# Register at https://openweathermap.org to get your API key
api_key = "YOUR_API_KEY"

# This is for Montreal, find your city at https://openweathermap.org
# The id will be the last part of the URL
city_id = "6077243"

# Display settings
units = "metric"
display_symbol = "°C"
```

### Build
Run `cargo build --release`, the you'll find the binary `target/release/polybar-forecast`.
You can copy the binary anywhere you want, but `config.toml` has to be in the same directory. 

### Polybar integration
You can define your new module like this :

```
[module/weather]
type = custom/script
exec = ~/.config/polybar/modules/forecast/polybar-forecast
interval = 600
label-font = 3
```

Then you may add your new `weather` module on your bar. 