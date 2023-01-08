# Exercism

Exercises from this platform solved in multiple languages. For now, only doing Rust, but may explore other languages in the future.

## Rust exercises

All exercises done run through `cargo clippy`.

The difficulty level is taken from Exercism, therefore it does not represent my opinion, as for example `Accumulate` or `Queen Attack` are rather easy exercises, not medium difficulty. Learning and tutorial exercises are hereby mapped to easy.

| Exercise                                                    | Difficulty level | Done                  | Concepts involved                                                                                                                                               | Crates                          |
| ----------------------------------------------------------- | ---------------- | --------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------- |
| [Accumulate](rust/accumulate)                               | Medium           | :white_check_mark:    | - Generics <br> - Function as input parameters                                                                                                                  |                                 |
| [Acronym](rust/acronym)                                     | Medium           | :white_check_mark:    | - Iterators <br> - String manipulation                                                                                                                          |                                 |
| [Affine Ciphper](rust/affine-cipher)                        | Medium           | :white_check_mark:    | - Iterators <br> - Result                                                                                                                                       |                                 |
| [All Your Base](rust/all-your-base)                         | Medium           | :white_check_mark:    | - ControlFlow <br> - Iterators <br> - Result                                                                                                                    |                                 |
| [Allergies](rust/allergies)                                 | Medium           | :white_check_mark:    | - Bitset                                                                                                                                                        | - enum-iterator                 |
| [Alphametics](rust/alphametics)                             | Medium           | :white_check_mark:    | - BTreeMap <br> - HashMap <br> - Control flow label <br> - Custom Iterator                                                                                      | - itertools                     |
| [Anagram](rust/anagram)                                     | Medium           | :white_check_mark:    | - HashMap <br> - HashSet                                                                                                                                        |                                 |
| [Armstrong Numbers](rust/armstrong-numbers)                 | Easy             | :white_check_mark:    | - successors                                                                                                                                                    |                                 |
| [Assembly Line](rust/assembly-line)                         | Easy             | :white_check_mark:    | - match ranges arm                                                                                                                                              |                                 |
| [Atbash Cipher](rust/atbash-cipher)                         | Medium           | :white_check_mark:    | - char arithmetic <br> - Iterators                                                                                                                              |                                 |
| [Beer Song](rust/beer-song)                                 | Easy             | :white_check_mark:    | - Checked overflow arithmetic <br> - String manipulation                                                                                                        |                                 |
| [Binary Search](rust/binary-search)                         | Medium           | :white_check_mark:    | - AsRef <br> - Features <br> - Generics <br> - Ordering                                                                                                         |                                 |
| [Bob](rust/bob)                                             | Easy             | :white_check_mark:    | - match on tuples <br> - String manipulation                                                                                                                    |                                 |
| [Book Store](rust/book-store)                               | Medium           | :white_check_mark:    | - Iterator mutability <br> - Type alias                                                                                                                         |                                 |
| [Bowling](rust/bowling/)                                    | Medium           | :white_check_mark:    | - Checked overflow arithmetic <br> - Enum methods <br> - Error handling                                                                                         |                                 |
| [Circular Buffer](rust/circular-buffer)                     | Hard             | :white_check_mark:    | - Default <br> - Generics <br> - mem::swap <br> - Vec::resize_with                                                                                              |
| [Clock](rust/clock)                                         | Medium           | :white_check_mark:    | - Display trait <br> - wrapping arithmetic                                                                                                                      |                                 |
| [Collatz Conjecture](rust/collatz-conjecture)               | Easy             | :white_check_mark:    | - Checked arithmetic <br> - Option enum                                                                                                                         |                                 |
| [Crypto Square](rust/crypto-square)                         | Medium           | :white_check_mark:    | - Floating point operations <br> - Iterators                                                                                                                    |                                 |
| [Custom Set](rust/custom-set)                               | Medium           | :white_check_mark:    | - Build pattern <br> - must_use attribute <br> - Trait bounds                                                                                                   |                                 |
| [Decimal](rust/decimal)                                     | Medium           | :white_check_mark:    | - Associated types <br> - Impl Add, Sub, Mult trait <br> - Impl PartialOrd, Ord trait                                                                           | - num_bigint                    |
| [Diamond](rust/diamond)                                     | Medium           | :white_check_mark:    | - abs_diff <br> - char arithmetic <br> - format                                                                                                                 |                                 |
| [Difference of Squares](rust/difference-of-squares)         | Easy             | :white_check_mark:    | - Mathematical operations <br> - pow                                                                                                                            |                                 |
| [Diffie-Hellman](rust/diffie-hellman)                       | Easy             | :white_check_mark:    | - modpow <br> - PRNG                                                                                                                                            | - num_bigint <br> - rand        |
| [Dominoes](rust/dominoes)                                   | Hard             | :white_check_mark:    | - chunks <br> - Eulerian cycle <br> - flatten <br> - Slices <br> - Slice windows <br> - Stack                                                                   |
| [Dot DSL](rust/dot-dsl)                                     | Medium           | :white_check_mark:    | - Lifetimes <br> - Modules <br> - Traits                                                                                                                        |                                 |
| [Doubly Linked List](rust/doubly-linked-list)               | Hard             | :black_square_button: |
| [ETL](rust/etl)                                             | Medium           | :white_check_mark:    | - flat_map <br> - move to closure                                                                                                                               |                                 |
| [Fizzy](rust/fizzy)                                         | Medium           | :white_check_mark:    | - Function pointer <br> - Generics <br> - move to closure <br> - Iterator as input parameters <br> - Iterator as return <br> - From trait <br> - Rem trait      |                                 |
| [Forth](rust/forth)                                         | Hard             | :black_square_button: |
| [Gigasecond](rust/gigasecond)                               | Easy             | :white_check_mark:    | - Trait from crate                                                                                                                                              | - time                          |
| [Grade School](rust/grade-school)                           | Medium           | :white_check_mark:    | - BTreeMap <br> - BTreeSet <br> - Default trait <br>                                                                                                            |                                 |
| [Grains](rust/grains)                                       | Easy             | :white_check_mark:    | - Constants <br> - Ranges                                                                                                                                       |                                 |
| [Grep](rust/grep)                                           | Medium           | :white_check_mark:    | - BufReader <br> - Control flow label <br> - Cow <br> - File                                                                                                    | - anyhow                        |
| [Hamming](rust/hamming)                                     | Medium           | :white_check_mark:    | - Iterator::zip                                                                                                                                                 |                                 |
| [Health Statistics](rust/health-statistics)                 | Easy             | :white_check_mark:    | - Struct impl                                                                                                                                                   |                                 |
| [Hello World](rust/hello-world)                             | Easy             | :white_check_mark:    | - String slices                                                                                                                                                 |                                 |
| [High Scores](rust/high-scores)                             | Easy             | :white_check_mark:    | - Lifetimes <br> - Slices <br> - Sort (not in-place)                                                                                                            | - itertools                     |
| [ISBN Verifier](rust/isbn-verifier)                         | Medium           | :white_check_mark:    | - Iterators                                                                                                                                                     |                                 |
| [Isogram](rust/isogram)                                     | Medium           | :white_check_mark:    | - HashSet                                                                                                                                                       |                                 |
| [Largest Series Product](rust/largest-series-product)       | Medium           | :white_check_mark:    | - Collect into Result <br> - Result <br> - Slice windows                                                                                                        |                                 |
| [Leap](rust/leap)                                           | Easy             | :white_check_mark:    | - match <br> - Remainder                                                                                                                                        |                                 |
| [Low-Power Embedded Game](rust/low-power-embedded-game)     | Easy             | :white_check_mark:    | - Iterator as return                                                                                                                                            |                                 |
| [Lucian's Luscious Lasagna](rust/lucians-luscious-lasagna)  | Easy             | :white_check_mark:    | - Functions                                                                                                                                                     |                                 |
| [Luhn](rust/luhn)                                           | Medium           | :white_check_mark:    | - Iterators <br> - String manipulation                                                                                                                          |                                 |
| [Luhn From](rust/luhn-from)                                 | Medium           | :white_check_mark:    | - From trait <br> - ToString trait                                                                                                                              |                                 |
| [Luhn Trait](rust/luhn-trait)                               | Medium           | :white_check_mark:    | - Blanket implementation                                                                                                                                        |                                 |
| [Macros](rust/macros)                                       | Hard             | :black_square_button: |
| [Magazine Cutout](rust/magazine-cutout)                     | Easy             | :white_check_mark:    | - Entry::Occupied <br> - HashMap                                                                                                                                |                                 |
| [Matching Brackets](rust/matching-brackets)                 | Easy             | :white_check_mark:    | - HashMap <br> - let-else <br> - Stack                                                                                                                          |                                 |
| [Minesweeper](rust/minesweeper)                             | Medium           | :white_check_mark:    | - asBytes <br> - flatten                                                                                                                                        |                                 |
| [Nth Prime](rust/nth-prime)                                 | Easy             | :white_check_mark:    | - all <br> - nth <br> - Iterators                                                                                                                               |                                 |
| [Nucleotide Count](rust/nucleotide-count)                   | Medium           | :white_check_mark:    | - copied <br> - get_mut <br> - HashMap                                                                                                                          |                                 |
| [OCR Numbers](rust/ocr-numbers)                             | Hard             | :black_square_button: |
| [PaaS I/O](rust/paasio)                                     | Medium           | :white_check_mark:    | - Generics <br> - Read trait <br> - Write trait                                                                                                                 |                                 |
| [Palindrome Products](rust/palindrome-products)             | Medium           | :white_check_mark:    | - Function as input parameters <br> - Iterator::zip <br> - Iterator as input parameters <br> - successors <br> - Trait for function that returns a trait object |                                 |
| [Pangram](rust/pangram)                                     | Medium           | :white_check_mark:    | - Ranges                                                                                                                                                        |                                 |
| [Parallel Letter Frequency](rust/parallel-letter-frequency) | Hard             | :white_check_mark:    | - Benchmark <br> - Hashmap <br> - Newtype <br> - thread::scope <br> - thread::spawn                                                                             | - nohash-hasher                 |
| [Pascal's Triangle](rust/pascals-triangle)                  | Medium           | :white_check_mark:    | - fold <br> - Newtype                                                                                                                                           |                                 |
| [Perfect Numbers](rust/perfect-numbers)                     | Medium           | :white_check_mark:    | - Ordering                                                                                                                                                      |                                 |
| [Phone Number](rust/phone-number)                           | Medium           | :white_check_mark:    | - Slice windows                                                                                                                                                 |                                 |
| [Pig Latin](rust/pig-latin)                                 | Medium           | :white_check_mark:    | - Slices <br> - Split                                                                                                                                           |                                 |
| [Poker](rust/poker)                                         | Hard             | :black_square_button: |
| [Prime Factors](rust/prime-factors)                         | Easy             | :white_check_mark:    | - pow <br> - Vector                                                                                                                                             |                                 |
| [Protein Translation](rust/protein-translation)             | Medium           | :white_check_mark:    | - HashMap <br> - Lifetimes <br> - Ranges                                                                                                                        |                                 |
| [Proverb](rust/proverb)                                     | Easy             | :white_check_mark:    | - chain  <br> - once <br> - Slice windows                                                                                                                       |                                 |
| [Pythagorean Triplet](rust/pythagorean-triplet)             | Medium           | :white_check_mark:    | - HashSet <br> - pow                                                                                                                                            |                                 |
| [Queen Attack](rust/queen-attack)                           | Medium           | :white_check_mark:    | - abs_diff <br> - Type casting                                                                                                                                  |                                 |
| [Rail Fence Cipher](rust/rail-fence-cipher)                 | Medium           | :white_check_mark:    | - cycle <br> - Function as input parameters <br> - Newtype                                                                                                      |                                 |
| [Raindrops](rust/raindrops)                                 | Easy             | :white_check_mark:    | - Remainder                                                                                                                                                     |                                 |
| [React](rust/react)                                         | Hard             | :black_square_button: |
| [Rectangles](rust/rectangles)                               | Hard             | :black_square_button: |
| [Resistor Color](rust/resistor-color)                       | Easy             | :white_check_mark:    | - Enum discriminants                                                                                                                                            | - int-enum <br> - enum-iterator |
| [Reverse String](rust/reverse-string)                       | Easy             | :white_check_mark:    | - Conditional compilation <br> - Feature                                                                                                                        | - unicode-segmentation          |
| [RNA Transcription](rust/rna-transcription)                 | Medium           | :white_check_mark:    | - enumerate <br> - unreachable                                                                                                                                  |                                 |
| [Robot Name](rust/robot-name)                               | Medium           | :white_check_mark:    | - Break with value <br> - HashSet <br> - PRNG <br> - RefCell <br> - thread_local                                                                                | - rand                          |
| [Robot Simulator](rust/robot-simulator)                     | Medium           | :white_check_mark:    | - Tuple destructuring <br> - Struct update syntax                                                                                                               | - int-enum                      |
| [Role Playing Game](rust/role-playing-game)                 | Easy             | :white_check_mark:    | - match <br> - Option <br> - ref                                                                                                                                |                                 |
| [Roman Numerals](rust/roman-numerals)                       | Medium           | :white_check_mark:    | - @ Binding <br> - Display trait <br> - From trait                                                                                                              |                                 |
| [Rotational Cipher](rust/rotational-cipher)                 | Medium           | :white_check_mark:    | - char arithmetic <br> - Iterators                                                                                                                              |                                 |
| [RPN Calculator](rust/rpn-calculator)                       | Easy             | :white_check_mark:    | - Function as input parameters                                                                                                                                  |                                 |
| [Run-Length Encoding](rust/run-length-encoding)             | Medium           | :white_check_mark:    | - Peek <br> - Repeat on string                                                                                                                                  |                                 |
| [Saddle Points](rust/saddle-points)                         | Medium           | :white_check_mark:    | - Iterators                                                                                                                                                     |                                 |
| [Say](rust/say)                                             | Medium           | :white_check_mark:    | - @ Binding <br> - const <br> - Trim                                                                                                                            |                                 |
| [Scale Generator](rust/scale-generator)                     | Medium           | :white_check_mark:    | - Collect into Result <br> - const <br> - chain <br> - once <br> - Repeat on string <br> - Result                                                               |                                 |
| [Scrabble Score](rust/scrabble-score)                       | Medium           | :white_check_mark:    | - match <br> - Iterators                                                                                                                                        |                                 |
| [Semi Structured logs](rust/semi-structured-logs)           | Easy             | :white_check_mark:    | - Enums                                                                                                                                                         |                                 |
| [Series](rust/series)                                       | Easy             | :white_check_mark:    | - Checked overflow arithmetic <br> - Slices                                                                                                                     |                                 |
| [Short Fibonacci Sequence](rust/short-fibonacci)            | Easy             | :white_check_mark:    | - Vector                                                                                                                                                        |                                 |
| [Sieve](rust/sieve)                                         | Medium           | :white_check_mark:    | - map_while <br> - skip <br> - sqrt                                                                                                                             |                                 |
| [Simple Cipher](rust/simple-cipher)                         | Medium           | :white_check_mark:    | - char arithmetic <br> - cycle <br> - Iterators                                                                                                                 | - rand                          |
| [Simple Linked List](rust/simple-linked-list)               | Medium           | :white_check_mark:    | - as_ref <br> - Box <br> - Default trait <br> - FromIterator <br> - take                                                                                        |                                 |
| [Space Age](rust/space-age)                                 | Medium           | :white_check_mark:    | - macro_rules <br> - From trait                                                                                                                                 |                                 |
| [Spiral Matrix](rust/spiral-matrix)                         | Medium           | :white_check_mark:    | - Checked overflow arithmetic <br> - Enum methods <br> - Wrapping arithmetic                                                                                    |                                 |
| [Sublist](rust/sublist)                                     | Medium           | :white_check_mark:    | - Ordering <br> - Slice windows                                                                                                                                 |                                 |
| [Sum of Multiples](rust/sum-of-multiples)                   | Easy             | :white_check_mark:    | - Iterators <br> - Ranges                                                                                                                                       |                                 |
| [Tournament](rust/tournament)                               | Medium           | :white_check_mark:    | - Enum methods <br> - Ordering <br> - Slice destructuring <br> - Split <br> - TryFrom trait                                                                     | - indexmap                      |
| [Triangle](rust/triangle)                                   | Medium           | :white_check_mark:    | - Generics <br> - repeat <br> - Slice windows <br> - take                                                                                                       |                                 |
| [Two Bucket](rust/two-bucket)                               | Medium           | :white_check_mark:    | - Break with value <br> - BTreeMap <br> - Hash Trait <br> - HashSet <br> - VecDeque                                                                             | - num                           |
| [Variable Length Quantity](rust/variable-length-quantity)   | Medium           | :white_check_mark:    | - Checked shift <br> - Split <br> - VecDeque                                                                                                                    |                                 |
| [Word Count](rust/word-count)                               | Medium           | :white_check_mark:    | - HashMap <br> - Split <br> - Trim                                                                                                                              |                                 |
| [Wordy](rust/wordy)                                         | Medium           | :white_check_mark:    | - Checked overflow arithmetic <br> - Split At <br> - Tuple destructuring                                                                                        |                                 |
| [Xorcism](rust/xorcism)                                     | Hard             | :black_square_button: |
