// Package weather returns weather condition.
package weather

// CurrentCondition string with the state.
var CurrentCondition string 
// CurrentLocation string with the location.
var CurrentLocation string 

// Forecast returns the Weather condition depending on the location.
func Forecast(city, condition string) string {
	CurrentLocation, CurrentCondition = city, condition
	return CurrentLocation + " - current weather condition: " + CurrentCondition
}
