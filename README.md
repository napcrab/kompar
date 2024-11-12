# Contribution
If you, for some reason, want to contribute to this project, then please feel free to do so! It is not really meant to be an actually-useful-useable-in-prod type thing at all, it's just a solo-person-wanting-to-make-something-that-probably-already-exists type thing - but if you like the idea than feel free to contribute :)

# Ambition
The idea for Kompar is to be a language specialized in "no-side-effect human-readable-and-writable data manipulation" - So for example imagine you have some user that wants to manipulate some data (This can be a program configuration, metadata for some images, etc), and you want to give them the opportunity to do this programatically, so that they can use maps, filters, conditionals or whatever else on the data on a bigger scale than what would be possible for them to do manually, but you dont want them to just be able to enter raw code and execute it for them, in case they do something wrong and cause some side effect - that is the use-case Kompar covers: to allow for a programmatic interface with data that can't cause side effects 

For this to be realized it requires the following:
- Easy to write, easy to read: Must be intuitive to read/write, no unnecessary syntax rule, no weird quirks where something returns something you dont expect it to or where ambiguity is allowed, and must be functional, which means no mutability, this it to allow it to be read easier without having to remember internal state in your head
- Iterative use: If a user nevertheless makes a mistake in either syntax or logic there must be allowed for easily going back and fixing mistakes to carry on, this means easy to understand error messages, caching of previous computations across compilations to avoid doing work twice (it would also fall in line with this philosophy to be able to revert to previous version of the data you are manipulation however this is to be implemented by the program using Kompar)
- Interface with other languages: Must be easy for a Kompar type to convert to a type in other languages to allow for the data to be transferred between the "no side-effect" realm and the "can do stuff in the real world" realm

# Status
Not even close - if you are seriously in need of something similar to what is said in "Ambition" than I would recommend checking out gluon maybe, idk - I havent looked around

# TODO
## Design
- [ ] Language Features/Design
  - [x] Type system
  - [ ] Patterns and matching
- [ ] Language Syntax
  - [x] Primitive type creation
  - [x] Storing state (variable, import, etc)
  - [ ] Traits and Impls
  - [ ] Function bodies 
  - [ ] Patterns and matching
## Documentation
- [ ] Language book
  - [x] Simple types
  - [ ] Typeception
  - [ ] Functions
  - [ ] Complex types
  - [ ] Traits
  - [ ] Patterns and such
- [ ] Crate documentation
## Programming
- [ ] Syntax Parsing (GOOD ERROR MESSAGES PLEASE)
  - [x] Primitive types
  - [ ] Unary operators
  - [ ] Binary operators
  - [ ] Ident (fields, idents)
  - [ ] Control flow (if, match, etc)
  - [ ] Binding (let, import, apply)
  - [ ] Impls, traits and complex types
- [ ] Syntax Validation (GOOD ERROR MESSAGES PLEASE)
- [ ] Evaluation (In-built caching and whatnot)
- [ ] Interaction with Rust (type conversions)
- [ ] Standard libary
