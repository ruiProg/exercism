# Exercism

Exercises from this platform solved in multiple languages. For now, only doing Rust, but may explore other languages in the future.

## Rust exercises

All exercises done run through `cargo clippy`.

The difficulty level is taken from Exercism, therefore it does not represent my opinion, as for example `Accumulate` or `Queen Attack` are rather easy exercises, not medium difficulty. Learning and tutorial exercises are hereby mapped to easy.

The concepts traceability and the crates used are under construction :construction:.

| Exercise                                                    | Difficulty level | Done                  | Concepts involved                                                                                                                                          | Crates                   |
| ----------------------------------------------------------- | ---------------- | --------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------ |
| [Accumulate](rust/accumulate)                               | Medium           | :white_check_mark:    | - Generics <br> - Function as input parameters                                                                                                             |                          |
| [Acronym](rust/acronym)                                     | Medium           | :white_check_mark:    | - Iterators <br> - String manipulation                                                                                                                     |                          |
| [Affine Ciphper](rust/affine-cipher)                        | Medium           | :white_check_mark:    | - Iterators <br> - Result                                                                                                                                  |                          |
| [All Your Base](rust/all-your-base)                         | Medium           | :white_check_mark:    | - ControlFlow <br> - Iterators <br> - Result                                                                                                               |                          |
| [Allergies](rust/allergies)                                 | Medium           | :white_check_mark:    | - Bitset                                                                                                                                                   | - enum-iterator          |
| [Alphametics](rust/alphametics)                             | Medium           | :white_check_mark:    | - BTreeMap <br> - HashMap <br> - Control flow label <br> - Custom Iterator                                                                                 | - itertools              |
| [Anagram](rust/anagram)                                     | Medium           | :white_check_mark:    | - HashMap <br> - HashSet                                                                                                                                   |                          |
| [Armstrong Numbers](rust/armstrong-numbers)                 | Easy             | :white_check_mark:    | - successors                                                                                                                                               |                          |
| [Assembly Line](rust/assembly-line)                         | Easy             | :white_check_mark:    | - match ranges arm                                                                                                                                         |                          |
| [Atbash Cipher](rust/atbash-cipher)                         | Medium           | :white_check_mark:    | - char arithmetic <br> - Iterators                                                                                                                         |                          |
| [Beer Song](rust/beer-song)                                 | Easy             | :white_check_mark:    | - Checked overflow arithmetic <br> - String manipulation                                                                                                   |                          |
| [Binary Search](rust/binary-search)                         | Medium           | :white_check_mark:    | - AsRef <br> - Features <br> - Generics <br> - Ordering                                                                                                    |                          |
| [Bob](rust/bob)                                             | Easy             | :white_check_mark:    | - match on tuples <br> - String manipulation                                                                                                               |                          |
| [Book Store](rust/book-store)                               | Medium           | :white_check_mark:    | - Iterator mutability <br> - Type alias                                                                                                                    |                          |
| [Bowling](rust/bowling/)                                    | Medium           | :white_check_mark:    | - Checked overflow arithmetic <br> - Enum methods <br> - Error handling                                                                                    |                          |
| [Circular Buffer](rust/circular-buffer)                     | Hard             | :black_square_button: |
| [Clock](rust/clock)                                         | Medium           | :white_check_mark:    | - Display trait <br> - wrapping arithmetic                                                                                                                 |                          |
| [Collatz Conjecture](rust/collatz-conjecture)               | Easy             | :white_check_mark:    | - Checked arithmetic <br> - Option enum                                                                                                                    |                          |
| [Crypto Square](rust/crypto-square)                         | Medium           | :white_check_mark:    | - Floating point operations <br> - Iterators                                                                                                               |                          |
| [Custom Set](rust/custom-set)                               | Medium           | :white_check_mark:    | - Build pattern <br> - must_use attribute <br> - Trait bounds                                                                                              |                          |
| [Decimal](rust/decimal)                                     | Medium           | :white_check_mark:    | - Associated types <br> - Impl Add, Sub, Mult trait <br> - Impl PartialOrd, Ord trait                                                                      | - num_bigint             |
| [Diamond](rust/diamond)                                     | Medium           | :white_check_mark:    | - abs_diff <br> - char arithmetic <br> - format                                                                                                            |                          |
| [Difference of Squares](rust/difference-of-squares)         | Easy             | :white_check_mark:    | - Mathematical operations <br> - pow                                                                                                                       |                          |
| [Diffie-Hellman](rust/diffie-hellman)                       | Easy             | :white_check_mark:    | - modpow <br> - PRNG                                                                                                                                       | - num_bigint <br> - rand |
| [Dominoes](rust/dominoes)                                   | Hard             | :black_square_button: |
| [Dot DSL](rust/dot-dsl)                                     | Medium           | :white_check_mark:    | - Lifetimes <br> - Modules <br> - Traits                                                                                                                   |                          |
| [Doubly Linked List](rust/doubly-linked-list)               | Hard             | :black_square_button: |
| [ETL](rust/etl)                                             | Medium           | :white_check_mark:    | - flat_map <br> - move to closure                                                                                                                          |                          |
| [Fizzy](rust/fizzy)                                         | Medium           | :white_check_mark:    | - Function pointer <br> - Generics <br> - move to closure <br> - Iterator as input parameters <br> - Iterator as return <br> - From trait <br> - Rem trait |                          |
| [Forth](rust/forth)                                         | Hard             | :black_square_button: |
| [Gigasecond](rust/gigasecond)                               | Easy             | :white_check_mark:    | - Trait from crate                                                                                                                                         | - time                   |
| [Grade School](rust/grade-school)                           | Medium           | :white_check_mark:    | - BTreeMap <br> - BTreeSet <br> - Default trait <br>                                                                                                       |                          |
| [Grains](rust/grains)                                       | Easy             | :white_check_mark:    | - Constants <br> - Ranges                                                                                                                                  |                          |
| [Grep](rust/grep)                                           | Medium           | :white_check_mark:    | - BufReader <br> - Control flow label <br> - Cow <br> - File                                                                                               | - anyhow                 |
| [Hamming](rust/hamming)                                     | Medium           | :white_check_mark:    | - Iterator::zip                                                                                                                                            |                          |
| [Health Statistics](rust/health-statistics)                 | Easy             | :white_check_mark:    | - Struct impl                                                                                                                                              |                          |
| [Hello World](rust/hello-world)                             | Easy             | :white_check_mark:    | - String slices                                                                                                                                            |                          |
| [High Scores](rust/high-scores)                             | Easy             | :white_check_mark:    | - Lifetimes <br> - Slices <br> - Sort (not in-place)                                                                                                       | - itertools              |
| [ISBN Verifier](rust/isbn-verifier)                         | Medium           | :white_check_mark:    | - Iterators                                                                                                                                                |                          |
| [Isogram](rust/isogram)                                     | Medium           | :white_check_mark:    | - HashSet                                                                                                                                                  |                          |
| [Largest Series Product](rust/largest-series-product)       | Medium           | :white_check_mark:    | - Collect into Result <br> - Result <br> - Slice windows                                                                                                   |                          |
| [Leap](rust/leap)                                           | Easy             | :white_check_mark:    | - match <br> - reminder                                                                                                                                    |                          |
| [Low-Power Embedded Game](rust/low-power-embedded-game)     | Easy             | :white_check_mark:    | - Iterator as return                                                                                                                                       |                          |
| [Lucian's Luscious Lasagna](rust/lucians-luscious-lasagna)  | Easy             | :white_check_mark:    | - Functions                                                                                                                                                |                          |
| [Luhn](rust/luhn)                                           | Medium           | :white_check_mark:    | - Iterators <br> - String manipulation                                                                                                                     |                          |
| [Luhn From](rust/luhn-from)                                 | Medium           | :white_check_mark:    | - From trait <br> - ToString trait                                                                                                                         |                          |
| [Luhn Trait](rust/luhn-trait)                               | Medium           | :white_check_mark:    | - Blanket implementation                                                                                                                                   |                          |
| [Macros](rust/macros)                                       | Hard             | :black_square_button: |
| [Magazine Cutout](rust/magazine-cutout)                     | Easy             | :white_check_mark:    |
| [Matching Brackets](rust/matching-brackets)                 | Easy             | :white_check_mark:    |
| [Minesweeper](rust/minesweeper)                             | Medium           | :white_check_mark:    |
| [Nth Prime](rust/nth-prime)                                 | Easy             | :white_check_mark:    |
| [Nucleotide Count](rust/nucleotide-count)                   | Medium           | :white_check_mark:    |
| [OCR Numbers](rust/ocr-numbers)                             | Hard             | :black_square_button: |
| [PaaS I/O](rust/paasio)                                     | Medium           | :white_check_mark:    |
| [Palindrome Products](rust/palindrome-products)             | Medium           | :white_check_mark:    |
| [Pangram](rust/pangram)                                     | Medium           | :white_check_mark:    |
| [Parallel Letter Frequency](rust/parallel-letter-frequency) | Hard             | :black_square_button: |
| [Pascal's Triangle](rust/pascals-triangle)                  | Medium           | :white_check_mark:    |
| [Perfect Numbers](rust/perfect-numbers)                     | Medium           | :white_check_mark:    |
| [Phone Number](rust/phone-number)                           | Medium           | :white_check_mark:    |
| [Pig Latin](rust/pig-latin)                                 | Medium           | :white_check_mark:    |
| [Poker](rust/poker)                                         | Hard             | :black_square_button: |
| [Prime Factors](rust/prime-factors)                         | Easy             | :white_check_mark:    |
| [Protein Translation](rust/protein-translation)             | Medium           | :white_check_mark:    |
| [Proverb](rust/proverb)                                     | Easy             | :white_check_mark:    |
| [Pythagorean Triplet](rust/pythagorean-triplet)             | Medium           | :white_check_mark:    |
| [Queen Attack](rust/queen-attack)                           | Medium           | :white_check_mark:    |
| [Rail Fence Cipher](rust/rail-fence-cipher)                 | Medium           | :white_check_mark:    |
| [Raindrops](rust/raindrops)                                 | Easy             | :white_check_mark:    |
| [React](rust/react)                                         | Hard             | :black_square_button: |
| [Rectangles](rust/rectangles)                               | Hard             | :black_square_button: |
| [Resistor Color](rust/resistor-color)                       | Easy             | :white_check_mark:    |
| [Reverse String](rust/reverse-string)                       | Easy             | :white_check_mark:    |
| [RNA Transcription](rust/rna-transcription)                 | Medium           | :white_check_mark:    |
| [Robot Name](rust/robot-name)                               | Medium           | :white_check_mark:    |
| [Robot Simulator](rust/robot-simulator)                     | Medium           | :white_check_mark:    |
| [Role Playing Game](rust/role-playing-game)                 | Easy             | :white_check_mark:    |
| [Roman Numerals](rust/roman-numerals)                       | Medium           | :white_check_mark:    |
| [Rotational Cipher](rust/rotational-cipher)                 | Medium           | :white_check_mark:    |
| [RPN Calculator](rust/rpn-calculator)                       | Easy             | :white_check_mark:    |
| [Run-Length Encoding](rust/run-length-encoding)             | Medium           | :white_check_mark:    |
| [Saddle Points](rust/saddle-points)                         | Medium           | :white_check_mark:    |
| [Say](rust/say)                                             | Medium           | :white_check_mark:    |
| [Scale Generator](rust/scale-generator)                     | Medium           | :white_check_mark:    |
| [Scrabble Score](rust/scrabble-score)                       | Medium           | :white_check_mark:    |
| [Semi Structured logs](rust/semi-structured-logs)           | Easy             | :white_check_mark:    |
| [Series](rust/series)                                       | Easy             | :white_check_mark:    |
| [Short Fibonacci Sequence](rust/short-fibonacci)            | Easy             | :white_check_mark:    |
| [Sieve](rust/sieve)                                         | Medium           | :white_check_mark:    |
| [Simple Cipher](rust/simple-cipher)                         | Medium           | :white_check_mark:    |
| [Simple Linked List](rust/simple-linked-list)               | Medium           | :white_check_mark:    |
| [Space Age](rust/space-age)                                 | Medium           | :white_check_mark:    |
| [Spiral Matrix](rust/spiral-matrix)                         | Medium           | :white_check_mark:    |
| [Sublist](rust/sublist)                                     | Medium           | :white_check_mark:    |
| [Sum of Multiples](rust/sum-of-multiples)                   | Easy             | :white_check_mark:    |
| [Tournament](rust/tournament)                               | Medium           | :white_check_mark:    |
| [Triangle](rust/triangle)                                   | Medium           | :white_check_mark:    |
| [Two Bucket](rust/two-bucket)                               | Medium           | :white_check_mark:    |
| [Variable Length Quantity](rust/variable-length-quantity)   | Medium           | :white_check_mark:    |
| [Word Count](rust/word-count)                               | Medium           | :white_check_mark:    |
| [Wordy](rust/wordy)                                         | Medium           | :white_check_mark:    |
| [Xorcism](xorcism)                                          | Hard             | :black_square_button: |