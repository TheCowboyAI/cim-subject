<!-- Copyright (c) 2025 Cowboy AI, LLC. -->

# CIM-Subject Examples

This directory contains comprehensive examples demonstrating the usage of the cim-subject module in real-world scenarios.

## Running Examples

To run any example:

```bash
cargo run --example <example_name>
```

## Examples Overview

### 1. Basic Routing (`01_basic_routing.rs`)
Demonstrates fundamental subject-based message routing patterns in microservices.

```bash
cargo run --example basic_routing
```

**Key concepts:**
- Subject patterns with wildcards
- Service-specific routing
- Pattern specificity comparison
- Event broadcasting

### 2. Correlation Tracking (`02_correlation_tracking.rs`)
Shows how to maintain message correlation across distributed transactions.

```bash
cargo run --example correlation_tracking
```

**Key concepts:**
- Message identity (ID, correlation, causation)
- Root message identification
- Causation chain tracing
- NATS header mapping

### 3. Subject Validation (`03_subject_validation.rs`)
Illustrates subject validation and safe construction patterns.

```bash
cargo run --example subject_validation
```

**Key concepts:**
- Subject structure validation
- Builder pattern usage
- Custom validation rules
- Pattern-based validation

### 4. Permissions (`04_permissions.rs`)
Demonstrates fine-grained permission control for message access.

```bash
cargo run --example permissions
```

**Key concepts:**
- Allow/deny rules
- Service-specific permissions
- Permission precedence
- Audit logging

### 5. Algebra Operations (`05_algebra_operations.rs`)
Shows algebraic composition of message flows.

```bash
cargo run --example algebra_operations
```

**Key concepts:**
- Sequential operations (→)
- Parallel operations (⊗)
- Choice operations (⊕)
- Complex workflow composition
- Lattice operations

### 6. Translation (`06_translation.rs`)
Illustrates translation between different naming conventions.

```bash
cargo run --example translation
```

**Key concepts:**
- Version migration
- Legacy system integration
- Bidirectional translation
- Context-aware translation
- Multi-stage pipelines

### 7. NATS Integration (`07_nats_integration.rs`)
Comprehensive example of integration with NATS messaging.

```bash
cargo run --example nats_integration
```

**Key concepts:**
- NATS client integration
- Distributed transaction flow
- Pattern-based routing
- Permission enforcement
- Request-reply pattern

### 8. Mortgage Lending Routing (`08_mortgage_lending_routing.rs`)
Private mortgage lending application routing based on broker tiers and compliance.

```bash
cargo run --example mortgage_lending_routing
```

**Key concepts:**
- Broker tier-based routing (VIP, Premium, Standard, New)
- State-specific compliance requirements
- Priority calculation with subject algebra
- Broker-specific permissions
- Workflow composition by tier

### 9. Document Validation (`09_document_validation.rs`)
Document collection, OCR processing, and validation workflows for lending.

```bash
cargo run --example document_validation
```

**Key concepts:**
- Document type routing (income, asset, property, identity)
- OCR service integration
- Document expiration tracking
- Validation rule enforcement
- Automated reminder workflows

### 10. Rate Shopping (`10_rate_shopping.rs`)
Multi-lender rate shopping with dynamic pricing and normalization.

```bash
cargo run --example rate_shopping
```

**Key concepts:**
- Lender qualification based on loan profile
- Rate request broadcasting
- Response normalization
- Effective rate calculation
- Risk-based pricing adjustments
- Rate lock workflows

## NATS Setup

For the NATS integration example, you need a NATS server running:

```bash
# Install NATS server
# macOS
brew install nats-server

# Linux
wget https://github.com/nats-io/nats-server/releases/latest/download/nats-server-amd64.deb
sudo dpkg -i nats-server-amd64.deb

# Run NATS with JetStream
nats-server -js
```

## Example Patterns

### Service Communication Pattern
```rust
// Service subscribes to its domain
let pattern = Pattern::new("orders.>")?;

// Publish command
let subject = Subject::new("orders.commands.order.create")?;

// Broadcast event
let event = Subject::new("orders.events.order.created")?;
```

### Correlation Pattern
```rust
// Root message (all IDs same)
let root = MessageIdentity {
    message_id: IdType::Uuid(id),
    correlation_id: CorrelationId(IdType::Uuid(id)),
    causation_id: CausationId(IdType::Uuid(id)),
};

// Subsequent message
let next = MessageIdentity {
    message_id: IdType::Uuid(new_id),
    correlation_id: root.correlation_id.clone(),
    causation_id: CausationId(root.message_id.clone()),
};
```

### Permission Pattern
```rust
let mut permissions = PermissionSet::new();

// Allow pattern
permissions.add(Permission::allow(Pattern::new("orders.>")?));

// Deny overrides allow
permissions.add(Permission::deny(Pattern::new("orders.internal.>")?));
```

## Best Practices Demonstrated

1. **Subject Naming**: Consistent hierarchical structure
2. **Correlation Tracking**: Maintain chains for debugging
3. **Pattern Design**: Start specific, generalize as needed
4. **Error Handling**: Validate early and clearly
5. **Performance**: Cache patterns and batch operations

## Integration with Your Application

To use these patterns in your application:

1. Add cim-subject to your dependencies
2. Design your subject hierarchy
3. Implement message handlers
4. Set up permission rules
5. Configure NATS connections
6. Monitor message flows

## Additional Resources

- [User Stories](../USER_STORIES.md) - Comprehensive usage scenarios
- [API Documentation](../doc/08-api-reference.md) - Complete API reference
- [Best Practices](../doc/09-best-practices.md) - Design guidelines