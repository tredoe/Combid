# Identifiers for databases

## Sequential numbers

The benefits of a standard autoincrementing integer are:

1. Better usage of index
2. Allow usage of clustered keys (to be verified in NLB scenarios)
3. Less disk usage
4. Better performance at a minimum cost

## Universally Unique Identifiers (UUIDs)

An UUID is a 16-byte (128-bit) data structure whose advantages over
autoincrementing integers are:

1. They can be created anywhere without contacting the database
2. They are identifiers that are entirely unique within your application
  (and in the case of UUIDs, universally unique)
3. Given one identifier, there is no way to guess the next or previous
  (or even any other valid identifiers) outside of brute-forcing a huge keyspace.

### COMB UUID

A COMB is a type of UUID where a number of bits have been replaced with a
timestamp-like value. This means that the COMBs can be ordered, and when used
as a primary key result in less index page splits when inserting new values.

## Combid (Combined Identifier)

INTs are 4 bytes, BIGINTs ar 8 bytes, and GUIDS are 16 bytes. The more space required
to represent the data, the more resources required to process it (disk space, memory).

Combids are 4 or 8 bytes. The first array of bytes is a timestamp, which both reduces
the chance of collision and allows the IDs created consecutively to sort chronologically.
The timestamp is followed of some bits for randomness, which ensures that even two people
creating IDs at the exact same second are extremely unlikely to generate identical IDs.

The advantages over anterior identifiers:

1. The length is of 4 or 8 bytes so it can be used into a primitive data type,
   like INT and BIGINT.
2. The timestamp creates sequential numbers to get a better index performance.
3. Have enough randomness to make hard the guessing of consecutive identifiers.
