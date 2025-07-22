<!-- Copyright (c) 2025 Cowboy AI, LLC. -->

# CIM-Subject User Stories

This document provides comprehensive user stories that demonstrate the real-world usage of the cim-subject module in distributed domain-driven systems with NATS messaging.

## Prerequisites

- NATS server running on localhost:4222
- Rust environment with cim-subject dependency
- Basic understanding of Domain-Driven Design (DDD) concepts

## User Stories

### 1. As a Microservice Developer, I want to route messages between services using subject patterns

**Scenario**: Building an e-commerce system with separate services for orders, inventory, and notifications.

**Acceptance Criteria**:
- Define subject hierarchy for each service domain
- Route commands to appropriate service handlers
- Broadcast events to interested services
- Use wildcards for flexible subscription patterns

**Example Implementation**:
```rust
use cim_subject::{Subject, Pattern, PatternMatcher};

// Service subscribes to its domain commands
let order_pattern = Pattern::new("orders.commands.>")?;
let inventory_pattern = Pattern::new("inventory.commands.>")?;

// Publish a command
let subject = Subject::new("orders.commands.order.create")?;

// Check routing
assert!(order_pattern.matches(&subject));
assert!(!inventory_pattern.matches(&subject));

// Event broadcasting
let event_subject = Subject::new("orders.events.order.created")?;
let event_pattern = Pattern::new("*.events.order.created")?;
assert!(event_pattern.matches(&event_subject));
```

### 2. As a System Architect, I want to maintain message correlation across distributed transactions

**Scenario**: Implementing a saga pattern where multiple services must coordinate to complete a business transaction.

**Acceptance Criteria**:
- Track correlation IDs across all related messages
- Maintain causation chains for debugging
- Support both synchronous and asynchronous flows
- Enable transaction tracing and monitoring

**Example Implementation**:
```rust
use cim_subject::{MessageIdentity, CorrelationId, CausationId, IdType};
use uuid::Uuid;

// Start a new business transaction
let transaction_id = Uuid::new_v4();
let root_identity = MessageIdentity {
    message_id: IdType::Uuid(transaction_id),
    correlation_id: CorrelationId(IdType::Uuid(transaction_id)),
    causation_id: CausationId(IdType::Uuid(transaction_id)),
};

// Subsequent messages in the transaction
let order_created_id = Uuid::new_v4();
let order_event_identity = MessageIdentity {
    message_id: IdType::Uuid(order_created_id),
    correlation_id: root_identity.correlation_id.clone(),
    causation_id: CausationId(root_identity.message_id.clone()),
};

// Further downstream
let inventory_reserved_id = Uuid::new_v4();
let inventory_identity = MessageIdentity {
    message_id: IdType::Uuid(inventory_reserved_id),
    correlation_id: root_identity.correlation_id.clone(),
    causation_id: CausationId(IdType::Uuid(order_created_id)),
};
```

### 3. As a Domain Expert, I want to enforce business rules through subject validation

**Scenario**: Ensuring that all messages follow domain-specific naming conventions and validation rules.

**Acceptance Criteria**:
- Validate subject structure matches domain standards
- Enforce naming conventions programmatically
- Provide clear error messages for violations
- Support custom validation rules per domain

**Example Implementation**:
```rust
use cim_subject::{Subject, SubjectParts, SubjectBuilder};

// Define domain-specific subject structure
let parts = SubjectParts::new(
    "orders",        // domain
    "order",         // aggregate
    "create",        // action
    "v1"            // version
);

let subject = Subject::from_parts(parts);
assert_eq!(subject.as_str(), "orders.order.create.v1");

// Use builder for validation
let result = SubjectBuilder::new()
    .domain("orders")
    .aggregate("order")
    .action("create")
    .version("v1")
    .build();

assert!(result.is_ok());

// Invalid subjects fail validation
let invalid = Subject::new("invalid..subject");
assert!(invalid.is_err());
```

### 4. As a DevOps Engineer, I want to implement fine-grained permissions for message routing

**Scenario**: Controlling which services can publish or subscribe to specific subject patterns.

**Acceptance Criteria**:
- Define allow/deny rules for subjects
- Support wildcard patterns in permissions
- Apply permissions at runtime
- Log permission violations for security audit

**Example Implementation**:
```rust
use cim_subject::{Subject, Pattern, permissions::{Permission, PermissionSet}};

let mut permissions = PermissionSet::new();

// Allow order service to publish order events
permissions.add(Permission::allow(
    Pattern::new("orders.events.>").unwrap()
));

// Deny access to internal subjects
permissions.add(Permission::deny(
    Pattern::new("*.internal.>").unwrap()
));

// Check permissions
let allowed_subject = Subject::new("orders.events.order.created").unwrap();
let denied_subject = Subject::new("orders.internal.cache.refresh").unwrap();

assert!(permissions.is_allowed(&allowed_subject));
assert!(!permissions.is_allowed(&denied_subject));
```

