# Cite-Me-Bro: A Comprehensive CLI Tool for BibTeX Reference Formatting

## Abstract

In the realm of software development and academic research, proper citation of references is paramount. This document introduces `cite-me-bro`, a Command Line Interface (CLI) tool designed to read BibTeX files and output formatted references to stdout in various citation styles. The tool aims to streamline the citation process within code and documentation.

## Introduction

The integration of citations within software projects and academic documentation is often cumbersome. Traditional methods require manual formatting, which is prone to errors and time-consuming. In many academci fields there exist tools to automate such work, such as Mendeley, EndNote, Zotero, in addition to this functionality being included in many popular word processing software such as Microsoft Word and BibLatex. However such tools are not suitable for coding. `cite-me-bro` addresses this challenge by providing an automated solution for generating citations directly from BibTeX files. This tool is particularly beneficial for developers and researchers who frequently incorporate references into their code and documentation, or people needing to paste formatted references somewhere in plain text such as a presentation.

## Installation

The installation of `cite-me-bro` is facilitated through the crates.io index, ensuring ease of access and deployment. The tool can be installed as demonstrated below

```sh
cargo install cite-me-bro
```

## Methodology

### Usage

The core functionality of `cite-me-bro` is encapsulated in its command-line interface, which accepts a BibTeX file as input and outputs a formatted reference. Users can specify the desired citation style through command-line arguments. If no citation keys are provided all references will be formatted to stdout. Users have the option to provide space seperated list of citation keys to be found in the bib file. If the user does this only the references corresponding to the citations will be printed. 

Currently supported citations styles are: 
- IEEE (default)
- APA

While this list is limited at the time of writing, due to the somewhat cumbersome work of adding styles, should the reader have a need for different styles to be included, they are encouraged to open an issue or Pull Request (PR) detailing their request. 

### Example

Consider a scenario where a user wishes to format a reference in IEEE style. As an example we will use [1] which has the citaiton key `breiman2001`, as can be seen in Apendix A (`cite.bib`). The following command illustrates the procedure:

```sh
cite-me-bro -b cite.bib --style ieee breiman2001
```

Upon execution, the tool produces a formatted reference such as:

```
L. Breiman, "Random forests," Machine learning, vol. 45, no. 1, pp. 5-32, 2001. doi: https://doi.org/10.1023/a:1010933404324.
```

## Discussion

The development of `cite-me-bro` is driven by the necessity for a reliable and efficient citation tool within both academic and software development contexts. By automating the citation process, this tool minimizes the potential for formatting errors and encourages adding citation in code and documentation where appropriate without significantly increasing complexity of the workflow.

## Features

1. **Automation compatible CLI**: Provides a streamlined, text-based interaction model that can work both in an interactive and automated environoment.
2. **Multi-Style Support**: Accommodates various citation styles including IEEE and APA, with more styles available upon request.
3. **Unicode Support**: While references are almost always (to the author's knowledge) written in a superset of the latin script, `cite-me-bro` does know how to handle unicode characters and will apply accents and other unicode charachters without problem.
4. **Speed**: Because `cite-me-bro` is not a full Tex engine, but rather a simple focused CLI tool written in Rust, it is very fast and suitable for quick workflows. 

## Limitations

1. **LaTex**: As stated `cite-me-bro` does not have an embeded Tex Engine of any kind. Tools like BibLaTex, allow users to supply arbirtrary tex commands, which `cite-me-bro` does not support. It is just for formatting references, though given the Unicode support mentioned above, a lot of Tex commands used for things such as adding accents should not be necessary.
2. **Customisation**: The development of `cite-me-bro` was focused on simplicity and ease of use. Unfortunately this means that there is no option to customise the output of `cite-me-bro` outside of changing the citation style or modifying the code. Customisation is considered a non-goal for this piece of software. 
3. **Guarantees**: While great care has been taken to make sure that the styles provided by `cite-me-bro` are correct, comparing both with reference style guides and the output of citation generation tools such as BibLaTex itself, it should be noted that `cite-me-bro` was developed as a hobby project and therefore may not correctly cover every use case, and no guarantees are to be extended as to it's correctness. While the author strives to be as correct as possible and encourages any user noticing incorrect behavioiur to open either an issue or PR to fix this, no liability can be attributed to the author as a result of using this software. 

## Conclusion

`cite-me-bro` represents a significant advancement in the management of bibliographic references. By leveraging this tool, users can achieve greater accuracy and efficiency in their citation practices, ultimately contributing to higher standards of documentation in software development projects.

## License

This project is licensed under the MIT License, promoting open-source collaboration and innovation.

## [Apendix A](cite.bib)
