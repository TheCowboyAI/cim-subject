// Copyright 2025 Cowboy AI, LLC.

//! # CIM Subject Algebra
//!
//! This crate provides a comprehensive Subject Algebra system for NATS-based
//! domain routing and message translation. It implements the mathematical
//! foundations for subject-based messaging patterns used throughout CIM.
//!
//! ## Core Concepts
//!
//! - **Subject**: A hierarchical address for messages (e.g.,
//!   `context.aggregate.event.version`)
//! - **Pattern**: Wildcard-based subject matching using `*` and `>` operators
//! - **Algebra**: Compositional operations on subjects and patterns
//! - **Translation**: Bidirectional mapping between different subject schemas
//! - **Correlation**: Message tracking and causation chains for distributed
//!   tracing
//!
//! ## Example
//!
//! ```rust
//! use cim_subject::{
//!     AlgebraOperation,
//!     Pattern,
//!     Subject,
//!     SubjectAlgebra,
//! };
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Create a subject
//! let subject = Subject::new("people.person.created.v1")?;
//!
//! // Create patterns
//! let pattern = Pattern::new("people.*.created.>")?;
//! assert!(pattern.matches(&subject));
//!
//! // Use algebra operations
//! let algebra = SubjectAlgebra::new();
//! let workflow_subject = Subject::new("workflow.process.triggered.v1")?;
//! let composed = algebra.compose(&subject, &workflow_subject, AlgebraOperation::Sequence)?;
//! # Ok(())
//! # }
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

pub mod algebra;
pub mod correlation;
pub mod error;
pub mod message_algebra;
pub mod parser;
pub mod pattern;
pub mod permissions;
pub mod subject;
pub mod translator;

// Re-export main types
pub use algebra::{
    AlgebraOperation,
    CompositionRule,
    SubjectAlgebra,
};
pub use correlation::{
    CausationId,
    CorrelationError,
    CorrelationId,
    CorrelationValidator,
    IdType,
    MessageFactory,
    MessageIdentity,
    SerializableCid,
};
pub use error::{
    Result,
    SubjectError,
};
pub use message_algebra::{
    CorrelationChain,
    MessageAlgebra,
};
pub use parser::{
    ParseRule,
    SubjectParser,
};
pub use pattern::{
    Pattern,
    PatternMatcher,
};
pub use permissions::{
    PermissionRule,
    Permissions,
};
pub use subject::{
    Subject,
    SubjectBuilder,
    SubjectParts,
};
pub use translator::{
    MessageTranslator,
    NatsMessage,
    TranslationRule,
    Translator,
};

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::{
        AlgebraOperation,
        CausationId,
        CorrelationChain,
        CorrelationError,
        CorrelationId,
        CorrelationValidator,
        IdType,
        MessageAlgebra,
        MessageFactory,
        MessageIdentity,
        NatsMessage,
        Pattern,
        PatternMatcher,
        PermissionRule,
        Permissions,
        Result,
        SerializableCid,
        Subject,
        SubjectAlgebra,
        SubjectBuilder,
        SubjectError,
        SubjectParts,
        TranslationRule,
        Translator,
    };
}
