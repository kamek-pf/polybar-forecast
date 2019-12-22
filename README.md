## Weather forecast for Polybar
<p align="center">
    <img src="https://github.com/kamek-pf/polybar-forecast/blob/master/screenshots/preview.png" />
</p>

This is a simple weather forecast module for Polybar. \
The first number is the current temperature and the second one is a 3 hour forecast, the output is fully configurable.

You need Weather Icons and Material Icons for this to work properly. \
For Arch users, both are available in the AUR:
- [Weather Icons](https://aur.archlinux.org/packages/ttf-weather-icons/)
- [Material Icons](https://aur.archlinux.org/packages/ttf-material-icons/)

### Configuration
Look at the example TOML configuration file.

```toml
# Register at https://openweathermap.org to get your API key
api_key = "YOUR_API_KEY"

# This is for Montreal, find your city at https://openweathermap.org
# The id will be the last part of the URL
city_id = "6077243"

# Output format, using Handlebars syntax, meaning variables should be used like {{ this }}
# Available tokens are:
# - temp_celcius
# - temp_kelvin
# - temp_fahrenheit
# - temp_icon
# - trend
# - forecast_celcius
# - forecast_kelvin
# - forecast_fahrenheit
# - forecast_icon
display = "{{ temp_icon }} {{ temp_celcius }}°C {{ trend }} {{ forecast_icon }} {{ forecast_celcius }}°C"
```

### Setup
Grab a pre built binary from the [release page](https://github.com/kamek-pf/polybar-forecast/releases) (you might need to `chmod +x`), or run `cargo build --release`, in which case you'll find the binary at `target/release/polybar-forecast`. \
You can copy the binary anywhere you want, the config files has to placed in either:
- `$HOME/.config/polybar-forecast/config.toml`
- The same directory as the binary

### Polybar integration
You can define your new module like this:

```
[module/weather]
type = custom/script
exec = /path/to/polybar-forecast
exec-if = ping openweathermap.org -c 1
interval = 600
label-font = 3
```
Don't forget to add Weather Icons to your config or it won't render correctly:
```
...
font-2 = Weather Icons:size=12;0
...
```

Then you may add your new `weather` module on your bar.
