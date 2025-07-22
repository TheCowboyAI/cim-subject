// Copyright 2025 Cowboy AI, LLC.

//! Private Mortgage Lending - Multi-Lender Rate Shopping
//!
//! This example demonstrates how to shop rates across multiple lenders,
//! aggregate responses, and manage rate lock workflows.

use std::sync::Arc;

use chrono::{
    DateTime,
    Duration,
    Utc,
};
use cim_subject::{
    permissions::{
        Operation,
        PermissionsBuilder,
        Policy,
    },
    translator::{
        TranslationRule,
        Translator,
    },
    Pattern,
    Subject,
    SubjectAlgebra,
    SubjectBuilder,
};

#[derive(Debug, Clone)]
struct Lender {
    id: String,
    name: String,
    tier: LenderTier,
    products: Vec<LoanProduct>,
    response_pattern: Pattern,
}

#[derive(Debug, Clone, PartialEq)]
enum LenderTier {
    Prime,     // Best rates, strict requirements
    AltA,      // Alternative documentation
    NonQM,     // Non-qualified mortgage
    HardMoney, // Asset-based lending
}

#[derive(Debug, Clone)]
struct LoanProduct {
    name: String,
    min_fico: u32,
    max_ltv: f64,
    property_types: Vec<PropertyType>,
}

#[derive(Debug, Clone, PartialEq)]
enum PropertyType {
    SingleFamily,
    MultiFamily,
    Commercial,
    Construction,
}

#[derive(Debug, Clone)]
struct LoanProfile {
    loan_amount: f64,
    property_value: f64,
    property_type: PropertyType,
    fico_score: u32,
    doc_type: DocType,
}

#[derive(Debug, Clone, PartialEq)]
enum DocType {
    FullDoc,
    BankStatement,
    AssetDepletion,
    NoDoc,
}

#[derive(Debug, Clone)]
struct RateQuote {
    lender_id: String,
    rate: f64,
    apr: f64,
    points: f64,
    fees: f64,
    lock_days: u32,
    quote_id: String,
    expires_at: DateTime<Utc>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Multi-Lender Rate Shopping Example ===\n");

    // Set up lenders
    let lenders = setup_lenders()?;

    // Example 1: Basic rate shopping
    println!("1. Basic Rate Shopping Workflow:\n");

    let loan = LoanProfile {
        loan_amount: 850_000.0,
        property_value: 1_000_000.0,
        property_type: PropertyType::SingleFamily,
        fico_score: 740,
        doc_type: DocType::FullDoc,
    };

    shop_rates(&loan, &lenders)?;

    // Example 2: Lender qualification routing
    println!("\n2. Lender Qualification Routing:\n");

    let test_loans = [LoanProfile {
            loan_amount: 2_500_000.0,
            property_value: 3_000_000.0,
            property_type: PropertyType::SingleFamily,
            fico_score: 800,
            doc_type: DocType::FullDoc,
        },
        LoanProfile {
            loan_amount: 600_000.0,
            property_value: 800_000.0,
            property_type: PropertyType::Commercial,
            fico_score: 680,
            doc_type: DocType::BankStatement,
        },
        LoanProfile {
            loan_amount: 1_200_000.0,
            property_value: 1_500_000.0,
            property_type: PropertyType::Construction,
            fico_score: 720,
            doc_type: DocType::AssetDepletion,
        },
        LoanProfile {
            loan_amount: 400_000.0,
            property_value: 600_000.0,
            property_type: PropertyType::SingleFamily,
            fico_score: 650,
            doc_type: DocType::NoDoc,
        }];

    for (i, loan) in test_loans.iter().enumerate() {
        println!(
            "\n  Loan {}: ${:.0} @ {:.0}% LTV, FICO {}, Doc Type: {:?}",
            i + 1,
            loan.loan_amount,
            (loan.loan_amount / loan.property_value * 100.0),
            loan.fico_score,
            loan.doc_type
        );

        let qualified = qualify_lenders(&lenders, loan);
        println!("  Qualified lenders: {}", qualified.len());
        for lender in qualified {
            println!("    - {} ({:?})", lender.name, lender.tier);
            // Show the products that qualify
            for product in &lender.products {
                println!("      Product: {}", product.name);
            }
        }
    }

