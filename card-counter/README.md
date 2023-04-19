# Concept

So this is a answer for an interview question that I had a year ago,

The idea is to write a function that receive an array of `string`.
Each element of the array would be compose by two char. (Match the following Regex /[0-9jqka]{1}[cdpt]/)

Where:
- The first char represent the number of the card from 1-14. Using (1,2,3,4,5,6,7,8,9,0,j,q,k,a)
- The second the family which its belongs. Using c = Corazones (Heart) d = Diamonds p = Pikas and h = Trebols

The main goal is to write a function which given a array of string card, return the number of entire deck of card.
    
