# Architect

## Functions

* Deserialize XML to a tree of stones
* Serialize a stone tree using Serde
* Call plugins on stone serialization/deserialization
* Log syntax errors & allow plugins to do so.
* Provide element selection functions

# Electrician

## Functions

* Handle meta elements
    * Meta
    * Import
    * Script
    * Transform

# Designer

## Functions

* Handle rendering elements
    * Style
    * Color

# Painter

## Functions

* Render XML to buffer

# Ground Control

## Functions

* Handle input
    * Convert input to machine-readable generic codes
        * I.E. "Left Arrow" > "move_left"
        * Configurable

# Construct

## Functions

Example program that acts like a web browser.

* Run Scripts
    * Just going to have these run with full privileges for now; I'll think about security later.