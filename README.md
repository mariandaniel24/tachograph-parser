# Tachograph File Parser (WIP)

A Rust-based parser for digital tachograph files, supporting both driver cards and vehicle units. Please note that this project is currently a work in progress, features and functionality may be incomplete or subject to change.

## Overview

This project aims to provide a parser for digital tachograph files, which are used in the transportation industry to record driving times, speeds, and other vehicle data. The parser is being developed to support both driver card data files (.ddd) and vehicle unit files, complying with EU regulations.

## Features

- [x] Driver card Generation 1
- [x] Driver card Generation 2
- [x] Driver card Generation 2 Version 2
- [x] Vehicle unit Generation 1
- [x] Vehicle unit Generation 2
- [x] Vehicle unit Generation 2 Version 2
- [ ] Workshop/Control/Company cards
- [ ] Signature validation

## Documentation

For detailed information about the tachograph file structure and regulations, refer to:

- [EU Regulation 2016/799 (latest consolidated version)](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821)
- [UNECE Specifications (older version)](https://unece.org/DAM/trans/doc/2019/sc1/ECE-TRANS-SC1-GE21-INF-JUNE-2019-2e.-Rev.pdf)

# License

MIT License

Copyright (c) 2024 Daniel Stelea

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
