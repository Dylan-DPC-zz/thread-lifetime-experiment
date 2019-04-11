# Scratchpadn for a threadded high performance application in rust

The (simplified) project statement:

There are three threads:
1. Thread one reads and parses data
2. Thread two modifies the data
3. Threa three outputs the data

This communication is asyncronous in the sense that while 2. is modifying data 1. could fetc and parse the next bit.

For good performance 1. uses a zero copy/allocation strategy to parsing data, meaning when possible it references data in the raw input.

The problem: Lifetimes they seems to make impossible to do this.
