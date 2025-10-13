package lasagna

// TODO: define the 'PreparationTime()' function
func PreparationTime(layers []string, avg_time int) int {
    if avg_time == 0 {
        avg_time +=2
    }
    return avg_time * len(layers)
}
// TODO: define the 'Quantities()' function
func Quantities(layers []string) (int, float64) {
    var n int
    var s float64
    for  _, layer := range layers {
        if layer == "sauce" {
            s += 0.2
        } else if layer == "noodles" {
            n += 50
        } 
    }
    return n, s
}
// TODO: define the 'AddSecretIngredient()' function
func AddSecretIngredient(friendsList []string, myList []string) {
    if len(friendsList) > 0 && len(myList) > 0 {
        secretIngredient := friendsList[len(friendsList)-1]
        myList[len(myList)-1] = secretIngredient
    }
}
// TODO: define the 'ScaleRecipe()' function
func ScaleRecipe(quantities []float64, portions int) []float64 {
    var scaledQuantities []float64
    x := float64(portions) / 2
    for _, q := range quantities {
        scaledQuantities = append(scaledQuantities, q*x)
    }
    return scaledQuantities
}

