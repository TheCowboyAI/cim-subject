// Copyright 2025 Cowboy AI, LLC.

//! Basic message routing example demonstrating subject patterns and matching
//!
//! This example shows how to use cim-subject for service-to-service routing
//! in a microservices architecture.

use cim_subject::{
    Pattern,
    Subject,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Basic Message Routing Example ===\n");

    // Define service subscription patterns
    let order_service = Pattern::new("orders.>")?;
    let inventory_service = Pattern::new("inventory.>")?;
    let notification_service = Pattern::new("*.events.>")?;
    let audit_service = Pattern::new(">")?; // Subscribes to everything

    println!("Service subscription patterns:");
    println!("  Order Service: {}", order_service.as_str());
    println!("  Inventory Service: {}", inventory_service.as_str());
    println!("  Notification Service: {}", notification_service.as_str());
    println!("  Audit Service: {}\n", audit_service.as_str());

    // Test command routing
    println!("Testing command routing:");
    let commands = vec![
        "orders.commands.order.create",
        "orders.commands.order.cancel",
        "inventory.commands.stock.reserve",
        "inventory.commands.stock.release",
    ];

    for cmd in commands {
        let subject = Subject::new(cmd)?;
        println!("\n  Command: {}", subject.as_str());

        if order_service.matches(&subject) {
            println!("    ✓ Routed to Order Service");
        }
        if inventory_service.matches(&subject) {
            println!("    ✓ Routed to Inventory Service");
        }
        if notification_service.matches(&subject) {
            println!("    ✓ Routed to Notification Service");
        }
        if audit_service.matches(&subject) {
            println!("    ✓ Routed to Audit Service");
        }
    }

    // Test event broadcasting
    println!("\n\nTesting event broadcasting:");
    let events = vec![
        "orders.events.order.created",
        "orders.events.order.cancelled",
        "inventory.events.stock.reserved",
        "payments.events.payment.processed",
    ];

    for evt in events {
        let subject = Subject::new(evt)?;
        println!("\n  Event: {}", subject.as_str());

        if order_service.matches(&subject) {
            println!("    ✓ Received by Order Service");
        }
        if inventory_service.matches(&subject) {
            println!("    ✓ Received by Inventory Service");
        }
        if notification_service.matches(&subject) {
            println!("    ✓ Received by Notification Service");
        }
        if audit_service.matches(&subject) {
            println!("    ✓ Received by Audit Service");
        }
    }

    // Demonstrate pattern specificity
    println!("\n\nPattern specificity comparison:");
    let p1 = Pattern::new("orders.events.order.created")?;
    let p2 = Pattern::new("orders.events.*.created")?;
    let p3 = Pattern::new("orders.events.>")?;
    let p4 = Pattern::new("*.events.>")?;

    println!(
        "  {} is more specific than {}: {}",
        p1.as_str(),
        p2.as_str(),
        p1.is_more_specific_than(&p2)
    );
    println!(
        "  {} is more specific than {}: {}",
        p2.as_str(),
        p3.as_str(),
        p2.is_more_specific_than(&p3)
    );
    println!(
        "  {} is more specific than {}: {}",
        p3.as_str(),
        p4.as_str(),
        p3.is_more_specific_than(&p4)
    );

    Ok(())
}
