- Feature Name: (fill me in with a unique ident, `my_awesome_feature`)
- Start Date: 2019-09-23

# Summary
[summary]: #summary

This module will be responsible for handling the current project files in the means of loading them to the parser.

# Motivation
[motivation]: #motivation

Having the project root loaded is essential for all features to be available to the editor. Virtual Environments also
need to be supported since they are used to avoid userspace pollution.

# Guide-level explanation
[guide-level-explanation]: #guide-level-explanation

Poetry: pyproject.toml
Pipenv: Pipfile

Explain the proposal as if it was already included in the language and you were teaching it to another Rust programmer. That generally means:

- Introducing new named concepts.
- Explaining the feature largely in terms of examples.
- Explaining how Rust programmers should *think* about the feature, and how it should impact the way they use Rust. It should explain the impact as concretely as possible.
- If applicable, provide sample error messages, deprecation warnings, or migration guidance.
- If applicable, describe the differences between teaching this to existing Rust programmers and new Rust programmers.


For implementation-oriented RFCs (e.g. for compiler internals), this section should focus on how compiler contributors should think about the change, and give examples of its concrete impact. For policy RFCs, this section should provide an example-driven introduction to the policy, and explain its impact in concrete terms.

# Reference-level explanation
[reference-level-explanation]: #reference-level-explanation

This is the technical portion of the RFC. Explain the design in sufficient detail that:

- Its interaction with other features is clear.
- It is reasonably clear how the feature would be implemented.
- Corner cases are dissected by example.

The section should return to the examples given in the previous section, and explain more fully how the detailed proposal makes those examples work.

# Rationale and alternatives
[rationale-and-alternatives]: #rationale-and-alternatives

- Why is this design the best in the space of possible designs?
- What other designs have been considered and what is the rationale for not choosing them?
- What is the impact of not doing this?

# Prior art
[prior-art]: #prior-art

Jedi completion tool, microsoft python language server.

# Unresolved questions
[unresolved-questions]: #unresolved-questions

- Which order has to be followed to allow correctly loading of dependencies? Should we check for configuration files, 
environment variables like WORKON_HOME or VIRTUAL_ENV? 