### 5. As a Data Analyst, I want to query message flows using algebraic operations

**Scenario**: Analyzing message patterns and flows to optimize system performance and identify bottlenecks.

**Acceptance Criteria**:
- Compose complex subject queries
- Apply set operations on subject patterns
- Transform subjects for analysis
- Generate reports on message flow patterns

**Example Implementation**:
```rust
use cim_subject::{Subject, SubjectAlgebra, AlgebraOperation};

let algebra = SubjectAlgebra::new();

// Sequential operation: order must be created before payment
let order_create = Subject::new("orders.commands.order.create").unwrap();
let payment_process = Subject::new("payments.commands.payment.process").unwrap();

let sequential_flow = algebra.sequence(order_create, payment_process);

// Parallel operation: notify multiple services
let email_notify = Subject::new("notifications.commands.email.send").unwrap();
let sms_notify = Subject::new("notifications.commands.sms.send").unwrap();

let parallel_notifications = algebra.parallel(email_notify, sms_notify);

// Choice operation: either credit or debit payment
let credit_payment = Subject::new("payments.commands.credit.charge").unwrap();
let debit_payment = Subject::new("payments.commands.debit.charge").unwrap();

let payment_choice = algebra.choice(credit_payment, debit_payment);
```

### 6. As an Integration Engineer, I want to translate between different subject naming conventions

**Scenario**: Integrating with legacy systems that use different naming conventions while maintaining internal consistency.

**Acceptance Criteria**:
- Define translation rules between naming schemes
- Support bidirectional translation
- Handle version migrations
- Preserve message semantics during translation

**Example Implementation**:
```rust
use cim_subject::{Subject, Pattern, translator::{TranslationRule, Translator}};
use std::sync::Arc;

let mut translator = Translator::new();

// Legacy system uses different format
translator.add_rule(TranslationRule::new(
    "legacy_to_modern",
    Pattern::new("*.*.event").unwrap(),
    Arc::new(|subject| {
        let parts: Vec<&str> = subject.as_str().split('.').collect();
        if parts.len() >= 3 {
            let new_subject = format!("{}.events.{}.v1", parts[0], parts[1]);
            Subject::new(new_subject)
        } else {
            Ok(subject.clone())
        }
    })
));

// Translate legacy format
let legacy = Subject::new("orders.created.event").unwrap();
let modern = translator.translate(&legacy).unwrap();
assert_eq!(modern.as_str(), "orders.events.created.v1");
```

### 7. As a Platform Engineer, I want to implement message routing with NATS integration

**Scenario**: Building a production-ready message routing system using cim-subject with NATS.

**Acceptance Criteria**:
- Connect to NATS server on localhost:4222
- Route messages based on subject patterns
- Handle connection failures gracefully
- Monitor message flow metrics

**Example Implementation**:
```rust
use cim_subject::{Subject, Pattern, MessageIdentity};
use async_nats::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to NATS
    let client = async_nats::connect("localhost:4222").await?;
    
    // Subscribe to order commands
    let order_pattern = Pattern::new("orders.commands.>")?;
    let mut subscriber = client.subscribe("orders.commands.>").await?;
    
    // Publish a command with correlation
    let subject = Subject::new("orders.commands.order.create")?;
    let identity = MessageIdentity::new_root();
    
    let message = async_nats::Message::builder()
        .subject(subject.as_str())
        .header("X-Correlation-ID", identity.correlation_id.to_string())
        .header("X-Causation-ID", identity.causation_id.to_string())
        .payload(b"{'orderId': '12345'}")
        .build();
    
    client.publish_with_headers(
        subject.as_str(),
        message.headers.unwrap(),
        message.payload
    ).await?;
    
    Ok(())
}
```

### 8. As a QA Engineer, I want to test message flows with pattern matching

**Scenario**: Creating comprehensive tests for message routing logic.

**Acceptance Criteria**:
- Test pattern matching accuracy
- Verify wildcard behavior
- Validate permission enforcement
- Ensure correlation tracking works correctly

