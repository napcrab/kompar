# Numbers

Kompar has a lot of number types for all of your number needs! Each with a varying degree of complexity.

In the spirit of Kompar you should always strife for the least complex number as that one has more guarantees, but can at any time increase to a higher complexity effortlessly

The number types are formed on a sort of ladder, wherein each type on the ladder has a state space that is fully contained by the state spaces of types higher up the ladder

The ladder goes like this:
|Rank|Name            |Type    |State space                                                                |Examples  |
|----|----------------|--------|---------------------------------------------------------------------------|----------|
|1   |Natural numbers |natural |1, 2¸ 3, 4, 5, 6... and so forth, with no 0                                |37        |
|2   |Whole numbers   |whole   |like natural numbers but with 0                                            |0         |
|3   |Integars        |int     |like whole numbers but with negatives, still no fractions tho              |-37       |
|4   |Rational        |rational|like integars but with fractions, does not allow for irrational numbers tho|37.381313 |
|5   |Complex         |complex |Like rational numbers but with an imaginary component                      |3.5+8.3\*i|
|6   |Math Expressions|mexpr   |Like complex numbers but allows for irrationals                            |2.sqrt()  |

The state space of a number on this list is entirely contained in all the state spaces of the higher order numbers, so f.x. all integars can always be converted to a rational, complex or mexpr

so if we take an example
```
let n: natural = 3 // 3 is natural number

//Does not compile!
// let n: natural = 0

let m: whole = n //No problem

let z: whole = 0 //Also no problem

// Does not compile! Big problem
// let k: whole = -3

//---

let i: int = -3 //Cool

let j: int = 3

let n: natural = j //Also cool

//---

let c: complex = 1+2*i

// NO!
// let r: rational = c
```
