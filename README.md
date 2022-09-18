# gandalf

Gandalf is a proof assistant written in Rust based on the LF logical framework, an extension of the simply typed lambda calculus with dependent types. In LF, an object logic is encoded as an LF signature. Syntax is embedded as LF types and term constructors. Judgments in the object logic are represented as dependent LF types. Proofs or derivations of those judgments correspond to inhabitants of those types.

Using Gandalf, a user may describe a formal logic and write proofs concerning that logic. When a Gandalf program type-checks, the proofs encoded in the program are sound.