**Example Implementation**:
```rust
#[cfg(test)]
mod tests {
    use cim_subject::{Subject, Pattern, PatternMatcher};
    
    #[test]
    fn test_service_routing() {
        // Define service patterns
        let patterns = vec![
            ("order_service", Pattern::new("orders.>").unwrap()),
            ("inventory_service", Pattern::new("inventory.>").unwrap()),
            ("notification_service", Pattern::new("*.events.>").unwrap()),
        ];
        
        // Test command routing
        let order_cmd = Subject::new("orders.commands.order.create").unwrap();
        assert!(patterns[0].1.matches(&order_cmd));
        assert!(!patterns[1].1.matches(&order_cmd));
        
        // Test event broadcasting
        let order_event = Subject::new("orders.events.order.created").unwrap();
        assert!(patterns[0].1.matches(&order_event));
        assert!(patterns[2].1.matches(&order_event)); // Notifications subscribe to all events
    }
    
    #[test]
    fn test_pattern_specificity() {
        let specific = Pattern::new("orders.commands.order.create").unwrap();
        let general = Pattern::new("orders.commands.>").unwrap();
        let very_general = Pattern::new("orders.>").unwrap();
        
        assert!(specific.is_more_specific_than(&general));
        assert!(general.is_more_specific_than(&very_general));
    }
}
```

### 9. As a Security Officer, I want to audit message access patterns

**Scenario**: Implementing comprehensive audit logging for compliance and security monitoring.

**Acceptance Criteria**:
- Log all permission checks with outcomes
- Track subject access patterns
- Generate audit reports
- Alert on suspicious patterns

**Example Implementation**:
```rust
use cim_subject::{Subject, permissions::{Permission, PermissionSet}};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Debug)]
struct AuditEntry {
    timestamp: DateTime<Utc>,
    subject: String,
    allowed: bool,
    service: String,
}

struct AuditLog {
    entries: Vec<AuditEntry>,
}

impl AuditLog {
    fn log_access(&mut self, subject: &Subject, allowed: bool, service: &str) {
        self.entries.push(AuditEntry {
            timestamp: Utc::now(),
            subject: subject.as_str().to_string(),
            allowed,
            service: service.to_string(),
        });
    }
    
    fn generate_report(&self) -> HashMap<String, usize> {
        let mut denied_counts = HashMap::new();
        
        for entry in &self.entries {
            if !entry.allowed {
                *denied_counts.entry(entry.service.clone()).or_insert(0) += 1;
            }
        }
        
        denied_counts
    }
}
```

### 10. As a Performance Engineer, I want to optimize message routing performance

**Scenario**: Implementing high-performance message routing with minimal latency.

**Acceptance Criteria**:
- Cache frequently used patterns
- Minimize pattern matching overhead
- Support batch operations
- Profile and optimize hot paths

**Example Implementation**:
```rust
use cim_subject::{Subject, Pattern, PatternMatcher};
use std::sync::Arc;
use dashmap::DashMap;

struct OptimizedRouter {
    pattern_cache: DashMap<String, Arc<Pattern>>,
    match_cache: DashMap<(String, String), bool>,
}

impl OptimizedRouter {
    fn new() -> Self {
        Self {
            pattern_cache: DashMap::new(),
            match_cache: DashMap::with_capacity(10000),
        }
    }
    
    fn get_or_create_pattern(&self, pattern_str: &str) -> Arc<Pattern> {
        self.pattern_cache
            .entry(pattern_str.to_string())
            .or_insert_with(|| Arc::new(Pattern::new(pattern_str).unwrap()))
            .clone()
    }
    
    fn matches(&self, pattern_str: &str, subject: &Subject) -> bool {
        let key = (pattern_str.to_string(), subject.as_str().to_string());
        
        *self.match_cache
            .entry(key)
            .or_insert_with(|| {
                let pattern = self.get_or_create_pattern(pattern_str);
                pattern.matches(subject)
            })
    }
}
```

## Integration Examples

### Complete NATS Integration Example

