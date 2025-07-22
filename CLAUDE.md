<!-- Copyright (c) 2025 Cowboy AI, LLC. -->

# CLAUDE.md - CIM-Subject Module Guide

## Overview

This document provides guidance for AI assistants working with the `cim-subject` module, which is a core component of the CIM Domain framework hosted in the separate `cim-domain` repository.

## Module Location and Structure

**Important**: `cim-subject` is NOT a local module in this repository. It is part of the `cim-domain` Git dependency:

```toml
cim-subject = { git = "https://github.com/TheCowboyAI/cim-domain.git", branch = "main" }
```

## Core Responsibilities

The `cim-subject` module provides:

1. **Subject Algebra**: Mathematical operations for NATS subject manipulation
2. **Message Identity**: Correlation and causation tracking for distributed systems
3. **Routing Patterns**: Subject-based message routing with wildcards
4. **DDD Support**: Infrastructure for Domain-Driven Design patterns

## Key Concepts to Understand

### 1. Message Identity Trinity

Every message has three IDs that form its identity:
- `message_id`: Unique identifier for this specific message
- `correlation_id`: Groups all messages in a business transaction
- `causation_id`: Points to the message that caused this one

**Rule**: For root messages (start of a flow): `message_id = correlation_id = causation_id`

### 2. Subject Naming Convention

Follow this hierarchical pattern:
```
{domain}.{message_type}.{aggregate}.{action}
```

Examples:
- `location.commands.location.define`
- `workflow.events.execution.started`
- `graph.queries.structure.get_nodes`

### 3. Algebraic Operations

The module supports these operations:
- **Sequential (→)**: A must complete before B
- **Parallel (⊗)**: A and B can run concurrently  
- **Choice (⊕)**: Either A or B (exclusive)
- **Lattice (⊔/⊓)**: Join/meet operations on subject hierarchy

## Common Patterns and Best Practices

### 1. Creating Command Envelopes

Always create proper message identity:

```rust
use cim_subject::{MessageIdentity, IdType, CorrelationId, CausationId};

// For root commands (starting a new flow)
let command_id = CommandId::new();
let id_uuid = *command_id.as_uuid();

let envelope = CommandEnvelope {
    id: command_id,
    identity: MessageIdentity {
        message_id: IdType::Uuid(id_uuid),
        correlation_id: CorrelationId(IdType::Uuid(id_uuid)),
        causation_id: CausationId(IdType::Uuid(id_uuid)),
    },
    command: your_command,
    issued_by: user_id,
};
```

### 2. Publishing Events with Correlation

Always maintain the correlation chain:

```rust
// In command handlers
self.event_publisher.publish_events(
    vec![YourDomainEvent { ... }],
    envelope.identity.correlation_id.clone()  // Pass correlation forward
)
```

### 3. Subject Pattern Matching

Use wildcards appropriately:
- `*` for single token: `domain.events.*.created`
- `>` for multiple tokens: `domain.commands.>`

## Integration Points

### With Domain Modules

Each domain module (location, workflow, graph, etc.) uses cim-subject for:
- Command/event routing
- Cross-domain communication
- Maintaining correlation chains

### With Infrastructure

The `cim-infrastructure` module relies on cim-subject for:
- NATS message headers (X-Correlation-ID, X-Causation-ID)
- Subject-based routing tables
- Message handler registration

## Common Mistakes to Avoid

1. **Breaking Correlation Chains**
   - Always pass `correlation_id` from commands to events
   - Never create new correlation IDs mid-flow

2. **Incorrect Subject Patterns**
   - Follow the naming convention strictly
   - Don't mix command/event/query in subject names

3. **Missing Causation Links**
   - Set `causation_id` to the triggering message's `message_id`
   - Don't reuse the same `causation_id` for multiple effects

4. **Ignoring Algebra Rules**
   - Sequential operations must respect ordering
   - Parallel operations need proper synchronization
   - Choice operations are mutually exclusive

## Testing Considerations

When testing code that uses cim-subject:

1. **Mock Message Identity**: Create test helpers for message identity
2. **Verify Correlation**: Check that correlation IDs flow correctly
3. **Test Routing**: Ensure subjects match expected patterns
4. **Validate Causation**: Confirm causation chains are maintained

## Performance Tips

1. **Subject Caching**: Frequently used subjects can be pre-compiled
2. **Batch Operations**: Group related messages under same correlation
3. **Wildcard Efficiency**: More specific patterns perform better

## Debugging Message Flows

To trace message flows:

1. Start with a `correlation_id` to find all related messages
2. Use `causation_id` to build the causal graph
3. Check subject patterns for routing issues
4. Verify message identity consistency

## Module Dependencies

`cim-subject` is used by:
- All `cim-domain-*` modules for DDD support
- `cim-infrastructure` for NATS integration
- `cim-bridge` for cross-system communication
- Application services for command/event handling

## Future Considerations

The module may evolve to include:
- Subject templates for common patterns
- Built-in subject versioning
- Performance optimizations for pattern matching
- Enhanced debugging capabilities

## When Working with This Module

1. **Read First**: Check existing usage in domain modules
2. **Follow Patterns**: Use established subject hierarchies
3. **Maintain Identity**: Never break correlation/causation chains
4. **Test Thoroughly**: Message flows are critical infrastructure
5. **Document Changes**: Update subject patterns in documentation

Remember: `cim-subject` is the foundation for all message-based communication in the CIM ecosystem. Treat it with appropriate care and attention to maintain system integrity.