    // Example 3: Rate request broadcasting
    println!("\n\n3. Rate Request Broadcasting:\n");

    let rate_request = SubjectBuilder::new()
        .context("lending.rates")
        .aggregate("requests")
        .event_type("quote_requested")
        .version("v1")
        .build()?;

    println!("  Broadcasting: {}", rate_request.as_str());

    // Show lender-specific response patterns
    println!("\n  Lender response patterns:");
    for lender in &lenders {
        println!(
            "    {} ({:?}) → {}",
            lender.name,
            lender.tier,
            lender.response_pattern.as_str()
        );
    }

    // Example 4: Response aggregation
    println!("\n\n4. Rate Response Aggregation:\n");

    let quotes = simulate_rate_responses(&loan, &lenders)?;

    let mut sorted_quotes = quotes.clone();
    sorted_quotes.sort_by(|a, b| a.rate.partial_cmp(&b.rate).unwrap());

    println!("  Received {} quotes:", quotes.len());
    for (i, quote) in sorted_quotes.iter().take(3).enumerate() {
        let lender = lenders.iter().find(|l| l.id == quote.lender_id).unwrap();
        println!("\n  #{} {} ({})", i + 1, lender.name, quote.lender_id);
        println!("  Rate: {:.3}%", quote.rate);
        println!("  APR: {:.3}%, Fees: ${:.0}", quote.apr, quote.fees);
        println!(
            "  Lock: {} days, Expires: {}",
            quote.lock_days,
            quote.expires_at.format("%Y-%m-%d %H:%M:%S")
        );
    }

    // Example 5: Rate lock workflow
    println!("\n\n5. Rate Lock Workflow:\n");

    if let Some(best_quote) = sorted_quotes.first() {
        initiate_rate_lock(best_quote, &loan)?;
    }

    // Example 6: Lender response translation
    println!("\n\n6. Lender Response Translation:\n");

    let response_translator = create_response_translator()?;

    let lender_responses = vec![
        "lenders.prime.bank1.rates.quoted",
        "lenders.nonqm.fund1.pricing.provided",
        "lenders.alta.lender1.quote.generated",
    ];

    for response in lender_responses {
        let subject = Subject::new(response)?;
        let normalized = response_translator.translate(&subject)?;
        println!("  {} → {}", response, normalized.as_str());
    }

    // Example 7: Multi-tier rate shopping strategy
    println!("\n\n7. Multi-Tier Shopping Strategy:\n");

    let shopping_strategy = vec![
        (
            "Tier 1: Prime Lenders",
            Pattern::new("lending.lenders.prime.>")?,
            5,
        ),
        (
            "Tier 2: Alt-A Lenders",
            Pattern::new("lending.lenders.alta.>")?,
            10,
        ),
        (
            "Tier 3: Non-QM Lenders",
            Pattern::new("lending.lenders.nonqm.>")?,
            15,
        ),
    ];

    for (tier, pattern, timeout_seconds) in shopping_strategy {
        println!("  {tier} (timeout: {timeout_seconds}s)");
        println!("    Pattern: {}", pattern.as_str());
    }

    Ok(())
}