```rust
use cim_subject::{Subject, Pattern, MessageIdentity, SubjectBuilder};
use async_nats::{Client, Message};
use futures::StreamExt;
use std::sync::Arc;

struct MessageRouter {
    client: Client,
    patterns: Vec<(String, Pattern)>,
}

impl MessageRouter {
    async fn new(nats_url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let client = async_nats::connect(nats_url).await?;
        Ok(Self {
            client,
            patterns: Vec::new(),
        })
    }
    
    fn add_route(&mut self, name: String, pattern: Pattern) {
        self.patterns.push((name, pattern));
    }
    
    async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        for (name, pattern) in &self.patterns {
            let mut subscriber = self.client.subscribe(pattern.as_str()).await?;
            let route_name = name.clone();
            
            tokio::spawn(async move {
                while let Some(message) = subscriber.next().await {
                    println!("Route {} received: {}", route_name, message.subject);
                    
                    // Extract correlation information
                    if let Some(headers) = &message.headers {
                        if let Some(correlation_id) = headers.get("X-Correlation-ID") {
                            println!("  Correlation ID: {}", correlation_id);
                        }
                    }
                }
            });
        }
        
        Ok(())
    }
    
    async fn publish_with_identity(
        &self,
        subject: Subject,
        identity: MessageIdentity,
        payload: Vec<u8>
    ) -> Result<(), Box<dyn std::error::Error>> {
        let headers = async_nats::HeaderMap::from_iter([
            ("X-Message-ID".to_string(), identity.message_id.to_string()),
            ("X-Correlation-ID".to_string(), identity.correlation_id.to_string()),
            ("X-Causation-ID".to_string(), identity.causation_id.to_string()),
        ]);
        
        self.client.publish_with_headers(
            subject.as_str(),
            headers,
            payload.into()
        ).await?;
        
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut router = MessageRouter::new("localhost:4222").await?;
    
    // Configure routes
    router.add_route(
        "order_commands".to_string(),
        Pattern::new("orders.commands.>").unwrap()
    );
    
    router.add_route(
        "all_events".to_string(),
        Pattern::new("*.events.>").unwrap()
    );
    
    // Start routing
    router.start().await?;
    
    // Publish a command
    let subject = SubjectBuilder::new()
        .domain("orders")
        .aggregate("order")
        .action("create")
        .version("v1")
        .build()?;
    
    let identity = MessageIdentity::new_root();
    
    router.publish_with_identity(
        subject,
        identity,
        b"{'orderId': '12345'}".to_vec()
    ).await?;
    
    // Keep running
    tokio::signal::ctrl_c().await?;
    Ok(())
}
```

## Testing Strategy

### Unit Tests
- Test individual subject operations
- Verify pattern matching logic
- Validate permission enforcement
- Check translation accuracy

### Integration Tests
- Test with real NATS server
- Verify end-to-end message flows
- Test failure scenarios
- Measure performance metrics

### Load Tests
- High-volume message routing
- Pattern matching performance
- Cache effectiveness
- Memory usage patterns

## Best Practices

1. **Subject Naming**
   - Use consistent hierarchical structure
   - Include version information
   - Follow domain boundaries
   - Keep subjects readable

2. **Correlation Tracking**
   - Always maintain correlation chains
   - Use root identity for new flows
   - Include causation for debugging
   - Log correlation IDs

3. **Pattern Design**
   - Start specific, generalize as needed
   - Use wildcards judiciously
   - Consider performance implications
   - Document pattern intentions

4. **Error Handling**
   - Validate subjects early
   - Handle pattern compilation errors
   - Log permission violations
   - Implement circuit breakers

5. **Performance**
   - Cache compiled patterns
   - Batch similar operations
   - Monitor pattern complexity
   - Profile hot paths

## Conclusion

These user stories demonstrate the comprehensive capabilities of the cim-subject module in real-world scenarios. From basic routing to complex distributed transactions, the module provides the foundation for building robust, domain-driven messaging systems with NATS.

## Domain-Specific User Stories

### Private Mortgage Lending

The cim-subject module includes comprehensive support for private mortgage lending workflows:

- **[Detailed User Stories](doc/design/cim-subject-user-stories.md)** - Complete mortgage lending scenarios with 5 epics covering:
  - Loan Application Processing (multi-broker routing, document validation, identity verification)
  - Property Valuation and Title Processing (appraisal coordination, title searches)
  - Rate Shopping and Loan Pricing (multi-lender aggregation, dynamic pricing)
  - Underwriting and Decision Engine (automated decisioning, exception handling)
  - Closing and Post-Closing (coordination, quality control)

- **[Workflow Diagrams](doc/design/cim-subject-workflow-diagrams.md)** - Visual representations with Mermaid diagrams

- **Working Examples**:
  - `cargo run --example mortgage_lending_routing` - Multi-broker application routing
  - `cargo run --example document_validation` - Document collection and validation
  - `cargo run --example rate_shopping` - Multi-lender rate shopping

These domain-specific examples demonstrate:
- Broker tier management (VIP, Premium, Standard, New)
- State-specific compliance (NY CEMA, FL Homestead, CA Seismic)
- Document lifecycle management (OCR, validation, expiration tracking)
- Multi-lender rate aggregation and normalization
- Risk-based pricing adjustments
- Complex workflow orchestration with subject algebra

## Additional Resources

- [API Documentation](doc/08-api-reference.md) - Complete API reference
- [Best Practices](doc/09-best-practices.md) - Design guidelines
- [Examples Directory](examples/) - All runnable examples with detailed comments