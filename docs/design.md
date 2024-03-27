# The QED Project: Design (WIP)

The QED Project wants to advance mathematical rigor, accessibility, and collaboration through computational mathematics.

## Goals

- make mathematics more accessible for everyone
- one single place where all mathematics is combined
- provide a database of all mathematics known to mankind
- everything for free, no paywalls or ads!
- TODO

## Database

At the core of the QED Project is the database (hopefully at one point) containing all mathematics known to mankind.

TODO

### Design

The design for the database is not yet finalized. Before this happens we need to formalize the intermediate language of the theorems and proofs so they can be stored efficiently and also indexed more easily.

TODO

#### Physical Design

There should be multiple copies of the database which continuously synchronize between each other located all around the world. This also helps in minimizing latency.

TODO

## Search

TODO

### Signatures

One tool which could make the search easier and more efficient would be some kind of hash of the statements (theorems / definitions / axioms) and the proofs. But this could mean that equivalent statements are considered different.

Initial idea for creating signature for mathematical statements:

1. **Parsing:** parse the statement to a structure representing the fundamental parts of the statement
2. **Canonicalization:** transform the tokens into a canonical form to eliminate variations that do not affect the meaning of the statement. For example, we might standardize variable names or use placeholders, use a consistent order for commutative operators, and remove unnecessary whitespaces.
3. **Concatenation:** Concatenate the canonicalized tokens into a single string.
4. **Checksum or Encoding:** Compute a checksum or use a specific encoding scheme to derive a short, unique value from the concatenated string. This can involve techniques like CRC (Cyclic Redundancy Check), a simple checksum algorithm, or other encoding methods.

This signature would be created if a statement is inserted into the database. If the user now searches for a statement, the search algorithm would create the signature of that statement and then check if it exists in the database.

Using this approach we eliminate the possibility of two similar looking and equivalent statement creating two completely different hashes resulting in bad search results.

## Proof System

Everything added to the math database needs to have a proof attached to it. This way we ensure that only rigorously proved mathematics is shown on this website. But how do we proof the theorems we want to add to the database? Do we write it on paper, take a picture and upload it? If we use a computer based proof system using a formal language for the proof, which one do we use? Those are all good questions and not that easy to answer except the second one. We will use a computer based proof system, but which one?

There exist a lot of different proof systems, all with their own syntax and logic foundation. All projects have created substantial collections of definitions, axioms, theorems and proofs. The problem is that they are not compatible with each other. This results in a lot of duplication and unnecessary work.

The QED Project aims to create a "bridge" between all (or at least most) of the already existing proof systems. This "bridge" should be an intermediate language to which all other proof systems can compile to. At first nobody from the proof system communities will be interested in trying to accomplish this more than difficult task for which reason the QED Project needs to develop alternative programs which can parse the syntax of the systems and convert them into a intermediate language which will be common to all (most) proof systems.

As soon as this task is complete the QED Project will really start to work. Theorems and proofs can be submitted from all proof systems. The end-goal is to develop a new custom proof system specifically designed for this project which would be the preferred system for submissions.

Maybe you noticed the use of 'most' or 'mostly' when referring to all proof systems. Since they are all very different even in their foundation it will be really difficult if not impossible to create an intermediate language representation for ALL proof systems.

### Already existing proof systems

- Coq
- Lean 4
- Mizar
- HOL
- … TODO

### Types of proof systems

- Type based
- Set based
- axiomatic
- natural deduction
- Propositional logic
- Predicate logic
- higher-order logic
- … TODO
or any combination of some of them

## Website

As already stated, the QED website should not contain any ads because they can destroy the professional appearance.

### Core components

- User authentication
- Community forum
- Blog / Articles / related publications
- Search
- Theorem / Proof / Definition / etc. submission page

### Backend Design

#### Routes

Authentication

- `/auth/login`
- `/auth/register`
- `/auth/verify`
- `/auth/reset`

Account

- `/account`
- `/account/settings`
- `/account/profile`
- `/account/profile/edit`

About

- `/about`
- `/about/goals`
- `/about/contributors`

Legal Documents

- `/legal`
- `/legal/terms`
- `/legal/privacy`
- `/legal/cookies`

Forum

- `/forum`
- `/forum/topics`
- `/forum/search`
- `/forum/view/{id}`
- `/forum/create`

Search

- `/search`
- `/search?q=<query>[&...]`

Database

- `/database`
- `/database/view/{id}`
- `/database/submit`
- `/database/review/{id}`

## Projects

For the QED Project to work we need some smaller projects which provide vital functionality to the bigger project to fulfil all goals.

- `Tafelwerk` a linear algebra library written in Rust
- `Blueprint` HTML templating engine written in Rust
- `Signature` generates unique identifiers for mathematical statements
- `Argument` parser for mathematical statements
- `Formats` a collection of parsers for different storage formats for mathematical expressions
  - `MathML`
  - `openmath`
  - `ASCII Math`
  - `Unicode Math`
  - `...`
- `Hypothesis` custom proof system preferred by the QED Project
- `Vision` math expression recognition using artificial intelligence
