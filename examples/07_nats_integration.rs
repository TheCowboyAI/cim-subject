// Copyright 2025 Cowboy AI, LLC.

//! NATS integration example
//! 
//! This example demonstrates complete integration with NATS server
//! running on localhost:4222, showing real-world usage patterns.
//!
//! Prerequisites:
//! - NATS server running: nats-server -js
//! - Add to Cargo.toml: async-nats = "0.33", tokio = { version = "1", features = ["full"] }

use cim_subject::{
    Subject, Pattern, MessageIdentity, 
    CorrelationId, IdType, SubjectBuilder,
    permissions::{Permissions, PermissionsBuilder, Operation, Policy}
};
use uuid::Uuid;

// Simulated NATS types (replace with actual async-nats when running)
mod mock_nats {
    use std::collections::HashMap;
    
    pub struct Client;
    pub struct Message {
        pub subject: String,
        pub payload: Vec<u8>,
        pub headers: Option<HeaderMap>,
    }
    
    pub type HeaderMap = HashMap<String, Vec<String>>;
    
    impl Client {
        pub async fn connect(_url: &str) -> Result<Self, Box<dyn std::error::Error>> {
            Ok(Self)
        }
        
        pub async fn publish_with_headers(&self, subject: String, headers: HeaderMap, _payload: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
            println!("  NATS PUB {} with headers {:?}", subject, headers);
            Ok(())
        }
        
        pub async fn subscribe(&self, subject: String) -> Result<Subscription, Box<dyn std::error::Error>> {
            println!("  NATS SUB {}", subject);
            Ok(Subscription { subject })
        }
    }
    
    pub struct Subscription {
        subject: String,
    }
    
    impl Subscription {
        pub async fn next(&mut self) -> Option<Message> {
            // In real implementation, this would receive messages
            // For demo, return a sample message once
            static mut SENT: bool = false;
            unsafe {
                if !SENT && self.subject.contains("orders") {
                    SENT = true;
                    return Some(Message {
                        subject: self.subject.clone(),
                        payload: b"Sample order message".to_vec(),
                        headers: None,
                    });
                }
            }
            None
        }
    }
}

// Service with NATS integration
struct NatsService {
    name: String,
    client: mock_nats::Client,
    permissions: Permissions,
    subscriptions: Vec<Pattern>,
}

impl NatsService {
    async fn new(
        name: impl Into<String>, 
        nats_url: &str,
        permissions: Permissions,
        subscriptions: Vec<Pattern>
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let client = mock_nats::Client::connect(nats_url).await?;
        Ok(Self {
            name: name.into(),
            client,
            permissions,
            subscriptions,
        })
    }
    
