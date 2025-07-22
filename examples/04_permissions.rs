// Copyright 2025 Cowboy AI, LLC.

//! Permission-based access control example
//!
//! This example demonstrates how to implement fine-grained permissions
//! for message routing and access control.

use std::collections::HashMap;

use cim_subject::{
    permissions::{
        Operation,
        Permissions,
        PermissionsBuilder,
        Policy,
    },
    Subject,
};

#[derive(Debug, Clone)]
struct ServicePermissions {
    name: String,
    permissions: Permissions,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Permission-Based Access Control Example ===\n");

    // Define service permissions
    let mut services = HashMap::new();

    // Order Service permissions
    let order_permissions = PermissionsBuilder::new()
        .default_policy(Policy::Deny)
        // Can publish order commands and events
        .allow("orders.commands.>", &[Operation::Publish])?
        .allow("orders.events.>", &[Operation::Publish])?
        // Can subscribe to inventory events
        .allow("inventory.events.>", &[Operation::Subscribe])?
        // Can request from catalog service
        .allow("catalog.queries.>", &[Operation::Request])?
        // Deny all internal subjects
        .deny_all("*.internal.>")?
        .build();

    let order_service = ServicePermissions {
        name: "Order Service".to_string(),
        permissions: order_permissions,
    };

    services.insert("order_service", order_service);

    // Inventory Service permissions
    let inventory_permissions = PermissionsBuilder::new()
        .default_policy(Policy::Deny)
        // Can publish inventory events
        .allow("inventory.events.>", &[Operation::Publish])?
        // Can handle inventory commands
        .allow("inventory.commands.>", &[Operation::Subscribe])?
        // Can subscribe to order events
        .allow("orders.events.>", &[Operation::Subscribe])?
        // Full access to warehouse subjects
        .allow_all("warehouse.>")?
        .build();

    let inventory_service = ServicePermissions {
        name: "Inventory Service".to_string(),
        permissions: inventory_permissions,
    };

    services.insert("inventory_service", inventory_service);

    // Admin Service - has full access
    let admin_permissions = PermissionsBuilder::new()
        .default_policy(Policy::Allow)
        // But still deny access to encryption keys
        .deny_all("security.keys.>")?
        .build();

    let admin_service = ServicePermissions {
        name: "Admin Service".to_string(),
        permissions: admin_permissions,
    };

    services.insert("admin_service", admin_service);

    // Read-only Analytics Service
    let analytics_permissions = PermissionsBuilder::new()
        .default_policy(Policy::Deny)
        // Can only subscribe to events, no publishing
        .allow("*.events.>", &[Operation::Subscribe])?
        // Can query data services
        .allow("*.queries.>", &[Operation::Request])?
        .build();

    let analytics_service = ServicePermissions {
        name: "Analytics Service".to_string(),
        permissions: analytics_permissions,
    };

    services.insert("analytics_service", analytics_service);

    // Test scenarios
    println!("1. Testing Order Service permissions:\n");
    test_service_permissions(&services["order_service"], vec![
        ("orders.commands.order.create", Operation::Publish, true),
        ("orders.events.order.created", Operation::Publish, true),
        (
            "inventory.events.stock.reserved",
            Operation::Subscribe,
            true,
        ),
        (
            "inventory.commands.stock.reserve",
            Operation::Publish,
            false,
        ),
        ("orders.internal.metrics", Operation::Subscribe, false),
        ("catalog.queries.product.get", Operation::Request, true),
    ])?;

    println!("\n2. Testing Inventory Service permissions:\n");
    test_service_permissions(&services["inventory_service"], vec![
        ("inventory.events.stock.updated", Operation::Publish, true),
        (
            "inventory.commands.stock.update",
            Operation::Subscribe,
            true,
        ),
        ("orders.events.order.placed", Operation::Subscribe, true),
        ("orders.commands.order.create", Operation::Publish, false),
        ("warehouse.locations.update", Operation::Publish, true),
        ("warehouse.reports.generate", Operation::Request, true),
    ])?;

    println!("\n3. Testing Admin Service permissions:\n");
    test_service_permissions(&services["admin_service"], vec![
        ("orders.commands.order.create", Operation::Publish, true),
        ("users.commands.user.delete", Operation::Publish, true),
        ("security.keys.master", Operation::Publish, false),
        ("security.keys.encryption", Operation::Subscribe, false),
        ("system.metrics.cpu", Operation::Subscribe, true),
    ])?;

    println!("\n4. Testing Analytics Service permissions:\n");
    test_service_permissions(&services["analytics_service"], vec![
        ("orders.events.order.created", Operation::Subscribe, true),
        (
            "payments.events.payment.processed",
            Operation::Subscribe,
            true,
        ),
        ("orders.commands.order.create", Operation::Publish, false),
        ("reporting.queries.sales.daily", Operation::Request, true),
        ("users.queries.count.active", Operation::Request, true),
    ])?;

    // Demonstrate permission inheritance with patterns
    println!("\n5. Permission inheritance with patterns:\n");

    let subject = Subject::new("orders.events.order.v1")?;

    println!("  Subject: {}", subject.as_str());
    for service in services.values() {
        let can_publish = service.permissions.is_allowed(&subject, Operation::Publish);
        let can_subscribe = service
            .permissions
            .is_allowed(&subject, Operation::Subscribe);

        println!("    {} ->", service.name);
        println!("      Can publish: {}", if can_publish { "✓" } else { "✗" });
        println!(
            "      Can subscribe: {}",
            if can_subscribe { "✓" } else { "✗" }
        );
    }

    // Example 6: Dynamic permission updates
    println!("\n\n6. Dynamic permission updates:\n");

    // Create a mutable service with evolving permissions
    let mut dynamic_service = ServicePermissions {
        name: "Dynamic Service".to_string(),
        permissions: PermissionsBuilder::new()
            .default_policy(Policy::Deny)
            .allow("temp.>", &[Operation::Subscribe])?
            .build(),
    };

    println!("  Initial permissions:");
    test_service_permissions(&dynamic_service, vec![
        ("temp.data.upload", Operation::Subscribe, true),
        ("temp.data.upload", Operation::Publish, false),
    ])?;

    // Grant additional permissions
    dynamic_service.permissions = PermissionsBuilder::new()
        .default_policy(Policy::Deny)
        .allow("temp.>", &[Operation::Subscribe])?
        .allow("temp.data.>", &[Operation::Publish])?
        .build();

    println!("\n  After granting publish permissions:");
    test_service_permissions(&dynamic_service, vec![
        ("temp.data.upload", Operation::Subscribe, true),
        ("temp.data.upload", Operation::Publish, true),
    ])?;

    Ok(())
}

fn test_service_permissions(
    service: &ServicePermissions,
    tests: Vec<(&str, Operation, bool)>,
) -> Result<(), Box<dyn std::error::Error>> {
    for (subject_str, operation, expected) in tests {
        let subject = Subject::new(subject_str)?;
        let allowed = service.permissions.is_allowed(&subject, operation);

        let status = if allowed == expected {
            "✓"
        } else {
            "✗ UNEXPECTED"
        };

        println!(
            "  {} {} on {} -> {} {}",
            match operation {
                Operation::Publish => "PUB",
                Operation::Subscribe => "SUB",
                Operation::Request => "REQ",
                Operation::All => "ALL",
            },
            subject_str,
            if allowed { "allowed " } else { "denied  " },
            status,
            if allowed != expected {
                format!("(expected {})", if expected { "allowed" } else { "denied" })
            } else {
                String::new()
            }
        );
    }

    Ok(())
}