fn setup_lenders() -> Result<Vec<Lender>, Box<dyn std::error::Error>> {
    let lenders = vec![
        Lender {
            id: "PRIME-001".to_string(),
            name: "First National Bank".to_string(),
            tier: LenderTier::Prime,
            products: vec![
                LoanProduct {
                    name: "Conforming 30-Year".to_string(),
                    min_fico: 720,
                    max_ltv: 0.80,
                    property_types: vec![PropertyType::SingleFamily],
                },
                LoanProduct {
                    name: "Jumbo 30-Year".to_string(),
                    min_fico: 740,
                    max_ltv: 0.75,
                    property_types: vec![PropertyType::SingleFamily],
                },
            ],
            response_pattern: Pattern::new("lending.rates.responses.prime.001.>")?,
        },
        Lender {
            id: "ALTA-001".to_string(),
            name: "Alternative Lending Corp".to_string(),
            tier: LenderTier::AltA,
            products: vec![LoanProduct {
                name: "Bank Statement 30-Year".to_string(),
                min_fico: 680,
                max_ltv: 0.75,
                property_types: vec![PropertyType::SingleFamily, PropertyType::MultiFamily],
            }],
            response_pattern: Pattern::new("lending.rates.responses.alta.001.>")?,
        },
        Lender {
            id: "NONQM-001".to_string(),
            name: "Non-QM Capital".to_string(),
            tier: LenderTier::NonQM,
            products: vec![LoanProduct {
                name: "Asset Depletion".to_string(),
                min_fico: 660,
                max_ltv: 0.70,
                property_types: vec![
                    PropertyType::SingleFamily,
                    PropertyType::MultiFamily,
                    PropertyType::Commercial,
                ],
            }],
            response_pattern: Pattern::new("lending.rates.responses.nonqm.001.>")?,
        },
        Lender {
            id: "HARD-001".to_string(),
            name: "Fast Capital LLC".to_string(),
            tier: LenderTier::HardMoney,
            products: vec![LoanProduct {
                name: "Bridge Loan".to_string(),
                min_fico: 600,
                max_ltv: 0.65,
                property_types: vec![
                    PropertyType::SingleFamily,
                    PropertyType::MultiFamily,
                    PropertyType::Commercial,
                    PropertyType::Construction,
                ],
            }],
            response_pattern: Pattern::new("lending.rates.responses.hardmoney.001.>")?,
        },
    ];

    Ok(lenders)
}

fn shop_rates(loan: &LoanProfile, lenders: &[Lender]) -> Result<(), Box<dyn std::error::Error>> {
    println!("  Shopping rates for:");
    println!("    Amount: ${:.0}", loan.loan_amount);
    println!(
        "    LTV: {:.0}%",
        (loan.loan_amount / loan.property_value * 100.0)
    );
    println!("    FICO: {}", loan.fico_score);
    println!("    Doc Type: {:?}", loan.doc_type);

    // Create rate request subject for each qualified lender
    let qualified = qualify_lenders(lenders, loan);

    println!("\n  Sending requests to {} lenders:", qualified.len());

    for lender in qualified {
        let request_subject = SubjectBuilder::new()
            .context("lending")
            .aggregate("rates")
            .event_type(format!("request_{}", lender.id.to_lowercase()))
            .version("v1")
            .build()?;

        println!("    → {}: {}", lender.name, request_subject.as_str());
    }

    Ok(())
}

fn qualify_lenders<'a>(lenders: &'a [Lender], loan: &LoanProfile) -> Vec<&'a Lender> {
    let ltv = loan.loan_amount / loan.property_value;

    lenders
        .iter()
        .filter(|lender| {
            lender.products.iter().any(|product| {
                loan.fico_score >= product.min_fico
                    && ltv <= product.max_ltv
                    && product.property_types.contains(&loan.property_type)
            })
        })
        .collect()
}

fn simulate_rate_responses(
    loan: &LoanProfile,
    lenders: &[Lender],
) -> Result<Vec<RateQuote>, Box<dyn std::error::Error>> {
    let mut quotes = Vec::new();
    let base_rate = 7.0; // Current market base

    for lender in qualify_lenders(lenders, loan) {
        let rate_adjustment = match lender.tier {
            LenderTier::Prime => 0.0,
            LenderTier::AltA => 0.5,
            LenderTier::NonQM => 1.5,
            LenderTier::HardMoney => 3.0,
        };

        let fico_adjustment = if loan.fico_score >= 760 {
            -0.25
        } else if loan.fico_score >= 720 {
            0.0
        } else if loan.fico_score >= 680 {
            0.25
        } else {
            0.5
        };

        let rate = base_rate + rate_adjustment + fico_adjustment;

        quotes.push(RateQuote {
            lender_id: lender.id.clone(),
            rate,
            apr: rate + 0.125,
            points: if lender.tier == LenderTier::Prime {
                0.0
            } else {
                1.0
            },
            fees: match lender.tier {
                LenderTier::Prime => 2_500.0,
                LenderTier::AltA => 3_500.0,
                LenderTier::NonQM => 5_000.0,
                LenderTier::HardMoney => 10_000.0,
            },
            lock_days: 30,
            quote_id: format!("Q-{}-{}", lender.id, Utc::now().timestamp()),
            expires_at: Utc::now() + Duration::hours(24),
        });
    }

    Ok(quotes)
}

