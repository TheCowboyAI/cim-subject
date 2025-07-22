// Copyright 2025 Cowboy AI, LLC.

//! Subject algebra operations example
//!
//! This example demonstrates algebraic operations for composing and
//! transforming subjects.

use std::sync::Arc;

use cim_subject::{
    algebra::{
        TransformFn,
        Transformation,
    },
    AlgebraOperation,
    CompositionRule,
    Pattern,
    Subject,
    SubjectAlgebra,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Subject Algebra Operations Example ===\n");

    // Create algebra instance
    let algebra = SubjectAlgebra::new();

    // Register custom composition rules
    register_custom_rules(&algebra)?;

    // Example 1: Sequential composition
    println!("1. Sequential composition (→):\n");

    let validate = Subject::new("orders.commands.order.validate")?;
    let process = Subject::new("orders.commands.order.process")?;
    let _notify = Subject::new("notifications.commands.email.send")?;
    let _bill = Subject::new("billing.commands.invoice.create")?;

    // Compose a sequence using the compose method
    let sequence = algebra.compose(&validate, &process, AlgebraOperation::Sequence)?;
    println!(
        "  {} → {} = {}",
        validate.as_str(),
        process.as_str(),
        sequence.as_str()
    );

    // Example 2: Parallel composition
    println!("\n2. Parallel composition (⊗):\n");

    let send_email = Subject::new("notifications.commands.email.send")?;
    let send_sms = Subject::new("notifications.commands.sms.send")?;
    let _send_push = Subject::new("notifications.commands.push.send")?;

    let parallel = algebra.compose(&send_email, &send_sms, AlgebraOperation::Parallel)?;
    println!(
        "  {} ⊗ {} = {}",
        send_email.as_str(),
        send_sms.as_str(),
        parallel.as_str()
    );

    // Example 3: Choice composition
    println!("\n3. Choice composition (⊕):\n");

    let credit_payment = Subject::new("payments.commands.credit.charge")?;
    let debit_payment = Subject::new("payments.commands.debit.charge")?;
    let _crypto_payment = Subject::new("payments.commands.crypto.charge")?;

    let choice = algebra.compose(&credit_payment, &debit_payment, AlgebraOperation::Choice {
        condition: "payment_method == 'credit'".to_string(),
    })?;
    println!(
        "  {} ⊕ {} (when payment_method == 'credit') = {}",
        credit_payment.as_str(),
        debit_payment.as_str(),
        choice.as_str()
    );

    // Example 4: Transform operation
    println!("\n4. Transform operations:\n");

    // Register a version upgrade transformation
    algebra.register_transformation("upgrade_v1_to_v2", Transformation {
        name: "upgrade_v1_to_v2".to_string(),
        input_pattern: Pattern::new("*.*.*.v1")?,
        transform: Arc::new(|subject: &Subject| Ok(subject.with_version("v2"))),
    });

    let v1_subject = Subject::new("orders.events.order.v1")?;
    let v2_subject = algebra.compose(
        &v1_subject,
        &v1_subject, // second param ignored for transform
        AlgebraOperation::Transform {
            name: "upgrade_v1_to_v2".to_string(),
        },
    )?;

    println!(
        "  Transform {} → {}",
        v1_subject.as_str(),
        v2_subject.as_str()
    );

    // Example 5: Project operation
    println!("\n5. Project operations:\n");

    let full_event = Subject::new("orders.events.order.v1")?;
    let projected = algebra.compose(
        &full_event,
        &full_event, // second param ignored for project
        AlgebraOperation::Project {
            fields: vec!["context".to_string(), "event_type".to_string()],
        },
    )?;

    println!(
        "  Project {} with [context, event_type] = {}",
        full_event.as_str(),
        projected.as_str()
    );

    // Example 6: Inject context
    println!("\n6. Inject context:\n");

    let base_subject = Subject::new("orders.commands.order.v1")?;
    let injected = algebra.compose(
        &base_subject,
        &base_subject, // second param ignored for inject
        AlgebraOperation::Inject {
            context: "tenant-123".to_string(),
        },
    )?;

    println!(
        "  Inject context 'tenant-123' into {} = {}",
        base_subject.as_str(),
        injected.as_str()
    );

    // Example 7: Complex workflow composition
    println!("\n7. Complex workflow composition:\n");

    // Build a complex order processing workflow
    let order_validate = Subject::new("orders.commands.order.validate")?;
    let inventory_check = Subject::new("inventory.queries.stock.check")?;
    let payment_process = Subject::new("payments.commands.payment.process")?;
    let order_confirm = Subject::new("orders.events.order.confirmed")?;

    // First: validate and check inventory in parallel
    let validation_phase = algebra.compose(
        &order_validate,
        &inventory_check,
        AlgebraOperation::Parallel,
    )?;

    // Then: process payment
    let payment_phase = algebra.compose(
        &validation_phase,
        &payment_process,
        AlgebraOperation::Sequence,
    )?;

    // Finally: confirm order
    let complete_workflow =
        algebra.compose(&payment_phase, &order_confirm, AlgebraOperation::Sequence)?;

    println!("  Complete workflow: {}", complete_workflow.as_str());
    println!("  Steps:");
    println!(
        "    1. ({} ⊗ {})",
        order_validate.as_str(),
        inventory_check.as_str()
    );
    println!("    2. → {}", payment_process.as_str());
    println!("    3. → {}", order_confirm.as_str());

    // Example 8: Lattice operations
    println!("\n8. Lattice operations:\n");

    let subjects = vec![
        Subject::new("orders.events.order.v1")?,
        Subject::new("orders.events.order.v2")?,
        Subject::new("orders.commands.order.v1")?,
        Subject::new("inventory.events.stock.v1")?,
    ];

    let lattice = algebra.create_lattice(&subjects);

    println!("  Lattice created with {} subjects", subjects.len());
    println!("  Join operations:");

    for i in 0..subjects.len() {
        for j in i + 1..subjects.len() {
            if let Some(joined) = lattice.join(&subjects[i], &subjects[j]) {
                println!(
                    "    {} ⊔ {} = {}",
                    subjects[i].as_str(),
                    subjects[j].as_str(),
                    joined.as_str()
                );
            }
        }
    }

    Ok(())
}

fn register_custom_rules(algebra: &SubjectAlgebra) -> Result<(), Box<dyn std::error::Error>> {
    // Register a custom rule for order->payment sequence
    let order_payment_rule = CompositionRule {
        name: "order_to_payment".to_string(),
        left_pattern: Pattern::new("orders.*.order.*")?,
        right_pattern: Pattern::new("payments.*.payment.*")?,
        composer: Arc::new(|left, right| {
            // Custom logic for composing order and payment subjects
            let parts = cim_subject::SubjectParts::new(
                "workflow",
                "order_payment",
                format!("{}_then_{}", left.event_type(), right.event_type()),
                "v1",
            );
            Ok(Subject::from_parts(parts))
        }),
    };

    algebra.register_rule("order_payment_sequence", order_payment_rule);

    // Register a custom transformation for adding tenant context
    let tenant_transform = Transformation {
        name: "add_tenant".to_string(),
        input_pattern: Pattern::new("*.*.*.*")?,
        transform: Arc::new(|subject: &Subject| {
            let parts = cim_subject::SubjectParts::new(
                format!("tenant.{}", subject.context()),
                subject.aggregate(),
                subject.event_type(),
                subject.version(),
            );
            Ok(Subject::from_parts(parts))
        }) as TransformFn,
    };

    algebra.register_transformation("add_tenant", tenant_transform);

    Ok(())
}
