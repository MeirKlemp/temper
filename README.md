# temper
A cli tool for converting between temperature scales.

## Usage
    temper <degrees> (origin scale) (result scale) [options]
## Temperature Scales
    Celsius, 'c' for short.
    Fahrenheit, 'f' for short.
    Kelvin, 'k' for short.
    Rankine, 'r' for short.
## Options
    -ro: Result Only.
## Examples
    $ temper 50 celsius fahrenheit
    50.00 celsius is 122.00 fahrenheit
### using -ro option:
    $ temper 0 k r -ro
    0.00