fn initiate_rate_lock(
    quote: &RateQuote,
    loan: &LoanProfile,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("  Initiating rate lock:");
    println!("  Quote ID: {}", quote.quote_id);
    println!("  Rate: {:.3}%", quote.rate);
    println!(
        "  Points: {} (${:.0})",
        quote.points,
        loan.loan_amount * quote.points / 100.0
    );
    println!("  Lock Period: {} days", quote.lock_days);
    println!(
        "  Total Fees: ${:.0}",
        quote.fees + (loan.loan_amount * quote.points / 100.0)
    );

    // Create rate lock request
    let lock_request = SubjectBuilder::new()
        .context("lending")
        .aggregate("locks")
        .event_type("requested")
        .version("v1")
        .build()?;

    println!("\n  Lock Request: {}", lock_request.as_str());

    // Create permissions for lock operations
    let lock_permissions = PermissionsBuilder::new()
        .default_policy(Policy::Deny)
        .allow("lending.locks.>", &[
            Operation::Publish,
            Operation::Subscribe,
        ])?
        .allow("lending.lenders.*.locks.>", &[Operation::Request])?
        .build();

    // Verify lock permissions
    let lock_subject = Subject::new("lending.locks.confirm")?;
    if lock_permissions.is_allowed(&lock_subject, Operation::Publish) {
        println!("  ✓ Lock operations permitted");
    }

    // Simulate lock workflow
    let algebra = SubjectAlgebra::new();

    // Register lock workflow transformations
    algebra.register_transformation("lock_verification", cim_subject::algebra::Transformation {
        name: "lock_verification".to_string(),
        input_pattern: Pattern::new("lending.locks.requested")?,
        transform: Arc::new(|subject| {
            let parts = cim_subject::SubjectParts::parse(subject.as_str())?;
            let verified = cim_subject::SubjectParts::new(
                parts.context,
                "verification",
                "quote_validity",
                "v1",
            );
            Ok(Subject::from_parts(verified))
        }),
    });

    println!("  Lock workflow initiated with proper permissions");

    Ok(())
}

fn create_response_translator() -> Result<Translator, Box<dyn std::error::Error>> {
    let translator = Translator::new();

    // Normalize different lender response formats
    translator.register_rule(
        "prime_normalize",
        TranslationRule::new(
            "prime_normalize",
            Pattern::new("lenders.prime.*.rates.quoted")?,
            Arc::new(|subject| {
                let parts: Vec<&str> = subject.as_str().split('.').collect();
                let normalized = format!("lending.rates.responses.{}.quoted", parts[2]);
                Subject::new(normalized)
            }),
        ),
    );

    translator.register_rule(
        "nonqm_normalize",
        TranslationRule::new(
            "nonqm_normalize",
            Pattern::new("lenders.nonqm.*.pricing.provided")?,
            Arc::new(|subject| {
                let parts: Vec<&str> = subject.as_str().split('.').collect();
                let normalized = format!("lending.rates.responses.{}.quoted", parts[2]);
                Subject::new(normalized)
            }),
        ),
    );

    translator.register_rule(
        "alta_normalize",
        TranslationRule::new(
            "alta_normalize",
            Pattern::new("lenders.alta.*.quote.generated")?,
            Arc::new(|subject| {
                let parts: Vec<&str> = subject.as_str().split('.').collect();
                let normalized = format!("lending.rates.responses.{}.quoted", parts[2]);
                Subject::new(normalized)
            }),
        ),
    );

    Ok(translator)
}
