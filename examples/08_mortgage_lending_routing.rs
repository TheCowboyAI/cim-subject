// Copyright 2025 Cowboy AI, LLC.

//! Private Mortgage Lending - Multi-Broker Application Routing
//!
//! This example demonstrates how to route loan applications from different
//! brokers through appropriate workflows based on broker tier, region, and
//! compliance requirements.

use std::collections::HashMap;

use cim_subject::{
    permissions::{
        Operation,
        Permissions,
        PermissionsBuilder,
        Policy,
    },
    AlgebraOperation,
    Pattern,
    Subject,
    SubjectAlgebra,
    SubjectBuilder,
};

#[derive(Debug, Clone)]
struct BrokerProfile {
    id: String,
    name: String,
    tier: BrokerTier,
    regions: Vec<String>,
    permissions: Permissions,
}

#[derive(Debug, Clone, PartialEq)]
enum BrokerTier {
    Platinum, // Top tier - expedited processing
    Gold,     // Trusted - standard fast-track
    Silver,   // Regular - standard processing
    Bronze,   // New - enhanced validation
}

#[derive(Debug, Clone)]
struct LoanApplication {
    id: String,
    broker_id: String,
    region: String,
    loan_amount: f64,
    property_type: PropertyType,
    ltv_ratio: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum PropertyType {
    SingleFamily,
    MultiFamily,
    Commercial,
    Construction,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Private Mortgage Lending Routing Example ===\n");

    // Set up broker profiles
    let brokers = setup_broker_profiles()?;

    // Example 1: Route applications based on broker tier
    println!("1. Broker Tier-Based Routing:\n");

    let applications = vec![
        LoanApplication {
            id: "LA-001".to_string(),
            broker_id: "BRK-PLAT-001".to_string(),
            region: "southwest".to_string(),
            loan_amount: 2_500_000.0,
            property_type: PropertyType::SingleFamily,
            ltv_ratio: 0.65,
        },
        LoanApplication {
            id: "LA-002".to_string(),
            broker_id: "BRK-GOLD-001".to_string(),
            region: "northeast".to_string(),
            loan_amount: 750_000.0,
            property_type: PropertyType::MultiFamily,
            ltv_ratio: 0.75,
        },
        LoanApplication {
            id: "LA-003".to_string(),
            broker_id: "BRK-BRNZ-001".to_string(),
            region: "midwest".to_string(),
            loan_amount: 500_000.0,
            property_type: PropertyType::Commercial,
            ltv_ratio: 0.80,
        },
    ];

    for app in applications {
        route_application(&app, &brokers)?;
    }

    // Example 2: Multi-broker submission handling
    println!("\n2. Multi-Broker Submission Patterns:\n");

    // Multiple brokers can submit for the same property
    let property_id = "PROP-12345";
    let submissions = vec![
        (
            "BRK-GOLD-001",
            "lending.gold.001.northeast.submissions.property.submit",
        ),
        (
            "BRK-GOLD-002",
            "lending.gold.002.northeast.submissions.property.submit",
        ),
        (
            "BRK-SILV-001",
            "lending.silver.001.northeast.submissions.property.submit",
        ),
    ];

    // Use patterns to detect multiple submissions
    let multi_submit_pattern = Pattern::new("lending.*.*.*.submissions.property.submit")?;

    println!(
        "  Detecting multiple submissions for property {}:",
        property_id
    );
    for (broker_id, subject_str) in submissions {
        let subject = Subject::new(subject_str)?;
        if multi_submit_pattern.matches(&subject) {
            println!("    ✓ {} submitted via {}", broker_id, subject.as_str());
        }
    }

    // Example 3: Regional compliance routing
    println!("\n\n3. Regional Compliance Routing:\n");

    let regional_rules = HashMap::from([
        ("california", vec![
            "lending.compliance.california.disclosure.required",
        ]),
        ("texas", vec!["lending.compliance.texas.homestead.verify"]),
        ("newyork", vec!["lending.compliance.newyork.cema.check"]),
    ]);

    for (region, rules) in regional_rules {
        println!("  Region: {}", region);
        for rule in rules {
            let subject = Subject::new(rule)?;
            println!("    → {}", subject.as_str());
        }
    }

    // Example 4: Document validation workflow
    println!("\n\n4. Document Validation Workflow:\n");

    let doc_workflow = create_document_workflow()?;

    let doc_types = vec![
        "lending.documents.income.w2",
        "lending.documents.income.tax_returns",
        "lending.documents.assets.bank_statements",
        "lending.documents.property.appraisal",
    ];

    for doc_type in doc_types {
        let subject = Subject::new(doc_type)?;
        let validation_subject = doc_workflow.compose(
            &subject,
            &Subject::new("lending.validation.standard.v1")?,
            AlgebraOperation::Sequence,
        )?;
        println!("  {} → {}", doc_type, validation_subject.as_str());
    }

    // Example 5: Priority routing based on loan characteristics
    println!("\n\n5. Priority Routing:\n");

    let priority_rules = vec![
        (2_000_000.0, "high", "lending.priority.high.queue"),
        (1_000_000.0, "medium", "lending.priority.medium.queue"),
        (0.0, "standard", "lending.priority.standard.queue"),
    ];

    let test_amounts = vec![3_000_000.0, 1_500_000.0, 750_000.0];

    for amount in test_amounts {
        for (threshold, priority, queue) in &priority_rules {
            if amount >= *threshold {
                println!("  ${:.2} → {} priority → {}", amount, priority, queue);
                break;
            }
        }
    }

    // Example 6: Automated valuation model (AVM) routing
    println!("\n\n6. AVM Routing by Property Type:\n");

    let avm_routes = HashMap::from([
        (
            PropertyType::SingleFamily,
            "lending.valuation.avm.residential",
        ),
        (
            PropertyType::MultiFamily,
            "lending.valuation.avm.multifamily",
        ),
        (PropertyType::Commercial, "lending.valuation.avm.commercial"),
        (
            PropertyType::Construction,
            "lending.valuation.manual.required",
        ),
    ]);

    for (prop_type, route) in avm_routes {
        println!("  {:?} → {}", prop_type, route);
    }

    // Example 7: Lender tier matching
    println!("\n\n7. Lender Tier Matching:\n");

    // Match brokers to appropriate lender tiers
    let lender_patterns = vec![
        (
            "Tier 1 Lenders",
            Pattern::new("lending.lenders.tier1.>")?,
            vec![BrokerTier::Platinum],
        ),
        (
            "Tier 2 Lenders",
            Pattern::new("lending.lenders.tier2.>")?,
            vec![BrokerTier::Gold, BrokerTier::Silver],
        ),
        (
            "Tier 3 Lenders",
            Pattern::new("lending.lenders.tier3.>")?,
            vec![BrokerTier::Bronze],
        ),
    ];

    for (lender_group, pattern, allowed_tiers) in lender_patterns {
        println!("\n  {} ({})", lender_group, pattern.as_str());
        for tier in allowed_tiers {
            println!("    ✓ {:?} brokers allowed", tier);
        }
    }

    Ok(())
}

fn setup_broker_profiles() -> Result<HashMap<String, BrokerProfile>, Box<dyn std::error::Error>> {
    let mut brokers = HashMap::new();

    // Platinum broker - full permissions
    let platinum_perms = PermissionsBuilder::new()
        .default_policy(Policy::Allow)
        .deny_all("lending.internal.>")?
        .build();

    brokers.insert("BRK-PLAT-001".to_string(), BrokerProfile {
        id: "BRK-PLAT-001".to_string(),
        name: "Premier Mortgage Partners".to_string(),
        tier: BrokerTier::Platinum,
        regions: vec!["southwest".to_string(), "west".to_string()],
        permissions: platinum_perms,
    });

    // Gold broker - standard fast-track
    let gold_perms = PermissionsBuilder::new()
        .default_policy(Policy::Deny)
        .allow("lending.gold.*.*.submissions.>", &[Operation::Publish])?
        .allow("lending.gold.*.*.events.>", &[Operation::Subscribe])?
        .allow("lending.valuation.avm.>", &[Operation::Request])?
        .build();

    brokers.insert("BRK-GOLD-001".to_string(), BrokerProfile {
        id: "BRK-GOLD-001".to_string(),
        name: "Trusted Lending Solutions".to_string(),
        tier: BrokerTier::Gold,
        regions: vec!["northeast".to_string()],
        permissions: gold_perms,
    });

    // Bronze broker - limited permissions
    let bronze_perms = PermissionsBuilder::new()
        .default_policy(Policy::Deny)
        .allow("lending.bronze.*.*.submissions.>", &[Operation::Publish])?
        .allow("lending.bronze.*.*.events.>", &[Operation::Subscribe])?
        .build();

    brokers.insert("BRK-BRNZ-001".to_string(), BrokerProfile {
        id: "BRK-BRNZ-001".to_string(),
        name: "New Horizons Mortgage".to_string(),
        tier: BrokerTier::Bronze,
        regions: vec!["midwest".to_string()],
        permissions: bronze_perms,
    });

    Ok(brokers)
}

fn route_application(
    app: &LoanApplication,
    brokers: &HashMap<String, BrokerProfile>,
) -> Result<(), Box<dyn std::error::Error>> {
    let broker = brokers.get(&app.broker_id).ok_or("Broker not found")?;

    println!(
        "\n  Application {} from {} ({:?})",
        app.id, broker.name, broker.tier
    );
    println!("  Loan Amount: ${:.2}", app.loan_amount);
    println!("  Property Type: {:?}", app.property_type);
    println!("  LTV: {:.0}%", app.ltv_ratio * 100.0);
    println!("  Broker operates in regions: {:?}", broker.regions);

    // Create subject based on broker tier and region
    let subject = SubjectBuilder::new()
        .context(format!("lending.{:?}.{}.{}", broker.tier, broker.id, app.region).to_lowercase())
        .aggregate("applications")
        .event_type("submitted")
        .version("v1")
        .build()?;

    println!("  → Routed to: {}", subject.as_str());

    // Check permissions
    if broker.permissions.is_allowed(&subject, Operation::Publish) {
        println!("  ✓ Broker has permission to submit");
    } else {
        println!("  ✗ Broker lacks permission");
    }

    // Determine workflow based on tier
    let workflow = match broker.tier {
        BrokerTier::Platinum => "express",
        BrokerTier::Gold => "fast-track",
        BrokerTier::Silver => "standard",
        BrokerTier::Bronze => "enhanced-validation",
    };

    println!("  → Workflow: {}", workflow);

    Ok(())
}

fn create_document_workflow() -> Result<SubjectAlgebra, Box<dyn std::error::Error>> {
    let algebra = SubjectAlgebra::new();

    // Register document validation transformations
    algebra.register_transformation("validate_income", cim_subject::algebra::Transformation {
        name: "validate_income".to_string(),
        input_pattern: Pattern::new("lending.documents.income.*")?,
        transform: std::sync::Arc::new(|subject| {
            let parts = cim_subject::SubjectParts::parse(subject.as_str())?;
            let validated = cim_subject::SubjectParts::new(
                "lending",
                "validation",
                format!("{}_validated", parts.event_type),
                "v1",
            );
            Ok(Subject::from_parts(validated))
        }),
    });

    Ok(algebra)
}
