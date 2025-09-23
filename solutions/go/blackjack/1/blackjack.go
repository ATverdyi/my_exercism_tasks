package blackjack

// ParseCard returns the integer value of a card following blackjack ruleset.
func ParseCard(card string) int {
    var res int;
	switch card {
        case "ace":
        	res = 11
        case "two":
        	res = 2
        case "three":
        	res = 3
        case "four":
        	res = 4
        case "five":
        	res = 5
        case "six":
        	res = 6
        case "seven":
        	res = 7
        case "eight":
        	res = 8
        case "nine":
        	res = 9
        case "ten", "jack", "queen", "king":
        	res = 10
        default:
        	res = 0
    }
    return res
}

// FirstTurn returns the decision for the first turn, given two cards of the
// player and one card of the dealer.
func FirstTurn(card1, card2, dealerCard string) string {
    var res string;
	sum := ParseCard(card1) + ParseCard(card2)
    dealer_sum := ParseCard(dealerCard)
    switch {
        case sum > 21:
        	res = "P"
        case sum == 21 && dealer_sum < 10:
        	res = "W"
        case sum == 21 && dealer_sum >= 10:
        	res = "S"
        case sum >= 17 && sum <=20:
        	res = "S"
        case sum < 17 && sum >=12 && dealer_sum < 7:
        	res = "S"
        case sum < 17 && sum >=12 && dealer_sum >= 7:
        	res = "H"
        case sum < 12:
        	res = "H"
    }
    return res
}
