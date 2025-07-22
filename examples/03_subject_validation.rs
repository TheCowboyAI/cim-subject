// Copyright 2025 Cowboy AI, LLC.

//! Subject validation and building example
//! 
//! This example shows how to validate subjects according to domain rules
//! and use the builder pattern for safe subject construction.

use cim_subject::{Subject, SubjectParts, SubjectBuilder, SubjectError};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Subject Validation Example ===\n");

    // Example 1: Valid subject construction
    println!("1. Valid subject construction:");
    let valid_subjects = vec![
        "orders.commands.order.create",
        "inventory.events.stock.updated",
        "payments.queries.balance.check",
        "notifications.events.email.sent",
    ];

    for subject_str in valid_subjects {
        match Subject::new(subject_str) {
            Ok(subject) => {
                println!("  ✓ {} is valid", subject.as_str());
                
                // Parse into parts
                match SubjectParts::parse(subject_str) {
                    Ok(parts) => {
                        println!("    - Domain: {}", parts.context);
                        println!("    - Aggregate: {}", parts.aggregate);
                        println!("    - Action: {}", parts.event_type);
                        println!("    - Version: {}", parts.version);
                    }
                    Err(e) => println!("    ✗ Failed to parse: {}", e),
                }
            }
            Err(e) => println!("  ✗ {} is invalid: {}", subject_str, e),
        }
        println!();
    }

    // Example 2: Invalid subjects
    println!("\n2. Invalid subject examples:");
    let invalid_subjects = vec![
        ("", "empty subject"),
        ("orders", "too few parts"),
        ("orders.commands.order", "missing version"),
        ("orders..commands.create", "empty token"),
        ("orders.commands.order.create.extra", "too many parts"),
        ("orders.commands.order.create!", "invalid character"),
        ("Orders.Commands.Order.Create", "uppercase not allowed"),
    ];

    for (subject_str, reason) in invalid_subjects {
        match Subject::new(subject_str) {
            Ok(_) => println!("  ✗ {} should have failed!", subject_str),
            Err(e) => println!("  ✓ {} correctly rejected ({}): {}", subject_str, reason, e),
        }
    }

    // Example 3: Using SubjectBuilder
    println!("\n\n3. Using SubjectBuilder for safe construction:");
    
    // Valid builder usage
    let subject = SubjectBuilder::new()
        .context("orders")
        .aggregate("order")
        .event_type("create")
        .version("v1")
        .build()?;
    
    println!("  Built subject: {}", subject.as_str());

    // Builder with validation
    println!("\n  Testing builder validation:");
    
    let test_cases = vec![
        (
            SubjectBuilder::new()
                .context("orders")
                .aggregate("order")
                .event_type("create")
                // Missing version
                .build(),
            "missing version"
        ),
        (
            SubjectBuilder::new()
                .context("orders!")
                .aggregate("order")
                .event_type("create")
                .version("v1")
                .build(),
            "invalid context characters"
        ),
        (
            SubjectBuilder::new()
                .context("")
                .aggregate("order")
                .event_type("create")
                .version("v1")
                .build(),
            "empty context"
        ),
    ];

    for (result, description) in test_cases {
        match result {
            Ok(_) => println!("    ✗ Builder should have failed for: {}", description),
            Err(e) => println!("    ✓ Builder correctly rejected {}: {}", description, e),
        }
    }

    // Example 4: SubjectParts for structured creation
    println!("\n\n4. Using SubjectParts for structured subjects:");
    
    let parts = SubjectParts::new(
        "inventory",
        "product", 
        "stock_updated",
        "v2"
    );
    
    let subject = Subject::from_parts(parts.clone());
    println!("  Created from parts: {}", subject.as_str());
    println!("  Components:");
    println!("    - Context: {}", parts.context);
    println!("    - Aggregate: {}", parts.aggregate);
    println!("    - Event Type: {}", parts.event_type);
    println!("    - Version: {}", parts.version);

    // Example 5: Custom validation rules
    println!("\n\n5. Implementing custom validation rules:");
    
    fn validate_domain_subject(subject_str: &str) -> Result<Subject, SubjectError> {
        let subject = Subject::new(subject_str)?;
        
        // Custom rule: orders domain can only have specific aggregates
        if subject_str.starts_with("orders.") {
            let parts = SubjectParts::parse(subject_str)?;
            let allowed_aggregates = vec!["order", "cart", "checkout"];
            
            if !allowed_aggregates.contains(&parts.aggregate.as_str()) {
                return Err(SubjectError::invalid_format(
                    format!("Orders domain only allows aggregates: {:?}", allowed_aggregates)
                ));
            }
        }
        
        // Custom rule: version must be v1, v2, etc.
        let parts = SubjectParts::parse(subject_str)?;
        if !parts.version.starts_with('v') || parts.version.len() < 2 {
            return Err(SubjectError::invalid_format(
                "Version must be in format v1, v2, etc.".to_string()
            ));
        }
        
        Ok(subject)
    }
    
    let test_subjects = vec![
        "orders.commands.order.create",
        "orders.commands.product.create", // Invalid aggregate for orders
        "inventory.events.stock.updated",
        "payments.commands.payment.process",
    ];
    
    for subject_str in test_subjects {
        match validate_domain_subject(subject_str) {
            Ok(_) => println!("  ✓ {} passes custom validation", subject_str),
            Err(e) => println!("  ✗ {} fails custom validation: {}", subject_str, e),
        }
    }

    // Example 6: Pattern-based validation
    println!("\n\n6. Pattern-based subject validation:");
    use cim_subject::Pattern;
    
    // Define allowed patterns for different message types
    let command_pattern = Pattern::new("*.commands.*.v*")?;
    let event_pattern = Pattern::new("*.events.*.v*")?;
    let query_pattern = Pattern::new("*.queries.*.v*")?;
    
    let subjects_to_validate = vec![
        "orders.commands.order.v1",
        "orders.events.order.v1",
        "orders.requests.order.v1", // Invalid message type
        "inventory.queries.stock.v2",
    ];
    
    for subject_str in subjects_to_validate {
        if let Ok(subject) = Subject::new(subject_str) {
            let message_type = if command_pattern.matches(&subject) {
                "command"
            } else if event_pattern.matches(&subject) {
                "event"
            } else if query_pattern.matches(&subject) {
                "query"
            } else {
                "unknown"
            };
            
            println!("  {} is a valid {} message", subject_str, message_type);
        }
    }

    Ok(())
}