<!-- Copyright (c) 2025 Cowboy AI, LLC. -->

# CIM-Subject Module Definition

## Overview

The `cim-subject` module is a foundational component of the CIM (Composable Information Machine) ecosystem that provides subject-based message routing, correlation tracking, and algebraic operations for NATS messaging in distributed domain-driven systems.

## Module Identity

- **Name**: cim-subject
- **Version**: 0.5.0
- **License**: MIT
- **Repository**: https://github.com/thecowboyai/cim-subject
- **Crate**: https://crates.io/crates/cim-subject

## Purpose

This module enables sophisticated event-driven architectures by providing:

1. **Subject Algebra** - Mathematical operations for composing and manipulating NATS subjects
2. **Message Identity** - Correlation and causation tracking across distributed systems
3. **Pattern Matching** - Flexible routing with wildcard support
4. **Permission System** - Fine-grained access control for subjects
5. **Translation Framework** - Adaptation between different naming conventions

## Architecture

### Core Components

```
cim-subject/
├── Subject           # Core subject type with validation
├── Pattern           # Pattern matching with wildcards (* and >)
├── MessageIdentity   # Correlation/causation tracking
├── SubjectAlgebra    # Algebraic operations on subjects
├── Permissions       # Access control system
├── Translator        # Subject translation framework
└── Parser            # Flexible subject parsing
```

### Design Principles

1. **Type Safety** - Strong typing prevents invalid subject construction
2. **Zero-Copy** - Efficient string handling where possible
3. **Composability** - Algebraic operations enable complex workflows
4. **Extensibility** - Parser and translator frameworks allow customization
5. **Performance** - Optimized pattern matching and caching

## Public API

### Core Types

```rust
// Subject representation
pub struct Subject { ... }
pub struct SubjectParts {
    pub context: String,
    pub aggregate: String,
    pub event_type: String,
    pub version: String,
}

// Pattern matching
pub struct Pattern { ... }
pub trait PatternMatcher {
    fn matches(&self, subject: &Subject) -> bool;
    fn is_more_specific_than(&self, other: &Pattern) -> bool;
}

// Message identity
pub struct MessageIdentity {
    pub message_id: IdType,
    pub correlation_id: CorrelationId,
    pub causation_id: CausationId,
}

// Algebraic operations
pub enum AlgebraOperation {
    Sequence(Box<AlgebraOperation>, Box<AlgebraOperation>),
    Parallel(Vec<Box<AlgebraOperation>>),
    Choice(Vec<Box<AlgebraOperation>>),
    Transform(Box<dyn Fn(SubjectParts) -> SubjectParts>),
    Inject(String, String),
}
```

### Builder Pattern

```rust
let subject = SubjectBuilder::new()
    .domain("orders")
    .aggregate("order")
    .action("create")
    .version("v1")
    .build()?;
```

### Pattern Matching

```rust
let pattern = Pattern::new("orders.*.*.v1")?;
let subject = Subject::new("orders.events.created.v1")?;
assert!(pattern.matches(&subject));
```

### Permissions

```rust
let mut permissions = PermissionSet::new();
permissions.add(Permission::allow(Pattern::new("orders.>")?));
permissions.add(Permission::deny(Pattern::new("orders.internal.>")?));

let allowed = permissions.is_allowed(&subject);
```

### Translation

```rust
let mut translator = Translator::new();
translator.add_rule(TranslationRule::new(
    "legacy_to_modern",
    Pattern::new("*.*.event")?,
    Arc::new(|subject| {
        // Transform logic
    })
));

let modern = translator.translate(&legacy_subject)?;
```

## Dependencies

### Required Dependencies

- `thiserror` - Error handling
- `serde` - Serialization support
- `uuid` - Unique identifiers
- `dashmap` - Concurrent caching
- `tracing` - Structured logging

### Optional Dependencies

- `tokio` - Async runtime (for async traits)
- `cim-ipld` - IPLD integration for content addressing

## Integration Points

### With CIM-Domain

The subject module is used throughout CIM-Domain for:
- Command/Event/Query routing
- Aggregate identification
- Cross-domain communication
- Workflow orchestration

### With NATS

Subjects map directly to NATS subjects:
- Wildcards (* and >) work identically
- Headers carry correlation/causation IDs
- Permissions align with NATS security model

### With Other Systems

Translation framework enables integration with:
- Legacy systems with different naming conventions
- External APIs with custom formats
- Multi-version support during migrations

## Usage Patterns

### 1. Domain Event Publishing

```rust
let event = Subject::new("orders.events.order.created.v1")?;
let identity = MessageIdentity::new_root();

// Publish with correlation
publisher.publish(event, identity, payload).await?;
```

### 2. Service Subscription

```rust
let pattern = Pattern::new("orders.commands.>")?;
let subscription = nats.subscribe(pattern.as_str()).await?;

while let Some(msg) = subscription.next().await {
    // Process commands for orders service
}
```

### 3. Workflow Composition

```rust
let workflow = algebra.sequence(
    Subject::new("orders.commands.validate.v1")?,
    algebra.parallel(
        Subject::new("inventory.commands.reserve.v1")?,
        Subject::new("payments.commands.authorize.v1")?
    )
);
```

### 4. Cross-Domain Translation

```rust
// Internal format to external API
let internal = Subject::new("orders.events.created.v1")?;
let external = translator.translate(&internal)?;
// Result: "order-service/events/order-created/v1"
```

## Performance Characteristics

- **Subject Creation**: O(n) validation, cached after first use
- **Pattern Matching**: O(n) where n is token count
- **Permission Check**: O(log n) with sorted rules
- **Translation**: O(1) with rule caching
- **Memory Usage**: Minimal, subjects are typically < 100 bytes

## Security Considerations

1. **Input Validation** - All subjects are validated on creation
2. **Permission Enforcement** - Deny rules override allow rules
3. **Injection Prevention** - Special characters are rejected
4. **Audit Trail** - All operations can be logged with correlation

## Error Handling

All operations return `Result<T, SubjectError>` where:

```rust
pub enum SubjectError {
    InvalidFormat(String),
    InvalidCharacter(char),
    EmptyComponent(String),
    TooManyComponents(usize),
    PatternCompilationError(String),
    PermissionDenied(String),
    TranslationError(String),
}
```

## Testing

The module includes:
- Unit tests for all components
- Property-based tests for pattern matching
- Integration tests with mock NATS
- Benchmark tests for performance
- Example applications demonstrating usage

## Versioning

This module follows Semantic Versioning:
- MAJOR: Breaking API changes
- MINOR: New features, backward compatible
- PATCH: Bug fixes, backward compatible

## Future Enhancements

Planned improvements include:
- Async trait support for translators
- Subject templates for common patterns
- GraphQL schema generation from subjects
- OpenTelemetry integration for tracing
- WebAssembly support for browser usage

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on:
- Code style and formatting
- Testing requirements
- Documentation standards
- Pull request process

## Support

- **Documentation**: [API Docs](https://docs.rs/cim-subject)
- **Examples**: See `/examples` directory
- **Issues**: [GitHub Issues](https://github.com/thecowboyai/cim-subject/issues)
- **Discussions**: [GitHub Discussions](https://github.com/thecowboyai/cim-subject/discussions)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.