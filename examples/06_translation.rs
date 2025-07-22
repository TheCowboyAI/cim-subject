// Copyright 2025 Cowboy AI, LLC.

//! Subject translation example
//! 
//! This example shows how to translate between different naming conventions
//! when integrating with legacy systems or external services.

use cim_subject::{Subject, Pattern, translator::{TranslationRule, Translator}};
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Subject Translation Example ===\n");

    // Example 1: Simple version migration
    println!("1. Version Migration Translation\n");
    
    let version_translator = Translator::new();
    
    // Translate v1 subjects to v2
    version_translator.register_rule("v1_to_v2", TranslationRule::new(
        "v1_to_v2",
        Pattern::new("*.*.*.v1")?,
        Arc::new(|subject| {
            let mut new_subject = subject.as_str().to_string();
            new_subject = new_subject.replace(".v1", ".v2");
            Subject::new(new_subject)
        })
    ));
    
    // Test translation
    let v1_subjects = vec![
        "orders.events.order.v1",
        "inventory.commands.stock.v1",
        "payments.queries.balance.v1",
    ];
    
    for subject_str in v1_subjects {
        let v1 = Subject::new(subject_str)?;
        let v2 = version_translator.translate(&v1)?;
        println!("  {} → {}", v1.as_str(), v2.as_str());
    }
    
    // Example 2: Legacy system integration
    println!("\n2. Legacy System Translation\n");
    
    let legacy_translator = Translator::new();
    
    // Map legacy subjects to modern format
    // Legacy format: SERVICE_TYPE_ACTION
    // Modern format: service.type.entity.action
    legacy_translator.register_rule("legacy_to_modern", TranslationRule::new(
        "legacy_to_modern",
        Pattern::new("*_*_*")?,
        Arc::new(|subject| {
            let parts: Vec<&str> = subject.as_str().split('_').collect();
            if parts.len() == 3 {
                let service = parts[0].to_lowercase();
                let msg_type = match parts[1] {
                    "CMD" => "commands",
                    "EVT" => "events",
                    "QRY" => "queries",
                    _ => "unknown",
                };
                let action = parts[2].to_lowercase();
                
                // Infer entity from service
                let entity = match service.as_str() {
                    "orders" => "order",
                    "inventory" => "stock",
                    "payments" => "payment",
                    _ => "entity",
                };
                
                let new_subject = format!("{}.{}.{}.{}", service, msg_type, entity, action);
                Subject::new(new_subject)
            } else {
                Ok(subject.clone())
            }
        })
    ));
    
    // Test legacy translation
    let legacy_subjects = vec![
        "ORDERS_CMD_CREATE",
        "INVENTORY_EVT_UPDATED",
        "PAYMENTS_QRY_STATUS",
    ];
    
    for legacy in legacy_subjects {
        let old = Subject::new(legacy)?;
        let modern = legacy_translator.translate(&old)?;
        println!("  {} → {}", old.as_str(), modern.as_str());
    }
    
    // Example 3: Bidirectional translation
    println!("\n3. Bidirectional Translation\n");
    
    // Create bidirectional rules for system integration
    let forward_rule = TranslationRule::new(
        "internal_to_external",
        Pattern::new("internal.*.*.*")?,
        Arc::new(|subject| {
            let new_str = subject.as_str().replace("internal.", "external.");
            Subject::new(new_str)
        })
    ).with_reverse(Arc::new(|subject| {
        let new_str = subject.as_str().replace("external.", "internal.");
        Subject::new(new_str)
    }));
    
    let bi_translator = Translator::new();
    bi_translator.register_rule("internal_external", forward_rule);
    
    // Test bidirectional translation
    let internal = Subject::new("internal.orders.order.created")?;
    let external = bi_translator.translate(&internal)?;
    let back = bi_translator.reverse_translate(&external)?;
    
    println!("  Internal: {}", internal.as_str());
    println!("  → External: {}", external.as_str());
    println!("  → Back: {}", back.as_str());
    
    // Example 4: Context-aware translation
    println!("\n4. Context-Aware Translation\n");
    
    let context_translator = Translator::new();
    
    // Add tenant context to subjects
    context_translator.register_rule("add_tenant", TranslationRule::new(
        "add_tenant",
        Pattern::new("*.commands.*.*")?,
        Arc::new(|subject| {
            // In real-world, tenant would come from context
            let tenant = "tenant-123";
            let parts = cim_subject::SubjectParts::parse(subject.as_str())?;
            let new_subject = format!("{}.{}.{}.{}.{}", 
                tenant, parts.context, parts.aggregate, parts.event_type, parts.version
            );
            Subject::new(new_subject)
        })
    ));
    
    let commands = vec![
        "orders.commands.order.create",
        "inventory.commands.stock.update",
    ];
    
    for cmd_str in commands {
        let cmd = Subject::new(cmd_str)?;
        let with_tenant = context_translator.translate(&cmd)?;
        println!("  {} → {}", cmd.as_str(), with_tenant.as_str());
    }
    
    // Example 5: Chain translations
    println!("\n5. Translation Pipeline\n");
    
    // Create a pipeline of translators
    let pipeline = Translator::new();
    
    // Step 1: Normalize casing
    pipeline.register_rule("normalize_case", TranslationRule::new(
        "normalize",
        Pattern::new("*.*.*.*")?,
        Arc::new(|subject| {
            let normalized = subject.as_str().to_lowercase();
            Subject::new(normalized)
        })
    ));
    
    // Step 2: Add versioning if missing
    pipeline.register_rule("add_version", TranslationRule::new(
        "versioning",
        Pattern::new("*.*.*")?,
        Arc::new(|subject| {
            // Check if already has version (4 parts)
            let parts: Vec<&str> = subject.as_str().split('.').collect();
            if parts.len() == 3 {
                let versioned = format!("{}.v1", subject.as_str());
                Subject::new(versioned)
            } else {
                Ok(subject.clone())
            }
        })
    ));
    
    // Step 3: Add namespace
    pipeline.register_rule("add_namespace", TranslationRule::new(
        "namespace",
        Pattern::new("*.*.*.*")?,
        Arc::new(|subject| {
            let namespaced = format!("production.{}", subject.as_str());
            Subject::new(namespaced)
        })
    ));
    
    // Test pipeline
    let raw_subjects = vec![
        "ORDERS.Commands.Order.CREATE",
        "inventory.events.stock",
    ];
    
    for raw in raw_subjects {
        let original = Subject::new(raw)?;
        let translated = pipeline.translate(&original)?;
        println!("  {} → {}", original.as_str(), translated.as_str());
    }
    
    // Example 6: Validation during translation
    println!("\n6. Translation with Validation\n");
    
    let validating_translator = Translator::new();
    
    validating_translator.register_rule("validated_translation", TranslationRule::new(
        "validate_and_translate",
        Pattern::new("*.events.*.*")?,
        Arc::new(|subject| {
            let parts = cim_subject::SubjectParts::parse(subject.as_str())?;
            
            // Validate event types
            let valid_events = vec!["created", "updated", "deleted", "published"];
            if !valid_events.contains(&parts.event_type.as_str()) {
                return Err(cim_subject::SubjectError::validation_error(
                    format!("Invalid event type: {}", parts.event_type)
                ));
            }
            
            // Transform to past tense
            let past_tense = match parts.event_type.as_str() {
                "create" => "created",
                "update" => "updated",
                "delete" => "deleted",
                "publish" => "published",
                other => other,
            };
            
            let new_parts = cim_subject::SubjectParts::new(
                parts.context,
                parts.aggregate,
                past_tense,
                parts.version
            );
            
            Ok(Subject::from_parts(new_parts))
        })
    ).with_target_pattern(Pattern::new("*.events.*.*")?));
    
    let events = vec![
        "orders.events.order.created",
        "orders.events.order.create",  // Will be transformed
        "orders.events.order.invalid", // Will fail validation
    ];
    
    for event_str in events {
        match Subject::new(event_str) {
            Ok(event) => {
                match validating_translator.translate(&event) {
                    Ok(translated) => println!("  ✓ {} → {}", event.as_str(), translated.as_str()),
                    Err(e) => println!("  ✗ {} failed: {}", event.as_str(), e),
                }
            }
            Err(e) => println!("  ✗ {} invalid: {}", event_str, e),
        }
    }
    
    Ok(())
}