    async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Starting {} with subscriptions:", self.name);
        for pattern in &self.subscriptions {
            println!("  - {}", pattern.as_str());
            self.client.subscribe(pattern.as_str().to_string()).await?;
        }
        Ok(())
    }
    
    async fn publish_with_identity(
        &self,
        subject: Subject,
        identity: MessageIdentity,
        payload: Vec<u8>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Check permissions
        if !self.permissions.is_allowed(&subject, Operation::Publish) {
            return Err("Permission denied".into());
        }
        
        // Create headers with correlation info
        let mut headers = mock_nats::HeaderMap::new();
        headers.insert("X-Correlation-ID".to_string(), vec![identity.correlation_id.to_string()]);
        headers.insert("X-Causation-ID".to_string(), vec![identity.causation_id.to_string()]);
        headers.insert("X-Message-ID".to_string(), vec![identity.message_id.to_string()]);
        
        // Publish to NATS
        self.client.publish_with_headers(
            subject.as_str().to_string(),
            headers,
            payload
        ).await?;
        
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== NATS Integration Example ===\n");
    
    let nats_url = "localhost:4222";
    
    // Example 1: Service setup with permissions
    println!("1. Setting up microservices with NATS:\n");
    
    // Order Service
    let order_permissions = PermissionsBuilder::new()
        .default_policy(Policy::Deny)
        .allow("orders.>", &[Operation::Publish, Operation::Subscribe])?
        .allow("inventory.events.>", &[Operation::Subscribe])?
        .allow("payments.commands.>", &[Operation::Publish])?
        .build();
    
    let order_service = NatsService::new(
        "Order Service",
        nats_url,
        order_permissions,
        vec![
            Pattern::new("orders.commands.>")?,
            Pattern::new("inventory.events.>")?,
            Pattern::new("payments.events.>")?,
        ]
    ).await?;
    
    order_service.start().await?;
    
    // Show example of receiving a message
    println!("\n  Example of receiving messages:");
    let mut sub = order_service.client.subscribe("orders.commands.>".to_string()).await?;
    if let Some(msg) = sub.next().await {
        println!("    Received message on {}: {}", msg.subject, String::from_utf8_lossy(&msg.payload));
        if let Some(headers) = &msg.headers {
            println!("    Headers: {} entries", headers.len());
        }
    }
    
    // Example 2: Command handler with event publishing
    println!("\n2. Command handler pattern:\n");
    
    async fn handle_order_command(
        subject: &Subject,
        _payload: &[u8],
        _identity: MessageIdentity,
    ) -> Result<Option<(Subject, Vec<u8>)>, Box<dyn std::error::Error>> {
        let parts: Vec<&str> = subject.as_str().split('.').collect();
        if parts.len() >= 4 && parts[1] == "commands" {
            let action = parts[3];
            
            // Generate event based on command
            let event_subject = SubjectBuilder::new()
                .context("orders")
                .aggregate("events")
                .event_type(match action {
                    "create" => "created",
                    "update" => "updated",
                    "cancel" => "cancelled",
                    _ => return Ok(None),
                })
                .version("v1")
                .build()?;
            
            let response = b"{'status': 'processed'}".to_vec();
            Ok(Some((event_subject, response)))
        } else {
            Ok(None)
        }
    }
    
    // Simulate processing
    let command = Subject::new("orders.commands.order.create")?;
    let command_payload = b"{'id': '123', 'items': [...]}";
    let command_identity = MessageIdentity::root(IdType::Uuid(Uuid::new_v4()));
    
    if let Some((event_subject, event_payload)) = 
        handle_order_command(&command, command_payload, command_identity.clone()).await? {
        
        // Create event identity (caused by command)
        let event_identity = MessageIdentity::caused_by(
            IdType::Uuid(Uuid::new_v4()),
            command_identity.correlation_id.clone(),
            command_identity.message_id.clone(),
        );
        
        println!("  Command: {} → Event: {}", command.as_str(), event_subject.as_str());
        println!("  Correlation maintained: {}", event_identity.correlation_id);
        println!("  Event payload: {}", String::from_utf8_lossy(&event_payload));
    }
    
    // Example 3: Saga orchestration
    println!("\n\n3. Saga orchestration pattern:\n");
    
    struct OrderSaga {
        order_id: Uuid,
        correlation_id: CorrelationId,
        current_step: String,
    }
    
    impl OrderSaga {
        fn new(order_id: Uuid) -> Self {
            Self {
                order_id,
                correlation_id: CorrelationId(IdType::Uuid(order_id)),
                current_step: "started".to_string(),
            }
        }
        
        async fn execute(&mut self, service: &NatsService) -> Result<(), Box<dyn std::error::Error>> {
            let steps = vec![
                ("validate_order", "orders.commands.order.validate"),
                ("reserve_stock", "inventory.commands.stock.reserve"),
                ("process_payment", "payments.commands.payment.process"),
                ("confirm_order", "orders.commands.order.confirm"),
            ];
            
            for (step_name, subject_str) in steps {
                println!("  Step: {}", step_name);
                
                let subject = Subject::new(subject_str)?;
                let step_identity = MessageIdentity::caused_by(
                    IdType::Uuid(Uuid::new_v4()),
                    self.correlation_id.clone(),
                    IdType::Uuid(self.order_id),
                );
                
                service.publish_with_identity(
                    subject,
                    step_identity,
                    format!("{{'order_id': '{}'}}", self.order_id).into_bytes()
                ).await?;
                
                self.current_step = step_name.to_string();
            }
            
            Ok(())
        }
    }
    
    let mut saga = OrderSaga::new(Uuid::new_v4());
    println!("  Starting saga for order: {}", saga.order_id);
    saga.execute(&order_service).await?;
    
    // Example 4: Event sourcing pattern
    println!("\n\n4. Event sourcing with subject streams:\n");
    
    let stream_pattern = Pattern::new("orders.events.>")?;
    println!("  Creating stream for pattern: {}", stream_pattern.as_str());
    
    // In real NATS, you'd create a JetStream with subject filter
    // For this example, we'll simulate event storage
    
    struct EventStore {
        stream_name: String,
        subject_filter: Pattern,
        events: Vec<(Subject, MessageIdentity, Vec<u8>)>,
    }
    
    impl EventStore {
        fn new(stream_name: impl Into<String>, subject_filter: Pattern) -> Self {
            Self {
                stream_name: stream_name.into(),
                subject_filter,
                events: Vec::new(),
            }
        }
        
        fn append(&mut self, subject: Subject, identity: MessageIdentity, payload: Vec<u8>) {
            if self.subject_filter.matches(&subject) {
                self.events.push((subject, identity, payload));
                println!("  Event stored in stream '{}': {} (total: {})", 
                    self.stream_name,
                    self.events.last().unwrap().0.as_str(), 
                    self.events.len()
                );
            }
        }
        
        fn replay_for_correlation(&self, correlation_id: &CorrelationId) -> Vec<&(Subject, MessageIdentity, Vec<u8>)> {
            self.events.iter()
                .filter(|(_, identity, _)| &identity.correlation_id == correlation_id)
                .collect()
        }
    }
    
    let mut event_store = EventStore::new("ORDER_EVENTS", stream_pattern);
    
    // Simulate storing events
    let order_id = Uuid::new_v4();
    let root_identity = MessageIdentity::root(IdType::Uuid(order_id));
    
    let events = vec![
        ("orders.events.order.created", "Order created"),
        ("orders.events.order.validated", "Order validated"),
        ("orders.events.order.confirmed", "Order confirmed"),
    ];
    
    for (subject_str, description) in events {
        let subject = Subject::new(subject_str)?;
        let event_identity = MessageIdentity::caused_by(
            IdType::Uuid(Uuid::new_v4()),
            root_identity.correlation_id.clone(),
            root_identity.message_id.clone(),
        );
        
        event_store.append(
            subject,
            event_identity,
            description.as_bytes().to_vec()
        );
    }
    
    // Replay events for correlation
    println!("\n  Replaying events for correlation: {}", root_identity.correlation_id);
    let replayed = event_store.replay_for_correlation(&root_identity.correlation_id);
    for (subject, _, payload) in replayed {
        println!("    - {}: {}", subject.as_str(), String::from_utf8_lossy(payload));
    }
    
    // Example 5: Request-Reply pattern
    println!("\n\n5. Request-Reply pattern:\n");
    
    let query_subject = Subject::new("catalog.queries.product.get_details")?;
    let reply_inbox = format!("_INBOX.{}", Uuid::new_v4());
    
    println!("  Request: {}", query_subject.as_str());
    println!("  Reply-To: {}", reply_inbox);
    
    // In real NATS, you'd subscribe to reply_inbox before sending
    // Then include reply_inbox in the request headers
    
    let query_identity = MessageIdentity::root(IdType::Uuid(Uuid::new_v4()));
    let query_payload = b"{'product_id': 'ABC123'}";
    
    // Simulate request with reply
    async fn send_request_reply(
        service: &NatsService,
        subject: Subject,
        identity: MessageIdentity,
        payload: Vec<u8>,
        reply_to: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Check permissions
        if !service.permissions.is_allowed(&subject, Operation::Request) {
            return Err("Permission denied for request".into());
        }
        
        let mut headers = mock_nats::HeaderMap::new();
        headers.insert("X-Correlation-ID".to_string(), vec![identity.correlation_id.to_string()]);
        headers.insert("X-Causation-ID".to_string(), vec![identity.causation_id.to_string()]);
        headers.insert("X-Message-ID".to_string(), vec![identity.message_id.to_string()]);
        headers.insert("Reply-To".to_string(), vec![reply_to]);
        
        service.client.publish_with_headers(
            subject.as_str().to_string(),
            headers,
            payload
        ).await?;
        
        Ok(())
    }
    
    // Need to update permissions for request
    let query_permissions = PermissionsBuilder::new()
        .default_policy(Policy::Deny)
        .allow("catalog.queries.>", &[Operation::Request])?
        .build();
    
    let query_service = NatsService::new(
        "Query Service",
        nats_url,
        query_permissions,
        vec![]
    ).await?;
    
    send_request_reply(
        &query_service,
        query_subject,
        query_identity,
        query_payload.to_vec(),
        reply_inbox
    ).await?;
    
    // Example 6: Wildcard subscriptions and routing
    println!("\n\n6. Wildcard subscriptions and routing:\n");
    
    let routing_patterns = vec![
        ("All Commands", Pattern::new("*.commands.>")?),
        ("All Order Events", Pattern::new("orders.events.>")?),
        ("All Created Events", Pattern::new("*.events.*.created")?),
        ("Everything", Pattern::new(">")?),
    ];
    
    let test_subjects = vec![
        "orders.commands.order.create",
        "orders.events.order.created",
        "inventory.events.stock.updated",
        "payments.events.payment.created",
        "users.commands.user.delete",
    ];
    
    for (name, pattern) in &routing_patterns {
        println!("\n  Pattern '{}' ({})", pattern.as_str(), name);
        for subject_str in &test_subjects {
            let subject = Subject::new(*subject_str)?;
            if pattern.matches(&subject) {
                println!("    ✓ {}", subject_str);
            }
        }
    }
    
    println!("\n\nNATS integration example completed!");
    Ok(())
}