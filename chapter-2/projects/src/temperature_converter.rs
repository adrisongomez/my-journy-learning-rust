
// Convert temperature from Celcius to Fahrenheit
pub fn ctof(temp_in_celcius: f32) -> f32{
    (temp_in_celcius * 1.8) + 32.0
}

// Convert temperature from Fahrenheit to Celcius
pub fn ftoc(temp_in_fahrenheit: f32) -> f32 {
    (temp_in_fahrenheit - 32.0) * 9.0/5.0
}

// Convert temperature from kelvin to celcius
pub fn ktoc(temp_in_kelvin: f32) -> f32 {
    temp_in_kelvin - 273.15
}

// Convert temperature from celcius to kelvin
pub fn ctok(temp_in_celcius: f32) -> f32 {
    temp_in_celcius + 273.15
}
