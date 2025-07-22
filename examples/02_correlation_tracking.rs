// Copyright 2025 Cowboy AI, LLC.

//! Correlation and causation tracking example
//! 
//! This example demonstrates how to maintain message correlation across
//! distributed transactions and track causation chains for debugging.

use cim_subject::{MessageIdentity, CorrelationId, CausationId, IdType};
use uuid::Uuid;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct MessageFlow {
    name: String,
    identity: MessageIdentity,
    caused_by: Option<Uuid>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Correlation and Causation Tracking Example ===\n");

    // Simulate a distributed transaction flow
    let mut message_flows: HashMap<Uuid, MessageFlow> = HashMap::new();

    // 1. Customer places an order (root message)
    let order_id = Uuid::new_v4();
    let order_placed = MessageFlow {
        name: "OrderPlaced".to_string(),
        identity: MessageIdentity {
            message_id: IdType::Uuid(order_id),
            correlation_id: CorrelationId(IdType::Uuid(order_id)),
            causation_id: CausationId(IdType::Uuid(order_id)),
        },
        caused_by: None,
    };
    
    println!("1. Customer places order:");
    println!("   Message ID: {}", order_id);
    println!("   Correlation ID: {}", order_placed.identity.correlation_id.0);
    println!("   Causation ID: {}", order_placed.identity.causation_id.0);
    println!("   This is a ROOT message (all IDs are the same)\n");
    
    message_flows.insert(order_id, order_placed.clone());

    // 2. Order service validates and creates order (caused by order placement)
    let order_created_id = Uuid::new_v4();
    let order_created = MessageFlow {
        name: "OrderCreated".to_string(),
        identity: MessageIdentity {
            message_id: IdType::Uuid(order_created_id),
            correlation_id: order_placed.identity.correlation_id.clone(),
            causation_id: CausationId(IdType::Uuid(order_id)),
        },
        caused_by: Some(order_id),
    };
    
    println!("2. Order service creates order:");
    println!("   Message ID: {}", order_created_id);
    println!("   Correlation ID: {}", order_created.identity.correlation_id.0);
    println!("   Causation ID: {}", order_created.identity.causation_id.0);
    println!("   Caused by: OrderPlaced ({})\n", order_id);
    
    message_flows.insert(order_created_id, order_created.clone());

    // 3. Inventory service reserves stock (caused by order creation)
    let stock_reserved_id = Uuid::new_v4();
    let stock_reserved = MessageFlow {
        name: "StockReserved".to_string(),
        identity: MessageIdentity {
            message_id: IdType::Uuid(stock_reserved_id),
            correlation_id: order_placed.identity.correlation_id.clone(),
            causation_id: CausationId(IdType::Uuid(order_created_id)),
        },
        caused_by: Some(order_created_id),
    };
    
    println!("3. Inventory service reserves stock:");
    println!("   Message ID: {}", stock_reserved_id);
    println!("   Correlation ID: {}", stock_reserved.identity.correlation_id.0);
    println!("   Causation ID: {}", stock_reserved.identity.causation_id.0);
    println!("   Caused by: OrderCreated ({})\n", order_created_id);
    
    message_flows.insert(stock_reserved_id, stock_reserved.clone());

    // 4. Payment service processes payment (also caused by order creation)
    let payment_processed_id = Uuid::new_v4();
    let payment_processed = MessageFlow {
        name: "PaymentProcessed".to_string(),
        identity: MessageIdentity {
            message_id: IdType::Uuid(payment_processed_id),
            correlation_id: order_placed.identity.correlation_id.clone(),
            causation_id: CausationId(IdType::Uuid(order_created_id)),
        },
        caused_by: Some(order_created_id),
    };
    
    println!("4. Payment service processes payment:");
    println!("   Message ID: {}", payment_processed_id);
    println!("   Correlation ID: {}", payment_processed.identity.correlation_id.0);
    println!("   Causation ID: {}", payment_processed.identity.causation_id.0);
    println!("   Caused by: OrderCreated ({})\n", order_created_id);
    
    message_flows.insert(payment_processed_id, payment_processed.clone());

    // 5. Notification sent (caused by both stock reservation and payment)
    let notification_sent_id = Uuid::new_v4();
    let notification_sent = MessageFlow {
        name: "NotificationSent".to_string(),
        identity: MessageIdentity {
            message_id: IdType::Uuid(notification_sent_id),
            correlation_id: order_placed.identity.correlation_id.clone(),
            causation_id: CausationId(IdType::Uuid(payment_processed_id)), // Last in chain
        },
        caused_by: Some(payment_processed_id),
    };
    
    println!("5. Notification service sends confirmation:");
    println!("   Message ID: {}", notification_sent_id);
    println!("   Correlation ID: {}", notification_sent.identity.correlation_id.0);
    println!("   Causation ID: {}", notification_sent.identity.causation_id.0);
    println!("   Caused by: PaymentProcessed ({})\n", payment_processed_id);
    
    message_flows.insert(notification_sent_id, notification_sent);

    // Demonstrate correlation tracking
    println!("\n=== Correlation Analysis ===\n");
    println!("All messages in this transaction share correlation ID: {}\n", 
        order_placed.identity.correlation_id.0);
    
    println!("Message flow for this transaction:");
    for (id, flow) in &message_flows {
        if let IdType::Uuid(correlation_uuid) = &flow.identity.correlation_id.0 {
            if correlation_uuid == &order_id {
                println!("  - {} (ID: {})", flow.name, id);
                if let Some(caused_by) = flow.caused_by {
                    if let Some(parent) = message_flows.get(&caused_by) {
                        println!("    └─ caused by: {}", parent.name);
                    }
                }
            }
        }
    }

    // Demonstrate causation chain tracing
    println!("\n=== Causation Chain Tracing ===\n");
    println!("Tracing backwards from NotificationSent:");
    
    let mut current_id = Some(notification_sent_id);
    let mut depth = 0;
    
    while let Some(id) = current_id {
        if let Some(flow) = message_flows.get(&id) {
            println!("{}{} ({})", "  ".repeat(depth), flow.name, id);
            current_id = flow.caused_by;
            depth += 1;
        } else {
            break;
        }
    }

    // Demonstrate NATS header mapping
    println!("\n=== NATS Header Mapping ===\n");
    println!("When publishing via NATS, set these headers:");
    println!("  X-Message-ID: {}", notification_sent_id);
    println!("  X-Correlation-ID: {}", order_placed.identity.correlation_id.0);
    println!("  X-Causation-ID: {}", payment_processed_id);

    Ok(())
}