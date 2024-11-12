# Primitive Types

As Kompar is a language that primarily deals with the changing of information it would be therefore require a very concrete system of defining how that information is categorised and organised

That is the purpose of a type. "A type" is some representation of a specific type of data, which has some space of all the possible states that data can be in, to manipulate data is to provide af mapping from one state space to another, or to transform the state space in some way

Let us for example take the english letters, this a type of information with a state space of size 26, going through all the letters, a transformation of this type would be a mapping of every letter to some other value in another type (or the same type)

If we for example take the transformation "next letter in the alphabet, with wraparound", this would be a transformation that doesnt change the type, where in A -> B, B -> C ... Z -> A

Meanwhile a transformation like "True if letter is K, false if otherwise" is a transformation to a different type (and therefore different state space), after the transformation the new type would be a Boolean (True or False), wherein every letter except K gets transformed to false, and K gets transformed to True

State spaces do not have to be finite, if we for example take the natural numbers, "Natural Number" is a type of information that encapsulates some "amount" of things, its state space is all the whole numbers from 1 and up

An operation like "n + 1" would transform all of the infinite possible values of natural numbers one up, 1 becomes 2, 2 becomes 3, and so forth

An operation like "n / 2" would instead require a change of type, going from natural numbers to rational numbers, 1 becomes 0.5, 2 becomes 1, 3 becomes 1.5, and so forth

This Chapter will be going over the different types that Kompar makes available by default
