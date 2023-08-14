[![codecov](https://codecov.io/gh/AdRiley/programming_the_z80/branch/main/graph/badge.svg?token=2O0TVLZRDI)](https://codecov.io/gh/AdRiley/programming_the_z80)

Never.
Stop.
Learning.

# What is this project?

This is an attempt to build a Z80 emulator using Rust. Following along with Rodnay Zaks book "How to program the Z80" http://www.z80.info/zip/zaks_book.pdf and coding up every example in Rust code and tests.

# But Why?

Why not? But more seriously: to learn. I am currently reading "The Rust Book" and wanted a project to actually make some of what I have learnt stick in my head. I am also interested in taking my programming knowledge down an absraction level closer to the metal. While at a Meeting C++ conference (pre-pandemic) I remember getting into a conversation about understanding more about how processors actually work, with two members of the C++ community, who I highly respected, and one of them making the comment that if you understood "How to program the Z80" then modern chipsets weren't really that different. I'm not sure how flippant a comment that was, but I found reading Rodnay's book interesting. But again I needed a way to make the details stick and this project was born.

# The details

Now obviously I am re-inventing things that the Rust language already supports naturally, so many thing do not make sense if this were a real program. But remember this is a learning exercise and the goal is to have the concepts described in Rodnay's book represented in Rust code. So please forgive me while I re-implement binary on top of a sytem that cleaerly natively supports it.

I am working though the pdf version of the book linked above and will reference pages and examples from the book in the unit tests that implement that section, in case any one wants to follow along

Adam Riley